pub mod vector;
pub use vector::*;

pub mod point;
pub use point::*;

pub mod segment;
pub use segment::*;

pub mod line;
pub use line::*;

pub mod imp;
// pub use imp::*;

//------------------------- Distance -------------------------

use sity::*;
use std::ops::*;

//------------------------- ToVector -------------------------

pub trait ToVector<T>
where
    T: Number,
{
    fn to_vector(&self) -> Vector<T>;
}

//------------------------- Distance -------------------------

pub trait Distance<T, P, Other>
where
    T: Number,
    P: Prefix,
{
    fn distance(&self, other: &Other) -> Metre_<T, P>;
    fn distance2(&self, other: &Other) -> Metre2_<T, P>;
}

//------------------------- Parallel -------------------------

// pub trait Parallel<Other> {
//     fn is_parallel(&self, other: Other) -> bool;
// }

pub fn is_parallel<T, U, V>(x: &T, y: &U) -> bool
where
    T: ToVector<V>,
    U: ToVector<V>,
    V: Number,
    <V as HasValue>::Output:
        Sub<Output = <V as HasValue>::Output> + Mul<Output = <V as HasValue>::Output>,
{
    let v1 = x.to_vector();
    let v2 = y.to_vector();
    v1.to_value().cross_product(v2.to_value()).abs() <= <V as HasValue>::Output::EPSILON
}
