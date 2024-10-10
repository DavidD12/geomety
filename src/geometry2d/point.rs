use sity::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Point<T: Copy, P: Prefix> {
    pub x: Metre_<T, P>,
    pub y: Metre_<T, P>,
}

impl<T: Copy, P: Prefix> Point<T, P> {
    pub fn new(x: Metre_<T, P>, y: Metre_<T, P>) -> Self {
        Self { x, y }
    }
}

impl<T: Copy, P: Prefix> From<(Metre_<T, P>, Metre_<T, P>)> for Point<T, P> {
    fn from(value: (Metre_<T, P>, Metre_<T, P>)) -> Self {
        Point::new(value.0, value.1)
    }
}

impl<T: Copy, P: Prefix> Into<(Metre_<T, P>, Metre_<T, P>)> for Point<T, P> {
    fn into(self) -> (Metre_<T, P>, Metre_<T, P>) {
        (self.x, self.y)
    }
}

impl<T: Copy, P: Prefix> Into<(Metre_<T, P>, Metre_<T, P>)> for &Point<T, P> {
    fn into(self) -> (Metre_<T, P>, Metre_<T, P>) {
        (self.x, self.y)
    }
}

//-------------------------------------------------- Display --------------------------------------------------

impl<T: Copy + Display, P: Prefix> Display for Point<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}
