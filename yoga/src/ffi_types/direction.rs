use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Direction {
    Inherit = 0,
    LTR = 1,
    RTL = 2,
}

impl From<Direction> for internal::YGDirection {
    fn from(d: Direction) -> internal::YGDirection {
        match d {
            Direction::Inherit => internal::YGDirectionInherit,
            Direction::LTR => internal::YGDirectionLTR,
            Direction::RTL => internal::YGDirectionRTL,
        }
    }
}

impl From<internal::YGDirection> for Direction {
    fn from(d: internal::YGDirection) -> Direction {
        match d {
            internal::YGDirectionInherit => Direction::Inherit,
            internal::YGDirectionLTR => Direction::LTR,
            internal::YGDirectionRTL => Direction::RTL,
            _ => unreachable!("invalid C enum received"),
        }
    }
}
