use std::ops::*;

use super::*;
use sity::*;

//-------------------------------------------------- Follow --------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct FollowResult<T: Number, S: Number> {
    pub pose: Pose<T>,
    pub complete: Option<S>,
}

impl<T: Number, S: Number> FollowResult<T, S> {
    pub fn new(pose: Pose<T>, complete: Option<S>) -> Self {
        Self { pose, complete }
    }
}

pub trait Follow<T, V, S>
where
    T: Number,
    V: Number,
    S: Number,
    T: Div<V, Output = S>,
    V: Mul<S, Output = T>,
{
    fn follow(&self, velocity: V, duration: S) -> FollowResult<T, S>;
}

//-------------------------------------------------- Segment --------------------------------------------------

impl<T, V, S> Follow<T, V, S> for Segment<T>
where
    T: Number,
    V: Number,
    S: Number,
    T: Div<V, Output = S>,
    V: Mul<S, Output = T>,
    //
    T: Div<V, Output = S>,
    T: Mul<<T as HasValue>::Output, Output = T>,
    T: Div<<T as HasValue>::Output, Output = T>,
    //
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
{
    fn follow(&self, velocity: V, duration: S) -> FollowResult<T, S> {
        let direction = self.to_vector().normalized();
        let l = self.length();
        let complete_duration = l / velocity;
        //
        if complete_duration <= duration {
            let pose = Pose::new(self.second().clone(), direction);
            return FollowResult::new(pose, Some(complete_duration));
        }
        let v = direction.scale(velocity * duration);
        let point = self.first() + v;
        let pose = Pose::new(point, direction);
        FollowResult::new(pose, None)
    }
}

//-------------------------------------------------- DirectedArc --------------------------------------------------

impl<T, V, S> Follow<T, V, S> for DirectedArc<T>
where
    T: Number,
    V: Number,
    S: Number,
    T: Div<V, Output = S>,
    V: Mul<S, Output = T>,
    //
    T: Neg<Output = T>,
    T: Mul<<T as HasValue>::Output, Output = T>,
    T: Div<T>,
    <T as Div<T>>::Output: HasValue<Output = <T as HasValue>::Output>,
    //
    <T as HasValue>::Output: AngleOps,
{
    fn follow(&self, velocity: V, duration: S) -> FollowResult<T, S> {
        let l = self.length();
        let complete_duration = l / velocity;
        //
        if complete_duration <= duration {
            let direction = self.circle().tangent(self.finish_angle());
            let pose = Pose::new(self.finish_point().clone(), direction);
            return FollowResult::new(pose, Some(complete_duration));
        }
        let delta = velocity * duration / l;
        let angle = self.start_angle() + self.delta_angle() * delta.value();
        //
        let x = self.center().x + self.radius() * angle.cos();
        let y = self.center().y + self.radius() * angle.sin();
        let point = Point::new(x, y);
        let direction = self.circle().tangent(angle);
        let pose = Pose::new(point, direction);
        FollowResult::new(pose, None)
    }
}

//-------------------------------------------------- Trajectory --------------------------------------------------

impl<T, V, S> Follow<T, V, S> for Trajectory<T>
where
    T: Number,
    V: Number,
    S: Number,
    T: Div<V, Output = S>,
    V: Mul<S, Output = T>,
    //
    T: Neg<Output = T>,
    T: Mul<<T as HasValue>::Output, Output = T>,
    T: Div<<T as HasValue>::Output, Output = T>,
    T: Div<T>,
    <T as Div<T>>::Output: HasValue<Output = <T as HasValue>::Output>,
    //
    <T as HasValue>::Output: AngleOps,
    //
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
{
    fn follow(&self, velocity: V, duration: S) -> FollowResult<T, S> {
        let mut total = S::ZERO;
        // First Rotation
        let res = self.start_rotation().follow(velocity, duration - total);
        match res.complete {
            Some(d) => {
                total += d;
            }
            None => return res,
        }
        // Segement
        let res = self.segment().follow(velocity, duration - total);
        match res.complete {
            Some(d) => {
                total += d;
            }
            None => return res,
        }
        // Last Rotation
        let res = self.finish_rotation().follow(velocity, duration - total);
        match res.complete {
            Some(d) => {
                total += d;
                FollowResult::new(res.pose, Some(total))
            }
            None => res,
        }
    }
}

//-------------------------------------------------- Path --------------------------------------------------

impl<T, V, S> Follow<T, V, S> for Path<T>
where
    T: Number,
    V: Number,
    S: Number,
    T: Div<V, Output = S>,
    V: Mul<S, Output = T>,
    //
    T: Neg<Output = T>,
    T: Mul<<T as HasValue>::Output, Output = T>,
    T: Div<<T as HasValue>::Output, Output = T>,
    T: Div<T>,
    <T as Div<T>>::Output: HasValue<Output = <T as HasValue>::Output>,
    //
    <T as HasValue>::Output: AngleOps,
    //
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
{
    fn follow(&self, velocity: V, duration: S) -> FollowResult<T, S> {
        let mut total = S::ZERO;
        let mut t = duration;
        let (last, others) = self.trajectories().split_last().unwrap();
        for traj in others.iter() {
            let res = traj.follow(velocity, t);
            match res.complete {
                Some(d) => {
                    total += d;
                    t -= d;
                }
                None => return res,
            }
        }
        last.follow(velocity, t)
    }
}
