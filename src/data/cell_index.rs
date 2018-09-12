use data::{
    CornerDirection, Direction, EdgeDirection, EdgeIndex,
    HorizontalDirection::*, IntersectionIndex, VerticalDirection::*,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CellIndex {
    pub row: usize,
    pub column: usize,
}

impl CellIndex {
    pub fn index_intersection(
        &self,
        direction: CornerDirection,
    ) -> IntersectionIndex {
        let column = match direction.horizontal {
            East => self.column + 1,
            West => self.column,
        };

        let row = match direction.vertical {
            North => self.row,
            South => self.row + 1,
        };

        IntersectionIndex { row, column }
    }

    pub fn index_edges(&self) -> impl Iterator<Item = EdgeIndex> + Clone + '_ {
        Direction::iter_all().map(move |direction| self.index_edge(direction))
    }

    pub fn index_corner_edges(
        &self,
        direction: CornerDirection,
    ) -> impl Iterator<Item = EdgeIndex> + Clone + '_ {
        direction
            .into_iter()
            .map(move |index| self.index_edge(index))
    }

    fn index_edge(&self, direction: Direction) -> EdgeIndex {
        let &Self { row, column } = self;

        match direction {
            Direction::Horizontal(East) => EdgeIndex {
                row,
                column: column + 1,
                direction: EdgeDirection::Vertical,
            },
            Direction::Horizontal(West) => EdgeIndex {
                row,
                column,
                direction: EdgeDirection::Vertical,
            },
            Direction::Vertical(North) => EdgeIndex {
                row,
                column,
                direction: EdgeDirection::Horizontal,
            },
            Direction::Vertical(South) => EdgeIndex {
                row: row + 1,
                column,
                direction: EdgeDirection::Horizontal,
            },
        }
    }
}
