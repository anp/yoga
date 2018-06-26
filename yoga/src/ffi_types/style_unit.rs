use internal;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum StyleUnit {
    UndefinedValue,
    Point(OrderedFloat<f32>),
    Percent(OrderedFloat<f32>),
    Auto,
}

impl From<StyleUnit> for internal::YGUnit {
    fn from(s: StyleUnit) -> internal::YGUnit {
        match s {
            StyleUnit::UndefinedValue => internal::YGUnitUndefined,
            StyleUnit::Point(_) => internal::YGUnitPoint,
            StyleUnit::Percent(_) => internal::YGUnitPercent,
            StyleUnit::Auto => internal::YGUnitAuto,
        }
    }
}

impl From<internal::YGValue> for StyleUnit {
    fn from(v: internal::YGValue) -> StyleUnit {
        match v.unit {
            internal::YGUnitUndefined => StyleUnit::UndefinedValue,
            internal::YGUnitPoint => StyleUnit::Point(OrderedFloat(v.value)),
            internal::YGUnitPercent => StyleUnit::Percent(OrderedFloat(v.value)),
            internal::YGUnitAuto => StyleUnit::Auto,
            _ => unreachable!("invalid C enum received"),
        }
    }
}
