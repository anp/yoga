use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Wrap {
    NoWrap = 0,
    Wrap = 1,
    WrapReverse = 2,
}

impl From<Wrap> for internal::YGWrap {
    fn from(w: Wrap) -> internal::YGWrap {
        match w {
            Wrap::NoWrap => internal::YGWrapNoWrap,
            Wrap::Wrap => internal::YGWrapWrap,
            Wrap::WrapReverse => internal::YGWrapWrapReverse,
        }
    }
}

impl From<internal::YGWrap> for Wrap {
    fn from(w: internal::YGWrap) -> Wrap {
        match w {
            internal::YGWrapNoWrap => Wrap::NoWrap,
            internal::YGWrapWrap => Wrap::Wrap,
            internal::YGWrapWrapReverse => Wrap::WrapReverse,
            _ => unreachable!("invalid C enum received"),
        }
    }
}
