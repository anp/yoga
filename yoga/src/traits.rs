use ffi_types::value::Value;
use ordered_float::OrderedFloat;

pub trait Percent {
    fn percent(self) -> Value;
}

impl Percent for f32 {
    fn percent(self) -> Value {
        Value::Percent(OrderedFloat(self))
    }
}

impl Percent for i32 {
    fn percent(self) -> Value {
        Value::Percent(OrderedFloat(self as f32))
    }
}

pub trait Point {
    fn point(self) -> Value;
}

impl Point for f32 {
    fn point(self) -> Value {
        Value::Point(OrderedFloat(self))
    }
}

impl Point for i32 {
    fn point(self) -> Value {
        Value::Point(OrderedFloat(self as f32))
    }
}
