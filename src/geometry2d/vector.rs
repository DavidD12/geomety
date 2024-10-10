use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use sity::{Float, Pow2, Root2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T: Copy> {
    pub dx: T,
    pub dy: T,
}

impl<T: Copy> Vector<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T> Add<T> for Vector<T>
where
    T: Copy + Add<Output = T>,
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
    T: Copy + Add<Output = T>,
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
    T: Copy + Sub<Output = T>,
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
    T: Copy + Sub<Output = T>,
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
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

//-------------------------------------------------- Norm --------------------------------------------------

impl<T> Vector<T>
where
    T: Copy + Add<Output = T> + Pow2<Output = T> + Root2<Output = T>,
{
    pub fn norm(&self) -> T {
        (self.dx.pow2() + self.dy.pow2()).root2()
    }
}

impl<T> Vector<T>
where
    T: Copy + Add<Output = T> + Pow2<Output = T>,
{
    pub fn sqare_norm(&self) -> T {
        self.dx.pow2() + self.dy.pow2()
    }
}

//-------------------------------------------------- Product --------------------------------------------------

impl<T> Vector<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    pub fn dot_product(&self, other: Self) -> T {
        self.dx * other.dx + self.dy * other.dy
    }
}

impl<T> Vector<T>
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
{
    pub fn cross_product(&self, other: Self) -> T {
        self.dx * other.dy - self.dy * other.dx
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Copy> From<(T, T)> for Vector<T> {
    fn from(value: (T, T)) -> Self {
        Vector::new(value.0, value.1)
    }
}

impl<T: Copy> Into<(T, T)> for Vector<T> {
    fn into(self) -> (T, T) {
        (self.dx, self.dy)
    }
}

impl<T: Copy> Into<(T, T)> for &Vector<T> {
    fn into(self) -> (T, T) {
        (self.dx, self.dy)
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Copy + Display> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({}, {})", self.dx, self.dy)
    }
}
