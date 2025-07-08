use super::*;
use std::fmt::Display;
use std::ops::{Div, DivAssign, Mul};

use sity::*;

/// Represents a 2D trajectory consisting of two arcs (start and finish rotations)
/// connected by a straight segment. The trajectory starts at `start` pose,
/// follows an arc (`start_rotation`), then a straight segment (`segment`),
/// then another arc (`finish_rotation`), and ends at `finish` pose.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Trajectory<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    /// The starting pose of the trajectory.
    start: Pose<T>,
    /// The arc representing the initial rotation from the start pose.
    start_rotation: DirectedArc<T>,
    /// The straight segment connecting the two arcs.
    segment: Segment<T>,
    /// The arc representing the final rotation to the finish pose.
    finish_rotation: DirectedArc<T>,
    /// The ending pose of the trajectory.
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

//-------------------------------------------------- Length --------------------------------------------------

impl<T> Trajectory<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
    // DirectecArc
    T: Mul<<T as HasValue>::Output, Output = T>,
    // Segment
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
{
    pub fn length(&self) -> T {
        self.start_rotation.length() + self.segment.length() + self.finish_rotation.length()
    }
}

//--------------------------------------------------  --------------------------------------------------

impl<T> Trajectory<T>
where
    T: Number + AngleFactory,
    //
    <T as HasValue>::Output: AngleOps,
    <T as HasValue>::Output: Mul<T, Output = T>,
    <T as HasValue>::Output: Mul<<T as Pow2>::Output, Output = <T as Pow2>::Output>,
    //
    T: std::ops::Neg<Output = T>,
    //
    T: Mul<<T as HasValue>::Output, Output = T>,
    T: Mul<T, Output = <T as Pow2>::Output>,
    T: Mul<<T as Pow2>::Output, Output = <T as Pow3>::Output>,
    //
    T: Div<<T as HasValue>::Output, Output = T>,
    T: DivAssign<<T as HasValue>::Output>,
    //
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Pow2>::Output: Number + AngleFactory,
    <T as Pow2>::Output: HasValue<Output = <T as HasValue>::Output>,
    <T as Pow2>::Output: Mul<T, Output = <T as Pow3>::Output>,
    //
    T: Pow3,
    <T as Pow3>::Output: Div<<T as Pow2>::Output, Output = T>,
    //
    T: Pow4,
    <T as Pow3>::Output: Mul<T, Output = <T as Pow4>::Output>,
    <T as Pow4>::Output: Root2<Output = <T as Pow2>::Output>,
{
    pub fn create_all(start: &Pose<T>, finish: &Pose<T>, radius: T) -> Vec<Self> {
        // Start Clock
        let start_clock = {
            let v = start.orientation().perpendicular_clockwise().scale(radius);
            let p = start.position() + v;
            let c = Circle::new(p, radius);
            DirectedCircle::new(c, Direction::ClockWise)
        };
        // Start Counter
        let start_counter = {
            let v = start
                .orientation()
                .perpendicular_counterclockwise()
                .scale(radius);
            let p = start.position() + v;
            let c = Circle::new(p, radius);
            DirectedCircle::new(c, Direction::CounterClockWise)
        };

        // Finish Clock
        let finish_clock = {
            let v = finish.orientation().perpendicular_clockwise().scale(radius);
            let p = finish.position() + v;
            let c = Circle::new(p, radius);
            DirectedCircle::new(c, Direction::ClockWise)
        };
        // Finish Counter
        let finish_counter = {
            let v = finish
                .orientation()
                .perpendicular_counterclockwise()
                .scale(radius);
            let p = finish.position() + v;
            let c = Circle::new(p, radius);
            DirectedCircle::new(c, Direction::CounterClockWise)
        };
        //
        let mut res = vec![];
        // Clock -> Clock
        if let Some(seg) =
            start_clock.tangents_to_circle(finish_clock.center(), finish_clock.direction())
        {
            // Start
            let start_angle = start_clock.center().angle_from_point(start.position());
            let mut delta_angle =
                Point::angle_from_points(seg.first(), start_clock.center(), start.position());
            if delta_angle < Radian::ZERO {
                delta_angle += Radian::PI + Radian::PI;
            }
            let start_arc = DirectedArc::new(start_clock.clone(), start_angle, delta_angle);
            // Finish
            let finish_angle = finish_clock.center().angle_from_point(seg.second());
            let mut delta_angle =
                Point::angle_from_points(finish.position(), finish_clock.center(), seg.second());
            if delta_angle < Radian::ZERO {
                delta_angle += Radian::PI + Radian::PI;
            }
            let finish_arc = DirectedArc::new(finish_clock.clone(), finish_angle, delta_angle);
            //
            let traj = Trajectory::new(start.clone(), start_arc, seg, finish_arc, finish.clone());
            res.push(traj);
        }
        // Counter -> Counter
        if let Some(seg) =
            start_counter.tangents_to_circle(finish_counter.center(), finish_counter.direction())
        {
            // Start
            let start_angle = start_counter.center().angle_from_point(start.position());
            let mut delta_angle =
                Point::angle_from_points(start.position(), start_counter.center(), seg.first());
            if delta_angle < Radian::ZERO {
                delta_angle += Radian::PI + Radian::PI;
            }
            let start_arc = DirectedArc::new(start_counter.clone(), start_angle, delta_angle);
            // Finish
            let finish_angle = finish_counter.center().angle_from_point(seg.second());
            let mut delta_angle =
                Point::angle_from_points(seg.second(), finish_counter.center(), finish.position());
            if delta_angle < Radian::ZERO {
                delta_angle += Radian::PI + Radian::PI;
            }
            let finish_arc = DirectedArc::new(finish_counter.clone(), finish_angle, delta_angle);
            //
            let traj = Trajectory::new(start.clone(), start_arc, seg, finish_arc, finish.clone());
            res.push(traj);
        }
        // Clock -> Counter
        if let Some(seg) =
            start_clock.tangents_to_circle(finish_counter.center(), finish_counter.direction())
        {
            // Start
            let start_angle = start_clock.center().angle_from_point(start.position());
            let mut delta_angle =
                Point::angle_from_points(seg.first(), start_clock.center(), start.position());
            if delta_angle < Radian::ZERO {
                delta_angle += Radian::PI + Radian::PI;
            }
            let start_arc = DirectedArc::new(start_clock.clone(), start_angle, delta_angle);
            // Finish
            let finish_angle = finish_counter.center().angle_from_point(seg.second());
            let mut delta_angle =
                Point::angle_from_points(seg.second(), finish_counter.center(), finish.position());
            if delta_angle < Radian::ZERO {
                delta_angle += Radian::PI + Radian::PI;
            }
            let finish_arc = DirectedArc::new(finish_counter.clone(), finish_angle, delta_angle);
            //
            let traj = Trajectory::new(start.clone(), start_arc, seg, finish_arc, finish.clone());
            res.push(traj);
        }
        // Counter -> Clock
        if let Some(seg) =
            start_counter.tangents_to_circle(finish_clock.center(), finish_clock.direction())
        {
            // Start
            let start_angle = start_counter.center().angle_from_point(start.position());
            let mut delta_angle =
                Point::angle_from_points(start.position(), start_counter.center(), seg.first());
            if delta_angle < Radian::ZERO {
                delta_angle += Radian::PI + Radian::PI;
            }
            let start_arc = DirectedArc::new(start_counter.clone(), start_angle, delta_angle);
            // Finish
            let finish_angle = finish_clock.center().angle_from_point(seg.second());
            let mut delta_angle =
                Point::angle_from_points(finish.position(), finish_clock.center(), seg.second());
            if delta_angle < Radian::ZERO {
                delta_angle += Radian::PI + Radian::PI;
            }
            let finish_arc = DirectedArc::new(finish_clock.clone(), finish_angle, delta_angle);
            //
            let traj = Trajectory::new(start.clone(), start_arc, seg, finish_arc, finish.clone());
            res.push(traj);
        }

        //
        res
    }

    pub fn create(start: &Pose<T>, finish: &Pose<T>, radius: T) -> Option<Self> {
        let mut length = T::ZERO;
        let mut res = None;

        for traj in Self::create_all(start, finish, radius) {
            let l = traj.length();
            if res.is_none() || l < length {
                length = l;
                res = Some(traj)
            }
        }

        res
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
