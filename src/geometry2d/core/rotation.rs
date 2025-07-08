/// Represents the direction of rotation in 2D geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Rotation in the clockwise direction.
    ClockWise,
    /// Rotation in the counter-clockwise direction.
    CounterClockWise,
}

impl Default for Direction {
    fn default() -> Self {
        Self::CounterClockWise
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::ClockWise => write!(f, "ClockWise"),
            Direction::CounterClockWise => write!(f, "CounterClockWise"),
        }
    }
}
