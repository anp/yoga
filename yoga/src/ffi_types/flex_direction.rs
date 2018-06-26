use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum FlexDirection {
    Column = 0,
    ColumnReverse = 1,
    Row = 2,
    RowReverse = 3,
}

impl From<FlexDirection> for internal::YGFlexDirection {
    fn from(f: FlexDirection) -> internal::YGFlexDirection {
        match f {
            FlexDirection::Column => internal::YGFlexDirectionColumn,
            FlexDirection::ColumnReverse => internal::YGFlexDirectionColumnReverse,
            FlexDirection::Row => internal::YGFlexDirectionRow,
            FlexDirection::RowReverse => internal::YGFlexDirectionRowReverse,
        }
    }
}

impl From<internal::YGFlexDirection> for FlexDirection {
    fn from(f: internal::YGFlexDirection) -> FlexDirection {
        match f {
            internal::YGFlexDirectionColumn => FlexDirection::Column,
            internal::YGFlexDirectionColumnReverse => FlexDirection::ColumnReverse,
            internal::YGFlexDirectionRow => FlexDirection::Row,
            internal::YGFlexDirectionRowReverse => FlexDirection::RowReverse,
            _ => unreachable!("invalid C enum received"),
        }
    }
}
