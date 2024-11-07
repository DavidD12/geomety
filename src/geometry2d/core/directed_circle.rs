use super::*;
use sity::*;

use std::fmt::Display;
use std::ops::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DirectedCircle<T: Number> {
    circle: Circle<T>,
    direction: Direction,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> DirectedCircle<T> {
    pub fn new(circle: Circle<T>, direction: Direction) -> Self {
        Self { circle, direction }
    }

    pub fn circle(&self) -> &Circle<T> {
        &self.circle
    }

    pub fn center(&self) -> &Point<T> {
        self.circle.center()
    }

    pub fn radius(&self) -> T {
        self.circle.radius()
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> DirectedCircle<T>
where
    T: Number,
{
    pub fn to_value(&self) -> DirectedCircle<<T as HasValue>::Output> {
        DirectedCircle::new(self.circle.to_value(), self.direction)
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

//------------------------- Add -------------------------

impl<T> Add<Vector<T>> for DirectedCircle<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle + rhs,
            direction: self.direction,
        }
    }
}

impl<T> Add<Vector<T>> for &DirectedCircle<T>
where
    T: Number,
{
    type Output = DirectedCircle<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() + rhs,
            direction: self.direction,
        }
    }
}

impl<T> Add<&Vector<T>> for DirectedCircle<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() + rhs,
            direction: self.direction,
        }
    }
}

impl<T> Add<&Vector<T>> for &DirectedCircle<T>
where
    T: Number,
{
    type Output = DirectedCircle<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() + rhs,
            direction: self.direction,
        }
    }
}

//------------------------- Sub -------------------------

impl<T> Sub<Vector<T>> for DirectedCircle<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() - rhs,
            direction: self.direction,
        }
    }
}

impl<T> Sub<Vector<T>> for &DirectedCircle<T>
where
    T: Number,
{
    type Output = DirectedCircle<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() - rhs,
            direction: self.direction,
        }
    }
}

impl<T> Sub<&Vector<T>> for DirectedCircle<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() - rhs,
            direction: self.direction,
        }
    }
}

impl<T> Sub<&Vector<T>> for &DirectedCircle<T>
where
    T: Number,
{
    type Output = DirectedCircle<T>;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            circle: self.circle() - rhs,
            direction: self.direction,
        }
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> DirectedCircle<T>
where
    T: Number,
{
    pub fn translate(&mut self, dx: T, dy: T) {
        self.circle.translate(dx, dy);
    }

    pub fn translated(&self, dx: T, dy: T) -> Self {
        Self {
            circle: self.circle.translated(dx, dy),
            direction: self.direction,
        }
    }
}

//-------------------------------------------------- Contains --------------------------------------------------

impl<T> Contains<Point<T>> for DirectedCircle<T>
where
    T: Number,
    T: Mul,
    <T as Mul>::Output: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Mul>::Output: Div<T, Output = T>,
{
    fn contains(&self, point: &Point<T>) -> bool {
        self.circle.contains(point)
    }
}

//-------------------------------------------------- Tangent --------------------------------------------------

impl<T> DirectedCircle<T>
where
    // distance
    T: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    T: DivAssign<<T as HasValue>::Output>,
    T: std::ops::Neg<Output = T>,
    T: Mul<<T as HasValue>::Output, Output = T>,
{
    pub fn external_tangents_to_circle(
        &self,
        point: &Point<T>,
        direction: Direction,
    ) -> Option<Segment<T>> {
        let mut init: Vector<T> = (point, self.center()).into();
        init.normalize();
        let segment = &Segment::new(self.center().clone(), point.clone());
        //
        if self.direction == Direction::ClockWise && direction == Direction::ClockWise {
            let v = init.perpendicular_clockwise();
            let s = segment + v * self.radius().value();
            return Some(s);
        }
        if self.direction == Direction::CounterClockWise && direction == Direction::CounterClockWise
        {
            let v = init.perpendicular_counterclockwise();
            let s = segment + v * self.radius().value();
            return Some(s);
        }
        None
    }
}

impl<T> DirectedCircle<T>
where
    T: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
    T: Mul<T, Output = <T as Pow2>::Output>,
    <T as Mul>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    // o
    T: Pow3,
    <T as Pow2>::Output: Mul<T, Output = <T as Pow3>::Output>,
    T: Pow4,
    <T as Pow3>::Output: Mul<T, Output = <T as Pow4>::Output>,
    <T as Pow4>::Output: Root2<Output = <T as Pow2>::Output>,
    // xa
    T: Div<<T as HasValue>::Output, Output = T>,
    T: Mul<<T as Pow2>::Output, Output = <T as Pow3>::Output>,
    <T as HasValue>::Output: Mul<<T as Pow2>::Output, Output = <T as Pow2>::Output>,
    <T as Pow3>::Output: Div<<T as Pow2>::Output, Output = T>,
    // xb
    <T as HasValue>::Output: Mul<T, Output = T>,
{
    pub fn internal_tangents_to_circle(
        &self,
        point: &Point<T>,
        direction: Direction,
    ) -> Option<Segment<T>> {
        if self.direction == direction {
            return None;
        }
        let d = self.center().distance(point);
        if d <= self.radius() + self.radius() {
            return None;
        }

        let two = (T::ONE + T::ONE).value();
        let four = two + two;
        let quarter = (T::ONE).value() / four;
        let l = (d.pow2() - (self.radius() + self.radius()).pow2()).root2();
        let r = (l.pow2() + self.radius().pow2()).root2();
        let o = quarter
            * ((d + self.radius() + r)
                * (d + self.radius() - r)
                * (d - self.radius() + r)
                * (self.radius() + r - d))
                .root2();
        // A: self
        // x
        let xa_1 = (self.center().x + point.x) / two;
        let xa_2 =
            (point.x - self.center().x) * (self.radius().pow2() - r.pow2()) / (two * d.pow2());
        let xa_3 = two * (self.center().y - point.y) * o / d.pow2();
        // y
        let ya_1 = (self.center().y + point.y) / two;
        let ya_2 =
            (point.y - self.center().y) * (self.radius().pow2() - r.pow2()) / (two * d.pow2());
        let ya_3 = two * (self.center().x - point.x) * o / d.pow2();

        // B: point
        // x
        let xb_1 = (point.x + self.center().x) / two;
        let xb_2 =
            (self.center().x - point.x) * (self.radius().pow2() - r.pow2()) / (two * d.pow2());
        let xb_3 = two * (point.y - self.center().y) * o / d.pow2();
        // y
        let yb_1 = (point.y + self.center().y) / two;
        let yb_2 =
            (self.center().y - point.y) * (self.radius().pow2() - r.pow2()) / (two * d.pow2());
        let yb_3 = two * (point.x - self.center().x) * o / d.pow2();

        if self.direction == Direction::CounterClockWise {
            let xa1 = xa_1 + xa_2 - xa_3;
            let ya1 = ya_1 + ya_2 + ya_3;
            let pta1 = Point::new(xa1, ya1);
            let xb1 = xb_1 + xb_2 - xb_3;
            let yb1 = yb_1 + yb_2 + yb_3;
            let ptb1 = Point::new(xb1, yb1);
            let seg = Segment::new(pta1, ptb1);
            Some(seg)
        } else {
            let xa2 = xa_1 + xa_2 + xa_3;
            let ya2 = ya_1 + ya_2 - ya_3;
            let pta2 = Point::new(xa2, ya2);
            let xb2 = xb_1 + xb_2 + xb_3;
            let yb2 = yb_1 + yb_2 - yb_3;
            let ptb2 = Point::new(xb2, yb2);
            let seg = Segment::new(pta2, ptb2);
            Some(seg)
        }
    }
}

impl<T> DirectedCircle<T>
where
    T: Number,
    //
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
    //
    T: Pow3,
    <T as Pow2>::Output: Mul<T, Output = <T as Pow3>::Output>,
    <T as Pow3>::Output: Mul<T, Output = <T as Pow4>::Output>,
    <T as Pow3>::Output: Div<<T as Pow2>::Output, Output = T>,
    //
    T: Pow4,
    <T as Pow4>::Output: Root2<Output = <T as Pow2>::Output>,
{
    pub fn tangents_to_circle(&self, point: &Point<T>, direction: Direction) -> Option<Segment<T>> {
        if self.direction == direction {
            self.external_tangents_to_circle(point, direction)
        } else {
            self.internal_tangents_to_circle(point, direction)
        }
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for DirectedCircle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DirectedCircle({}, {})", self.circle, self.direction)
    }
}
