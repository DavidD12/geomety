use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    Collinear,
    ClockWise,
    CounterClockWise,
}

pub fn points_orientation<T>(p0: &Point<T>, p1: &Point<T>, p2: &Point<T>) -> Orientation
where
    T: Number,
    T: std::ops::Mul,
    <T as std::ops::Mul>::Output: Number,
{
    let x = (p1.y - p0.y) * (p2.x - p1.x) - (p1.x - p0.x) * (p2.y - p1.y);
    if x == <T as std::ops::Mul>::Output::ZERO {
        return Orientation::Collinear;
    }
    if x > <T as std::ops::Mul>::Output::ZERO {
        return Orientation::ClockWise;
    }
    if x < <T as std::ops::Mul>::Output::ZERO {
        return Orientation::CounterClockWise;
    }
    panic!("undefined orientation")
}

// pub fn is_convex<T: Float>(polygon: &Vec<Point2D<T>>) -> bool {
//     assert!(polygon.len() >= 3);

//     let orientation = points_orientation(&polygon[0], &polygon[1], &polygon[2]);
//     for index in 1..=polygon.len() {
//         let p0 = &polygon[index];
//         let p1 = &polygon[(index + 1) % polygon.len()];
//         let p2 = &polygon[(index + 2) % polygon.len()];
//         let o = points_orientation(p0, p1, p2);
//         if o != orientation {
//             return false;
//         }
//     }
//     true
// }
