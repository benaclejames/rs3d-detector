use ndarray::Array1;

pub struct CameraModel {
    pub focal_length: f64,
    pub resolution: Array1<f64>
}