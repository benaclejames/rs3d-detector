mod utils {
    use ndarray::{Array1, ArrayD};

    fn l2_norm(v: &Array1<f64>) -> f64 {
        v.dot(v).sqrt()
    }

    pub fn cart2sph(x: Array1<f64>) -> (f64, f64) {
        let phi = x[[2]].atan2(x[[0]]);
        let theta = (x[[1]] / l2_norm(&x)).acos();

        (phi, theta)
    }

    pub fn sph2cart(phi: f64, theta: f64) -> Array1<f64> {
        let result = Array1::zeros(3);

        result[[0]] = theta.sin() * phi.cos();
        result[[1]] = theta.cos();
        result[[2]] = theta.sin() * phi.sin();

        result
    }
}