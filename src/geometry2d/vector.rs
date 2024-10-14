use super::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use sity::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector<T>
where
    T: Number,
{
    pub dx: T,
    pub dy: T,
}

//-------------------------------------------------- New --------------------------------------------------

impl<T> Vector<T>
where
    T: Number,
{
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }
}

//-------------------------------------------------- From/Into --------------------------------------------------

impl<T: Number> From<(T, T)> for Vector<T> {
    fn from(value: (T, T)) -> Self {
        Vector::new(value.0, value.1)
    }
}

impl<T: Number> Into<(T, T)> for Vector<T> {
    fn into(self) -> (T, T) {
        (self.dx, self.dy)
    }
}

impl<T: Number> Into<(T, T)> for &Vector<T> {
    fn into(self) -> (T, T) {
        (self.dx, self.dy)
    }
}

//-------------------------------------------------- ToValue --------------------------------------------------

impl<T> Vector<T>
where
    T: Number + HasValue,
{
    pub fn to_value(&self) -> Vector<<T as HasValue>::Output> {
        Vector::new(self.dx.value(), self.dy.value())
    }
}

//-------------------------------------------------- Ops --------------------------------------------------

impl<T> Add<T> for Vector<T>
where
    T: Number + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            dx: self.dx + rhs,
            dy: self.dy + rhs,
        }
    }
}

impl<T> Add for Vector<T>
where
    T: Number + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl<T> Sub<T> for Vector<T>
where
    T: Number + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        Self {
            dx: self.dx - rhs,
            dy: self.dy - rhs,
        }
    }
}

impl<T> Sub for Vector<T>
where
    T: Number + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
        }
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Number + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl<T, U> Mul for Vector<T>
where
    T: Number + Mul<Output = U>,
    U: Number,
{
    type Output = Vector<U>;

    fn mul(self, rhs: Self) -> Vector<U> {
        Vector::new(self.dx * rhs.dx, self.dy * rhs.dy)
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Number + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

impl<T, U> Div for Vector<T>
where
    T: Number + Div<Output = U>,
    U: Number,
{
    type Output = Vector<U>;

    fn div(self, rhs: Self) -> Vector<U> {
        Vector::new(self.dx / rhs.dx, self.dy / rhs.dy)
    }
}

//-------------------------------------------------- Length --------------------------------------------------

impl<T> Vector<T>
where
    T: Number + Pow2,
    <T as Pow2>::Output: Add,
{
    pub fn length2(self) -> <<T as Pow2>::Output as Add>::Output {
        self.dx.pow2() + self.dy.pow2()
    }
}

impl<T> Vector<T>
where
    T: Number + Pow2,
    <T as Pow2>::Output: Add,
    <<T as Pow2>::Output as Add>::Output: Root2,
{
    pub fn length(self) -> <<<T as Pow2>::Output as Add>::Output as Root2>::Output {
        (self.dx.pow2() + self.dy.pow2()).root2()
    }
}

//-------------------------------------------------- Normalize --------------------------------------------------

impl<T> Vector<T>
where
    T: Number
        + Pow2
        + Div<
            <<<<T as Pow2>::Output as Add>::Output as Root2>::Output as HasValue>::Output,
            Output = T,
        >,
    <T as Pow2>::Output: Add,
    <<T as Pow2>::Output as Add>::Output: Root2,
    <<<T as Pow2>::Output as Add>::Output as Root2>::Output: HasValue,
{
    pub fn normalize(&self) -> Self {
        let length = self.length().value();
        Self {
            dx: self.dx / length,
            dy: self.dy / length,
        }
    }
}

//-------------------------------------------------- Product --------------------------------------------------

impl<T> Vector<T>
where
    T: Number + Add<Output = T> + Mul<Output = T>,
{
    pub fn dot_product(&self, other: Self) -> T {
        self.dx * other.dx + self.dy * other.dy
    }
}

impl<T> Vector<T>
where
    T: Number + Sub<Output = T> + Mul<Output = T>,
{
    pub fn cross_product(&self, other: Self) -> T {
        self.dx * other.dy - self.dy * other.dx
    }
}

//-------------------------------------------------- ToVector --------------------------------------------------

impl<T> ToVector<T> for Vector<T>
where
    T: Number,
{
    fn to_vector(&self) -> Vector<T> {
        *self
    }
}

//-------------------------------------------------- Parallel --------------------------------------------------

impl<T> Vector<T>
where
    T: Number,
    <T as HasValue>::Output:
        Sub<Output = <T as HasValue>::Output> + Mul<Output = <T as HasValue>::Output>,
{
    pub fn is_parallel<U: ToVector<T>>(&self, other: &U) -> bool {
        is_parallel(self, other)
    }
}

// impl<T, U> Parallel<U> for Vector<T>
// where
//     T: Number,
//     U: ToVector<T>,
//     <T as HasValue>::Output:
//         Sub<Output = <T as HasValue>::Output> + Mul<Output = <T as HasValue>::Output>,
// {
//     fn is_parallel(&self, other: U) -> bool {
//         let v = other.to_vector();
//         self.to_value().cross_product(v.to_value()).abs() <= <T as HasValue>::Output::EPSILON
//     }
// }

// impl<T, P> Parallel<Line<T, P>> for Vector<Metre_<T, P>>
// where
//     T: Number + Sub<Output = T> + Mul<Output = T>,
//     P: Prefix,
// {
//     fn is_parallel(&self, other: Line<T, P>) -> bool {
//         self.is_parallel(other.vector)
//     }
// }

// impl<T, P> Parallel<Segment<T, P>> for Vector<Metre_<T, P>>
// where
//     T: Number + Sub<Output = T> + Mul<Output = T>,
//     P: Prefix,
// {
//     fn is_parallel(&self, other: Segment<T, P>) -> bool {
//         self.is_parallel(other.to_vector())
//     }
// }

// --------------------------------------------------
// TODO: ajouter les suivantes ?
// Est-ce semantiquement correct ?
// --------------------------------------------------

// impl<T, P> Parallel<Line<T, P>> for Vector<T>
// where
//     T: Number + Sub<Output = T> + Mul<Output = T>,
//     P: Prefix,
// {
//     fn is_parallel(&self, other: Line<T, P>) -> bool {
//         let v = other.vector.to_value();
//         self.is_parallel(v)
//     }
// }

// impl<P, U> Parallel<Line<<U as HasValue>::Output, P>> for Vector<U>
// where
//     <U as HasValue>::Output:
//         Number + Sub<Output = <U as HasValue>::Output> + Mul<Output = <U as HasValue>::Output>,
//     U: Number,
//     P: Prefix,
// {
//     fn is_parallel(&self, other: Line<<U as HasValue>::Output, P>) -> bool {
//         let ov = other.vector.to_value();
//         let sv = self.to_value();
//         sv.is_parallel(ov)
//     }
// }

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Number + Display> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({}, {})", self.dx, self.dy)
    }
}
