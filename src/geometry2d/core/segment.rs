use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Segment<T: Number> {
    pub points: (Point<T>, Point<T>),
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Segment<T> {
    pub fn new(pt1: Point<T>, pt2: Point<T>) -> Self {
        Self { points: (pt1, pt2) }
    }

    pub fn first(&self) -> Point<T> {
        self.points.0
    }

    pub fn second(&self) -> Point<T> {
        self.points.1
    }
}

//-------------------------------------------------- Into/From --------------------------------------------------

impl<T: Number> From<(Point<T>, Point<T>)> for Segment<T> {
    fn from(value: (Point<T>, Point<T>)) -> Self {
        Self::new(value.0, value.1)
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Segment<T>
where
    T: Number + HasValue,
{
    pub fn to_value(&self) -> Segment<<T as HasValue>::Output> {
        Segment::new(self.points.0.to_value(), self.points.1.to_value())
    }
}

//-------------------------------------------------- ToVector --------------------------------------------------

impl<T> ToVector<T> for Segment<T>
where
    T: Number + Sub<Output = T>,
{
    fn to_vector(&self) -> Vector<T> {
        let dx = self.points.1.x - self.points.0.x;
        let dy = self.points.1.y - self.points.0.y;
        Vector::new(dx, dy)
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T> Add<T> for Segment<T>
where
    T: Number + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Segment::new(self.points.0 + rhs, self.points.1 + rhs)
    }
}

impl<T> Sub<T> for Segment<T>
where
    T: Number + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Segment::new(self.points.0 - rhs, self.points.1 - rhs)
    }
}

impl<T> Mul<T> for Segment<T>
where
    T: Number + Mul,
    <T as Mul>::Output: Number,
{
    type Output = Segment<<T as Mul>::Output>;

    fn mul(self, rhs: T) -> Segment<<T as Mul>::Output> {
        Segment::new(self.points.0 * rhs, self.points.1 * rhs)
    }
}

impl<T> Div<T> for Segment<T>
where
    T: Number + Div,
    <T as Div>::Output: Number,
{
    type Output = Segment<<T as Div>::Output>;

    fn div(self, rhs: T) -> Segment<<T as Div>::Output> {
        Segment::new(self.points.0 / rhs, self.points.1 / rhs)
    }
}

//------------------------- Ops Vector -------------------------

impl<T> Add<Vector<T>> for Segment<T>
where
    T: Number + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Segment::new(self.points.0 + rhs, self.points.1 + rhs)
    }
}

impl<T> Sub<Vector<T>> for Segment<T>
where
    T: Number + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self {
        Segment::new(self.points.0 - rhs, self.points.1 - rhs)
    }
}

impl<T, U> Mul<Vector<U>> for Segment<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Segment<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vector<U>) -> Segment<<T as Mul<U>>::Output> {
        Segment::new(self.points.0 * rhs, self.points.1 * rhs)
    }
}

impl<T, U> Div<Vector<U>> for Segment<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Segment<<T as Div<U>>::Output>;

    fn div(self, rhs: Vector<U>) -> Segment<<T as Div<U>>::Output> {
        Segment::new(self.points.0 / rhs, self.points.1 / rhs)
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> Segment<T>
where
    T: Number + Add<Output = T>,
{
    pub fn translate(&self, dx: T, dy: T) -> Self {
        Self::new(
            self.points.0.translate(dx, dy),
            self.points.1.translate(dx, dy),
        )
    }
}

//-------------------------------------------------- Norm --------------------------------------------------

impl<T> Segment<T>
where
    T: Number,
    T: Sub<Output = T>,
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
{
    pub fn norm2(self) -> <T as Pow2>::Output {
        self.points.0.distance2(&self.points.1)
    }
}

impl<T> Segment<T>
where
    T: Number,
    T: Sub<Output = T>,
    T: Pow2,
    <T as Pow2>::Output: Add<Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Root2<Output = T>,
{
    pub fn norm(self) -> T {
        self.points.0.distance(&self.points.1)
    }
}

//-------------------------------------------------- Distance --------------------------------------------------

impl<T> Distance<T, Point<T>> for Segment<T>
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
    fn distance(&self, other: &Point<T>) -> T {
        other.distance(self)
    }
}

//-------------------------------------------------- Parallel --------------------------------------------------

impl<T> IsParallel<Segment<T>> for Segment<T>
where
    T: Number,
    // to_vector
    T: Sub<Output = T>,
    // vector is_parallel
    T: Mul,
    <T as Mul>::Output: Sub,
    <<T as Mul>::Output as Sub>::Output: Number,
{
    fn is_parallel(&self, other: &Segment<T>) -> bool {
        let v1 = self.to_vector();
        let v2 = other.to_vector();
        v1.is_parallel(&v2)
    }
}

impl<T> IsParallel<Line<T>> for Segment<T>
where
    T: Number,
    // to_vector
    T: Sub<Output = T>,
    // vector is_parallel
    T: Mul,
    <T as Mul>::Output: Sub,
    <<T as Mul>::Output as Sub>::Output: Number,
{
    fn is_parallel(&self, other: &Line<T>) -> bool {
        other.is_parallel(self)
    }
}

//-------------------------------------------------- Intersection --------------------------------------------------

impl<T> Intersection<T, Line<T>> for Segment<T>
where
    T: Number,
    // to_vector
    T: Sub<Output = T>,
    // vector is_parallel
    T: Mul,
    <T as Mul>::Output: Sub,
    <<T as Mul>::Output as Sub>::Output: Number,
    // /
    <<T as Mul>::Output as Sub>::Output: Div,
    // *
    Vector<T>: Mul<<<<T as Mul>::Output as Sub>::Output as Div>::Output, Output = Vector<T>>,
    <<<T as Mul>::Output as Sub>::Output as Div>::Output: Number,
    // +
    T: Add<Output = T>,
{
    fn intersection(&self, other: &Line<T>) -> Option<Point<T>> {
        other.intersection(self)
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Segment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Segment({}, {})", self.first(), self.second())
    }
}
