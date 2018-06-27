use internal::YGValue;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct ResolvedDimensions {
    pub width: *const YGValue,
    pub height: *const YGValue,
}

// TODO(anp): unify this with the "in progress" dimensions, probably via non-repr-C types
#[derive(Copy, Clone)]
pub struct MeasuredDimensions {
    pub height: f32,
    pub width: f32,
}

#[derive(Copy, Clone)]
pub struct Dimensions {
    pub height: YGValue,
    pub width: YGValue,
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

index_with_dimension!(Dimensions, YGValue);
index_with_dimension!(MeasuredDimensions, f32);
index_with_dimension!(ResolvedDimensions, *const YGValue);

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Dimension {
    Width = 0,
    Height = 1,
}
