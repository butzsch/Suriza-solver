use std::iter::{once, Chain, IntoIterator, Once};

use data::{
    Direction, HorizontalDirection, HorizontalDirection::*, VerticalDirection,
    VerticalDirection::*,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CornerDirection {
    pub horizontal: HorizontalDirection,
    pub vertical: VerticalDirection,
}

impl CornerDirection {
    /// Array of possible states provided for convenient iteration.
    pub const ALL: [CornerDirection; 4] = [
        CornerDirection {
            horizontal: East,
            vertical: North,
        },
        CornerDirection {
            horizontal: East,
            vertical: South,
        },
        CornerDirection {
            horizontal: West,
            vertical: South,
        },
        CornerDirection {
            horizontal: West,
            vertical: North,
        },
    ];

    pub fn get_opposite(self) -> Self {
        let Self {
            horizontal,
            vertical,
        } = self;

        Self {
            horizontal: horizontal.get_opposite(),
            vertical: vertical.get_opposite(),
        }
    }

    pub fn get_adjacent(self) -> [Self; 2] {
        let a = [
            Self {
                horizontal: East,
                vertical: South,
            },
            Self {
                horizontal: West,
                vertical: North,
            },
        ];
        let b = [
            Self {
                horizontal: East,
                vertical: North,
            },
            Self {
                horizontal: West,
                vertical: South,
            },
        ];

        if a.contains(&self) {
            b
        } else {
            a
        }
    }
}

impl IntoIterator for CornerDirection {
    type Item = Direction;
    type IntoIter = Chain<Once<Direction>, Once<Direction>>;

    fn into_iter(self) -> Self::IntoIter {
        once(Direction::Horizontal(self.horizontal))
            .chain(once(Direction::Vertical(self.vertical)))
    }
}
