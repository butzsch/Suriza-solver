use data::{
    edge_direction::{
        EdgeDirection,
        EdgeDirection::{Horizontal, Vertical},
    },
    intersection_index::IntersectionIndex,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EdgeIndex {
    pub direction: EdgeDirection,
    pub row: usize,
    pub column: usize,
}

impl EdgeIndex {
    /// Returns indices to the two intersections that are adjacent to this edge.
    pub fn get_intersections(&self) -> [IntersectionIndex; 2] {
        let &EdgeIndex {
            direction,
            row,
            column,
        } = self;

        let next = match direction {
            Horizontal => IntersectionIndex {
                row,
                column: column + 1,
            },
            Vertical => IntersectionIndex {
                row: row + 1,
                column,
            },
        };

        [IntersectionIndex { row, column }, next]
    }
}
