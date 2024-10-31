use std::cmp::Ordering;
use std::ops::*;

use super::*;
use sity::*;
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Polygon<T: Number> {
    points: Vec<Point<T>>,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Polygon<T> {
    pub fn points(&self) -> &Vec<Point<T>> {
        &self.points
    }
}

impl<T> Polygon<T>
where
    T: Number + AngleFactory,
    T: Mul,
    <T as Mul>::Output: Number,
    T: Pow2<Output = <T as Mul>::Output>,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Mul>::Output: Div,
    <<T as Mul>::Output as Div>::Output: Number,
    T: Mul<<<T as Mul>::Output as Div>::Output, Output = T>,
{
    pub fn convex_hull(mut points: Vec<Point<T>>) -> Option<Self> {
        if points.len() < 3 {
            return None;
        }

        // Find first point: reverse(lower y and then x)
        points.sort_by(|p1, p2| {
            if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
                Ordering::Greater
            } else if p1.x == p2.x && p1.y == p2.y {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        });
        let first = points.pop().unwrap();

        // Sort other points by angle with the first point
        points.sort_by(|a, b| {
            let order = first
                .angle(a)
                .partial_cmp(&first.angle(b))
                .unwrap_or(Ordering::Equal);
            if order == Ordering::Equal {
                first
                    .distance(a)
                    .partial_cmp(&first.distance(b))
                    .unwrap_or(Ordering::Equal)
            } else {
                order
            }
        });

        // create convex hull
        let second = points.pop().unwrap();
        let mut hull = vec![first, second];

        while !points.is_empty() {
            let point = points.pop().unwrap();
            while hull.len() > 1
                && points_orientation(&hull[hull.len() - 2], &hull[hull.len() - 1], &point)
                    != Orientation::ClockWise
            {
                hull.pop();
            }
            hull.push(point);
        }

        if hull.len() < 3 {
            return None;
        }

        Some(Self { points: hull })
    }
}

//--------------------------------------------------  --------------------------------------------------

impl<T> Polygon<T>
where
    T: Number,
{
    pub fn segments(&self) -> Vec<Segment<T>> {
        let mut segments = self
            .points
            .windows(2)
            .map(|pts| Segment::new(pts[0].clone(), pts[1].clone()))
            .collect::<Vec<_>>();
        segments.push(Segment::new(
            self.points.last().unwrap().clone(),
            self.points.first().unwrap().clone(),
        ));
        segments
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Polygon<T>
where
    T: Number,
{
    pub fn to_value(&self) -> Polygon<<T as HasValue>::Output> {
        let points = self
            .points
            .iter()
            .map(|pt| pt.to_value())
            .collect::<Vec<_>>();
        Polygon { points }
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

//------------------------- Add -------------------------

impl<T> Add<Vector<T>> for Polygon<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            points: self.points.iter().map(|pt| pt + &rhs).collect(),
        }
    }
}

impl<T> Add<Vector<T>> for &Polygon<T>
where
    T: Number,
{
    type Output = Polygon<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            points: self.points.iter().map(|pt| pt + &rhs).collect(),
        }
    }
}

impl<T> Add<&Vector<T>> for Polygon<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            points: self.points.iter().map(|pt| pt + rhs).collect(),
        }
    }
}

impl<T> Add<&Vector<T>> for &Polygon<T>
where
    T: Number,
{
    type Output = Polygon<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            points: self.points.iter().map(|pt| pt + rhs).collect(),
        }
    }
}

//------------------------- Sub -------------------------

impl<T> Sub<Vector<T>> for Polygon<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            points: self.points.iter().map(|pt| pt - &rhs).collect(),
        }
    }
}

impl<T> Sub<Vector<T>> for &Polygon<T>
where
    T: Number,
{
    type Output = Polygon<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            points: self.points.iter().map(|pt| pt - &rhs).collect(),
        }
    }
}

impl<T> Sub<&Vector<T>> for Polygon<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            points: self.points.iter().map(|pt| pt - rhs).collect(),
        }
    }
}

impl<T> Sub<&Vector<T>> for &Polygon<T>
where
    T: Number,
{
    type Output = Polygon<T>;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            points: self.points.iter().map(|pt| pt - rhs).collect(),
        }
    }
}

//-------------------------------------------------- Perimeter --------------------------------------------------

impl<T> Polygon<T>
where
    T: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
{
    pub fn perimeter2(self) -> <T as Pow2>::Output {
        if self.points.len() < 2 {
            return <T as Pow2>::Output::ZERO;
        }
        let init = self
            .points
            .first()
            .unwrap()
            .distance2(&self.points.last().unwrap());
        self.points
            .windows(2)
            .fold(init, |l, pts| l + pts[0].distance2(&pts[1]))
    }
}

impl<T> Polygon<T>
where
    T: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
{
    pub fn perimeter(self) -> T {
        if self.points.len() < 2 {
            return T::ZERO;
        }
        let init = self
            .points
            .first()
            .unwrap()
            .distance(&self.points.last().unwrap());
        self.points
            .windows(2)
            .fold(init, |l, pts| l + pts[0].distance(&pts[1]))
    }
}

//-------------------------------------------------- Contains --------------------------------------------------

impl<T> Contains<Point<T>> for Polygon<T>
where
    T: Number,
    T: std::ops::Mul,
    <T as std::ops::Mul>::Output: Number,
{
    fn contains(&self, point: &Point<T>) -> bool {
        let mut orientation = None;
        let mut previous = self.points.last().unwrap();

        for current in self.points.iter() {
            let new_o = points_orientation(previous, current, point);
            match orientation {
                Some(o) => {
                    if new_o != o {
                        return false;
                    }
                }
                None => orientation = Some(new_o),
            }
            previous = current;
        }
        true
    }
}

//-------------------------------------------------- Distance --------------------------------------------------
