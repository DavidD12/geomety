pub mod point;
pub use point::*;

pub mod vector;
pub use vector::*;

pub mod line;
pub use line::*;

pub mod segment;
pub use segment::*;

use sity::*;

//------------------------- ToVector -------------------------

pub trait ToVector<T>
where
    T: Number,
{
    fn to_vector(&self) -> Vector<T>;
}

//------------------------- Distance -------------------------

pub trait Distance<T, U>
where
    T: Number,
{
    fn distance(&self, other: &U) -> T;
}

//------------------------- Parallel -------------------------

pub trait IsParallel<O> {
    fn is_parallel(&self, other: &O) -> bool;
}

//------------------------- Intersection -------------------------

pub trait Intersection<T: Number, O> {
    fn intersection(&self, other: &O) -> Option<Point<T>>;
}
