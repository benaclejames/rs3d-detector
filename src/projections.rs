use crate::primitive::{Conic, Conicoid};
use ndarray::{array, Array1};
use crate::primitive::Ellipse;

pub struct Circle3D {
    pub center: Array1<f64>,
    pub normal: Array1<f64>,
    pub radius: f64
}

pub fn solve_1(a: f64) -> Result<f64, &'static str> {
    if a == 0.0 {
        return Ok(0.0)
    }
    Err("No Solution")
}

pub fn solve_2(a: f64, b: f64) -> Result<f64, &'static str> {
    if a == 0.0 {
        return solve_1(b)
    }
    Ok(-b / a)
}

pub fn solve_3(a: f64, b: f64, c: f64) -> Result<Array1<f64>, &'static str> {
    if a == 0.0 {
        return match solve_2(b, c) {
            Ok(root) => Ok(array!(root, root)),
            Err(e) => Err(e)
        }
    }

    // http://www.it.uom.gr/teaching/linearalgebra/NumericalRecipiesInC/c5-6.pdf
    // Pg 184
    let det = (b * b) - 4.0 * a * c;

    if det < 0.0 {
        return Err("No solution")
    }

    //auto sqrtdet = sqrt(det);
    let q = -0.5 * (b + (if b >= 0.0 { 1.0 } else { -1.0 }) * det.sqrt());
    return Ok(array!(q / a, c / q));
}

pub fn solve_4(a: f64, b: f64, c: f64, d: f64) -> Result<Array1<f64>, &'static str> {
    if a == 0.0 {
        return match solve_3(b, c, d) {
            Ok(roots) => Ok(array!(roots[0], roots[1], roots[1])),
            Err(e) => Err(e)
        };
    }

    // http://www.it.uom.gr/teaching/linearalgebra/NumericalRecipiesInC/c5-6.pdf
    // http://web.archive.org/web/20120321013251/http://linus.it.uts.edu.au/~don/pubs/solving.html
    let p = b / a;
    let q = c / a;
    let r = d / a;
    //auto Q = (p*p - 3*q) / 9;
    //auto R = (2*p*p*p - 9*p*q + 27*r)/54;
    let u = q - (p * p) / 3.0;
    let v = r - p * q / 3.0 + 2.0 * p * p * p / 27.0;
    let j = 4.0 * u * u * u / 27.0 + v * v;
    let M = f64::MAX;
    let sqrtM = M.sqrt();
    let cbrtM = M.sqrt();

    if b == 0.0 && c == 0.0 {
        return Ok(array!(-d.cbrt(), -d.cbrt(), -d.cbrt()));
    }

    if p.abs() > 27.0 * cbrtM {
        return Ok(array!(-p, -p, -p));
    }

    if q.abs() > sqrtM {
        return Ok(array!(-v.cbrt(), -v.cbrt(), -v.cbrt()));
    }

    if u.abs() > 3.0 * cbrtM / 4.0 {
        return Ok(array!(4.0_f64.cbrt() * u / 3.0, 4.0_f64.cbrt() * u / 3.0, 4.0_f64.cbrt() * u / 3.0));
    }

    return if j > 0.0 {
        // One real root
        let w = j.sqrt();
        let y = if v > 0.0 {
            (u / 3.0) * (2.0 / (w + v)).cbrt() - ((w + v) / 2.0).cbrt() - p / 3.0
        } else {
            ((w - v) / 2.0).cbrt() - (u / 3.0) * (2.0 / (w - v)).cbrt() - p / 3.0
        };

        Ok(array!(y, y, y))
    } else {
        // Three real roots
        let s = (-u / 3.0).sqrt();
        let t = -v / (2.0 * s * s * s);
        let k = t.acos() / 3.0;
        let y1 = 2.0 * s * k.cos() - p / 3.0;
        let y2 = s * (-k.cos() + 3.0_f64.sqrt() * k.sin()) - p / 3.0;
        let y3 = s * (-k.cos() - 3.0_f64.sqrt() * k.sin()) - p / 3.0;
        Ok(array!(y1, y2, y3))
    }
}

pub fn unproject_conicoid(
    a: f64,
    b: f64,
    c: f64,
    f: f64,
    g: f64,
    h: f64,
    u: f64,
    v: f64,
    w: f64,
    focal_length: f64,
    circle_radius: f64,
) {
    let lambda = solve_4(1., -(a + b + c), (b * c + c * a + a * b - f * f - g * g - h * h), -(a * b * c + 2.0 * f * g * h - a * f * f - b * g * g - c * h * h)).unwrap();

    let n = ((lambda[1] - lambda[2]) / (lambda[0] - lambda[2])).sqrt();
    let m = 0.0;
    let l = ((lambda[0] - lambda[1]) / (lambda[0] - lambda[2])).sqrt();

    let t1 = (b - lambda.clone()) * g - f * h;
    let t2 = (a - lambda.clone()) * f - g * h;
    let t3 = -(a - lambda.clone()) * (t1.clone() / t2.clone()) / g - h / g;

    let mut mi = 1.0 / (1.0 + (t1.clone() / t2.clone()).mapv(f64::sqrt) + t3.clone().mapv(f64::sqrt));
    let mut li = (t1.clone() / t2.clone()) * mi.clone();
    let mut ni = t3.clone() * mi.clone();
}

pub fn unproject_ellipse(ellipse: &Ellipse, focal_length: f64, radius: f64) -> Option<Array1<f64>> {
    let conic = Conic::new(ellipse);
    let pupil_cone = Conicoid::new(conic, array!(0.0, 0.0, -focal_length));

    //let circles = unproject_conicoid(

    //)
    todo!()
}