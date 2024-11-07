use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Segment<T: Number> {
    points: (Point<T>, Point<T>),
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Segment<T> {
    pub fn new(pt1: Point<T>, pt2: Point<T>) -> Self {
        Self { points: (pt1, pt2) }
    }

    pub fn first(&self) -> &Point<T> {
        &self.points.0
    }

    pub fn second(&self) -> &Point<T> {
        &self.points.1
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
    T: Number,
{
    pub fn to_value(&self) -> Segment<<T as HasValue>::Output> {
        Segment::new(self.points.0.to_value(), self.points.1.to_value())
    }
}

//-------------------------------------------------- ToVector --------------------------------------------------

impl<T> Segment<T>
where
    T: Number,
{
    pub fn to_vector(&self) -> Vector<T> {
        let dx = self.points.1.x - self.points.0.x;
        let dy = self.points.1.y - self.points.0.y;
        Vector::new(dx, dy)
    }
}

//-------------------------------------------------- Reverse --------------------------------------------------

impl<T> Segment<T>
where
    T: Number,
{
    pub fn reversed(&self) -> Self {
        Self {
            points: (self.points.1.clone(), self.points.0.clone()),
        }
    }

    pub fn reverse(&mut self) {
        self.points = (self.points.1.clone(), self.points.0.clone())
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> Segment<T>
where
    T: Number,
{
    pub fn translate(&mut self, dx: T, dy: T) {
        self.points.0.translate(dx, dy);
        self.points.1.translate(dx, dy);
    }

    pub fn translated(&self, dx: T, dy: T) -> Self {
        Self {
            points: (
                self.first().translated(dx, dy),
                self.second().translated(dx, dy),
            ),
        }
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

//------------------------- Add -------------------------

impl<T> Add<Vector<T>> for Segment<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            points: (self.first() + &rhs, self.second() + rhs),
        }
    }
}

impl<T> Add<Vector<T>> for &Segment<T>
where
    T: Number,
{
    type Output = Segment<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            points: (self.first() + &rhs, self.second() + rhs),
        }
    }
}

impl<T> Add<&Vector<T>> for Segment<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            points: (self.first() + rhs, self.second() + rhs),
        }
    }
}

impl<T> Add<&Vector<T>> for &Segment<T>
where
    T: Number,
{
    type Output = Segment<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            points: (self.first() + rhs, self.second() + rhs),
        }
    }
}

//------------------------- Sub -------------------------

impl<T> Sub<Vector<T>> for Segment<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            points: (self.first() - &rhs, self.second() - rhs),
        }
    }
}

impl<T> Sub<Vector<T>> for &Segment<T>
where
    T: Number,
{
    type Output = Segment<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            points: (self.first() - &rhs, self.second() - rhs),
        }
    }
}

impl<T> Sub<&Vector<T>> for Segment<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            points: (self.first() - rhs, self.second() - rhs),
        }
    }
}

impl<T> Sub<&Vector<T>> for &Segment<T>
where
    T: Number,
{
    type Output = Segment<T>;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            points: (self.first() - rhs, self.second() - rhs),
        }
    }
}

//------------------------- Mul -------------------------

impl<T, U> Mul<Vector<U>> for Segment<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Segment<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vector<U>) -> Self::Output {
        Self::Output {
            points: (self.first() * &rhs, self.second() * rhs),
        }
    }
}

impl<T, U> Mul<Vector<U>> for &Segment<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Segment<<T as Mul<U>>::Output>;

    fn mul(self, rhs: Vector<U>) -> Self::Output {
        Self::Output {
            points: (self.first() * &rhs, self.second() * rhs),
        }
    }
}

impl<T, U> Mul<&Vector<U>> for Segment<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Segment<<T as Mul<U>>::Output>;

    fn mul(self, rhs: &Vector<U>) -> Self::Output {
        Self::Output {
            points: (self.first() * rhs, self.second() * rhs),
        }
    }
}

impl<T, U> Mul<&Vector<U>> for &Segment<T>
where
    T: Number,
    U: Number,
    T: Mul<U>,
    <T as Mul<U>>::Output: Number,
{
    type Output = Segment<<T as Mul<U>>::Output>;

    fn mul(self, rhs: &Vector<U>) -> Self::Output {
        Self::Output {
            points: (self.first() * rhs, self.second() * rhs),
        }
    }
}

//------------------------- Div -------------------------

impl<T, U> Div<Vector<U>> for Segment<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Segment<<T as Div<U>>::Output>;

    fn div(self, rhs: Vector<U>) -> Self::Output {
        Self::Output {
            points: (self.first() / &rhs, self.second() / rhs),
        }
    }
}

impl<T, U> Div<Vector<U>> for &Segment<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Segment<<T as Div<U>>::Output>;

    fn div(self, rhs: Vector<U>) -> Self::Output {
        Self::Output {
            points: (self.first() / &rhs, self.second() / rhs),
        }
    }
}

impl<T, U> Div<&Vector<U>> for Segment<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Segment<<T as Div<U>>::Output>;

    fn div(self, rhs: &Vector<U>) -> Self::Output {
        Self::Output {
            points: (self.first() / rhs, self.second() / rhs),
        }
    }
}

impl<T, U> Div<&Vector<U>> for &Segment<T>
where
    T: Number,
    U: Number,
    T: Div<U>,
    <T as Div<U>>::Output: Number,
{
    type Output = Segment<<T as Div<U>>::Output>;

    fn div(self, rhs: &Vector<U>) -> Self::Output {
        Self::Output {
            points: (self.first() / rhs, self.second() / rhs),
        }
    }
}

//-------------------------------------------------- Length --------------------------------------------------

impl<T> Segment<T>
where
    T: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
{
    pub fn length2(&self) -> <T as Pow2>::Output {
        self.points.0.distance2(&self.points.1)
    }
}

impl<T> Segment<T>
where
    T: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
{
    pub fn length(&self) -> T {
        self.points.0.distance(&self.points.1)
    }
}

//-------------------------------------------------- Distance --------------------------------------------------

impl<T> Distance<T, Point<T>> for Segment<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
    <T as Mul>::Output: Div,
    <<T as Mul>::Output as Div>::Output: Number,
    T: Mul<<<T as Mul>::Output as Div>::Output, Output = T>,
    T: Pow2<Output = <T as Mul>::Output>,
    <T as Pow2>::Output: Root2<Output = T>,
{
    fn distance(&self, other: &Point<T>) -> T {
        other.distance(self)
    }
}

impl<T> Distance<T, Line<T>> for Segment<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
    <T as Mul>::Output: Div,
    <T as Mul>::Output: Div<T, Output = T>,
    <<T as Mul>::Output as Div>::Output: Number,
    T: Mul<<<T as Mul>::Output as Div>::Output, Output = T>,
    T: Pow2<Output = <T as Mul>::Output>,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
{
    fn distance(&self, other: &Line<T>) -> T {
        other.distance(self)
    }
}

impl<T> Distance<T, Segment<T>> for Segment<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
    <T as Mul>::Output: Div,
    <T as Mul>::Output: Div<T, Output = T>,
    <<T as Mul>::Output as Div>::Output: Number,
    T: Mul<<<T as Mul>::Output as Div>::Output, Output = T>,
    T: Pow2<Output = <T as Mul>::Output>,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
{
    fn distance(&self, other: &Self) -> T {
        match self.intersection(other) {
            Some(_) => T::ZERO,
            None => self
                .distance(other.first())
                .min(self.distance(other.second())),
        }
    }
}

//-------------------------------------------------- Parallel --------------------------------------------------

impl<T> IsParallel<Segment<T>> for Segment<T>
where
    T: Number,
    T: Pow2,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Number,
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
    T: Pow2,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Pow2>::Output: Number,
{
    fn is_parallel(&self, other: &Line<T>) -> bool {
        other.is_parallel(self)
    }
}

//-------------------------------------------------- Intersection --------------------------------------------------

impl<T> Segment<T>
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
    pub fn intersection_to_line(&self, other: &Line<T>) -> Option<Point<T>> {
        other.intersection_to_segment(self)
    }
}

impl<T> Segment<T>
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
    pub fn intersection(&self, other: &Segment<T>) -> Option<Point<T>> {
        let self_v = self.to_vector();
        let other_v = other.to_vector();
        let den = self_v.cross_product(&other_v);
        if den.abs() <= <<T as Mul>::Output as Sub>::Output::EPSILON {
            None
        } else {
            let v: Vector<_> = (self.first(), other.first()).into();
            let t = v.cross_product(&other_v) / den;
            let u = v.cross_product(&self_v) / den;
            if u >= <<T as Mul>::Output as Div>::Output::ZERO
                && u <= <<T as Mul>::Output as Div>::Output::ONE
                && t >= <<T as Mul>::Output as Div>::Output::ZERO
                && t <= <<T as Mul>::Output as Div>::Output::ONE
            {
                let delta = self_v * t;
                let x = self.first() + delta;
                Some(x)
            } else {
                None
            }
        }
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Segment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Segment({}, {})", self.first(), self.second())
    }
}
