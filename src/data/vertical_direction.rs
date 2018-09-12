use self::VerticalDirection::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VerticalDirection {
    North,
    South,
}

impl VerticalDirection {
    pub fn get_opposite(self) -> Self {
        match self {
            North => South,
            South => North,
        }
    }
}
