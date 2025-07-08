use super::*;
use std::ops::{Div, DivAssign, Mul};

use sity::*;

/// Represents a path composed of multiple trajectories in 2D geometry.
///
/// # Type Parameters
/// * `T` - A numeric type that implements `Number` and whose value type implements `AngleOps`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Path<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    /// The list of trajectories that make up the path.
    trajectories: Vec<Trajectory<T>>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T> Path<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn new(trajectories: Vec<Trajectory<T>>) -> Self {
        Self { trajectories }
    }

    pub fn trajectories(&self) -> &Vec<Trajectory<T>> {
        &self.trajectories
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Path<T>
where
    T: Number,
    <T as HasValue>::Output: AngleOps,
{
    pub fn to_value(&self) -> Path<<T as HasValue>::Output> {
        Path {
            trajectories: self
                .trajectories
                .iter()
                .map(|traj| traj.to_value())
                .collect(),
        }
    }
}

//-------------------------------------------------- Length --------------------------------------------------

impl<T> Path<T>
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
        let mut l = T::ZERO;
        for traj in self.trajectories.iter() {
            l += traj.length();
        }
        l
    }
}

//--------------------------------------------------  --------------------------------------------------

impl<T> Path<T>
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
    <T as Pow3>::Output: Mul<T, Output = <T as Pow4>::Output>,
    //
    T: Pow4,
    <T as Pow4>::Output: Root2<Output = <T as Pow2>::Output>,
{
    pub fn create(radius: T, poses: &Vec<Pose<T>>) -> Option<Self> {
        let mut trajectories = vec![];

        for pts in poses.windows(2) {
            match Trajectory::create(&pts[0], &pts[1], radius) {
                Some(traj) => trajectories.push(traj),
                None => return None,
            };
        }

        Some(Self { trajectories })
    }
}

//--------------------------------------------------  --------------------------------------------------

impl<T> Path<T>
where
    T: Number + AngleFactory,
    //
    <T as HasValue>::Output: AngleOps,
    <T as HasValue>::Output: FromValue<usize>,
    <T as HasValue>::Output: Mul<T, Output = T>,
    <T as HasValue>::Output: Mul<<T as Pow2>::Output, Output = <T as Pow2>::Output>,
    //
    T: std::ops::Neg<Output = T>,
    //
    T: Mul<<T as HasValue>::Output, Output = T>,
    T: Mul<T, Output = <T as Pow2>::Output>,
    T: Mul<<<T as Pow2>::Output as Div>::Output, Output = T>,
    T: Mul<<T as Pow2>::Output, Output = <T as Pow3>::Output>,
    //
    T: Div<<T as HasValue>::Output, Output = T>,
    T: DivAssign<<T as HasValue>::Output>,
    //
    T: Pow2,
    <T as Pow2>::Output: Number + AngleFactory,
    <T as Pow2>::Output: Div,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Pow2>::Output: HasValue<Output = <T as HasValue>::Output>,
    <T as Pow2>::Output: Mul<T, Output = <T as Pow3>::Output>,
    <<T as Pow2>::Output as Div>::Output: Number,
    //
    T: Pow3,
    <T as Pow3>::Output: Div<<T as Pow2>::Output, Output = T>,
    <T as Pow3>::Output: Mul<T, Output = <T as Pow4>::Output>,
    //
    T: Pow4,
    <T as Pow4>::Output: Root2<Output = <T as Pow2>::Output>,
{
    pub fn mapping(
        start: &Pose<T>,
        direction: &Vector<T>,
        radius: T,
        distance: T,
        polygon: &Polygon<T>,
    ) -> Option<Self> {
        let mut segments = polygon.mapping(direction, distance);
        if segments.is_empty() {
            return None;
        }
        // Find first
        if start.position().distance(segments.first().unwrap())
            > start.position().distance(segments.last().unwrap())
        {
            segments.reverse();
        }
        // Dermine segment polarity
        // let mut rev = false;
        // if start.position().distance(segments.first().unwrap().first())
        //     > start
        //         .position()
        //         .distance(segments.first().unwrap().second())
        // {
        //     // segments = segments.iter().map(|s| s.reversed()).collect();
        //     rev = true;
        // }
        // Generate Pose
        let mut last_point = start.position().clone();
        let mut poses = vec![start.clone()];
        for segment in segments.iter() {
            let seg =
                if last_point.distance(segment.first()) <= last_point.distance(segment.second()) {
                    segment.clone()
                } else {
                    segment.reversed()
                };
            let v = seg.to_vector();
            let p = seg.first().clone();
            let pose = Pose::new(p, v.clone());
            poses.push(pose);
            let p = seg.second().clone();
            let pose = Pose::new(p, v);
            last_point = pose.position().clone();
            poses.push(pose);
        }
        // Find complete trajectory
        Self::create(radius, &poses)
    }

    pub fn optimal_mapping(
        start: &Pose<T>,
        radius: T,
        distance: T,
        polygon: &Polygon<T>,
    ) -> Option<Self> {
        let mut length = T::ZERO;
        let mut path = None;
        for seg in polygon.segments() {
            let direction = seg.to_vector();
            match Self::mapping(start, &direction, radius, distance, polygon) {
                Some(p) => {
                    let l = p.length();
                    if path.is_none() || l < length {
                        length = l;
                        path = Some(p);
                    }
                }
                None => {}
            }
        }

        path
    }
}
