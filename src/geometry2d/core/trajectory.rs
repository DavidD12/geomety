use super::*;
use std::fmt::Display;
use std::ops::{Add, Div, DivAssign, Mul, Sub};

use sity::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Trajectory<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    start: Pose<T>,
    start_rotation: DirectedArc<T>,
    segment: Segment<T>,
    finish_rotation: DirectedArc<T>,
    finish: Pose<T>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T> Trajectory<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn new(
        start: Pose<T>,
        start_rotation: DirectedArc<T>,
        segment: Segment<T>,
        finish_rotation: DirectedArc<T>,
        finish: Pose<T>,
    ) -> Self {
        Self {
            start,
            start_rotation,
            segment,
            finish_rotation,
            finish,
        }
    }

    pub fn start(&self) -> &Pose<T> {
        &self.start
    }

    pub fn start_rotation(&self) -> &DirectedArc<T> {
        &self.start_rotation
    }

    pub fn segment(&self) -> &Segment<T> {
        &self.segment
    }

    pub fn finish_rotation(&self) -> &DirectedArc<T> {
        &self.finish_rotation
    }

    pub fn finish(&self) -> &Pose<T> {
        &self.finish
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Trajectory<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn to_value(&self) -> Trajectory<<T as HasValue>::Output> {
        Trajectory {
            start: self.start.to_value(),
            start_rotation: self.start_rotation.to_value(),
            segment: self.segment.to_value(),
            finish_rotation: self.finish_rotation.to_value(),
            finish: self.finish.to_value(),
        }
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

// TODO

//-------------------------------------------------- Ops --------------------------------------------------

// TODO

//-------------------------------------------------- Display --------------------------------------------------

impl<T> Display for Trajectory<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Trajectory({}, {}, {}, {}, {})",
            self.start, self.start_rotation, self.segment, self.finish_rotation, self.finish
        )
    }
}
