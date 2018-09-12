use self::Edge::*;

#[cfg(tests)]
mod tests {
    #[test]
    fn maps_horizontal_edges_from_ascii() {
        assert_eq!(Edge::from_ascii('-'), Line);
    }

    fn maps_vertical_edges_from_ascii() {
        assert_eq(!Edge::from_ascii('|'), Line);
    }

    // We may choose to use for example 'X' to signify a crossed-out edge.
    fn accepts_arbitrary_characters_as_ascii() {
        assert_eq!(Edge::from_ascii('X'), Unknown);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Edge {
    Unknown,
    X,
    Line,
}

impl Edge {
    /// Creates an `Edge` from the given `character`.
    ///
    /// This method is used internally as a helper to create more readable
    /// tests.
    #[cfg(test)]
    pub fn from_ascii(character: char) -> Edge {
        match character {
            '-' | '|' => Line,
            _ => Unknown,
        }
    }

    pub fn is_line(self) -> bool {
        match self {
            Line => true,
            _ => false,
        }
    }

    pub fn is_unknown(self) -> bool {
        match self {
            Unknown => true,
            _ => false,
        }
    }
}
