use pyo3::prelude::*;

use self::Cell::*;

/// `Enum` that represents a single cell in a Suriza puzzle. A cell contains
/// information about the number of lines that are expected to surround it in
/// order for a solution to be valid.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Any,
    Zero,
    One,
    Two,
    Three,
}

impl Cell {
    /// Returns the amount of lines that have to surround this cell in order for
    /// a solution to be valid, or `None` if the cell accepts any number of
    /// lines.
    pub fn get_expected_line_count(self) -> Option<usize> {
        match self {
            Any => None,
            Zero => 0.into(),
            One => 1.into(),
            Two => 2.into(),
            Three => 3.into(),
        }
    }
}

impl<'a> From<&'a str> for Cell {
    fn from(value: &'a str) -> Self {
        match value {
            "" | " " => Any,
            "0" => Zero,
            "1" => One,
            "2" => Two,
            "3" => Three,
            _ => panic!(),
        }
    }
}

impl<'a> FromPyObject<'a> for Cell {
    fn extract(object: &'a PyObjectRef) -> PyResult<Self> {
        let value = String::extract(object)?;
        Ok(Self::from(value.as_str()))
    }
}
