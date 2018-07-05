prelude!();

use std::f32::EPSILON;
use std::ops::{Index, IndexMut};

#[must_use]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Updated {
    Dirty,
    Clean,
}

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct ResolvedDimensions {
    pub width: Option<Value>,
    pub height: Option<Value>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct MeasuredDimensions {
    pub height: R32,
    pub width: R32,
}

impl Into<Dimensions> for MeasuredDimensions {
    fn into(self) -> Dimensions {
        Dimensions {
            width: Value::Point(self.width),
            height: Value::Point(self.height),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub height: Value,
    pub width: Value,
}

macro_rules! index_with_dimension {
    ($struct:ty, $output:ty) => {
        impl Index<Dimension> for $struct {
            type Output = $output;

            fn index(&self, d: Dimension) -> &Self::Output {
                match d {
                    Dimension::Height => &self.height,
                    Dimension::Width => &self.width,
                }
            }
        }

        impl IndexMut<Dimension> for $struct {
            fn index_mut(&mut self, d: Dimension) -> &mut Self::Output {
                match d {
                    Dimension::Height => &mut self.height,
                    Dimension::Width => &mut self.width,
                }
            }
        }
    };
}

index_with_dimension!(Dimensions, Value);
index_with_dimension!(MeasuredDimensions, R32);
index_with_dimension!(ResolvedDimensions, Option<Value>);

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Dimension {
    Width = 0,
    Height = 1,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Direction {
    Inherit = 0,
    LTR = 1,
    RTL = 2,
}

impl Direction {
    // was YGNodeResolveDirection
    pub fn resolve(&self, parent: Self) -> Self {
        use Direction::*;
        match (self, parent) {
            (Inherit, Inherit) => Direction::LTR,
            (Inherit, parent) => parent,
            _ => *self,
        }
    }
}

default!(Direction, Direction::LTR);

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Display {
    Flex = 0,
    None = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl FlexDirection {
    pub fn resolve_direction(&self, direction: Direction) -> FlexDirection {
        match (direction, *self) {
            (Direction::RTL, FlexDirection::Row) => FlexDirection::RowReverse,
            (Direction::RTL, FlexDirection::RowReverse) => FlexDirection::Row,
            _ => *self,
        }
    }

    pub fn dimension(&self) -> Dimension {
        match &self {
            FlexDirection::Column | FlexDirection::ColumnReverse => Dimension::Height,
            FlexDirection::Row | FlexDirection::RowReverse => Dimension::Width,
        }
    }

    pub fn leading_edge(&self) -> Edge {
        match self {
            FlexDirection::Column => Edge::Top,
            FlexDirection::ColumnReverse => Edge::Bottom,
            FlexDirection::Row => Edge::Left,
            FlexDirection::RowReverse => Edge::Right,
        }
    }

    pub fn trailing_edge(&self) -> Edge {
        match &self {
            FlexDirection::Column => Edge::Bottom,
            FlexDirection::ColumnReverse => Edge::Top,
            FlexDirection::Row => Edge::Right,
            FlexDirection::RowReverse => Edge::Left,
        }
    }

    pub fn cross(&self, direction: Direction) -> FlexDirection {
        if self.is_column() {
            FlexDirection::Row.resolve_direction(direction)
        } else {
            FlexDirection::Column
        }
    }

    pub fn is_column(&self) -> bool {
        match self {
            FlexDirection::Column | FlexDirection::ColumnReverse => true,
            FlexDirection::Row | FlexDirection::RowReverse => false,
        }
    }

    pub fn is_row(&self) -> bool {
        match self {
            FlexDirection::Column | FlexDirection::ColumnReverse => true,
            FlexDirection::Row | FlexDirection::RowReverse => false,
        }
    }
}
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Justify {
    FlexStart = 0,
    Center = 1,
    FlexEnd = 2,
    SpaceBetween = 3,
    SpaceAround = 4,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum MeasureMode {
    // undefined was 1
    Exactly, // 1
    AtMost,  // 2
}

impl MeasureMode {
    pub fn new_measure_size_is_stricter_and_still_valid(
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

    pub fn old_size_is_unspecified_and_still_fits(
        current: Option<MeasureMode>,
        size: R32,
        last_size_mode: Option<MeasureMode>,
        last_computed_size: R32,
    ) -> bool {
        current == Some(MeasureMode::AtMost)
            && last_size_mode.is_none()
            && (size >= last_computed_size || size.approx_eq(last_computed_size))
    }

    pub fn size_is_exact_and_matches_old_measured_size(
        mode: Option<MeasureMode>,
        size: R32,
        last_computed_size: R32,
    ) -> bool {
        mode == Some(MeasureMode::Exactly) && size.approx_eq(last_computed_size)
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Default = 0,
    Text = 1,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Overflow {
    Visible = 0,
    Hidden = 1,
    Scroll = 2,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum PositionType {
    Relative = 0,
    Absolute = 1,
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

impl Value {
    pub fn resolve(&self, parent_size: R32) -> Option<R32> {
        match *self {
            Value::Point(v) => Some(v),
            Value::Percent(v) => Some(v * parent_size / r32(100.0)),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Point(r32(0.0))
    }
}

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

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Wrap {
    NoWrap = 0,
    Wrap = 1,
    WrapReverse = 2,
}
