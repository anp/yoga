use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Display {
    Flex = 0,
    None = 1,
}

impl From<Display> for internal::YGDisplay {
    fn from(d: Display) -> internal::YGDisplay {
        match d {
            Display::Flex => internal::YGDisplayFlex,
            Display::None => internal::YGDisplayNone,
        }
    }
}
