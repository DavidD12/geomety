use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    circle: Circle<T>,
    angles: (
        Radian<<T as HasValue>::Output>,
        Radian<<T as HasValue>::Output>,
    ),
}

//-------------------------------------------------- New --------------------------------------------------

impl<T> DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn new(
        circle: Circle<T>,
        angles: (
            Radian<<T as HasValue>::Output>,
            Radian<<T as HasValue>::Output>,
        ),
    ) -> Self {
        Self { circle, angles }
    }

    pub fn circle(&self) -> &Circle<T> {
        &self.circle
    }

    pub fn start_angle(&self) -> Radian<<T as HasValue>::Output> {
        self.angles.0
    }

    pub fn finish_angle(&self) -> Radian<<T as HasValue>::Output> {
        self.angles.1
    }

    pub fn angle(&self) -> Radian<<T as HasValue>::Output> {
        self.angles.1 - self.angles.0
    }

    pub fn direction(&self) -> Direction {
        if self.start_angle() <= self.finish_angle() {
            Direction::CounterClockWise
        } else {
            Direction::ClockWise
        }
    }

    pub fn start_point(&self) -> Point<T>
    where
        T: Mul<<T as HasValue>::Output, Output = T>,
    {
        let x = self.circle.center().x + self.circle.radius() * self.start_angle().cos();
        let y = self.circle.center().y + self.circle.radius() * self.start_angle().sin();
        Point::new(x, y)
    }

    pub fn finish_point(&self) -> Point<T>
    where
        T: Mul<<T as HasValue>::Output, Output = T>,
    {
        let x = self.circle.center().x + self.circle.radius() * self.finish_angle().cos();
        let y = self.circle.center().y + self.circle.radius() * self.finish_angle().sin();
        Point::new(x, y)
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn to_value(&self) -> DirectedArc<<T as HasValue>::Output> {
        DirectedArc::new(self.circle.to_value(), self.angles)
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn translate(&mut self, dx: T, dy: T) {
        self.circle.translate(dx, dy);
    }

    pub fn translated(&self, dx: T, dy: T) -> Self {
        Self {
            circle: self.circle.translated(dx, dy),
            angles: self.angles,
        }
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

//------------------------- Add -------------------------

impl<T> Add<Vector<T>> for DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() + rhs,
            angles: self.angles,
        }
    }
}

impl<T> Add<Vector<T>> for &DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    type Output = DirectedArc<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() + rhs,
            angles: self.angles,
        }
    }
}

impl<T> Add<&Vector<T>> for DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    type Output = Self;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() + rhs,
            angles: self.angles,
        }
    }
}

impl<T> Add<&Vector<T>> for &DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    type Output = DirectedArc<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() + rhs,
            angles: self.angles,
        }
    }
}

//------------------------- Sub -------------------------

impl<T> Sub<Vector<T>> for DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() - rhs,
            angles: self.angles,
        }
    }
}

impl<T> Sub<Vector<T>> for &DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    type Output = DirectedArc<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() - rhs,
            angles: self.angles,
        }
    }
}

impl<T> Sub<&Vector<T>> for DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    type Output = Self;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() - rhs,
            angles: self.angles,
        }
    }
}

impl<T> Sub<&Vector<T>> for &DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    type Output = DirectedArc<T>;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() - rhs,
            angles: self.angles,
        }
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T> Display for DirectedArc<T>
where
    T: Number + Display,
    <T as HasValue>::Output: AngleOps,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DirectedArc({}, ({}, {}))",
            self.circle, self.angles.0, self.angles.1
        )
    }
}
