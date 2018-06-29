#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Overflow {
    Visible = 0,
    Hidden = 1,
    Scroll = 2,
}
