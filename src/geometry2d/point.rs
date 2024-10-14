use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T: Number, P: Prefix> {
    pub x: Metre_<T, P>,
    pub y: Metre_<T, P>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number, P: Prefix> Point<T, P> {
    pub fn new(x: Metre_<T, P>, y: Metre_<T, P>) -> Self {
        Self { x, y }
    }
}

//-------------------------------------------------- Default --------------------------------------------------

impl<T: Number, P: Prefix> Default for Point<T, P> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T, P> Point<T, P>
where
    T: Number + Add<Output = T>,
    P: Prefix,
{
    pub fn translate(&self, dx: Metre_<T, P>, dy: Metre_<T, P>) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T, P> Add<Vector<Metre_<T, P>>> for Point<T, P>
where
    T: Number + Add<Output = T>,
    P: Prefix,
{
    type Output = Point<T, P>;

    fn add(self, rhs: Vector<Metre_<T, P>>) -> Self::Output {
        Self {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl<T, P> Sub<Vector<Metre_<T, P>>> for Point<T, P>
where
    T: Number + Sub<Output = T>,
    P: Prefix,
{
    type Output = Point<T, P>;

    fn sub(self, rhs: Vector<Metre_<T, P>>) -> Self::Output {
        Self {
            x: self.x - rhs.dx,
            y: self.y - rhs.dy,
        }
    }
}

impl<T, P> Mul<Vector<T>> for Point<T, P>
where
    T: Number + Mul<Output = T>,
    P: Prefix,
{
    type Output = Point<T, P>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Self {
            x: self.x * rhs.dx,
            y: self.y * rhs.dy,
        }
    }
}

impl<T, P> Div<Vector<T>> for Point<T, P>
where
    T: Number + Div<Output = T>,
    P: Prefix,
{
    type Output = Point<T, P>;

    fn div(self, rhs: Vector<T>) -> Self::Output {
        Self {
            x: self.x / rhs.dx,
            y: self.y / rhs.dy,
        }
    }
}

//-------------------------------------------------- Distance --------------------------------------------------

impl<T, P> Distance<T, P, Self> for Point<T, P>
where
    T: Number + Add<Output = T> + Sub<Output = T> + Pow2<Output = T> + Root2<Output = T>,
    P: Prefix,
{
    fn distance(&self, other: &Self) -> Metre_<T, P> {
        ((other.x - self.x).pow2() + (other.y - self.y).pow2()).root2()
    }

    fn distance2(&self, other: &Self) -> Metre2_<T, P> {
        (other.x - self.x).pow2() + (other.y - self.y).pow2()
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Number, P: Prefix> From<(Metre_<T, P>, Metre_<T, P>)> for Point<T, P> {
    fn from(value: (Metre_<T, P>, Metre_<T, P>)) -> Self {
        Point::new(value.0, value.1)
    }
}

impl<T: Number, P: Prefix> Into<(Metre_<T, P>, Metre_<T, P>)> for Point<T, P> {
    fn into(self) -> (Metre_<T, P>, Metre_<T, P>) {
        (self.x, self.y)
    }
}

impl<T: Number, P: Prefix> Into<(Metre_<T, P>, Metre_<T, P>)> for &Point<T, P> {
    fn into(self) -> (Metre_<T, P>, Metre_<T, P>) {
        (self.x, self.y)
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display, P: Prefix> Display for Point<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}
