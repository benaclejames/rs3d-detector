use ndarray::{Array1, ArrayD};
use std::f64::consts::{PI};

pub struct Ellipse {
    pub center: Array1<f64>,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub angle: f64
}

pub struct Line {
    pub origin: ArrayD<f64>,
    pub direction: Array1<f64>,
    pub shape: Vec<usize>
}

pub struct Circle {
    pub center: Array1<f64>,
    pub normal: Array1<f64>,
    pub radius: f64
}

impl Ellipse {
    pub fn new(center: Array1<f64>, minor_radius: f64, major_radius: f64, angle: f64) -> Ellipse {
        let mut ellipse = Ellipse {
            center,
            major_radius,
            minor_radius,
            angle
        };

        if minor_radius > major_radius {
            let current_minor_radius = minor_radius;
            ellipse.minor_radius = major_radius;
            ellipse.major_radius = current_minor_radius;
            ellipse.angle = angle + PI / 2.0;
        }

        ellipse
    }

    pub fn circumference(&self) -> f64 {
        let a = self.minor_radius;
        let b = self.major_radius;
        PI * (3.0 * (a + b) - ((3.0 * a + b) * (a + 3.0 * b)).sqrt())
    }

    pub fn area(&self) -> f64 {
        PI * self.minor_radius * self.major_radius
    }

    pub fn circularity(&self) -> f64 {
        self.minor_radius / self.major_radius
    }
}

impl Line {
    pub fn new(origin: ArrayD<f64>, direction: Array1<f64>) -> Line {
        let shape = origin.shape().to_owned();
        Line {
            origin,
            direction,
            shape
        }
    }
}

impl Circle {
    pub fn new(center: Array1<f64>, normal: Array1<f64>, radius: f64) -> Circle {
        Circle {
            center,
            normal,
            radius
        }
    }

    pub fn spherical_representation(&self) -> (f64, f64, f64) {
        todo!()
    }
}