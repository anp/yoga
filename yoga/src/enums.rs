internal_prelude!();

use std::f32::EPSILON;
use std::ops::{Index, IndexMut};

#[cfg(feature = "web-default")]
default!(FlexDirection, FlexDirection::Row);

#[cfg(not(feature = "web-default"))]
default!(FlexDirection, FlexDirection::Column);

default!(Direction, Direction::LTR);
default!(Justify, Justify::FlexStart);
default!(PositionType, PositionType::Relative);
default!(Wrap, Wrap::NoWrap);
default!(Overflow, Overflow::Visible);
default!(Display, Display::Flex);

#[must_use]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Updated {
    Dirty,
    Clean,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Align {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Rectangle<V> {
    pub height: V,
    pub width: V,
}

pub type Dimensions = Rectangle<Value>;
pub type MeasuredDimensions = Rectangle<R32>;
pub type ResolvedDimensions = Rectangle<Option<Value>>;

impl Into<Dimensions> for MeasuredDimensions {
    fn into(self) -> Dimensions {
        Dimensions {
            width: Value::Point(self.width),
            height: Value::Point(self.height),
        }
    }
}
// pub(crate) fn is_dim_defined(&self, axis: FlexDirection) -> bool {
//     self.measured_dimensions[axis.dimension()] >= 0.0
// }

impl<V: Default> Default for Rectangle<V> {
    fn default() -> Self {
        Rectangle {
            height: V::default(),
            width: V::default(),
        }
    }
}

impl<V> Index<Dimension> for Rectangle<V> {
    type Output = V;

    fn index(&self, d: Dimension) -> &Self::Output {
        match d {
            Dimension::Height => &self.height,
            Dimension::Width => &self.width,
        }
    }
}

impl<V> IndexMut<Dimension> for Rectangle<V> {
    fn index_mut(&mut self, d: Dimension) -> &mut Self::Output {
        match d {
            Dimension::Height => &mut self.height,
            Dimension::Width => &mut self.width,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum Dimension {
    Width,
    Height,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Direction {
    Inherit,
    LTR,
    RTL,
}

impl Direction {
    pub(crate) fn resolve(&self, parent: Self) -> Self {
        use Direction::*;
        match (self, parent) {
            (Inherit, Inherit) => Direction::LTR,
            (Inherit, parent) => parent,
            _ => *self,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum Display {
    Flex,
    None,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl FlexDirection {
    pub(crate) fn resolve_direction(&self, direction: Direction) -> FlexDirection {
        match (direction, *self) {
            (Direction::RTL, FlexDirection::Row) => FlexDirection::RowReverse,
            (Direction::RTL, FlexDirection::RowReverse) => FlexDirection::Row,
            _ => *self,
        }
    }

    pub(crate) fn dimension(&self) -> Dimension {
        match &self {
            FlexDirection::Column | FlexDirection::ColumnReverse => Dimension::Height,
            FlexDirection::Row | FlexDirection::RowReverse => Dimension::Width,
        }
    }

    pub(crate) fn leading_edge(&self) -> PhysicalEdge {
        match self {
            FlexDirection::Column => PhysicalEdge::Top,
            FlexDirection::ColumnReverse => PhysicalEdge::Bottom,
            FlexDirection::Row => PhysicalEdge::Start,
            FlexDirection::RowReverse => PhysicalEdge::End,
        }
    }

    pub(crate) fn trailing_edge(&self) -> PhysicalEdge {
        match &self {
            FlexDirection::Column => PhysicalEdge::Bottom,
            FlexDirection::ColumnReverse => PhysicalEdge::Top,
            FlexDirection::Row => PhysicalEdge::End,
            FlexDirection::RowReverse => PhysicalEdge::Start,
        }
    }

    pub(crate) fn cross(&self, direction: Direction) -> FlexDirection {
        if self.is_column() {
            FlexDirection::Row.resolve_direction(direction)
        } else {
            FlexDirection::Column
        }
    }

    pub(crate) fn is_column(&self) -> bool {
        match self {
            FlexDirection::Column | FlexDirection::ColumnReverse => true,
            FlexDirection::Row | FlexDirection::RowReverse => false,
        }
    }

    pub(crate) fn is_row(&self) -> bool {
        match self {
            FlexDirection::Column | FlexDirection::ColumnReverse => false,
            FlexDirection::Row | FlexDirection::RowReverse => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum Justify {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum MeasureMode {
    Exactly,
    AtMost,
}

impl MeasureMode {
    pub(crate) fn new_measure_size_is_stricter_and_still_valid(
        old_mode: Option<MeasureMode>,
        old_size: R32,
        old_computed: R32,
        new_mode: Option<MeasureMode>,
        new_size: R32,
    ) -> bool {
        old_mode == Some(MeasureMode::AtMost)
            && new_mode == Some(MeasureMode::AtMost)
            && old_size > new_size
            && (old_computed <= new_size || new_size.approx_eq(old_computed))
    }

    pub(crate) fn old_size_is_unspecified_and_still_fits(
        current: Option<MeasureMode>,
        size: R32,
        last_size_mode: Option<MeasureMode>,
        last_computed_size: R32,
    ) -> bool {
        current == Some(MeasureMode::AtMost)
            && last_size_mode.is_none()
            && (size >= last_computed_size || size.approx_eq(last_computed_size))
    }

    pub(crate) fn size_is_exact_and_matches_old_measured_size(
        mode: Option<MeasureMode>,
        size: R32,
        last_computed_size: R32,
    ) -> bool {
        mode == Some(MeasureMode::Exactly) && size.approx_eq(last_computed_size)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum NodeType {
    Default,
    Text,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum PositionType {
    Relative,
    Absolute,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: R32,
    pub height: R32,
}

#[derive(Debug, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Value {
    Point(R32),
    Percent(R32),
    Auto,
}

pub(crate) trait ResolveValue {
    fn resolve(&self, parent_size: R32) -> Option<R32>;
}

impl ResolveValue for Value {
    fn resolve(&self, parent_size: R32) -> Option<R32> {
        match *self {
            Value::Point(v) => Some(v),
            Value::Percent(v) => Some(v * parent_size / r32(100.0)),
            _ => None,
        }
    }
}

default!(Value, Value::Point(r32(0.0)));

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (Value::Auto, Value::Auto) => true,
            (Value::Point(a), Value::Point(b)) | (Value::Percent(a), Value::Percent(b)) => {
                (*a - *b).abs() < EPSILON
            }
            _ => false,
        }
    }
}

pub trait AsValue {
    fn points(self) -> Value;
    fn percent(self) -> Value;
}

impl AsValue for i32 {
    #[inline]
    fn points(self) -> Value {
        Value::Point(r32(self as f32))
    }

    #[inline]
    fn percent(self) -> Value {
        Value::Percent(r32(self as f32))
    }
}

impl AsValue for f32 {
    #[inline]
    fn points(self) -> Value {
        Value::Point(r32(self))
    }

    #[inline]
    fn percent(self) -> Value {
        Value::Percent(r32(self))
    }
}

pub(crate) fn round_value_to_pixel_grid(
    value: R32,
    point_scale_factor: R32,
    force_ceiling: bool,
    force_floor: bool,
) -> R32 {
    let scaled_value = value * point_scale_factor;
    let fractional = scaled_value % 1.0;

    // first we check if the value is already rounded
    let scaled_value = if fractional.approx_eq(r32(0.0)) {
        scaled_value - fractional
    } else if fractional.approx_eq(r32(1.0)) {
        (scaled_value - fractional) + 1.0
    // Next we check if we need to use forced rounding
    } else if force_ceiling {
        scaled_value - fractional + 1.0
    } else if force_floor {
        scaled_value - fractional
    } else {
        // Finally we just round the value
        scaled_value - fractional + if fractional >= 0.5 || fractional.approx_eq(r32(0.5)) {
            1.0
        } else {
            0.0
        }
    };

    scaled_value / point_scale_factor
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum Wrap {
    NoWrap,
    Wrap,
    WrapReverse,
}
