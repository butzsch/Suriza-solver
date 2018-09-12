use std::convert::From;

use data::{
    HorizontalDirection, HorizontalDirection::*, VerticalDirection,
    VerticalDirection::*,
};

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal(HorizontalDirection),
    Vertical(VerticalDirection),
}

impl Direction {
    /// `Iterator` over the four possible `Direction`s.
    pub fn iter_all() -> impl Iterator<Item = Direction> + Clone {
        vec![North.into(), East.into(), South.into(), West.into()].into_iter()
    }
}

impl From<HorizontalDirection> for Direction {
    fn from(horizontal: HorizontalDirection) -> Self {
        Direction::Horizontal(horizontal)
    }
}

impl From<VerticalDirection> for Direction {
    fn from(vertical: VerticalDirection) -> Self {
        Direction::Vertical(vertical)
    }
}
