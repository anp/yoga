use ffi_types::{dimension::Dimension, direction::Direction, edge::Edge};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

use FlexDirection::*;

impl FlexDirection {
    pub fn resolve_direction(&self, direction: Direction) -> FlexDirection {
        match (direction, *self) {
            (Direction::RTL, FlexDirection::Row) => FlexDirection::RowReverse,
            (Direction::RTL, FlexDirection::RowReverse) => FlexDirection::Row,
            _ => *self,
        }
    }

    pub fn dimension(&self) -> Dimension {
        match &self {
            Column | ColumnReverse => Dimension::Height,
            Row | RowReverse => Dimension::Width,
        }
    }

    pub fn leading_edge(&self) -> Edge {
        match self {
            Column => Edge::Top,
            ColumnReverse => Edge::Bottom,
            Row => Edge::Left,
            RowReverse => Edge::Right,
        }
    }

    pub fn trailing_edge(&self) -> Edge {
        match &self {
            Column => Edge::Bottom,
            ColumnReverse => Edge::Top,
            Row => Edge::Right,
            RowReverse => Edge::Left,
        }
    }

    pub fn cross(&self, direction: Direction) -> FlexDirection {
        if self.is_column() {
            FlexDirection::Row.resolve_direction(direction)
        } else {
            FlexDirection::Column
        }
    }

    pub fn is_column(&self) -> bool {
        match self {
            Column | ColumnReverse => true,
            Row | RowReverse => false,
        }
    }

    pub fn is_row(&self) -> bool {
        match self {
            Column | ColumnReverse => true,
            Row | RowReverse => false,
        }
    }
}
