use crate::geometry2d::*;
use std::ops::{Add, Div, Mul, Sub};

//-------------------------------------------------- Vecor --------------------------------------------------

impl Add<Vector<f64>> for f64 {
    type Output = Vector<f64>;

    fn add(self, rhs: Vector<f64>) -> Self::Output {
        Vector::new(self + rhs.dx, self + rhs.dy)
    }
}

impl Sub<Vector<f64>> for f64 {
    type Output = Vector<f64>;

    fn sub(self, rhs: Vector<f64>) -> Self::Output {
        Vector::new(self - rhs.dx, self - rhs.dy)
    }
}

impl Mul<Vector<f64>> for f64 {
    type Output = Vector<f64>;

    fn mul(self, rhs: Vector<f64>) -> Self::Output {
        Vector::new(self * rhs.dx, self * rhs.dy)
    }
}

impl Div<Vector<f64>> for f64 {
    type Output = Vector<f64>;

    fn div(self, rhs: Vector<f64>) -> Self::Output {
        Vector::new(self / rhs.dx, self / rhs.dy)
    }
}
