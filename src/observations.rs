use std::iter::zip;
use std::time::SystemTime;
use ndarray::Array1;
use num_traits::ToPrimitive;
use crate::CameraModel::CameraModel;
use crate::primitive::Ellipse;
use crate::projections::unproject_ellipse;

pub struct Observation {
    pub ellipse: Ellipse,
    pub confidence_2d: f64,
    pub confidence: f64,
    pub timestamp: f64,
    pub invalid: bool,
    pub circle_3d_pair: Option<Array1<f64>>,
    pub gaze_3d_pair: Option<Array1<f64>>,
}

pub struct BasicStorage {
    pub storage: Vec<Observation>
}

pub struct BufferedObservationStorage {
    pub confidence_threshold: f64,
    pub storage: Vec<Observation>
}

pub struct BinBufferedObservationStorage {
    pub camera: *const CameraModel,
    pub confidence_threshold: f64,
    pub bin_buffer_length: usize,
    pub forget_min_observations: Option<usize>,
    pub forget_min_time: Option<usize>,
    pub pixels_per_bin: f64,
    pub w: usize,
    pub h: usize,
    pub storage: Vec<Observation>
}

impl Observation {
    pub fn new(ellipse: Ellipse, confidence: f64, timestamp: f64, focal_length: f64, ) -> Observation {
        let mut obs = Observation {
            ellipse,
            confidence_2d: confidence,
            confidence: 0.0,
            timestamp,
            circle_3d_pair: unproject_ellipse(&ellipse, focal_length, 1.0),
        };


    }
}

pub trait ObservationStorage {
    fn add(&mut self, observation: Observation);
    fn observations(&self) -> *const Vec<Observation>;
    fn clear(&mut self);
    fn count(&self) -> usize;
}

impl BasicStorage {
    pub fn new() -> BasicStorage {
        BasicStorage {
            storage: Vec::new()
        }
    }
}

impl ObservationStorage for BasicStorage {
    fn add(&mut self, observation: Observation) {
        if observation.invalid {
            return;
        }

        self.storage.push(observation)
    }

    fn observations(&self) -> *const Vec<Observation> {
        &self.storage
    }

    fn clear(&mut self) {
        self.storage.clear();
    }

    fn count(&self) -> usize {
        self.storage.len()
    }
}

impl BufferedObservationStorage {
    pub fn new(confidence_threshold: f64, buffer_length: usize) -> BufferedObservationStorage {
        BufferedObservationStorage {
            confidence_threshold,
            storage: Vec::with_capacity(buffer_length)
        }
    }
}

impl ObservationStorage for BufferedObservationStorage {
    fn add(&mut self, observation: Observation) {
        if observation.invalid || observation.confidence < self.confidence_threshold {
            return;
        }

        self.storage.push(observation)
    }

    fn observations(&self) -> *const Vec<Observation> {
        &self.storage
    }

    fn clear(&mut self) {
        self.storage.clear()
    }

    fn count(&self) -> usize {
        self.storage.len()
    }
}

impl BinBufferedObservationStorage {
    pub fn new(
        camera: &CameraModel,
        confidence_threshold: f64,
        n_bins_horizontal: usize,
        bin_buffer_length: usize,
        forget_min_observations: Option<usize>,
        forget_min_time: Option<usize>) -> BinBufferedObservationStorage
    {
        let camera_resolution = camera.resolution.to_owned();
        let pixels_per_bin = camera_resolution[0] / n_bins_horizontal as f64;
        BinBufferedObservationStorage {
            camera,
            confidence_threshold,
            bin_buffer_length,
            forget_min_observations,
            forget_min_time,
            pixels_per_bin,
            w: n_bins_horizontal,
            h: (camera_resolution[1] / pixels_per_bin).round().to_usize().unwrap(),
            storage: Vec::new()
        }
    }

    fn get_bin(&self, observation: Observation) {
        let mut ellipse_center = 0;
        let mut resolution = 0;

        //let meme = zip(observation.ellipse.center, &self.camera.resolution);
        println!("aaa")
    }
}

impl ObservationStorage for BinBufferedObservationStorage {
    fn add(&mut self, observation: Observation) {
        if observation.invalid || observation.confidence < self.confidence_threshold {
            return
        }


    }

    fn observations(&self) -> *const Vec<Observation> {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }
}



