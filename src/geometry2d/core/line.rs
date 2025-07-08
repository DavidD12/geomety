use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

/// A 2D line defined by a point and a direction vector.
///
/// The line is represented parametrically as:
///     L(t) = point + t * vector
/// where `point` is a point on the line and `vector` is the direction.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Line<T: Number> {
    /// A point on the line.
    point: Point<T>,
    /// The direction vector of the line.
    vector: Vector<T>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Line<T> {
    pub fn new(point: Point<T>, vector: Vector<T>) -> Self {
        Self { point, vector }
    }

    pub fn point(&self) -> &Point<T> {
        &self.point
    }

    pub fn vector(&self) -> &Vector<T> {
        &self.vector
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

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> Line<T>
where
    T: Number,
{
    pub fn translate(&mut self, dx: T, dy: T) {
        self.point.translate(dx, dy);
    }

    pub fn translated(&self, dx: T, dy: T) -> Self {
        Self {
            point: self.point.translated(dx, dy),
            vector: self.vector.clone(),
        }
    }
}

//-------------------------------------------------- To Vector --------------------------------------------------

// impl<T> ToVector<T> for Line<T>
// where
//     T: Number,
// {
//     fn to_vector(&self) -> Vector<T> {
//         self.vector.clone()
//     }
// }

//-------------------------------------------------- Ops --------------------------------------------------

//------------------------- Add -------------------------

impl<T> Add<Vector<T>> for Line<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            point: self.point() + &rhs,
            vector: self.vector() + rhs,
        }
    }
}

impl<T> Add<Vector<T>> for &Line<T>
where
    T: Number,
{
    type Output = Line<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            point: self.point() + &rhs,
            vector: self.vector() + rhs,
        }
    }
}

impl<T> Add<&Vector<T>> for Line<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            point: self.point() + rhs,
            vector: self.vector() + rhs,
        }
    }
}

impl<T> Add<&Vector<T>> for &Line<T>
where
    T: Number,
{
    type Output = Line<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            point: self.point() + rhs,
            vector: self.vector() + rhs,
        }
    }
}

//------------------------- Sub -------------------------

impl<T> Sub<Vector<T>> for Line<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            point: self.point() - &rhs,
            vector: self.vector() - rhs,
        }
    }
}

impl<T> Sub<Vector<T>> for &Line<T>
where
    T: Number,
{
    type Output = Line<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            point: self.point() - &rhs,
            vector: self.vector() - rhs,
        }
    }
}

impl<T> Sub<&Vector<T>> for Line<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            point: self.point() - rhs,
            vector: self.vector() - rhs,
        }
    }
}

impl<T> Sub<&Vector<T>> for &Line<T>
where
    T: Number,
{
    type Output = Line<T>;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            point: self.point() - rhs,
            vector: self.vector() - rhs,
        }
    }
}

//------------------------- Mul -------------------------

impl<T, U> Mul<Vector<U>> for Line<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Line<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vector<U>) -> Self::Output {
        Self::Output {
            point: self.point() * &rhs,
            vector: self.vector() * rhs,
        }
    }
}

impl<T, U> Mul<Vector<U>> for &Line<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Line<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vector<U>) -> Self::Output {
        Self::Output {
            point: self.point() * &rhs,
            vector: self.vector() * rhs,
        }
    }
}

impl<T, U> Mul<&Vector<U>> for Line<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Line<<T as Mul<U>>::Output>;

    fn mul(self, rhs: &Vector<U>) -> Self::Output {
        Self::Output {
            point: self.point() * rhs,
            vector: self.vector() * rhs,
        }
    }
}

impl<T, U> Mul<&Vector<U>> for &Line<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Line<<T as Mul<U>>::Output>;

    fn mul(self, rhs: &Vector<U>) -> Self::Output {
        Self::Output {
            point: self.point() * rhs,
            vector: self.vector() * rhs,
        }
    }
}

//------------------------- Div -------------------------

impl<T, U> Div<Vector<U>> for Line<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Line<<T as Div<U>>::Output>;

    fn div(self, rhs: Vector<U>) -> Self::Output {
        Self::Output {
            point: self.point() / &rhs,
            vector: self.vector() / rhs,
        }
    }
}

impl<T, U> Div<Vector<U>> for &Line<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Line<<T as Div<U>>::Output>;

    fn div(self, rhs: Vector<U>) -> Self::Output {
        Self::Output {
            point: self.point() / &rhs,
            vector: self.vector() / rhs,
        }
    }
}

impl<T, U> Div<&Vector<U>> for Line<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Line<<T as Div<U>>::Output>;

    fn div(self, rhs: &Vector<U>) -> Self::Output {
        Self::Output {
            point: self.point() / rhs,
            vector: self.vector() / rhs,
        }
    }
}

impl<T, U> Div<&Vector<U>> for &Line<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Line<<T as Div<U>>::Output>;

    fn div(self, rhs: &Vector<U>) -> Self::Output {
        Self::Output {
            point: self.point() / rhs,
            vector: self.vector() / rhs,
        }
    }
}

//-------------------------------------------------- Distance --------------------------------------------------

impl<T> Distance<T, Point<T>> for Line<T>
where
    T: Number,
    T: Pow2,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Pow2>::Output: Div<T, Output = T>,
{
    fn distance(&self, other: &Point<T>) -> T {
        other.distance(self)
    }
}

impl<T> Distance<T, Line<T>> for Line<T>
where
    T: Number,
    T: Pow2,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Div,
    <<T as Pow2>::Output as Div>::Output: Number,
    T: Mul<<<T as Pow2>::Output as Div>::Output, Output = T>,
    //
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

impl<T> Distance<T, Segment<T>> for Line<T>
where
    T: Number,
    T: Pow2,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Div,
    <<T as Pow2>::Output as Div>::Output: Number,
    T: Mul<<<T as Pow2>::Output as Div>::Output, Output = T>,
    //
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Mul>::Output: Div<T, Output = T>,
{
    fn distance(&self, other: &Segment<T>) -> T {
        match self.intersection_to_segment(other) {
            Some(_) => T::ZERO,
            None => self
                .distance(other.first())
                .min(self.distance(other.second())),
        }
    }
}

//-------------------------------------------------- Parallel --------------------------------------------------

impl<T> IsParallel<Line<T>> for Line<T>
where
    T: Number,
    T: Pow2,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Number,
{
    fn is_parallel(&self, other: &Line<T>) -> bool {
        self.vector.is_parallel(&other.vector)
    }
}

impl<T> IsParallel<Segment<T>> for Line<T>
where
    T: Number,
    T: Pow2,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Number,
{
    fn is_parallel(&self, other: &Segment<T>) -> bool {
        let v = other.to_vector();
        self.vector.is_parallel(&v)
    }
}

//-------------------------------------------------- Intersection --------------------------------------------------

impl<T> Line<T>
where
    T: Number,
    T: Pow2,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Div,
    <<T as Pow2>::Output as Div>::Output: Number,
    T: Mul<<<T as Pow2>::Output as Div>::Output, Output = T>,
{
    pub fn intersection(&self, other: &Self) -> Option<Point<T>> {
        let den = self.vector.cross_product(&other.vector);
        if den.abs() <= <<T as Mul>::Output as Sub>::Output::EPSILON {
            None
        } else {
            let v: Vector<_> = (&self.point, &other.point).into();
            let num = v.cross_product(&other.vector);
            let t = num / den;
            let delta = self.vector() * t;
            let x = self.point() + delta;
            Some(x)
        }
    }
}

impl<T> Line<T>
where
    T: Number,
    //
    T: Mul<T, Output = <T as Pow2>::Output>,
    T: Mul<<<T as Pow2>::Output as Div>::Output, Output = T>,
    //
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Div,
    <<T as Pow2>::Output as Div>::Output: Number,
{
    pub fn intersection_to_segment(&self, other: &Segment<T>) -> Option<Point<T>> {
        let seg_v = other.to_vector();
        let den = self.vector.cross_product(&seg_v);
        if den.abs() <= <T as Pow2>::Output::EPSILON {
            None
        } else {
            let v: Vector<_> = (&self.point, other.first()).into();
            let t = v.cross_product(&seg_v) / den;
            let u = v.cross_product(&self.vector) / den;
            if u >= <<T as Pow2>::Output as Div>::Output::ZERO
                && u <= <<T as Pow2>::Output as Div>::Output::ONE
            {
                let delta = self.vector() * t;
                let x = self.point() + delta;
                Some(x)
            } else {
                None
            }
        }
    }
}

impl<T> Line<T>
where
    T: Number,
    //
    T: Mul<T, Output = <T as Pow2>::Output>,
    T: Mul<<<T as Pow2>::Output as Div>::Output, Output = T>,
    //
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Div,
    <<T as Pow2>::Output as Div>::Output: Number,
{
    pub fn intersection_to_polygon(&self, other: &Polygon<T>) -> Vec<Point<T>> {
        let points = other
            .segments()
            .iter()
            .filter_map(|seg| self.intersection_to_segment(seg))
            .collect::<Vec<_>>();

        // unicity
        let mut v = vec![];
        for pt in points.iter() {
            if !v.contains(pt) {
                v.push(pt.clone());
            }
        }
        v
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Line<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line({}, {})", self.point, self.vector)
    }
}
