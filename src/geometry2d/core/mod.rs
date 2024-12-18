pub mod orientation;
pub use orientation::*;

pub mod rotation;
pub use rotation::*;

pub mod point;
pub use point::*;

pub mod vector;
pub use vector::*;

pub mod line;
pub use line::*;

pub mod segment;
pub use segment::*;

pub mod polygon;
pub use polygon::*;

pub mod circle;
pub use circle::*;

pub mod directed_circle;
pub use directed_circle::*;

pub mod directed_arc;
pub use directed_arc::*;

pub mod pose;
pub use pose::*;

pub mod trajectory;
pub use trajectory::*;

pub mod path;
pub use path::*;

pub mod follow;
pub use follow::*;

pub mod plot;
// pub use plot::*;

use sity::*;

//------------------------- ToVector -------------------------

// pub trait ToVector<T>
// where
//     T: Number,
// {
//     fn to_vector(&self) -> Vector<T>;
// }

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

//------------------------- Contains -------------------------

pub trait Contains<O> {
    fn contains(&self, other: &O) -> bool;
}
