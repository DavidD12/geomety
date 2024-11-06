#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    ClockWise,
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
