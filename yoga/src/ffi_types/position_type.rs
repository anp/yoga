#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum PositionType {
    Relative = 0,
    Absolute = 1,
}
