use ndarray::array;

mod kalman;
mod refractionizer;
mod two_sphere_model;
mod observations;
mod primitive;
mod projections;
mod utils;
mod CameraModel;
mod Detector3D;

fn main() {
    Detector3D::Detector3D::new(CameraModel::CameraModel {
        focal_length: 25.0,
        resolution: array![25.0, 25.0]
    }, None, None);
}