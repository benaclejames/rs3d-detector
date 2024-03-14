use std::iter::zip;
use std::time::SystemTime;
use ndarray::{array, Array1, Array2, Array3, Axis, concatenate, s};
use num_traits::ToPrimitive;
use opencv::imgproc::circle;
use crate::CameraModel::CameraModel;
use crate::primitive::{Ellipse, Line};
use crate::projections::{Circle3D, project_line_onto_image_plane, unproject_ellipse};

pub struct Observation {
    pub ellipse: Ellipse,
    pub confidence_2d: f64,
    pub confidence: f64,
    pub timestamp: f64,
    pub invalid: bool,
    pub circle_3d_pair: Option<[Circle3D; 2]>,
    pub gaze_3d_pair: Option<Array1<Line>>,
    pub gaze_2d: Option<Line>,
    pub gaze_2d_line: Option<Array1<f64>>,
    pub aux_2d: Option<Array2<f64>>,
    pub aux_3d: Option<Array3<f64>>
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
        let circle_3d_pair = unproject_ellipse(&ellipse, focal_length, 1.0);
        let mut gaze_3d_pair = None;
        let mut gaze_2d = None;
        let mut gaze_2d_line = None;
        if circle_3d_pair.is_some() {
            let circle = circle_3d_pair.clone().unwrap()[0].clone();
            gaze_3d_pair = Some(array!(Line::new(circle.center, circle.normal)));

            gaze_2d = Some(project_line_onto_image_plane(gaze_3d_pair.clone().unwrap()[0].clone(), focal_length));
            gaze_2d_line = Some(concatenate!(Axis(0), gaze_2d.clone().unwrap().origin, gaze_2d.clone().unwrap().direction));
        }
        let mut aux_2d = Array2::zeros((2, 3));
        let v = gaze_2d.clone().unwrap().direction.clone().into_shape((2, 1)).unwrap();
        let eye = Array2::eye(2);
        let vvt = &v * &v.t();
        let eye_minus_vvt = &eye - &vvt;
        aux_2d.slice_mut(s![.., ..2]).assign(&eye_minus_vvt);
        aux_2d.slice_mut(s![.., 2]).assign(&(&eye_minus_vvt * gaze_2d.clone().unwrap().origin.clone()));

        let mut observation = Observation {
            ellipse,
            confidence_2d: confidence,
            confidence,
            timestamp,
            invalid: true,
            circle_3d_pair,
            gaze_3d_pair,
            gaze_2d,
            gaze_2d_line,
            aux_2d: None,
            aux_3d: None
        };

        let mut aux_3d = Array3::zeros((2, 3, 4));
        for i in 0..2 {
            let dierkes_line = observation.get_dierkes_line(i);
            let v = dierkes_line.direction.to_shape((3, 1)).unwrap();
            let eye = Array2::eye(3);
            let vvt = &v * &v.t();
            let eye_minus_vvt = &eye - &vvt;

            aux_3d.slice_mut(s![i, .., ..3]).assign(&eye_minus_vvt);
            aux_3d.slice_mut(s![i, .., 3]).assign(&(&eye_minus_vvt * dierkes_line.origin.clone()));

        }

        observation.aux_3d = Some(aux_3d.to_owned());

        observation
    }

    pub fn get_dierkes_line(&self, i: usize) -> Line {
        let circle_pair = self.circle_3d_pair.clone().unwrap()[i].clone();
        let direction = circle_pair.center;
        let origin = direction.clone() - 10.392304845413264 * circle_pair.normal;
        Line::new(origin, direction)
    }
}

pub trait ObservationStorage {
    fn add(&mut self, observation: Observation);
    fn observations(&self) -> &Vec<Observation>;
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

    fn observations(&self) -> &Vec<Observation> {
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

    fn observations(&self) -> &Vec<Observation> {
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

    fn observations(&self) -> &Vec<Observation> {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }
}



