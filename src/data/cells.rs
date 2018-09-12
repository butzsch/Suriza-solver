#[cfg(test)]
extern crate unindent;

#[cfg(test)]
use std::iter::repeat;

use std::ops::Index;

use pyo3::prelude::*;

use data::{Cell, CellIndex, Size};

#[cfg(test)]
mod tests {
    use super::Cell::*;
    use super::*;

    #[test]
    fn correctly_maps_from_ascii() {
        let Cells { rows } = Cells::from_ascii(
            "
            + + + + +
             1   2
            + + + + +
             3   1 0
            + + + + +
        ",
        );

        let expected_rows =
            vec![vec![One, Any, Two, Any], vec![Three, Any, One, Zero]];

        assert_eq!(rows, expected_rows);
    }

    #[test]
    fn returns_correct_size() {
        let cells = {
            let rows = vec![vec![Any; 4]; 2];
            Cells { rows }
        };

        assert_eq!(
            cells.get_size(),
            Size {
                width: 4,
                height: 2
            }
        );
    }
}

/// A struct representing a rectangular grid of cells forming a Suriza puzzle.
#[derive(Clone, Debug)]
pub struct Cells {
    rows: Vec<Vec<Cell>>,
}

impl Cells {
    /// Creates a `Cells` instance from a `&str` containing an ASCII-image
    /// representing the numbers in the grid.
    ///
    /// This method is used internally in this crate to create more readable
    /// test cases. For convenient usage this method removes any indentation
    /// from the input string before further processing.
    /// No effort is put into making this function detect and report any invalid
    /// inputs.
    ///
    /// # Panics
    ///
    /// May panic on unexpected inputs, but does not guarantee to do so.
    #[cfg(test)]
    pub fn from_ascii(input: &str) -> Cells {
        let trimmed_input = unindent::unindent(input);
        let mut lines = trimmed_input.lines();

        // Input is not valid if it does not have at least one row.
        let first_line = lines.next().unwrap();

        // The first line might look like this "+ + + + +". We care for the
        // number of fields, which is equal to the number of spaces, which in
        // turn is equal to the number of characters in the first line divided
        // by two, rounded towards zero.
        let numbers_per_row = first_line.len() / 2;

        let rows = lines
            .step_by(2) // Skip the lines containing the horizontal edges.
            .map(|line| {
                line.chars()
                    .skip(1) // Skip the columns containing the vertical edges.
                    .step_by(2) //
                    .map(|value| Cell::from(value.to_string().as_str()))
                    .chain(repeat(Cell::Any)) // Fill up cells for short lines so
                    .take(numbers_per_row) // all rows have the same width.
                    .collect()
            })
            .collect();

        Cells { rows }
    }

    /// Returns the `Size` of the `Cells` instance.
    pub fn get_size(&self) -> Size {
        let height = self.rows.len();
        let width = self.rows[0].len();

        Size { width, height }
    }

    /// Returns an `Iterator` over the indices to all cells in this `Cell`
    /// instance.
    pub fn index_cells(&self) -> impl Iterator<Item = CellIndex> {
        let Size { width, height } = self.get_size();

        iproduct!(0..height, 0..width)
            .map(|(row, column)| CellIndex { row, column })
    }
}

impl<'a> Index<&'a CellIndex> for Cells {
    type Output = Cell;

    fn index(&self, &CellIndex { row, column }: &CellIndex) -> &Cell {
        &self.rows[row][column]
    }
}

impl<'a> FromPyObject<'a> for Cells {
    fn extract(object: &'a PyObjectRef) -> PyResult<Self> {
        let rows = Vec::<_>::extract(object)?;
        Ok(Self { rows })
    }
}
