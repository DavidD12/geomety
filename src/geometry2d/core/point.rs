use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point<T: Number> {
    pub x: T,
    pub y: T,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Number> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Point::new(value.0, value.1)
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Point<T>
where
    T: Number + HasValue,
{
    pub fn to_value(&self) -> Point<<T as HasValue>::Output> {
        Point::new(self.x.value(), self.y.value())
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T> Add<T> for Point<T>
where
    T: Number + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T> Sub<T> for Point<T>
where
    T: Number + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T, U> Mul<U> for Point<T>
where
    T: Number + Mul<U>,
    U: Number,
    <T as Mul<U>>::Output: Number,
{
    type Output = Point<<T as Mul<U>>::Output>;

    fn mul(self, rhs: U) -> Point<<T as Mul<U>>::Output> {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl<T, U> Div<U> for Point<T>
where
    T: Number + Div<U>,
    U: Number,
    <T as Div<U>>::Output: Number,
{
    type Output = Point<<T as Div<U>>::Output>;

    fn div(self, rhs: U) -> Point<<T as Div<U>>::Output> {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

//------------------------- Ops Vector -------------------------

impl<T> Add<Vector<T>> for Point<T>
where
    T: Number + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point::new(self.x + rhs.dx, self.y + rhs.dy)
    }
}

impl<T> Sub<Vector<T>> for Point<T>
where
    T: Number + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self {
        Point::new(self.x - rhs.dx, self.y - rhs.dy)
    }
}

impl<T, U> Mul<Vector<U>> for Point<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Point<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vector<U>) -> Point<<T as Mul<U>>::Output> {
        Point::new(self.x * rhs.dx, self.y * rhs.dy)
    }
}

impl<T, U> Div<Vector<U>> for Point<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Point<<T as Div<U>>::Output>;

    fn div(self, rhs: Vector<U>) -> Point<<T as Div<U>>::Output> {
        Point::new(self.x / rhs.dx, self.y / rhs.dy)
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> Point<T>
where
    T: Number + Add<Output = T>,
{
    pub fn translate(&self, dx: T, dy: T) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

//-------------------------------------------------- Distance2 --------------------------------------------------

impl<T> Point<T>
where
    T: Number,
    T: Sub<Output = T>,
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
{
    pub fn distance2(&self, other: &Self) -> <T as Pow2>::Output {
        (other.x - self.x).pow2() + (other.y - self.y).pow2()
    }
}

//-------------------------------------------------- Distance --------------------------------------------------

impl<T> Distance<T, Point<T>> for Point<T>
where
    T: Number,
    T: Sub<Output = T>,
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Root2<Output = T>,
{
    fn distance(&self, other: &Self) -> T {
        ((other.x - self.x).pow2() + (other.y - self.y).pow2()).root2()
    }
}

impl<T> Distance<T, Line<T>> for Point<T>
where
    T: Number,
    // into
    T: Sub<T, Output = T>,
    // corss product
    T: Mul,
    <T as Mul>::Output: Sub,
    // Length
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Root2<Output = T>,
    // abs
    <<T as Mul>::Output as Sub>::Output: Number,
    // /
    <<T as Mul>::Output as Sub>::Output: Div<T, Output = T>,
{
    fn distance(&self, other: &Line<T>) -> T {
        let w: Vector<_> = (self, &other.point).into();
        let c = w.cross_product(&other.vector);
        let l = other.vector.norm();
        let d = c.abs() / l;
        d
    }
}

impl<T> Distance<T, Segment<T>> for Point<T>
where
    T: Number,
    // to vector
    T: Sub<Output = T>,
    // Line distance
    T: Mul,
    <T as Mul>::Output: Sub,
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Root2<Output = T>,
    <<T as Mul>::Output as Sub>::Output: Number,
    <<T as Mul>::Output as Sub>::Output: Div<T, Output = T>,
{
    fn distance(&self, other: &Segment<T>) -> T {
        let v = other.to_vector();
        let line = Line::new(other.first(), v);
        let d_line = self.distance(&line);
        let d_first = self.distance(&other.first());
        let d_second = self.distance(&other.second());
        d_line.min(d_first).min(d_second)
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}
