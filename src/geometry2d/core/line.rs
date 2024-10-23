use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Line<T: Number> {
    pub point: Point<T>,
    pub vector: Vector<T>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Line<T> {
    pub fn new(point: Point<T>, vector: Vector<T>) -> Self {
        Self { point, vector }
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Number> From<(Point<T>, Vector<T>)> for Line<T> {
    fn from(value: (Point<T>, Vector<T>)) -> Self {
        Line::new(value.0, value.1)
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Line<T>
where
    T: Number,
{
    pub fn to_value(&self) -> Line<<T as HasValue>::Output> {
        Line::new(self.point.to_value(), self.vector.to_value())
    }
}

//-------------------------------------------------- To Vector --------------------------------------------------

impl<T> ToVector<T> for Line<T>
where
    T: Number,
{
    fn to_vector(&self) -> Vector<T> {
        self.vector
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T> Add<T> for Line<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            point: self.point + rhs,
            vector: self.vector,
        }
    }
}

impl<T> Sub<T> for Line<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            point: self.point - rhs,
            vector: self.vector,
        }
    }
}

impl<T> Mul<T> for Line<T>
where
    T: Number + Mul,
    <T as Mul>::Output: Number,
{
    type Output = Line<<T as Mul>::Output>;

    fn mul(self, rhs: T) -> Line<<T as Mul>::Output> {
        Line::new(self.point * rhs, self.vector * rhs)
    }
}

impl<T> Div<T> for Line<T>
where
    T: Number + Div,
    <T as Div>::Output: Number,
{
    type Output = Line<<T as Div>::Output>;

    fn div(self, rhs: T) -> Line<<T as Div>::Output> {
        Line::new(self.point / rhs, self.vector / rhs)
    }
}

//------------------------- Ops Vector -------------------------

impl<T> Add<Vector<T>> for Line<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Line::new(self.point + rhs.dx, self.vector)
    }
}

impl<T> Sub<Vector<T>> for Line<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self {
        Line::new(self.point - rhs.dx, self.vector)
    }
}

// impl<T, U> Mul<Vector<U>> for Line<T>
// where
//     T: Number,
//     U: Number,
//     Point<T>: Mul<U, Output = Point<T>>,
// {
//     type Output = Line<T>;

//     fn mul(self, rhs: Vector<U>) -> Line<T> {
//         Line::new(self.point * rhs.dx, self.vector)
//     }
// }

// impl<T, U> Div<Vector<U>> for Line<T>
// where
//     T: Number,
//     U: Number,
//     Point<T>: Div<U, Output = Point<T>>,
// {
//     type Output = Line<T>;

//     fn div(self, rhs: Vector<U>) -> Line<T> {
//         Line::new(self.point / rhs.dx, self.vector)
//     }
// }

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> Line<T>
where
    T: Number,
{
    pub fn translate(&self, dx: T, dy: T) -> Self {
        Self {
            point: self.point.translate(dx, dy),
            vector: self.vector,
        }
    }
}

//-------------------------------------------------- Distance --------------------------------------------------

impl<T> Distance<T, Point<T>> for Line<T>
where
    T: Number,
    // corss product
    T: Mul,
    <T as Mul>::Output: Number,
    // Length
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    // /
    <T as Mul>::Output: Div<T, Output = T>,
{
    fn distance(&self, other: &Point<T>) -> T {
        other.distance(self)
    }
}

impl<T> Distance<T, Line<T>> for Line<T>
where
    T: Number,
    // is_parallel
    T: Mul,
    <T as Mul>::Output: Number,
    // distance
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Mul>::Output: Div<T, Output = T>,
{
    fn distance(&self, other: &Self) -> T {
        if self.is_parallel(other) {
            self.point.distance(other)
        } else {
            T::ZERO
        }
    }
}

//-------------------------------------------------- Parallel --------------------------------------------------

impl<T> IsParallel<Line<T>> for Line<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
{
    fn is_parallel(&self, other: &Line<T>) -> bool {
        self.vector.is_parallel(&other.vector)
    }
}

impl<T> IsParallel<Segment<T>> for Line<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
{
    fn is_parallel(&self, other: &Segment<T>) -> bool {
        let v = other.to_vector();
        self.vector.is_parallel(&v)
    }
}

//-------------------------------------------------- Intersection --------------------------------------------------

impl<T> Intersection<T, Self> for Line<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
    <T as Mul>::Output: Div,
    <<T as Mul>::Output as Div>::Output: Number,
    T: Mul<<<T as Mul>::Output as Div>::Output, Output = T>,
{
    fn intersection(&self, other: &Self) -> Option<Point<T>> {
        let den = self.vector.cross_product(&other.vector);
        if den.abs() <= <<T as Mul>::Output as Sub>::Output::EPSILON {
            None
        } else {
            let v: Vector<_> = (&self.point, &other.point).into();
            let num = v.cross_product(&other.vector);
            let t = num / den;
            let delta = self.vector * t;
            let x = self.point + delta;
            Some(x)
        }
    }
}

impl<T> Intersection<T, Segment<T>> for Line<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
    <T as Mul>::Output: Div,
    <<T as Mul>::Output as Div>::Output: Number,
    T: Mul<<<T as Mul>::Output as Div>::Output, Output = T>,
{
    fn intersection(&self, other: &Segment<T>) -> Option<Point<T>> {
        let seg_v = other.to_vector();
        let den = self.vector.cross_product(&seg_v);
        if den.abs() <= <<T as Mul>::Output as Sub>::Output::EPSILON {
            None
        } else {
            let v: Vector<_> = (&self.point, &other.first()).into();
            let t = v.cross_product(&seg_v) / den;
            let u = v.cross_product(&self.vector) / den;
            if u >= <<<T as Mul>::Output as Sub>::Output as Div>::Output::ZERO
                && u <= <<<T as Mul>::Output as Sub>::Output as Div>::Output::ONE
            {
                let delta = self.vector * t;
                let x = self.point + delta;
                Some(x)
            } else {
                None
            }
        }
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Line<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line({}, {})", self.point, self.vector)
    }
}
