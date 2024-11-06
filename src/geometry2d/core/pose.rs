use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pose<T: Number> {
    position: Point<T>,
    orientation: Vector<T>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Pose<T> {
    pub fn new(position: Point<T>, orientation: Vector<T>) -> Self {
        Self {
            position,
            orientation,
        }
    }

    pub fn position(&self) -> &Point<T> {
        &self.position
    }

    pub fn orientation(&self) -> &Vector<T> {
        &self.orientation
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Pose<T>
where
    T: Number,
{
    pub fn to_value(&self) -> Pose<<T as HasValue>::Output> {
        Pose {
            position: self.position.to_value(),
            orientation: self.orientation.to_value(),
        }
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> Pose<T>
where
    T: Number,
{
    pub fn translate(&mut self, dx: T, dy: T) {
        self.position.translate(dx, dy);
    }

    pub fn translated(&self, dx: T, dy: T) -> Self {
        Self {
            position: self.position.translated(dx, dy),
            orientation: self.orientation.clone(),
        }
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

//------------------------- Add -------------------------

impl<T> Add<Vector<T>> for Pose<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            position: self.position + rhs,
            orientation: self.orientation.clone(),
        }
    }
}

impl<T> Add<Vector<T>> for &Pose<T>
where
    T: Number,
{
    type Output = Pose<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            position: self.position() + rhs,
            orientation: self.orientation.clone(),
        }
    }
}

impl<T> Add<&Vector<T>> for Pose<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            position: self.position + rhs,
            orientation: self.orientation.clone(),
        }
    }
}

impl<T> Add<&Vector<T>> for &Pose<T>
where
    T: Number,
{
    type Output = Pose<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            position: self.position() + rhs,
            orientation: self.orientation.clone(),
        }
    }
}

//------------------------- Sub -------------------------

impl<T> Sub<Vector<T>> for Pose<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            position: self.position - rhs,
            orientation: self.orientation.clone(),
        }
    }
}

impl<T> Sub<Vector<T>> for &Pose<T>
where
    T: Number,
{
    type Output = Pose<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            position: self.position() - rhs,
            orientation: self.orientation.clone(),
        }
    }
}

impl<T> Sub<&Vector<T>> for Pose<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            position: self.position - rhs,
            orientation: self.orientation.clone(),
        }
    }
}

impl<T> Sub<&Vector<T>> for &Pose<T>
where
    T: Number,
{
    type Output = Pose<T>;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            position: self.position() - rhs,
            orientation: self.orientation.clone(),
        }
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Pose<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pose({}, {})", self.position, self.orientation)
    }
}
