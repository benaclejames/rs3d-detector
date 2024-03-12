use ndarray::array;
use crate::two_sphere_model::TwoSphereModel;

mod Detector3D;
mod CameraModel;
mod kalman;
mod refractionizer;
mod two_sphere_model;
mod observations;
mod primitive;
mod projections;
mod utils;

fn main() {
    let a = array!([1, 2, 3], [4, 5, 6], [7,8,9]);
    for row in a.columns() {
        println!("{}", row);
    }

    TwoSphereModel::new();
}