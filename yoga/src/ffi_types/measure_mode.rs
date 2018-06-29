#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum MeasureMode {
    Undefined = 0,
    Exactly = 1,
    AtMost = 2,
}
