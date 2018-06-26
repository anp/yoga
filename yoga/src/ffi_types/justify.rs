use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Justify {
    FlexStart = 0,
    Center = 1,
    FlexEnd = 2,
    SpaceBetween = 3,
    SpaceAround = 4,
}

impl From<Justify> for internal::YGJustify {
    fn from(j: Justify) -> internal::YGJustify {
        match j {
            Justify::FlexStart => internal::YGJustifyFlexStart,
            Justify::Center => internal::YGJustifyCenter,
            Justify::FlexEnd => internal::YGJustifyFlexEnd,
            Justify::SpaceBetween => internal::YGJustifySpaceBetween,
            Justify::SpaceAround => internal::YGJustifySpaceAround,
        }
    }
}

impl From<internal::YGJustify> for Justify {
    fn from(j: internal::YGJustify) -> Justify {
        match j {
            internal::YGJustifyFlexStart => Justify::FlexStart,
            internal::YGJustifyCenter => Justify::Center,
            internal::YGJustifyFlexEnd => Justify::FlexEnd,
            internal::YGJustifySpaceBetween => Justify::SpaceBetween,
            internal::YGJustifySpaceAround => Justify::SpaceAround,
            _ => unreachable!("invalid C enum received"),
        }
    }
}
