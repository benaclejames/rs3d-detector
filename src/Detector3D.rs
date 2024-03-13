use std::option::Option;
use crate::CameraModel::CameraModel;
use crate::kalman::KalmanFilter;
use crate::observations::{BinBufferedObservationStorage, BufferedObservationStorage};
use crate::two_sphere_model::TwoSphereModel;

#[derive(PartialEq)]
pub enum DetectorMode {
    Blocking,
    Async
}

struct ModelUpdateSchedule {
    update_interval: f64,
    warmup_duration: f64,
    warmup_start: Option<f64>,
    paused: bool,
    last_update: Option<f64>
}

pub struct Detector3D {
    pub camera: CameraModel,
    pub threshold_swirski: f64,
    pub threshold_kalman: f64,
    pub threshold_short_term: f64,
    pub threshold_long_term: f64,
    pub long_term_buffer_size: usize,
    pub long_term_forget_time: usize,
    pub long_term_forget_observations: usize,
    pub long_term_mode: DetectorMode,
    pub model_update_interval_long_term: f64,
    pub model_update_interval_ult_long_term: f64,
    pub model_warmup_duration: f64,
    pub calculate_rms_residual: bool,
    pub kalman_filter: Option<KalmanFilter>,
    pub short_term_model: Option<TwoSphereModel>,
    pub long_term_model: Option<TwoSphereModel>,
    pub ultra_long_term_model: Option<TwoSphereModel>,

    long_term_schedule: Option<ModelUpdateSchedule>,
    ult_long_term_schedule: Option<ModelUpdateSchedule>
}

impl ModelUpdateSchedule {
    pub fn new(update_interval: f64, warmup_duration: f64) -> ModelUpdateSchedule {
        ModelUpdateSchedule {
            update_interval,
            warmup_duration,
            warmup_start: None,
            paused: false,
            last_update: None
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn pause(&mut self) {
        self.paused = true
    }

    pub fn resume(&mut self) {
        self.paused = false
    }

    pub fn update_due(&mut self, current_time: f64) -> bool {
        if self.paused {
            return false
        }
        if self.warmup_start == None {
            self.warmup_start = Some(current_time);
            return true;
        }
        if current_time - self.warmup_start.unwrap() < self.warmup_duration {
            return true;
        }
        if self.last_update == None {
            self.last_update = Some(current_time);
            return true;
        }
        if current_time - self.last_update.unwrap() > self.update_interval {
            self.last_update = Some(current_time);
            return true;
        }
        return false;
    }
}

impl Detector3D {
    pub fn new(
        camera: CameraModel,
        long_term_mode: Option<DetectorMode>,
        calculate_rms_residual: Option<bool>
    ) -> Detector3D {
        let mut detector = Detector3D {
            camera,
            threshold_swirski: 0.7,
            threshold_kalman: 0.98,
            threshold_short_term: 0.8,
            threshold_long_term: 0.98,
            long_term_buffer_size: 30,
            long_term_forget_time: 5,
            long_term_forget_observations: 300,
            long_term_mode: long_term_mode.unwrap_or(DetectorMode::Blocking),
            model_update_interval_long_term: 1.0,
            model_update_interval_ult_long_term: 10.0,
            model_warmup_duration: 5.0,
            calculate_rms_residual: calculate_rms_residual.unwrap_or(false),
            kalman_filter: None,
            short_term_model: None,
            long_term_model: None,
            ultra_long_term_model: None,
            long_term_schedule: None,
            ult_long_term_schedule: None
        };

        detector.reset();

        detector
    }

    pub fn get_camera(&self) -> &CameraModel {
        return &self.camera;
    }

    pub fn get_long_term_mode(&self) -> &DetectorMode {
        return &self.long_term_mode;
    }

    pub fn set_long_term_mode(&mut self, mode: DetectorMode) {
        let needs_reset = mode != self.long_term_mode;
        self.long_term_mode = mode;
        if needs_reset {
            self.reset();
        }
    }

    pub fn get_is_long_term_model_frozen(&self) -> bool {
        match &self.long_term_schedule {
            Some(schedule) => schedule.is_paused(),
            None => false
        }
    }

    pub fn set_is_long_term_model_frozen(&mut self, should_be_frozen: bool) {
        let long_term_schedule = &mut self.long_term_schedule.as_mut().unwrap();
        let ult_long_term_schedule = &mut self.ult_long_term_schedule.as_mut().unwrap();
        if should_be_frozen {
            long_term_schedule.pause();
            ult_long_term_schedule.pause();
        }
        else {
            long_term_schedule.resume();
            ult_long_term_schedule.resume();
        }
    }

    pub fn reset_camera(&mut self, camera: CameraModel) {
        self.camera = camera;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.initialize_models();

        self.long_term_schedule = Some(ModelUpdateSchedule::new(self.model_update_interval_long_term, self.model_warmup_duration));
        self.ult_long_term_schedule = Some(ModelUpdateSchedule::new(self.model_update_interval_ult_long_term, self.model_warmup_duration));

        self.kalman_filter = Option::from(KalmanFilter::new())
    }

    fn initialize_models(&mut self) {
        self.short_term_model = Some(
            TwoSphereModel::new(
                &self.camera,
                Box::new(
                    BufferedObservationStorage::new(
                        self.threshold_short_term,
                        10
                    )
                )
            )
        );

        self.long_term_model = Some(
            TwoSphereModel::new(
                &self.camera,
                Box::new(
                    BinBufferedObservationStorage::new(
                        &self.camera,
                        self.threshold_long_term,
                        10,
                        self.long_term_buffer_size,
                        Some(self.long_term_forget_observations),
                        Some(self.long_term_forget_time)
                    )
                )
            )
        );

        self.ultra_long_term_model = Some(
            TwoSphereModel::new(
                &self.camera,
                Box::new(
                    BinBufferedObservationStorage::new(
                        &self.camera,
                        self.threshold_long_term,
                        10,
                        self.long_term_buffer_size,
                        Some(2 * self.long_term_forget_observations),
                        Some(60)
                    )
                )
            )
        )
    }
}