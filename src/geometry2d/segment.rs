use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Segment<T: Number, P: Prefix> {
    pub points: (Point<T, P>, Point<T, P>),
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number, P: Prefix> Segment<T, P> {
    pub fn new(points: (Point<T, P>, Point<T, P>)) -> Self {
        Self { points }
    }

    pub fn first(&self) -> Point<T, P> {
        self.points.0
    }

    pub fn second(&self) -> Point<T, P> {
        self.points.1
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T, P> Segment<T, P>
where
    T: Number + Add<Output = T>,
    P: Prefix,
{
    pub fn translate(&self, dx: Metre_<T, P>, dy: Metre_<T, P>) -> Self {
        Self {
            points: (
                self.points.0.translate(dx, dy),
                self.points.1.translate(dx, dy),
            ),
        }
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T, P> Add<Vector<Metre_<T, P>>> for Segment<T, P>
where
    T: Number + Add<Output = T>,
    P: Prefix,
{
    type Output = Segment<T, P>;

    fn add(self, rhs: Vector<Metre_<T, P>>) -> Self::Output {
        Self {
            points: (self.points.0 + rhs, self.points.1 + rhs),
        }
    }
}

impl<T, P> Sub<Vector<Metre_<T, P>>> for Segment<T, P>
where
    T: Number + Sub<Output = T>,
    P: Prefix,
{
    type Output = Segment<T, P>;

    fn sub(self, rhs: Vector<Metre_<T, P>>) -> Self::Output {
        Self {
            points: (self.points.0 - rhs, self.points.1 - rhs),
        }
    }
}

impl<T, P> Mul<Vector<T>> for Segment<T, P>
where
    T: Number + Mul<Output = T>,
    P: Prefix,
{
    type Output = Segment<T, P>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Self {
            points: (self.points.0 * rhs, self.points.1 * rhs),
        }
    }
}

impl<T, P> Div<Vector<T>> for Segment<T, P>
where
    T: Number + Div<Output = T>,
    P: Prefix,
{
    type Output = Segment<T, P>;

    fn div(self, rhs: Vector<T>) -> Self::Output {
        Self {
            points: (self.points.0 / rhs, self.points.1 / rhs),
        }
    }
}

//-------------------------------------------------- ToVector --------------------------------------------------

impl<T, P> ToVector<Metre_<T, P>> for Segment<T, P>
where
    T: Number + Sub<Output = T>,
    P: Prefix,
{
    fn to_vector(&self) -> Vector<Metre_<T, P>> {
        let dx = self.points.1.x - self.points.0.x;
        let dy = self.points.1.y - self.points.0.y;
        Vector::new(dx, dy)
    }
}

//-------------------------------------------------- Length --------------------------------------------------

impl<T, P> Segment<T, P>
where
    T: Number + Sub<Output = T>,
    P: Prefix,
    Metre_<T, P>: Pow2,
    <Metre_<T, P> as Pow2>::Output: Add,
{
    pub fn length2(self) -> <<Metre_<T, P> as Pow2>::Output as Add>::Output {
        self.to_vector().length2()
    }
}

impl<T, P> Segment<T, P>
where
    T: Number + Sub<Output = T>,
    P: Prefix,
    Metre_<T, P>: Pow2,
    <Metre_<T, P> as Pow2>::Output: Add,
    <<Metre_<T, P> as Pow2>::Output as Add>::Output: Root2,
{
    pub fn length(self) -> <<<Metre_<T, P> as Pow2>::Output as Add>::Output as Root2>::Output {
        self.to_vector().length()
    }
}

//-------------------------------------------------- Parallel --------------------------------------------------

impl<T, P> Segment<T, P>
where
    T: Number,
    P: Prefix,
    <Metre_<T, P> as HasValue>::Output: Sub<Output = <Metre_<T, P> as HasValue>::Output>
        + Mul<Output = <Metre_<T, P> as HasValue>::Output>,
{
    pub fn is_parallel<U: ToVector<Metre_<T, P>>>(&self, other: &U) -> bool {
        is_parallel(self, other)
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display, P: Prefix> Display for Segment<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Segment({}, {})", self.first(), self.second())
    }
}
