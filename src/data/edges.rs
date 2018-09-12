extern crate boolinator;
extern crate itertools;

#[cfg(test)]
extern crate unindent;

#[cfg(test)]
use std::iter::repeat;

use std::iter::once;
use std::mem::replace;
use std::ops::{Index, IndexMut};

use self::boolinator::Boolinator;

use data::{
    CellIndex, CornerDirection, Direction, Edge, Edge::*, EdgeDirection::*,
    EdgeIndex, HorizontalDirection::*, IntersectionIndex, Size,
    VerticalDirection::*,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_members_to_correct_size_when_creating_empty() {
        let (width, height) = (10, 20);

        let Edges {
            horizontal,
            vertical,
        } = Edges::create_empty(&Size { width, height });

        assert_eq!(horizontal.len(), height + 1);
        assert_eq!(vertical.len(), height);

        assert!(horizontal
            .iter()
            .all(|horizontal_row| horizontal_row.len() == width));
        assert!(vertical
            .iter()
            .all(|vertical_row| vertical_row.len() == width + 1));
    }

    #[test]
    fn initializes_all_edges_to_be_no_lines_when_creating_empty() {
        let size = Size {
            width: 10,
            height: 20,
        };
        let Edges {
            horizontal,
            vertical,
        } = Edges::create_empty(&size);

        let there_are_lines = [horizontal, vertical]
            .iter()
            .flat_map(|rows| rows.iter())
            .flat_map(|row| row.iter())
            .any(|edge| edge.is_line());

        assert!(!there_are_lines);
    }

    #[test]
    #[should_panic]
    fn panics_if_the_width_is_zero_when_creating_empty() {
        Edges::create_empty(&Size {
            width: 0,
            height: 1,
        });
    }

    #[test]
    #[should_panic]
    fn panics_if_the_height_is_zero_when_creating_empty() {
        Edges::create_empty(&Size {
            width: 1,
            height: 0,
        });
    }

    #[test]
    fn creates_correctly_sized_empty_grid_from_ascii() {
        let edges = Edges::from_ascii(
            "
            + + + +

            + + + +

            + + + +
        ",
        );

        assert_eq!(
            edges,
            Edges::create_empty(&Size {
                width: 3,
                height: 2
            })
        );
    }

    #[test]
    fn correctly_maps_horizontal_lines() {
        let edges = Edges::from_ascii(
            "
            +-+ +-+

            +-+-+ +

            + + + +
        ",
        );
        let expected_line_states = vec![
            vec![Line, Unknown, Line],
            vec![Line, Line, Unknown],
            vec![Unknown, Unknown, Unknown],
        ];

        let correctly_mapped = iproduct!(0..2, 0..3).all(|(row, column)| {
            edges.horizontal[row][column] == expected_line_states[row][column]
        });

        assert!(correctly_mapped);
    }

    #[test]
    fn correctly_maps_vertical_lines() {
        let edges = Edges::from_ascii(
            "
            + + + +
            |   |
            + + + +
                | |
            + + + +
        ",
        );

        let expected_line_states = vec![
            vec![Line, Unknown, Line, Unknown],
            vec![Unknown, Unknown, Line, Line],
        ];

        let correctly_mapped = iproduct!(0..2, 0..3).all(|(row, column)| {
            edges.vertical[row][column] == expected_line_states[row][column]
        });

        assert!(correctly_mapped);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edges {
    horizontal: Vec<Vec<Edge>>,
    vertical: Vec<Vec<Edge>>,
}

impl Edges {
    /// Creates an `Edges` with none of the edges set to be a line.
    ///
    /// # Panics
    ///
    /// Panics if either the `width` or `height` of the given `Size` are equal
    /// to zero.
    pub fn create_empty(&Size { width, height }: &Size) -> Edges {
        assert!(width != 0);
        assert!(height != 0);

        let horizontal = vec![vec![Unknown; width]; height + 1];
        let vertical = vec![vec![Unknown; width + 1]; height];

        Edges {
            horizontal,
            vertical,
        }
    }

    /// Creates an `Edges` from a `&str` containing an ASCII-image
    /// representing the edges in the grid.
    ///
    /// This method is used internally in this crate to create more readable
    /// test cases. For convenient usage this method removes any indentation
    /// from the input string before further processing. Horizontal lines are
    /// represented by a '-', vertical lines by a '|' character.
    /// No effort is put into making this function detect and report any invalid
    /// inputs.
    ///
    /// # Panics
    ///
    /// May panic on unexpected inputs, but does not guarantee to do so.
    #[cfg(test)]
    pub fn from_ascii(input: &str) -> Edges {
        let input = unindent::unindent(input);

        // Iterating via .lines() means searching for '\n' characters. It seems
        // clearer to just do it once and store the result in a Vec.
        let lines: Vec<_> = input.lines().collect();

        // The first line may look like this "+ + + + +". We care for the number
        // of '+' characters, which is the total amount of characters divided by
        // two, rounded away from zero.
        // Because an empty input is not valid and this is intended for internal
        // use only, it is okay to index into the Vec here before checking its
        // size before.
        let width = lines[0].len() / 2 + 1;

        let horizontal = lines
            .iter()
            .step_by(2) // Select rows containing horizontal edges
            .map(|line| {
                line.chars()
                    .skip(1) // Select characters representing horizontal edges
                    .step_by(2) //
                    .map(Edge::from_ascii)
                    .collect()
            })
            .collect();

        let vertical = lines
            .iter()
            .skip(1) // Select rows containing vertical edges
            .step_by(2) //
            .map(|line| {
                line.chars()
                    .step_by(2) // Select characters representing vertical edges
                    .map(Edge::from_ascii)
                    .chain(repeat(Unknown)) // Fill up cells for short lines
                    .take(width)
                    .collect()
            })
            .collect();

        Edges {
            horizontal,
            vertical,
        }
    }

    /// Returns a `Option<EdgeIndex>` representing the edge adjacent to the
    /// point at the given `IntersectionIndex`.
    ///
    /// It filters non-existing edges for intersection points on one of the four
    /// sides of the puzzle. This function does not check whether the given
    /// `IntersectionIndex` is valid.
    pub fn index_adjacent_edge(
        &self,
        &IntersectionIndex { row, column }: &IntersectionIndex,
        direction: Direction,
    ) -> Option<EdgeIndex> {
        // Indexing of both the intersection and the edge indices starts at the
        // top-left corner of the grid.
        // Therefore the column of a horizontal edge to the left of an
        // intersection with the column m will be m - 1 while an edge to the
        // right will have a column of m.
        // Similarly a vertical edge above an intersection point at the n-th row
        // will have a row of n - 1, while the row of the edge below the
        // intersection point will be n.

        match direction {
            Direction::Horizontal(East) => {
                let is_valid = column < self.horizontal[0].len();

                if is_valid {
                    EdgeIndex {
                        row,
                        column,
                        direction: Horizontal,
                    }
                    .into()
                } else {
                    None
                }
            }
            Direction::Horizontal(West) => {
                let column = column.checked_sub(1)?;

                EdgeIndex {
                    row,
                    column,
                    direction: Horizontal,
                }
                .into()
            }
            Direction::Vertical(North) => {
                let row = row.checked_sub(1)?;

                EdgeIndex {
                    row,
                    column,
                    direction: Vertical,
                }
                .into()
            }
            Direction::Vertical(South) => {
                let is_valid = row < self.vertical.len();

                is_valid.as_some(EdgeIndex {
                    row,
                    column,
                    direction: Vertical,
                })
            }
        }
    }

    pub fn index_adjacent_edges(
        &self,
        index: IntersectionIndex,
    ) -> impl Iterator<Item = Option<EdgeIndex>> + Clone + '_ {
        Direction::iter_all()
            .map(move |direction| self.index_adjacent_edge(&index, direction))
    }

    pub fn index_adjacent_corner_edges(
        &self,
        index: IntersectionIndex,
        CornerDirection {
            horizontal,
            vertical,
        }: CornerDirection,
    ) -> impl Iterator<Item = Option<EdgeIndex>> + Clone {
        let horizontal =
            self.index_adjacent_edge(&index, Direction::Horizontal(horizontal));
        let vertical =
            self.index_adjacent_edge(&index, Direction::Vertical(vertical));

        once(horizontal).chain(once(vertical))
    }

    pub fn index_edges(&self) -> impl Iterator<Item = EdgeIndex> {
        let horizontal = {
            let height = self.horizontal.len();
            let width = self.horizontal[0].len();

            iproduct!(0..height, 0..width).map(|(row, column)| EdgeIndex {
                row,
                column,
                direction: Horizontal,
            })
        };

        let vertical = {
            let height = self.vertical.len();
            let width = self.vertical[0].len();

            iproduct!(0..height, 0..width).map(|(row, column)| EdgeIndex {
                row,
                column,
                direction: Vertical,
            })
        };

        horizontal.chain(vertical)
    }

    pub fn index_intersections(
        &self,
    ) -> impl Iterator<Item = IntersectionIndex> {
        let height = self.vertical.len();
        let width = self.horizontal[0].len();

        iproduct!(0..=height, 0..=width)
            .map(|(row, column)| IntersectionIndex { row, column })
    }

    #[cfg(test)]
    pub fn print_lines(&self) {
        use itertools::Itertools;

        // [Line, X, Unknown] -> "+-+x+ +"
        let horizontal_lines = self.horizontal.iter().map(|row| {
            let inner = row
                .iter()
                .map(|edge| match edge {
                    Edge::Line => '-',
                    Edge::X => 'x',
                    Edge::Unknown => ' ',
                })
                .join("+");

            format!("+{}+", inner)
        });

        // [Line, X, Unknown] -> "| x  "
        let vertical_lines = self.vertical.iter().map(|row| {
            row.iter()
                .map(|edge| match edge {
                    Edge::Line => '|',
                    Edge::X => 'x',
                    Edge::Unknown => ' ',
                })
                .join(" ")
        });

        // Write the lines in alternating order, beginning with a horizontal
        // line.
        for line in horizontal_lines.interleave(vertical_lines) {
            println!("{}", line);
        }
    }

    pub fn index_adjacent_intersection(
        &self,
        IntersectionIndex { row, column }: IntersectionIndex,
        direction: Direction,
    ) -> Option<IntersectionIndex> {
        match direction {
            Direction::Horizontal(East) => IntersectionIndex {
                row,
                column: column + 1,
            },
            Direction::Horizontal(West) => IntersectionIndex {
                row,
                column: column.checked_sub(1)?,
            },
            Direction::Vertical(North) => IntersectionIndex {
                row: row.checked_sub(1)?,
                column,
            },
            Direction::Vertical(South) => IntersectionIndex {
                row: row + 1,
                column,
            },
        }
        .into()
    }

    pub fn follow_line(
        &self,
        previous_index: &IntersectionIndex,
        intersection_index: &IntersectionIndex,
    ) -> Option<IntersectionIndex> {
        Direction::iter_all()
            .filter_map(|direction| {
                let edge_index =
                    self.index_adjacent_edge(intersection_index, direction)?;

                if self[edge_index].is_line() {
                    let next_index = self
                        .index_adjacent_intersection(
                            *intersection_index,
                            direction,
                        )
                        .unwrap();

                    if next_index != *previous_index {
                        return next_index.into();
                    }
                }

                None
            })
            .next()
    }

    pub fn index_diagonally_from_intersection(
        &self,
        IntersectionIndex { row, column }: IntersectionIndex,
        direction: CornerDirection,
    ) -> Option<CellIndex> {
        let row = match direction.vertical {
            North => row.checked_sub(1)?,
            South => {
                if row < self.vertical.len() {
                    row
                } else {
                    return None;
                }
            }
        };

        let column = match direction.horizontal {
            West => column.checked_sub(1)?,
            East => {
                if column < self.horizontal[0].len() {
                    column
                } else {
                    return None;
                }
            }
        };

        CellIndex { row, column }.into()
    }

    pub fn get_route(&self) -> Vec<(usize, usize)> {
        let mut intersection_indices = self.index_intersections();

        let mut previous = IntersectionIndex { row: 0, column: 0 };
        let mut index = intersection_indices
            .find(|intersection_index| {
                let edge_indices = Direction::iter_all().map(|direction| {
                    self.index_adjacent_edge(intersection_index, direction)
                });

                edge_indices
                    .flatten()
                    .any(|edge_index| self[edge_index].is_line())
            })
            .unwrap();

        let mut route = vec![index];
        while let Some(next) = self.follow_line(&previous, &index) {
            if next == route[0] {
                break;
            }

            previous = replace(&mut index, next);
            route.push(index);
        }

        let mut route: Vec<_> = route
            .iter()
            .map(|index| (index.column, index.row))
            .collect();

        let first = route[0];
        route.push(first);

        route
    }
}

impl Index<EdgeIndex> for Edges {
    type Output = Edge;

    fn index(
        &self,
        EdgeIndex {
            row,
            column,
            direction,
        }: EdgeIndex,
    ) -> &Self::Output {
        match direction {
            Horizontal => &self.horizontal[row][column],
            Vertical => &self.vertical[row][column],
        }
    }
}

impl IndexMut<EdgeIndex> for Edges {
    fn index_mut(
        &mut self,
        EdgeIndex {
            row,
            column,
            direction,
        }: EdgeIndex,
    ) -> &mut Edge {
        match direction {
            Horizontal => &mut self.horizontal[row][column],
            Vertical => &mut self.vertical[row][column],
        }
    }
}
