prelude!();

use std::f32::EPSILON;

#[derive(Debug, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Value {
    Point(F32),
    Percent(F32),
    Auto,
}

impl Value {
    pub fn resolve(&self, parent_size: F32) -> Option<F32> {
        match *self {
            Value::Point(v) => Some(v),
            // UNSAFE(anp): not going to NaN when dividing by a non-zero number
            Value::Percent(v) => Some(unsafe { F32::unchecked_new(*v * *parent_size / 100.0) }),
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (Value::Auto, Value::Auto) => true,
            (Value::Point(a), Value::Point(b)) | (Value::Percent(a), Value::Percent(b)) => {
                (**a - **b).abs() < EPSILON
            }
        }
    }
}

pub unsafe fn YGRoundValueToPixelGrid(
    value: c_float,
    pointScaleFactor: c_float,
    forceCeil: bool,
    forceFloor: bool,
) -> c_float {
    let mut scaledValue: c_float = value * pointScaleFactor;
    let mut fractial: c_float = scaledValue % 1.0;
    if YGFloatsEqual(fractial, 0.0) {
        scaledValue = scaledValue - fractial;
    } else {
        if YGFloatsEqual(fractial, 1.0f32) {
            scaledValue = (scaledValue - fractial) + 1.0f32;
        } else {
            if forceCeil {
                scaledValue = scaledValue - fractial + 1.0f32;
            } else {
                if forceFloor {
                    scaledValue = scaledValue - fractial;
                } else {
                    scaledValue = scaledValue - fractial
                        + if fractial > 0.5f32 || YGFloatsEqual(fractial, 0.5f32) {
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
