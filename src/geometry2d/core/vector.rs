use super::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use sity::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector<T: Number> {
    pub dx: T,
    pub dy: T,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Vector<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Number> From<(T, T)> for Vector<T> {
    fn from(value: (T, T)) -> Self {
        Vector::new(value.0, value.1)
    }
}

impl<T> From<(&Point<T>, &Point<T>)> for Vector<T>
where
    T: Number + Sub<Output = T>,
{
    fn from(value: (&Point<T>, &Point<T>)) -> Self {
        let dx = value.1.x - value.0.x;
        let dy = value.1.y - value.0.y;
        Vector::new(dx, dy)
    }
}

impl<T: Number> Into<(T, T)> for Vector<T> {
    fn into(self) -> (T, T) {
        (self.dx, self.dy)
    }
}

impl<T: Number> Into<(T, T)> for &Vector<T> {
    fn into(self) -> (T, T) {
        (self.dx, self.dy)
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Vector<T>
where
    T: Number + HasValue,
{
    pub fn to_value(&self) -> Vector<<T as HasValue>::Output> {
        Vector::new(self.dx.value(), self.dy.value())
    }
}

//-------------------------------------------------- ToVector --------------------------------------------------

impl<T> ToVector<T> for Vector<T>
where
    T: Number,
{
    fn to_vector(&self) -> Vector<T> {
        *self
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T> Add<T> for Vector<T>
where
    T: Number + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            dx: self.dx + rhs,
            dy: self.dy + rhs,
        }
    }
}

impl<T> Sub<T> for Vector<T>
where
    T: Number + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        Self {
            dx: self.dx - rhs,
            dy: self.dy - rhs,
        }
    }
}

impl<T, U> Mul<U> for Vector<T>
where
    T: Number + Mul<U>,
    U: Number,
    <T as Mul<U>>::Output: Number,
{
    type Output = Vector<<T as Mul<U>>::Output>;

    fn mul(self, rhs: U) -> Vector<<T as Mul<U>>::Output> {
        Vector::new(self.dx * rhs, self.dy * rhs)
    }
}

impl<T, U> Div<U> for Vector<T>
where
    T: Number + Div<U>,
    U: Number,
    <T as Div<U>>::Output: Number,
{
    type Output = Vector<<T as Div<U>>::Output>;

    fn div(self, rhs: U) -> Vector<<T as Div<U>>::Output> {
        Vector::new(self.dx / rhs, self.dy / rhs)
    }
}

//------------------------- Ops Vector -------------------------

impl<T> Add for Vector<T>
where
    T: Number + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl<T> Sub for Vector<T>
where
    T: Number + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
        }
    }
}

impl<T, U> Mul<Vector<U>> for Vector<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Vector<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vector<U>) -> Vector<<T as Mul<U>>::Output> {
        Vector::new(self.dx * rhs.dx, self.dy * rhs.dy)
    }
}

impl<T, U> Div<Vector<U>> for Vector<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Vector<<T as Div<U>>::Output>;

    fn div(self, rhs: Vector<U>) -> Vector<<T as Div<U>>::Output> {
        Vector::new(self.dx / rhs.dx, self.dy / rhs.dy)
    }
}

//-------------------------------------------------- Norm --------------------------------------------------

impl<T> Vector<T>
where
    T: Number,
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
{
    pub fn norm2(&self) -> <T as Pow2>::Output {
        self.dx.pow2() + self.dy.pow2()
    }
}

impl<T> Vector<T>
where
    T: Number,
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Root2<Output = T>,
{
    pub fn norm(&self) -> T {
        (self.dx.pow2() + self.dy.pow2()).root2()
    }
}

//-------------------------------------------------- Normalize --------------------------------------------------

impl<T> Vector<T>
where
    T: Number,
    // length
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Root2<Output = T>,
    // value
    T: HasValue,
    // /
    T: Div<<T as HasValue>::Output, Output = T>,
{
    pub fn normalize(&self) -> Self {
        let length = self.norm().value();
        Self {
            dx: self.dx / length,
            dy: self.dy / length,
        }
    }
}

//-------------------------------------------------- Product --------------------------------------------------

impl<T> Vector<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Add,
{
    pub fn dot_product(&self, other: &Self) -> <<T as Mul>::Output as Add>::Output {
        self.dx * other.dx + self.dy * other.dy
    }
}

impl<T> Vector<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Sub,
{
    pub fn cross_product(&self, other: &Self) -> <<T as Mul>::Output as Sub>::Output {
        self.dx * other.dy - self.dy * other.dx
    }
}

//-------------------------------------------------- Parallel --------------------------------------------------

impl<T> IsParallel<Vector<T>> for Vector<T>
where
    T: Number,
    // cros product
    T: Mul,
    <T as Mul>::Output: Sub,
    // abs
    <<T as Mul>::Output as Sub>::Output: Number,
{
    fn is_parallel(&self, other: &Vector<T>) -> bool {
        self.cross_product(other).abs() <= <<T as Mul>::Output as Sub>::Output::EPSILON
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({}, {})", self.dx, self.dy)
    }
}
