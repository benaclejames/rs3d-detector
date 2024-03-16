use ndarray::{Array1, ArrayD};
use std::f64::consts::{PI};

#[derive(Clone)]
pub struct Ellipse {
    pub center: Array1<f64>,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub angle: f64
}

#[derive(Clone)]
pub struct Line {
    pub origin: Array1<f64>,
    pub direction: Array1<f64>,
    pub shape: Vec<usize>
}

pub struct Circle {
    pub center: Array1<f64>,
    pub normal: Array1<f64>,
    pub radius: f64
}

pub struct Conic {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64
}

pub struct Conicoid {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub f: f64,
    pub g: f64,
    pub h: f64,
    pub u: f64,
    pub v: f64,
    pub w: f64,
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
    pub fn new(origin: Array1<f64>, direction: Array1<f64>) -> Line {
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

impl Conic {
    pub fn new(ellipse: &Ellipse) -> Conic {
        let ax = ellipse.angle.cos();
        let ay = ellipse.angle.sin();
        let a2 = ellipse.major_radius.powi(2);
        let b2 = ellipse.minor_radius.powi(2);

        let a = a2 * ay * ay * b2 * ax * ax;
        let b = 2.0 * (b2 - a2) * ax * ay;
        let c = a2 * ax * ax + b2 * ay * ay;
        Conic {
            a,
            b,
            c,
            d: -2.0 * a * ellipse.center[0] - b * ellipse.center[1],
            e: -b * ellipse.center[0] - 2.0 * c * ellipse.center[1],
            f: a * ellipse.center[0] * ellipse.center[0]
                + b * ellipse.center[0] * ellipse.center[1]
                + c * ellipse.center[1] * ellipse.center[1]
                - a2 * b2
        }
    }

    pub fn discriminant(&self) -> f64 {
        self.b.powi(2) - 4.0 * self.a * self.c
    }
}

impl Conicoid {
    pub fn new(conic: Conic, vertex: Array1<f64>) -> Conicoid {
        let alpha = vertex[0];
        let beta = vertex[1];
        let gamma = vertex[2];

        let a = gamma.powi(2) * conic.a;
        let b = gamma.powi(2) * conic.c;
        let c = conic.a * alpha.powi(2)
            + conic.b * alpha * beta
            + conic.c * beta.powi(2)
            + conic.d * alpha
            + conic.e * beta
            + conic.f;
        let f = -gamma * (conic.c * beta + conic.b / 2.0 * alpha + conic.e / 2.0);
        let g = -gamma * (conic.c / 2.0 * beta + conic.a * alpha + conic.d / 2.0);
        let h = gamma.powi(2) * conic.b / 2.0;
        let u = gamma.powi(2) * conic.d / 2.0;
        let v = gamma.powi(2) * conic.e / 2.0;
        let w = -gamma * (conic.e / 2.0 * beta + conic.d / 2.0 * alpha + conic.f);
        let d = gamma.powi(2) * conic.f;

        Conicoid {
            a,
            b,
            c,
            d,
            f,
            g,
            h,
            u,
            v,
            w
        }
    }
}