#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Direction {
    Inherit = 0,
    LTR = 1,
    RTL = 2,
}
