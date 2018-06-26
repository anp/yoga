use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Align {
    Auto = 0,
    FlexStart = 1,
    Center = 2,
    FlexEnd = 3,
    Stretch = 4,
    Baseline = 5,
    SpaceBetween = 6,
    SpaceAround = 7,
}

impl From<Align> for internal::YGAlign {
    fn from(a: Align) -> internal::YGAlign {
        match a {
            Align::Auto => internal::YGAlignAuto,
            Align::FlexStart => internal::YGAlignFlexStart,
            Align::Center => internal::YGAlignCenter,
            Align::FlexEnd => internal::YGAlignFlexEnd,
            Align::Stretch => internal::YGAlignStretch,
            Align::Baseline => internal::YGAlignBaseline,
            Align::SpaceBetween => internal::YGAlignSpaceBetween,
            Align::SpaceAround => internal::YGAlignSpaceAround,
        }
    }
}

impl From<internal::YGAlign> for Align {
    fn from(a: internal::YGAlign) -> Align {
        match a {
            internal::YGAlignAuto => Align::Auto,
            internal::YGAlignFlexStart => Align::FlexStart,
            internal::YGAlignCenter => Align::Center,
            internal::YGAlignFlexEnd => Align::FlexEnd,
            internal::YGAlignStretch => Align::Stretch,
            internal::YGAlignBaseline => Align::Baseline,
            internal::YGAlignSpaceBetween => Align::SpaceBetween,
            internal::YGAlignSpaceAround => Align::SpaceAround,
            _ => unreachable!("invalid C enum received"),
        }
    }
}
