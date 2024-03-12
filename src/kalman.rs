use opencv::prelude::*;
use opencv::core::{CV_32F, Mat, MatExprResult};

pub struct KalmanFilter {
    pub filter: opencv::video::KalmanFilter,
    pub last_call: i32
}

impl KalmanFilter {
    pub fn new() -> KalmanFilter {
        let mut filter = opencv::video::KalmanFilter::new(7, 3, 0, CV_32F).unwrap();

        let slice: [[f32; 7]; 3] = [
            [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        ];
        filter.set_measurement_matrix(Mat::from_slice_2d(&slice).unwrap());

        match Mat::eye(7, 7, CV_32F).unwrap() * 1e-4 {
            MatExprResult::Ok(expr) => {
                filter.set_process_noise_cov(expr.to_mat().unwrap());
            }
            MatExprResult::Err(_) => {}
        }

        match Mat::eye(3, 3, CV_32F).unwrap() * 1e-5 {
            MatExprResult::Ok(expr) => {
                filter.set_measurement_noise_cov(expr.to_mat().unwrap());
            }
            MatExprResult::Err(_) => {}
        }

        let state_slice: [f32; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0];
        filter.set_state_post(Mat::from_slice(&state_slice).unwrap());

        match Mat::eye(7, 7, CV_32F) {
            Ok(expr) => {
                filter.set_error_cov_post(expr.to_mat().unwrap());
            }
            Err(_) => {}
        }

        KalmanFilter {
            filter,
            last_call: -1
        }
    }

    pub fn predict(&mut self, t: i32) -> (f64, f64, f64) {
        let (phi, theta, pupil_radius);

        if self.last_call != -1 && t > self.last_call {
            let dt: f64 = (t - self.last_call) as f64;
            let slice: [[f64; 7]; 7] = [
                [1.0, 0.0, dt, 0.0, 0.5 * dt * dt, 0.0, 0.0],
                [0.0, 1.0, 0.0, dt, 0.0, 0.5 * dt * dt, 0.0],
                [0.0, 0.0, 1.0, 0.0, dt, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0, 0.0, dt, 0.0],
                [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            ];
            self.filter.set_measurement_matrix(Mat::from_slice_2d(&slice).unwrap());

            let prediction = self.filter.predict_def().unwrap();
            phi = *prediction.at_2d::<f64>(0, 0).unwrap();
            theta = *prediction.at_2d::<f64>(1, 0).unwrap();
            pupil_radius = *prediction.at_2d::<f64>(6, 0).unwrap();
        }
        else {
            (phi, theta, pupil_radius) = (-std::f64::consts::PI / 2.0, std::f64::consts::PI / 2.0, 0.0);
        }

        self.last_call = t;

        return (phi, theta, pupil_radius)
    }

    pub fn correct(&mut self, phi: f64, theta: f64, radius: f64) {
        let slice = [phi, theta, radius];
        self.filter.correct(&Mat::from_slice(&slice).unwrap()).unwrap();
    }
}
