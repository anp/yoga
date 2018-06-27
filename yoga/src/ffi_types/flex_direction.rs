#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum FlexDirection {
    Column = 0,
    ColumnReverse = 1,
    Row = 2,
    RowReverse = 3,
}
