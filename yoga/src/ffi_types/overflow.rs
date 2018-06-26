use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Overflow {
    Visible = 0,
    Hidden = 1,
    Scroll = 2,
}

impl From<Overflow> for internal::YGOverflow {
    fn from(o: Overflow) -> internal::YGOverflow {
        match o {
            Overflow::Visible => internal::YGOverflowVisible,
            Overflow::Hidden => internal::YGOverflowHidden,
            Overflow::Scroll => internal::YGOverflowScroll,
        }
    }
}

impl From<internal::YGOverflow> for Overflow {
    fn from(o: internal::YGOverflow) -> Overflow {
        match o {
            internal::YGOverflowVisible => Overflow::Visible,
            internal::YGOverflowHidden => Overflow::Hidden,
            internal::YGOverflowScroll => Overflow::Scroll,
            _ => unreachable!("invalid C enum received"),
        }
    }
}
