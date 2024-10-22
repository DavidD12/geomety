use super::{core::Point, Vector};
use sity::*;

pub type Position<T, P> = Point<Metre_<T, P>>;

pub type Velocity<T, PL, PT> = Vector<Velocity_<T, PL, PT>>;