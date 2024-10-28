use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Circle<T: Number> {
    pub center: Point<T>,
    pub radius: T,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Circle<T> {
    pub fn new(center: Point<T>, radius: T) -> Self {
        Self { center, radius }
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Number> From<(Point<T>, T)> for Circle<T> {
    fn from(value: (Point<T>, T)) -> Self {
        Circle::new(value.0, value.1)
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Circle<T>
where
    T: Number,
{
    pub fn to_value(&self) -> Circle<<T as HasValue>::Output> {
        Circle::new(self.center.to_value(), self.radius.value())
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

//------------------------- Ops Vector -------------------------

impl<T> Add<Vector<T>> for Circle<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Circle::new(self.center + rhs, self.radius)
    }
}

impl<T> Sub<Vector<T>> for Circle<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self {
        Circle::new(self.center - rhs.dx, self.radius)
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> Circle<T>
where
    T: Number,
{
    pub fn translate(&self, dx: T, dy: T) -> Self {
        Self {
            center: self.center.translate(dx, dy),
            radius: self.radius,
        }
    }
}

//-------------------------------------------------- Contains --------------------------------------------------

impl<T> Contains<Point<T>> for Circle<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Mul>::Output: Div<T, Output = T>,
{
    fn contains(&self, point: &Point<T>) -> bool {
        self.center.distance(point) <= self.radius
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Circle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle({}, {})", self.center, self.radius)
    }
}
