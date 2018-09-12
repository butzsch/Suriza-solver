use self::HorizontalDirection::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HorizontalDirection {
    East,
    West,
}

impl HorizontalDirection {
    pub fn get_opposite(self) -> Self {
        match self {
            East => West,
            West => East,
        }
    }
}
