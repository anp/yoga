prelude!();

macro_rules! default {
    ($struct:ty, $value:expr) => {
        impl ::std::default::Default for $struct {
            fn default() -> Self {
                $value
            }
        }
    };
}

pub use float_cmp::ApproxEq;
pub(crate) trait ApproxEqHackForReals {
    fn approx_eq(&self, other: Self) -> bool;
}

impl ApproxEqHackForReals for R32 {
    fn approx_eq(&self, other: Self) -> bool {
        // magic constants from the float-cmp docs:
        // https://docs.rs/float-cmp/0.4.0/float_cmp/
        self.raw()
            .approx_eq(&other.raw(), 2.0 * ::std::f32::EPSILON, 2)
    }
}
