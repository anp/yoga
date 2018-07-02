prelude!();

use std::ops::{Index, IndexMut};

use ffi_types::value::Value;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct ResolvedDimensions {
    pub width: Option<Value>,
    pub height: Option<Value>,
}

// TODO(anp): unify this with the "in progress" dimensions, probably via non-repr-C types
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct MeasuredDimensions {
    pub height: F32,
    pub width: F32,
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
index_with_dimension!(MeasuredDimensions, f32);
index_with_dimension!(ResolvedDimensions, Option<Value>);

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Dimension {
    Width = 0,
    Height = 1,
}
