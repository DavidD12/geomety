use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
use std::process::Output;

use sity::*;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl<T: Number> Default for Vector<T> {
    fn default() -> Self {
        Self {
            dx: T::ZERO,
            dy: T::ZERO,
        }
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

impl<T> Mul<T> for Vector<T>
where
    T: Number + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl<T, U> Mul for Vector<T>
where
    T: Number + Mul<Output = U>,
    U: Number,
{
    type Output = Vector<U>;

    fn mul(self, rhs: Self) -> Vector<U> {
        Vector::new(self.dx * rhs.dx, self.dy * rhs.dy)
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Number + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

impl<T, U> Div for Vector<T>
where
    T: Number + Div<Output = U>,
    U: Number,
{
    type Output = Vector<U>;

    fn div(self, rhs: Self) -> Vector<U> {
        Vector::new(self.dx / rhs.dx, self.dy / rhs.dy)
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

//-------------------------------------------------- Length --------------------------------------------------

impl<T> Vector<T>
where
    T: Number + Pow2,
    <T as Pow2>::Output: Add,
{
    pub fn length2(self) -> <<T as Pow2>::Output as Add>::Output {
        self.dx.pow2() + self.dy.pow2()
    }
}

impl<T> Vector<T>
where
    T: Number + Pow2,
    <T as Pow2>::Output: Add,
    <<T as Pow2>::Output as Add>::Output: Root2,
{
    pub fn length(self) -> <<<T as Pow2>::Output as Add>::Output as Root2>::Output {
        (self.dx.pow2() + self.dy.pow2()).root2()
    }
}

//-------------------------------------------------- Normalize --------------------------------------------------

impl<T> Vector<T>
where
    T: Number
        + Pow2
        + Div<
            <<<<T as Pow2>::Output as Add>::Output as Root2>::Output as HasValue>::Output,
            Output = T,
        >,
    <T as Pow2>::Output: Add,
    <<T as Pow2>::Output as Add>::Output: Root2,
    <<<T as Pow2>::Output as Add>::Output as Root2>::Output: HasValue,
{
    pub fn normalize(&self) -> Self {
        let length = self.length().value();
        Self {
            dx: self.dx / length,
            dy: self.dy / length,
        }
    }
}

//-------------------------------------------------- Product --------------------------------------------------

impl<T> Vector<T>
where
    T: Number + Add<Output = T> + Mul<Output = T>,
{
    pub fn dot_product(&self, other: Self) -> T {
        self.dx * other.dx + self.dy * other.dy
    }
}

impl<T> Vector<T>
where
    T: Number + Sub<Output = T> + Mul<Output = T>,
{
    pub fn cross_product(&self, other: Self) -> T {
        self.dx * other.dy - self.dy * other.dx
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Number> From<(T, T)> for Vector<T> {
    fn from(value: (T, T)) -> Self {
        Vector::new(value.0, value.1)
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

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({}, {})", self.dx, self.dy)
    }
}
