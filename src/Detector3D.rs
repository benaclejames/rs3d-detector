use crate::CameraModel::CameraModel;
use crate::kalman::KalmanFilter;

pub enum DetectorMode {
    Blocking,
    Async
}

pub struct Detector3D {
    pub camera: CameraModel,
    pub threshold_swirski: f32,
    pub threshold_kalman: f32,
    pub threshold_short_term: f32,
    pub threshold_long_term: f32,
    pub long_term_buffer_size: i32,
    pub long_term_forget_time: i32,
    pub long_term_forget_observations: i32,
    pub long_term_mode: DetectorMode,
    pub model_update_interval_long_term: f32,
    pub model_update_interval_ult_long_term: f32,
    pub model_warmup_duration: f32,
    pub calculate_rms_residual: bool,
    pub kalman_filter: Option<KalmanFilter>
}

impl Detector3D {
    pub fn new(
        camera: CameraModel,
        long_term_mode: Option<DetectorMode>,
        calculate_rms_residual: Option<bool>
    ) -> Detector3D {
        Detector3D {
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
        }
    }

    pub fn cleanup_models(&self) {
        //self.threshold_short_term.cleanup()
    }

    pub fn reset(&mut self) {
        self.cleanup_models();
        /*self._initialize_models(
            long_term_model_cls=self._long_term_mode.value,
            ultra_long_term_model_cls=self._long_term_mode.value,
        )
        self._long_term_schedule = _ModelUpdateSchedule(
            update_interval=self._settings["model_update_interval_long_term"],
            warmup_duration=self._settings["model_warmup_duration"],
        )
        self._ult_long_term_schedule = _ModelUpdateSchedule(
            update_interval=self._settings["model_update_interval_ult_long_term"],
            warmup_duration=self._settings["model_warmup_duration"],
        )*/

        self.kalman_filter = Option::from(KalmanFilter::new())
    }
}