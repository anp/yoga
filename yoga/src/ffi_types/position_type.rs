use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum PositionType {
    Relative = 0,
    Absolute = 1,
}

impl From<PositionType> for internal::YGPositionType {
    fn from(p: PositionType) -> internal::YGPositionType {
        match p {
            PositionType::Relative => internal::YGPositionTypeRelative,
            PositionType::Absolute => internal::YGPositionTypeAbsolute,
        }
    }
}

impl From<internal::YGPositionType> for PositionType {
    fn from(p: internal::YGPositionType) -> PositionType {
        match p {
            internal::YGPositionTypeRelative => PositionType::Relative,
            internal::YGPositionTypeAbsolute => PositionType::Absolute,
            _ => unreachable!("invalid C enum received"),
        }
    }
}
