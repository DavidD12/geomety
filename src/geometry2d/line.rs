use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line<T: Number, P: Prefix> {
    pub point: Point<T, P>,
    pub vector: Vector<Metre_<T, P>>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number, P: Prefix> Line<T, P> {
    pub fn new(point: Point<T, P>, vector: Vector<Metre_<T, P>>) -> Self {
        Self { point, vector }
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T, P> Line<T, P>
where
    T: Number + Add<Output = T>,
    P: Prefix,
{
    pub fn translate(&self, dx: Metre_<T, P>, dy: Metre_<T, P>) -> Self {
        Self {
            point: self.point.translate(dx, dy),
            vector: self.vector,
        }
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T, P> Add<Vector<Metre_<T, P>>> for Line<T, P>
where
    T: Number + Add<Output = T>,
    P: Prefix,
{
    type Output = Line<T, P>;

    fn add(self, rhs: Vector<Metre_<T, P>>) -> Self::Output {
        Self {
            point: self.point + rhs,
            vector: self.vector,
        }
    }
}

impl<T, P> Sub<Vector<Metre_<T, P>>> for Line<T, P>
where
    T: Number + Sub<Output = T>,
    P: Prefix,
{
    type Output = Line<T, P>;

    fn sub(self, rhs: Vector<Metre_<T, P>>) -> Self::Output {
        Self {
            point: self.point - rhs,
            vector: self.vector,
        }
    }
}

impl<T, P> Mul<Vector<T>> for Line<T, P>
where
    T: Number + Mul<Output = T>,
    P: Prefix,
{
    type Output = Line<T, P>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Self {
            point: self.point * rhs,
            vector: self.vector,
        }
    }
}

impl<T, P> Div<Vector<T>> for Line<T, P>
where
    T: Number + Div<Output = T>,
    P: Prefix,
{
    type Output = Line<T, P>;

    fn div(self, rhs: Vector<T>) -> Self::Output {
        Self {
            point: self.point / rhs,
            vector: self.vector,
        }
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display, P: Prefix> Display for Line<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line({}, {})", self.point, self.vector)
    }
}
