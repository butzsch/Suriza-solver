use data::{
    Cell, CellIndex, Cells, Constraint, CornerDirection, Edge, EdgeIndex,
    Edges, IntersectionIndex,
};

#[cfg(test)]
mod tests {
    use super::*;

    // To make our tests as concise as possible, we want an easy and readable
    // way to represent the fields and the edge grids in test code. For
    // representing the edges we choose an ASCII graph as an appropriate option,
    // i.e.:
    //
    // +-+ +-+
    // | | | |
    // + +-+ +
    // |     |
    // +-+-+-+
    //
    // When we are trying to compare an edge grid which is represented in this
    // format as a string and an actual EdgeGrid instance, we have two options.
    // We can either
    //     - create a to_ascii method and compare the strings OR
    //     - create a from_ascii function and compare the EdgeGrid instances.
    //
    // We choose the second options because it allows us to to encode additional
    // information into the string which the from_ascii function can simply
    // ignore. This enables us to use a single string as a test input which
    // contains both the information of the input numbers as well as the
    // expected output edges:
    //
    // +-+ +-+
    // |3|3|3|
    // + +-+ +
    // |     |
    // +-+-+-+

    fn assert_same_lines(a: &Edges, b: &Edges) {
        let mut b_indices = b.index_edges();

        for a_index in a.index_edges() {
            let b_index = b_indices.next().unwrap();

            assert_eq!(a[a_index].is_line(), b[b_index].is_line());
        }

        assert!(b_indices.next().is_none());
    }

    /// Test helper function which asserts that the algorithm found the expected
    /// edges.
    ///
    /// * `input` a string with an ASCII-art representation of both the input
    ///   numbers and the expected output edges
    fn assert_solution(input: &str) {
        let fields = Cells::from_ascii(input);
        let expected_solution = Edges::from_ascii(input);

        let actual_solution = solve(&fields);

        println!("expected solution: ");
        expected_solution.print_lines();

        println!("actual solution: ");
        actual_solution.print_lines();

        assert_same_lines(&actual_solution, &expected_solution);
    }

    // The functions should work for inputs of arbitrary sizes. We can use small
    // inputs to check the basic functionality of the algorithm first.

    // We are going to assume that a solution which consists of no loop at all
    // is still valid, given that it is the only possible solution for the
    // input.
    #[test]
    fn returns_solution_for_single_cell_zero_puzzle() {
        assert_solution(
            "+ +
              0
             + +",
        );
    }

    // This test case represents an edge case because for almost all real inputs
    // two adjacent threes will not form a closed loop. Instead, most of the
    // time threes form the following pattern:
    //
    // + + +
    // |3|3|
    // + + +
    //
    // It is however also an example of the rule that threes in corners always
    // have lines on the outer edges.
    #[test]
    fn returns_correct_answer_for_two_adjacent_threes() {
        assert_solution(
            "+-+-+
             |3 3|
             +-+-+",
        );
    }

    // Only the basic components of the algorithm need to be implemented in
    // order to find a solution for this puzzle:
    // * check if a cell is surrounded by the correct amount of lines an x
    //   the other edges
    // * check if three edges of an intersection are x's and x the fourth
    //   edge
    // * check if two edges of an intersection are x's and third edge is a
    //   line, make the fourth edge a line also
    // * check if two edges of an intersection are lines, make the other two
    //   edges x's
    // * check if a cell is surrounded by the correct amount of x's and make
    //   the other edges lines
    #[test]
    fn solves_basic_puzzle() {
        assert_solution(
            "
            +-+-+ +-+-+
            |   | |   |
            +-+ + +-+ +
             3|2|1 3| |
            +-+ + +-+ +
            |3  | |   |
            +-+ +-+ +-+
             2|    2|
            + +-+ +-+ +
             0 2| |
            + + +-+ + +
        ",
        );
    }

    // To solve this puzzle, the algorithm additionally needs to check for
    // edges that would create closed sub-loops.
    #[test]
    fn solves_closed_loop_puzzle() {
        assert_solution(
            "
            +-+-+-+-+-+
            |        3|
            +-+ +-+-+-+
             3| |
            +-+ +-+ +-+
            |3 0 3|2|3|
            +-+ +-+ + +
             3| |  2| |
            +-+ +-+-+ +
            |         |
            +-+-+-+-+-+
        ",
        );
    }

    #[test]
    fn detects_ones_in_corners() {
        assert_solution(
            "
            + + +
             1 1
            +-+-+
            |   |
            +-+-+
        ",
        )
    }

    #[test]
    fn detects_twos_in_corners() {
        assert_solution(
            "
            + +-+ + +
             2       
            + + + +-+
            |  2   3|
            + + + +-+
                 2   
            + +-+ + +
            ",
        );
    }

    #[test]
    fn detects_threes_in_corners() {
        assert_solution(
            "
            +-+ +-+
            |3   3|
            + + + +

            + + + +
            |3   3|
            +-+ +-+
        ",
        );
    }

    #[test]
    fn test_solve() {
        let puzzles = vec![
            "
            +-+-+-+-+-+
            |3 1     3|
            +-+ + +-+-+
              |   |3 1
            + +-+ +-+ +
                |   |
            +-+ +-+ +-+
            | |   |1 3|
            + +-+-+ +-+
            |2 2 2 2|2
            +-+-+-+-+ +
            ",
            "
            +-+-+-+ + +
            |3   2|
            +-+-+ +-+ +
               2|   |
            +-+ +-+ +-+
            |3|  3|1 3|
            + +-+-+ +-+
            |  2    |
            + +-+-+ +-+
            |3|   |2 3|
            +-+ + +-+-+
            ",
            "
            +-+ +-+-+-+
            |3| |2   3|
            + + + +-+-+
            | |3| |3
            + +-+ +-+-+
            |  1   1 2|
            +-+ +-+ + +
             3|2| |1  |
            +-+ + + + +
            |3  | |2  |
            +-+-+ +-+-+
            ",
            "
            +-+-+ +-+-+
            |   | |   |
            + +-+ +-+ +
            |2|3 1 2| |
            + +-+-+ + +
            |    2|2| |
            + +-+ + + +
            | |3| | | |
            + + + +-+ +
            |3| |2    |
            +-+ +-+-+-+
            ",
            "
            +-+-+ +-+-+
            |  3| |2  |
            + +-+ + +-+
            |2|3  | |
            + +-+-+ +-+
            |        2|
            + +-+ +-+ +
            | | |2|3| |
            + + + + + +
            | |2|3| |3|
            +-+ +-+ +-+
            ",
            "
            +-+-+ +-+-+
            |   | |   |
            +-+ + +-+ +
             3|2|1 3| |
            +-+ + +-+ +
            |3  | |   |
            +-+ +-+ +-+
             2|    2|
            + +-+ +-+ +
             0 2| |
            + + +-+ + +
            ",
            //"
            //+-+-+ +-+-+-+ +
            //|3  | |  1  |2
            //+-+ + + + + +-+
            //  |2| |1   1 3|
            //+-+ +-+ +-+-+-+
            //|3 0 2 2|    2
            //+-+ +-+-+ + +-+
            //  | |    1 2|3|
            //+-+ +-+-+-+-+ +
            //|3           2|
            //+-+ +-+-+ +-+-+
            //  | |   |2|
            //+-+ + +-+ +-+-+
            //|3  | |3 1   3|
            //+-+-+ +-+-+-+-+
            //",
            //"
            //+-+-+-+-+-+-+-+
            //|3   2 2   1  |
            //+-+-+-+-+ + + +
            // 2 2    |  0  |
            //+-+-+-+ +-+ + +
            //|  1  |1 2|  1|
            //+-+ + + + +-+ +
            //  |1 2|     |2|
            //+ + +-+ +-+-+ +
            //  | |3  |2    |
            //+-+ +-+-+ +-+ +
            //|2     2 1|3|2|
            //+ +-+ +-+ + + +
            //|3|3| | |3| | |
            //+-+ +-+ +-+ +-+
            //",
            //"
            //+-+-+ +-+-+-+-+
            //|  2| |3     2|
            //+ + + +-+-+-+ +
            //|  1|1     3| |
            //+ + + +-+-+-+ +
            //|1  | |  1    |
            //+ +-+ +-+ + + +
            //|2|3 1 3|  1 1|
            //+ +-+-+-+ +-+ +
            //|2 1   2  | |2|
            //+-+ +-+-+-+ + +
            // 2|2|3 1    | |
            //+ + +-+ +-+-+ +
            //  |2  |2|    2|
            //+ +-+-+ +-+-+-+
            //",
            //"
            //+-+-+-+ + + +-+
            //|3 2 2|1 1  | |
            //+-+-+ + +-+ + +
            // 2  |2| | |3|2|
            //+-+ + +-+ +-+ +
            //| | |  2 1   2|
            //+ + + +-+-+ +-+
            //|2|3| |  3|2|
            //+ +-+ + +-+ +-+
            //|1   2| |3    |
            //+ +-+-+ +-+ +-+
            //| |    1 3|2|2
            //+ + +-+-+-+ + +
            //| |2|  2 2 2|
            //+-+ +-+-+-+-+ +
            //",
        ];

        for puzzle in puzzles {
            assert_solution(puzzle);
        }
    }
}

/// Checks if the value of any currently unknown edge can be set to be an x
/// because a line at that edge would create an invalid inner loop.
fn check_loops(edges: &mut Edges) -> bool {
    edges.index_edges().any(|index| {
        if edges[index].is_unknown() && would_close_loop(edges, index) {
            edges[index] = Edge::X;
            true
        } else {
            false
        }
    })
}

/// Checks whether a line at the given `EdgeIndex` would form a closed loop in
/// the `Edges` instance.
fn would_close_loop(edges: &Edges, edge_index: EdgeIndex) -> bool {
    let [mut index, end] = edge_index.get_intersections();

    // Initialize with some meaningless initial value.
    let mut previous: IntersectionIndex = Default::default();

    // Follow the lines starting at the first_intersection. If we end up at the
    // second intersection, we know that if the would make the edge a line, we
    // would form a closed loop.
    while let Some(next) = edges.follow_line(&previous, &index) {
        previous = std::mem::replace(&mut index, next);
    }

    index == end
}

/// Counts how many of the edges are `Line`s and how many are `X`'s. This
/// function is callable with both `Iterator`s over `EdgeIndex` and over
/// `Option<EdgeIndex>`.
fn count_edges<Index, Indices>(
    edges: &Edges,
    indices: Indices,
) -> (usize, usize)
where
    Index: Into<Option<EdgeIndex>>,
    Indices: IntoIterator<Item = Index>,
{
    let (mut line_count, mut x_count) = (0, 0);

    for index in indices {
        // We use .map() so that we only need a single match statement.
        let edge = index.into().map(|edge_index| edges[edge_index]);

        match edge {
            Some(Edge::Line) => line_count += 1,
            // An index which is None represents an edge that is out of bounds.
            // For our purposes, these edges are considered to be x's.
            Some(Edge::X) | None => x_count += 1,
            _ => {}
        };
    }

    (line_count, x_count)
}

/// Sets all of the `Edge`s that are indexed by the `indices` to the given
/// `value`. This function is callable with both `Iterator`s over `EdgeIndex`
/// and over `Option<EdgeIndex>`. Returns whether the value of any of the edges
/// was changed.
fn set_edges<Index, Indices>(
    edges: &mut Edges,
    indices: Indices,
    value: Edge,
) -> bool
where
    Index: Into<Option<EdgeIndex>>,
    Indices: IntoIterator<Item = Index>,
{
    indices.into_iter().any(|index| {
        // An index which is None represents an edge that is out of bounds.
        // Therefore, its value can not be changed.
        index.into().map_or(false, |index| match edges[index] {
            Edge::Unknown => {
                edges[index] = value;
                true
            }
            _ => false,
        })
    })
}

/// Sets the values of the unknown edges adjacent to the cell indexed by `index`
/// if the known edges provide enough information to do so. Returns whether any
/// edge was actually changed.
fn fill_cell(cells: &Cells, edges: &mut Edges, index: CellIndex) -> bool {
    let expected_line_count = cells[&index].get_expected_line_count();

    // Cells without a number cannot be filled.
    expected_line_count.map_or(false, |expected_line_count| {
        let indices = index.index_edges();
        let (line_count, x_count) = count_edges(edges, indices.clone());

        if line_count == expected_line_count {
            // We known already know where all the lines go, so all the other
            // edges must be x's.
            set_edges(edges, indices, Edge::X)
        } else if x_count == 4 - expected_line_count {
            // We are certain of all the x's and can fill the rest with lines.
            set_edges(edges, indices, Edge::Line)
        } else {
            false
        }
    })
}

/// Sets the values of the unknown edges adjacent to the intersection indexed by
/// `index` if the known edges provide enough information to do so. Returns
/// whether any edge was actually changed.
fn fill_intersection(edges: &mut Edges, index: IntersectionIndex) -> bool {
    let (indices, line_count) = {
        let indices = edges.index_adjacent_edges(index);
        let (line_count, _) = count_edges(edges, indices.clone());

        // Collect the values because the iterator immutably borrows from edges,
        // which collides with the mutable borrow further down.
        let indices: Vec<_> = indices.collect();

        (indices, line_count)
    };

    // Most of the intersection-related logic is handled by the constraint code,
    // but this seemed too simple to create a new constraint for (Which would
    // always result in the two edges being set to x's anyway).
    match line_count {
        // We know all the lines of the intersection, we can fill the rest with
        // x's.
        2 => set_edges(edges, indices, Edge::X),
        _ => false,
    }
}

/// Iterates through all of the cells and intersections and sets all of the
/// edges whose values we know for sure. Returns whether the value of any edge
/// was actually changed.
fn fill_certain_values(cells: &Cells, edges: &mut Edges) -> bool {
    let filled_cell = cells
        .index_cells()
        .any(|index| fill_cell(cells, edges, index));

    // For a better debugging-experience we want this function to change only a
    // single value at a time.
    filled_cell
        || edges
            .index_intersections()
            .any(|index| fill_intersection(edges, index))
}

fn apply_constraint(
    cells: &Cells,
    edges: &mut Edges,
    constraint: Constraint,
    from: IntersectionIndex,
    to: CornerDirection,
) -> bool {
    let near = edges.index_adjacent_corner_edges(from, to);
    let counts = count_edges(edges, near.clone());

    // We may be able to use the constraint without even looking at the
    // number in the cell, just by considering what edge was already set.
    // For example:
    //
    // +x+ +
    // x1
    // + +a+
    //   xC
    // + + +
    //
    // Without knowing what number the cell C has, we know that edge a has
    // to be a line.
    let set_value = {
        let value = match (&constraint, counts) {
            (Constraint::Line, (1, 0))
            | (Constraint::NoCorner, (1, 0))
            | (Constraint::NoLine, (0, 1)) => Edge::X.into(),
            (Constraint::Line, (0, 1)) | (Constraint::NoLine, (1, 0)) => {
                Edge::Line.into()
            }
            _ => None,
        };

        // No constraint means no value can be set. Because we explicitly check
        // that the other edge is not set in our match statement, set_edges will
        // always set the value of an edge and therefore always return true.
        value.map_or(false, |value| set_edges(edges, near, value))
    };

    // If a value was set, be can stop here because the constraint lead to an
    // actual change of an edge, which means that the next cell, if any, will be
    // able to pick up the constraint by just considering its own edges in the
    // next iteration.
    set_value || {
        let next_cell = edges.index_diagonally_from_intersection(from, to);
        next_cell.map_or(false, |next_cell| {
            apply_constraint_to_cell(cells, edges, constraint, next_cell, to)
        })
    }
}

/// Apply a constraint using the number in a cell. Depending on the cell, this
/// may cause the constraint to be cascaded to nearby intersections and cells.
/// Returns whether the value of an edge was actually changed.
fn apply_constraint_to_cell(
    cells: &Cells,
    edges: &mut Edges,
    constraint: Constraint,
    index: CellIndex,
    to: CornerDirection,
) -> bool {
    let near = index.index_corner_edges(to.get_opposite());
    let far = index.index_corner_edges(to);

    let next_intersection = index.index_intersection(to);

    let (line_count, x_count) = count_edges(edges, far.clone());

    match cells[&index] {
        Cell::One => match constraint {
            Constraint::Line => set_edges(edges, far, Edge::X),
            Constraint::NoLine => set_edges(edges, near, Edge::X),
            _ => false,
        },
        Cell::Two => match constraint {
            Constraint::Line => {
                if line_count > 0 {
                    set_edges(edges, far, Edge::X)
                } else if x_count > 0 {
                    set_edges(edges, far, Edge::Line)
                } else {
                    apply_constraint(
                        cells,
                        edges,
                        constraint,
                        next_intersection,
                        to,
                    )
                }
            }
            Constraint::NoCorner => apply_constraint(
                cells,
                edges,
                constraint,
                next_intersection,
                to,
            ),
            Constraint::NoLine => {
                if x_count > 0 {
                    set_edges(edges, near, Edge::Line)
                } else {
                    let adjacent_directions = to.get_adjacent();
                    let applied_adjacent =
                        adjacent_directions.iter().any(|&direction| {
                            apply_constraint(
                                cells,
                                edges,
                                Constraint::Line,
                                index.index_intersection(direction),
                                direction,
                            )
                        });

                    applied_adjacent
                        || apply_constraint(
                            cells,
                            edges,
                            Constraint::NoLine,
                            next_intersection,
                            to,
                        )
                }
            }
        },
        Cell::Three => match constraint {
            Constraint::Line => set_edges(edges, far, Edge::Line),
            Constraint::NoCorner => set_edges(edges, far, Edge::Line),
            Constraint::NoLine => {
                set_edges(edges, near, Edge::Line)
                    || apply_constraint(
                        cells,
                        edges,
                        Constraint::Line,
                        next_intersection,
                        to,
                    )
            }
        },
        _ => false,
    }
}

/// Checks for constraints in the edges near to the cell indexed by `index` that
/// can help us solve the puzzle. Returns whether the value of any edge was
/// actually changed.
fn check_cell_constraints(
    cells: &Cells,
    edges: &mut Edges,
    index: CellIndex,
) -> bool {
    // Constraints are always created by the number inside of the cell and
    // the value of two adjacent edges surrounding the cell. Because there are
    // four pairs of adjacent edges, every cell can provide up to four
    // constraints.
    CornerDirection::ALL.iter().any(|&direction| {
        let counts = {
            let indices = index.index_corner_edges(direction);
            count_edges(edges, indices)
        };

        let constraint = match (cells[&index], counts) {
            // +x+ +    +x+ +
            // x1       |2
            // + +a+    + +a+
            //   b        b
            // + + +    + + +
            //
            // Either a or b has to be a line, but never both.
            (Cell::One, (0, 2)) | (Cell::Two, (1, 1)) => {
                Constraint::Line.into()
            }
            // + + +    + + +
            // x2        3
            // + +a+    + +a+
            //   b        b
            // + + +    + + +
            //
            // Either a or b can be a line, but not both.
            (Cell::Two, (0, 1)) | (Cell::Three, _) => {
                Constraint::NoCorner.into()
            }
            _ => None,
        };

        constraint.map_or(false, |constraint| {
            // We may have looked at the two edges in the north and west of the
            // cell. The edges effected by the constraint are the ones in the
            // north-west of the cell which is south-east of the current cell.
            let direction = direction.get_opposite();
            let intersection = index.index_intersection(direction);

            apply_constraint(cells, edges, constraint, intersection, direction)
        })
    })
}

/// Checks for constraints in the edges near to the cell indexed by `index` that
/// can help us solve the puzzle. Returns whether the value of any edge was
/// actually changed.
fn check_intersection_constraints(
    cells: &Cells,
    edges: &mut Edges,
    index: IntersectionIndex,
) -> bool {
    // A constraint at an intersection is created by the value of any pair of
    // adjacent edges next to the intersection.
    CornerDirection::ALL.iter().any(|&direction| {
        let counts = {
            let indices = edges.index_adjacent_corner_edges(index, direction);
            count_edges(edges, indices)
        };

        let constraint = match counts {
            // + + +
            //
            // +-+a+
            //   b
            // + + +
            //
            // Either a or b can be an edge, but not both. If they were, an
            // intersection with three crossing lines would be created.
            (1, 0) => Constraint::NoCorner.into(),
            // + + +
            //   |
            // +x+a+
            //   b
            // + + +
            //
            // Either a or b has to be a line, but not both. If not, an
            // intersection with either one or three lines would be created.
            (1, 1) => Constraint::Line.into(),
            // + + +
            //   x
            // +x+a+
            //   b
            // + + +
            //
            //   Edges a and b have to both be lines or both be x's. If not, an
            //   intersection with a single line would be created.
            (0, 2) => Constraint::NoLine.into(),
            _ => None,
        };

        constraint.map_or(false, |constraint| {
            let to = direction.get_opposite();
            apply_constraint(cells, edges, constraint, index, to)
        })
    })
}

/// Iterates through all the cells and intersections and detects all the
/// constraints. A constraint is some piece of information about two edges that
/// can help us solve the puzzle.
fn check_constraints(cells: &Cells, edges: &mut Edges) -> bool {
    // For example, given the following grid:
    //
    // +x+ +
    // x1b
    // +a+c+
    //   x
    // + + +
    //
    // We know that either edge a or edge b has to be a line, which means that
    // either of the edges next to it must also be a line. Because we already
    // know that the edge next to edge c is an x, we therefore know that c has
    // to be a line. Note that we were able to find this information without
    // knowing the actual value of either a nor b.

    let applied_cell_constraints = cells
        .index_cells()
        .any(|index| check_cell_constraints(cells, edges, index));

    // For a better debugging-experience we want this function to change only a
    // single value at a time.
    applied_cell_constraints
        || edges
            .index_intersections()
            .any(|index| check_intersection_constraints(cells, edges, index))
}

/// Returns the solution for a given Suriza puzzle. Makes an attempt to find the
/// value of all edges but returns an incomplete edge grid if it fails to
/// do so.
/// The output is undefined if the input is not a valid Suriza puzzle.
pub fn solve(cells: &Cells) -> Edges {
    let mut edges = {
        let size = cells.get_size();
        Edges::create_empty(&size)
    };

    loop {
        if !fill_certain_values(cells, &mut edges)
            && !check_constraints(cells, &mut edges)
            && !check_loops(&mut edges)
        {
            break edges;
        }
    }
}
