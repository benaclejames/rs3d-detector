use ndarray::{array, Array1, Array2, Axis};
use crate::CameraModel::CameraModel;
use crate::observations::{ObservationStorage};
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
        let thing: Array2<f64> = self.sphere_center.to_owned().insert_axis(Axis(0));
        self.corrected_sphere_center = self.refractionizer.correct_sphere_center(thing).row(0).to_owned();
        self.rms_residual = f64::NAN;
    }

    pub fn set_sphere_center(&mut self, from_2d: Option<f64>, prior_3d: Option<f64>) {

    }

    pub fn estimate_sphere_center_2d(&self) {
        
    }
}