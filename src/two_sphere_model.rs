use ndarray::{array, Array1, Array2, Axis, stack};
use crate::CameraModel::CameraModel;
use crate::observations::{Observation, ObservationStorage};
use crate::refractionizer::Refractionizer;

pub struct TwoSphereModel {
    pub camera: *const CameraModel,
    pub refractionizer: Refractionizer,
    pub storage: Box<dyn ObservationStorage>,
    pub sphere_center: Array1<f64>,
    pub corrected_sphere_center: Array1<f64>,
    pub projected_sphere_center: f64,
    pub rms_residual: f64
}

impl TwoSphereModel {
    pub fn new(camera: *const CameraModel, storage: Box<dyn ObservationStorage>) -> TwoSphereModel {
        let mut model = TwoSphereModel {
            camera,
            storage,
            refractionizer: Refractionizer::new(),
            sphere_center: Array1::zeros(3),
            corrected_sphere_center: Array1::zeros(3),
            projected_sphere_center: 25.0,
            rms_residual: f64::NAN
        };

        model.set_default_model_params();

        model
    }

    fn set_default_model_params(&mut self) {
        self.sphere_center = array!(0.0, 0.0, 35.0);
        self.corrected_sphere_center = self.refractionizer.correct_sphere_center(self.sphere_center.to_owned().insert_axis(Axis(0))).row(0).to_owned();
        self.rms_residual = f64::NAN;
    }

    pub fn add_observation(&mut self, observation: Observation) {
        self.storage.add(observation)
    }

    pub fn set_sphere_center(&mut self, new_sphere_center: Array1<f64>) {
        self.sphere_center = new_sphere_center;
        self.corrected_sphere_center = self.refractionizer.correct_sphere_center(self.sphere_center.to_owned().insert_axis(Axis(0))).row(0).to_owned();
    }

    pub fn estimate_sphere_center(&mut self, from_2d: Option<f64>, prior_3d: Option<f64>, prior_strength: f64, calculate_rms_residual: bool) {
        self.projected_sphere_center = if from_2d.is_some() { from_2d.unwrap() } else { self.estimate_sphere_center_2d() };
        //let spher
    }

    pub fn estimate_sphere_center_2d(&self) -> f64 {
        let observations = self.storage.observations();

        let auxes = observations.iter().map(|x| x.clone().aux_2d.unwrap());

        let mut aux_2d_vec: Vec<Array2<f64>> = Vec::new();

        // Iterate through each observation and push its aux_2d to the aux_2d_vec
        for obs in auxes {
            aux_2d_vec.push(obs.clone());
        }

        // Concatenate all the arrays in aux_2d_vec along the first axis
        let aux_2d = stack!(Axis(0), &aux_2d_vec);

        // Reshape aux_2d to have shape (-1, 2, 3)
        let aux_2d_reshaped = aux_2d.clone().into_shape((aux_2d.len() / 6, 2, 3)).unwrap();






        return 0.0
    }
}