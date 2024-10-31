use super::*;
use sity::*;
use std::fmt::Display;
use std::ops::*;
use std::process::Output;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Circle<T: Number> {
    center: Point<T>,
    radius: T,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T: Number> Circle<T> {
    pub fn new(center: Point<T>, radius: T) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> &Point<T> {
        &self.center
    }

    pub fn radius(&self) -> T {
        self.radius
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Number> From<(Point<T>, T)> for Circle<T> {
    fn from(value: (Point<T>, T)) -> Self {
        Circle::new(value.0, value.1)
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Circle<T>
where
    T: Number,
{
    pub fn to_value(&self) -> Circle<<T as HasValue>::Output> {
        Circle::new(self.center.to_value(), self.radius.value())
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

//------------------------- Add -------------------------

impl<T> Add<Vector<T>> for Circle<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            center: self.center() + rhs,
            radius: self.radius,
        }
    }
}

impl<T> Add<Vector<T>> for &Circle<T>
where
    T: Number,
{
    type Output = Circle<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            center: self.center() + rhs,
            radius: self.radius,
        }
    }
}

impl<T> Add<&Vector<T>> for Circle<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            center: self.center() + rhs,
            radius: self.radius,
        }
    }
}

impl<T> Add<&Vector<T>> for &Circle<T>
where
    T: Number,
{
    type Output = Circle<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            center: self.center() + rhs,
            radius: self.radius,
        }
    }
}

//------------------------- Sub -------------------------

impl<T> Sub<Vector<T>> for Circle<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            center: self.center() - rhs,
            radius: self.radius,
        }
    }
}

impl<T> Sub<Vector<T>> for &Circle<T>
where
    T: Number,
{
    type Output = Circle<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            center: self.center() - rhs,
            radius: self.radius,
        }
    }
}

impl<T> Sub<&Vector<T>> for Circle<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            center: self.center() - rhs,
            radius: self.radius,
        }
    }
}

impl<T> Sub<&Vector<T>> for &Circle<T>
where
    T: Number,
{
    type Output = Circle<T>;

    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        Self::Output {
            center: self.center() - rhs,
            radius: self.radius,
        }
    }
}

//-------------------------------------------------- Translate --------------------------------------------------

impl<T> Circle<T>
where
    T: Number,
{
    pub fn translate(&self, dx: T, dy: T) -> Self {
        Self {
            center: self.center.translate(dx, dy),
            radius: self.radius,
        }
    }
}

//-------------------------------------------------- Contains --------------------------------------------------

impl<T> Contains<Point<T>> for Circle<T>
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
        self.center.distance(point) <= self.radius
    }
}

//-------------------------------------------------- Tangent --------------------------------------------------
/**
 * return external and internal tangent points if exists
 */
impl<T> Circle<T>
where
    T: Number,
    T: Mul<T>,
    <T as Mul>::Output: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Mul>::Output: Div<T, Output = T>,
    T: Div<T>,
    <T as Div<T>>::Output: AngleFactory,
    <T as Div<T>>::Output: HasValue<Output = <T as HasValue>::Output>,
    <T as HasValue>::Output: AngleOps,
    T: Mul<<T as HasValue>::Output, Output = T>,
{
    pub fn tangents(
        &self,
        other: &Circle<T>,
    ) -> (
        Option<(Segment<T>, Segment<T>)>,
        Option<(Segment<T>, Segment<T>)>,
    ) {
        let d = self.center.distance(&other.center);

        // External exists
        if d >= (self.radius() - other.radius()).abs() {
            let teta = ((other.center().y - self.center().y)
                / (other.center().x - self.center().x))
                .atan();
            println!("teta = {}", teta.to_degrees());
            let delta = ((other.radius() - self.radius()) / d).asin();
            println!("delta = {}", delta.to_degrees());
            // first
            let alpha = teta + delta;
            println!("alpha = {}", alpha.to_degrees());
            let x = self.center().x + self.radius * alpha.cos();
            let y = self.center().y + self.radius * alpha.sin();
            let p1 = Point::new(x, y);
            let x = other.center().x + other.radius * (alpha + Radian::PI).cos();
            let y = other.center().y + other.radius * (alpha + Radian::PI).sin();
            let p2 = Point::new(x, y);
            let first = Segment::new(p1, p2);
            // second
            let alpha = teta - delta;
            println!("alpha = {}", alpha.to_degrees());
            let x = self.center().x + self.radius * alpha.cos();
            let y = self.center().y + self.radius * alpha.sin();
            let p1 = Point::new(x, y);
            let x = other.center().x + other.radius * (alpha + Radian::PI).cos();
            let y = other.center().y + other.radius * (alpha + Radian::PI).sin();
            let p2 = Point::new(x, y);
            let second = Segment::new(p1, p2);
            let external = (first, second);
            // Internal exists
            if d >= self.radius() + other.radius() {
                let delta = ((self.radius() + other.radius()) / d).asin();
                // first
                let alpha = teta + delta;
                let x = self.center().x + self.radius * alpha.cos();
                let y = self.center().y + self.radius * alpha.sin();
                let p1 = Point::new(x, y);
                let x = other.center().x + other.radius * (alpha + Radian::PI).cos();
                let y = other.center().y + other.radius * (alpha + Radian::PI).sin();
                let p2 = Point::new(x, y);
                let first = Segment::new(p1, p2);
                // second
                let alpha = teta - delta;
                let x = self.center().x + self.radius * alpha.cos();
                let y = self.center().y + self.radius * alpha.sin();
                let p1 = Point::new(x, y);
                let x = other.center().x + other.radius * (alpha + Radian::PI).cos();
                let y = other.center().y + other.radius * (alpha + Radian::PI).sin();
                let p2 = Point::new(x, y);
                let second = Segment::new(p1, p2);
                let internal = (first, second);
                (Some(external), Some(internal))
            } else {
                (Some(external), None)
            }
        } else {
            (None, None)
        }
    }
}

impl<T> Circle<T>
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
    pub fn external_tangents_circle(&self, point: &Point<T>) -> (Segment<T>, Segment<T>) {
        let mut init: Vector<T> = (point, self.center()).into();
        init.normalize();
        let segment = &Segment::new(self.center.clone(), point.clone());
        //
        let v = init.perpendicular_clockwise();
        let s1 = segment + v * self.radius().value();
        //
        let v = init.perpendicular_counterclockwise();
        let s2 = segment + v * self.radius().value();
        (s1, s2)
    }
}

impl<T> Circle<T>
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
    pub fn internal_tangents_circle(&self, point: &Point<T>) -> Option<(Segment<T>, Segment<T>)> {
        let d = self.center.distance(point);

        if d > self.radius + self.radius {
            let two = (T::ONE + T::ONE).value();
            let four = two + two;
            let quarter = (T::ONE).value() / four;
            let l = (d.pow2() - (self.radius() + self.radius()).pow2()).root2();
            let r = (l.pow2() + self.radius.pow2()).root2();
            let o = quarter
                * ((d + self.radius + r)
                    * (d + self.radius - r)
                    * (d - self.radius + r)
                    * (self.radius + r - d))
                    .root2();
            //
            // x1
            let a = (self.center.x + point.x) / two;
            let b = (point.x - self.center.x) * (self.radius.pow2() - r.pow2()) / (two * d.pow2());
            let c = two * (self.center.y - point.y) * o / d.pow2();
            let xa2 = a + b + c;
            let xa1 = a + b - c;
            // y1
            let a = (self.center.y + point.y) / two;
            let b = (point.y - self.center.y) * (self.radius.pow2() - r.pow2()) / (two * d.pow2());
            let c = two * (self.center.x - point.x) * o / d.pow2();
            let ya1 = a + b + c;
            let ya2 = a + b - c;
            //
            let pta1 = Point::new(xa1, ya1);
            let pta2 = Point::new(xa2, ya2);
            // -----
            // x1
            let a = (point.x + self.center.x) / two;
            let b = (self.center.x - point.x) * (self.radius.pow2() - r.pow2()) / (two * d.pow2());
            let c = two * (point.y - self.center.y) * o / d.pow2();
            let xb1 = a + b - c;
            let xb2 = a + b + c;
            // y1
            let a = (point.y + self.center.y) / two;
            let b = (self.center.y - point.y) * (self.radius.pow2() - r.pow2()) / (two * d.pow2());
            let c = two * (point.x - self.center.x) * o / d.pow2();
            let yb1 = a + b + c;
            let yb2 = a + b - c;
            let ptb1 = Point::new(xb1, yb1);
            let ptb2 = Point::new(xb2, yb2);
            //
            let s1 = Segment::new(pta1, ptb1);
            let s2 = Segment::new(pta2, ptb2);
            Some((s1, s2))
        } else {
            None
        }
    }
}

impl<T> Circle<T>
where
    // distance
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
    pub fn internal_tangents(&self, other: &Circle<T>) -> Option<(Segment<T>, Segment<T>)> {
        let d = self.center.distance(&other.center);
        // External exists
        if d > (self.radius() - other.radius()).abs() {
            let two = (T::ONE + T::ONE).value();
            let four = two + two;
            let quarter = (T::ONE).value() / four;
            let l = (d.pow2() - (self.radius + other.radius()).pow2()).root2();
            let r = (l.pow2() + other.radius.pow2()).root2();
            let o = quarter
                * ((d + self.radius + r)
                    * (d + self.radius - r)
                    * (d - self.radius + r)
                    * (self.radius + r - d))
                    .root2();
            //
            // x1
            let a = (self.center.x + other.center.x) / two;
            let b = ((other.center.x - self.center.x) * (self.radius.pow2() - r.pow2())
                / (two * d.pow2()));
            let c = two * (self.center.y - other.center.y) * o / d.pow2();
            let x1 = a + b + c;
            // y1
            let a = (self.center.y + other.center.y) / two;
            let b = ((other.center.y - self.center.y) * (self.radius.pow2() - r.pow2())
                / (two * d.pow2()));
            let c = two * (self.center.x - other.center.x) * o / d.pow2();
            let y1 = a + b + c;
            let pt11 = Point::new(x1, y1);
            println!("pt1a = {}", pt11);
            //
            let x1 = a + b - c;
            let y1 = a + b - c;
            let pt12 = Point::new(x1, y1);
            println!("pt1a = {}", pt12);
            // -----

            // -----
            None
        } else {
            None
        }
    }
}

impl<T> Circle<T>
where
    T: Number,
    T: Mul<T>,
    <T as Mul>::Output: Number,
    T: Pow2,
    <T as Pow2>::Output: Number,
    <T as Pow2>::Output: Root2<Output = T>,
    <T as Mul>::Output: Div<T, Output = T>,
    T: Div<T>,
    <T as Div<T>>::Output: AngleFactory,
    <T as Div<T>>::Output: HasValue<Output = <T as HasValue>::Output>,
    <T as HasValue>::Output: AngleOps,
    T: Mul<<T as HasValue>::Output, Output = T>,
{
    pub fn external_tangents(&self, other: &Circle<T>) -> Option<(Segment<T>, Segment<T>)> {
        let d = self.center.distance(&other.center);

        // // Internal exists
        // if d > self.radius() + other.radius() {
        //     let teta = ((other.center().y - self.center().y)
        //         / (other.center().x - self.center().x))
        //         .atan();
        //     let delta = ((other.radius() + self.radius()) / d).asin();
        //     // first
        //     let alpha = teta + delta;
        //     let x = self.center().x + self.radius * alpha.cos();
        //     let y = self.center().y + self.radius * alpha.sin();
        //     let p1 = Point::new(x, y);
        //     let x = self.center().x + self.radius * (alpha.cos() + <T as HasValue>::Output::PI);
        //     let y = self.center().y + self.radius * (alpha.sin() + <T as HasValue>::Output::PI);
        //     let p2 = Point::new(x, y);
        //     let first = Segment::new(p1, p2);
        //     // second
        //     let alpha = teta - delta;
        //     let x = self.center().x + self.radius * alpha.cos();
        //     let y = self.center().y + self.radius * alpha.sin();
        //     let p1 = Point::new(x, y);
        //     let x = self.center().x + self.radius * (alpha.cos() + <T as HasValue>::Output::PI);
        //     let y = self.center().y + self.radius * (alpha.sin() + <T as HasValue>::Output::PI);
        //     let p2 = Point::new(x, y);
        //     let second = Segment::new(p1, p2);
        //     let internal = (first, second);
        //     Some(internal)
        // } else {
        //     None
        // }
        todo!()
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Circle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle({}, {})", self.center, self.radius)
    }
}
