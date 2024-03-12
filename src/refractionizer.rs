use std::fs::File;
use std::io::{Cursor, Read};
use ndarray::{Array, Array2, ArrayView, Ix2};
use serde_derive::Deserialize;

pub struct Refractionizer {
    pub pipeline_radius_as_list: Steps,
    pub pipeline_gaze_vector_as_list: Steps,
    pub pipeline_sphere_center_as_list: Steps,
    pub pipeline_pupil_circle_as_list: Steps
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LinearRegressionParams {
    pub copy_X: bool,
    pub fit_intercept: bool,
    pub n_jobs: Option<u8>,
    pub normalize: bool,
    pub positive: bool
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LinearRegression {
    pub params: LinearRegressionParams,
    #[serde(with = "serde_ndim")]
    pub coef: Array2<f64>,
    #[serde(with = "serde_ndim")]
    pub intercept: Array2<f64>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct StandardScalerParams {
    pub copy: bool,
    pub with_mean: bool,
    pub with_std: bool
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct StandardScaler {
    pub params: StandardScalerParams,
    #[serde(with = "serde_ndim")]
    pub mean: Array2<f64>,
    #[serde(with = "serde_ndim")]
    pub var: Array2<f64>
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PolynomialParams {
    pub degree: u8,
    pub include_bias: bool,
    pub interaction_only: bool,
    pub order: String
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PolynomialFeatures {
    pub params: PolynomialParams,
    #[serde(with = "serde_ndim")]
    pub powers: Array2<f64>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Steps {
    pub polynomial_features: PolynomialFeatures,
    pub standard_scaler: StandardScaler,
    pub linear_regression: LinearRegression
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Root {
    pub version: u8,
    pub steps: Steps
}

impl Refractionizer {
    pub fn new() -> Refractionizer {
        let degree = 3;
        Refractionizer{
            pipeline_radius_as_list: Self::load_config_from_msgpack("radius", "default", degree, None),
            pipeline_gaze_vector_as_list: Self::load_config_from_msgpack("gaze_vector", "default", degree, None),
            pipeline_sphere_center_as_list: Self::load_config_from_msgpack("sphere_center", "default", degree, None),
            pipeline_pupil_circle_as_list: Self::load_config_from_msgpack("pupil_circle", "default", degree, None)
        }
    }

    pub fn load_config_from_msgpack(feature: &str, type_: &str, degree: i8, custom_load_dir: Option<&str>) -> Steps {
        let resolved_name = format!("{}_refraction_model_{}_degree_{}.msgpack", type_, feature, degree);
        let mut path = "C:\\Users\\Ben\\RustroverProjects\\rs3d-detector\\".to_owned();
        path.push_str(&resolved_name);
        let f = File::open(path);
        let mut buf = Vec::new();
        f.unwrap().read_to_end(&mut buf).unwrap();
        let cursor = Cursor::new(buf);
        let root: Root = rmp_serde::decode::from_read(cursor).unwrap();
        // Ensure version is 1
        root.steps
    }

    fn apply_correction_pipeline(
        x: ArrayView<f64, Ix2>,
        powers: &Array2<f64>,
        mean: &Array2<f64>,
        var: &Array2<f64>,
        coef: &Array2<f64>,
        intercept: &Array2<f64>,
    ) -> Array2<f64> {
        let mut features: Array<f64, _> = Array::<f64, _>::zeros((powers.len(), x.len()));

        for k in 0..x.len() {
            for i in 0..powers.len() {
                features[[i, k]] = 1.0;
                for j in 0..powers.len() {
                    for _ in 0..powers.len() {
                        features[[i, k]] *= x[[k, j]];
                    }
                }
                features[[i, k]] -= mean[[0, i]];
                features[[i, k]] /= var[[0, i]].sqrt();
            }
        }

        ((*&coef * &features).reversed_axes() + &intercept.column(0)).t().to_owned()
    }

    fn _apply_correction_pipeline(x: Array2<f64>, pipeline_arrays: &Steps) -> Array2<f64> {
        Self::apply_correction_pipeline(
            x.t(),
            &pipeline_arrays.polynomial_features.powers,
            &pipeline_arrays.standard_scaler.mean,
            &pipeline_arrays.standard_scaler.var,
            &pipeline_arrays.linear_regression.coef,
            &pipeline_arrays.linear_regression.intercept
        )
    }

    pub fn correct_radius(&self, x: Array2<f64>) -> Array2<f64> {
        Self::_apply_correction_pipeline(x, &self.pipeline_radius_as_list)
    }

    pub fn correct_gaze_vector(&self, x: Array2<f64>) -> Array2<f64> {
        Self::_apply_correction_pipeline(x, &self.pipeline_gaze_vector_as_list)
    }

    pub fn correct_sphere_center(&self, x: Array2<f64>) -> Array2<f64> {
        Self::_apply_correction_pipeline(x, &self.pipeline_sphere_center_as_list)
    }

    pub fn correct_pupil_circle(&self, x: Array2<f64>) -> Array2<f64> {
        Self::_apply_correction_pipeline(x, &self.pipeline_pupil_circle_as_list)
    }
}