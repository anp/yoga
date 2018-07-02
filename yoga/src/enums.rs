prelude!();

use std::f32::EPSILON;
use std::fmt::Debug;
use std::hash::Hash;
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
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Display {
    Flex = 0,
    None = 1,
}

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Edge {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Start = 4,
    End = 5,
    Horizontal = 6,
    Vertical = 7,
    All = 8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Edges<V>([Option<V>; 9])
where
    V: 'static + Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Copy + Clone;

impl<V> Edges<V>
where
    V: 'static + Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Copy + Clone,
{
    pub fn empty() -> Self {
        Edges([None, None, None, None, None, None, None, None, None])
    }

    pub fn computed(&mut self, edge: Edge) -> Option<V> {
        if let Some(v) = self[edge] {
            return Some(v);
        }

        match edge {
            Edge::Top | Edge::Bottom => if let Some(v) = self[Edge::Vertical] {
                return Some(v);
            },

            Edge::Left | Edge::Right | Edge::Start | Edge::End => {
                if let Some(v) = self[Edge::Horizontal] {
                    return Some(v);
                }
            }

            _ => (),
        };

        if let Some(v) = self[Edge::All] {
            return Some(v);
        }

        return None;
    }
}

impl<V> ::std::ops::Index<Edge> for Edges<V>
where
    V: 'static + Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Copy + Clone,
{
    type Output = Option<V>;
    fn index(&self, index: Edge) -> &Self::Output {
        // UNSAFE(anp): we know that Edge and Edges cannot index incorrectly
        unsafe { self.0.get_unchecked(index as usize) }
    }
}

impl<V> ::std::ops::IndexMut<Edge> for Edges<V>
where
    V: 'static + Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Copy + Clone,
{
    fn index_mut(&mut self, index: Edge) -> &mut Self::Output {
        // UNSAFE(anp): we know that Edge and Edges cannot index incorrectly
        unsafe { self.0.get_unchecked_mut(index as usize) }
    }
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
        &self,
        old_size: R32,
        old_computed: R32,
        new_mode: MeasureMode,
        new_size: R32,
    ) -> bool {
        self == &MeasureMode::AtMost
            && new_mode == MeasureMode::AtMost
            && old_size > new_size
            && (old_computed <= new_size || new_size.approx_eq(old_computed))
    }
}

pub unsafe fn MeasureModeOldSizeIsUnspecifiedAndStillFits(
    mut sizeMode: MeasureMode,
    mut size: R32,
    mut lastSizeMode: Option<MeasureMode>,
    mut lastComputedSize: R32,
) -> bool {
    return sizeMode == MeasureMode::AtMost
        && lastSizeMode == None
        && (size >= lastComputedSize || size.approx_eq(lastComputedSize));
}

pub unsafe fn MeasureModeSizeIsExactAndMatchesOldMeasuredSize(
    mut sizeMode: MeasureMode,
    mut size: R32,
    mut lastComputedSize: R32,
) -> bool {
    return sizeMode == MeasureMode::Exactly && size.approx_eq(lastComputedSize);
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

pub unsafe fn YGRoundValueToPixelGrid(
    value: R32,
    pointScaleFactor: R32,
    forceCeil: bool,
    forceFloor: bool,
) -> R32 {
    let mut scaledValue = value * pointScaleFactor;
    let mut fractial = scaledValue % 1.0;
    if fractial.approx_eq(r32(0.0)) {
        scaledValue = scaledValue - fractial;
    } else {
        if fractial.approx_eq(r32(1.0)) {
            scaledValue = (scaledValue - fractial) + 1.0f32;
        } else {
            if forceCeil {
                scaledValue = scaledValue - fractial + 1.0f32;
            } else {
                if forceFloor {
                    scaledValue = scaledValue - fractial;
                } else {
                    scaledValue = scaledValue - fractial
                        + if fractial > 0.5f32 || fractial.approx_eq(r32(0.5)) {
                            1.0f32
                        } else {
                            0.0f32
                        };
                };
            };
        };
    };
    return scaledValue / pointScaleFactor;
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Wrap {
    NoWrap = 0,
    Wrap = 1,
    WrapReverse = 2,
}
