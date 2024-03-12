use crate::CameraModel::CameraModel;
use crate::refractionizer::Refractionizer;

pub struct TwoSphereModel {
    pub camera: CameraModel,
    pub refractionizer: Refractionizer,
    pub sphere_center: f64,
    pub corrected_sphere_center: f64,
    pub projected_sphere_center: f64
}

impl TwoSphereModel {
    pub fn new() -> TwoSphereModel {
        TwoSphereModel {
            camera: CameraModel {
                focal_length: 25.0,
                resolution: (25.0, 25.0)
            },
            refractionizer: Refractionizer::new(),
            sphere_center: 25.0,
            corrected_sphere_center: 25.0,
            projected_sphere_center: 25.0
        }
    }

    pub fn set_sphere_center(&mut self, from_2d: Option<f64>, prior_3d: Option<f64>) {

    }

    pub fn estimate_sphere_center_2d(&self) {
        
    }
}