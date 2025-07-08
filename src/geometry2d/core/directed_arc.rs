use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::*;

/// A directed arc on a circle, defined by a center, radius, direction, start angle, and delta angle.
///
/// - `circle`: The underlying directed circle (center, radius, direction).
/// - `start_angle`: The angle (in radians) where the arc starts, relative to the circle's center.
/// - `delta_angle`: The angle (in radians) the arc sweeps from the start angle. The sign and value
///    of `delta_angle` together with the direction determine the arc's extent.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    /// The underlying directed circle (center, radius, direction).
    circle: DirectedCircle<T>,
    /// The angle (in radians) where the arc starts.
    start_angle: Radian<<T as HasValue>::Output>,
    /// The angle (in radians) the arc sweeps from the start angle.
    delta_angle: Radian<<T as HasValue>::Output>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T> DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn new(
        circle: DirectedCircle<T>,
        start_angle: Radian<<T as HasValue>::Output>,
        delta_angle: Radian<<T as HasValue>::Output>,
    ) -> Self {
        Self {
            circle,
            start_angle,
            delta_angle,
        }
    }

    pub fn circle(&self) -> &DirectedCircle<T> {
        &self.circle
    }

    pub fn center(&self) -> &Point<T> {
        self.circle.center()
    }

    pub fn radius(&self) -> T {
        self.circle.radius()
    }

    pub fn start_angle(&self) -> Radian<<T as HasValue>::Output> {
        self.start_angle
    }

    pub fn delta_angle(&self) -> Radian<<T as HasValue>::Output> {
        self.delta_angle
    }

    pub fn finish_angle(&self) -> Radian<<T as HasValue>::Output> {
        if self.direction() == Direction::CounterClockWise {
            self.start_angle + self.delta_angle
        } else {
            self.start_angle - self.delta_angle
        }
    }

    pub fn direction(&self) -> Direction {
        self.circle.direction()
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

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T>
    From<(
        Circle<T>,
        Direction,
        Radian<<T as HasValue>::Output>,
        Radian<<T as HasValue>::Output>,
    )> for DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    fn from(
        value: (
            Circle<T>,
            Direction,
            Radian<<T as HasValue>::Output>,
            Radian<<T as HasValue>::Output>,
        ),
    ) -> Self {
        let circle = DirectedCircle::new(value.0, value.1);
        Self {
            circle,
            start_angle: value.2,
            delta_angle: value.3,
        }
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn to_value(&self) -> DirectedArc<<T as HasValue>::Output> {
        DirectedArc::new(self.circle.to_value(), self.start_angle, self.delta_angle)
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
        }
    }
}

//-------------------------------------------------- Length --------------------------------------------------

impl<T> DirectedArc<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
    T: Mul<<T as HasValue>::Output, Output = T>,
{
    pub fn length(&self) -> T {
        self.radius() * self.delta_angle().value()
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
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
            start_angle: self.start_angle,
            delta_angle: self.delta_angle,
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
            self.circle,
            self.start_angle.to_degrees(),
            self.delta_angle.to_degrees()
        )
    }
}
