pub use ffi_types::align::*;
pub use ffi_types::dimension::*;
pub use ffi_types::direction::*;
pub use ffi_types::display::*;
pub use ffi_types::edge::*;
pub use ffi_types::flex_direction::*;
pub use ffi_types::justify::*;
pub use ffi_types::measure_mode::*;
pub use ffi_types::node_ref::*;
pub use ffi_types::node_type::*;
pub use ffi_types::overflow::*;
pub use ffi_types::position_type::*;
pub use ffi_types::size::*;
pub use ffi_types::undefined::*;
pub use ffi_types::value::*;
pub use ffi_types::wrap::*;
use libc::c_void;
use ordered_float::OrderedFloat;
use std::any::Any;
use std::ops::Deref;

pub type BaselineFunc = Option<extern "C" fn(NodeRef, f32, f32) -> f32>;
pub type MeasureFunc = Option<extern "C" fn(NodeRef, f32, MeasureMode, f32, MeasureMode) -> Size>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum FlexStyle {
    AlignContent(Align),
    AlignItems(Align),
    AlignSelf(Align),
    AspectRatio(OrderedFloat<f32>),
    BorderBottom(OrderedFloat<f32>),
    BorderEnd(OrderedFloat<f32>),
    BorderLeft(OrderedFloat<f32>),
    BorderRight(OrderedFloat<f32>),
    BorderStart(OrderedFloat<f32>),
    BorderTop(OrderedFloat<f32>),
    Border(OrderedFloat<f32>),
    Bottom(Value),
    Display(Display),
    End(Value),
    Flex(OrderedFloat<f32>),
    FlexBasis(Value),
    FlexDirection(FlexDirection),
    FlexGrow(OrderedFloat<f32>),
    FlexShrink(OrderedFloat<f32>),
    FlexWrap(Wrap),
    Height(Value),
    JustifyContent(Justify),
    Left(Value),
    Margin(Value),
    MarginBottom(Value),
    MarginEnd(Value),
    MarginHorizontal(Value),
    MarginLeft(Value),
    MarginRight(Value),
    MarginStart(Value),
    MarginTop(Value),
    MarginVertical(Value),
    MaxHeight(Value),
    MaxWidth(Value),
    MinHeight(Value),
    MinWidth(Value),
    Overflow(Overflow),
    Padding(Value),
    PaddingBottom(Value),
    PaddingEnd(Value),
    PaddingHorizontal(Value),
    PaddingLeft(Value),
    PaddingRight(Value),
    PaddingStart(Value),
    PaddingTop(Value),
    PaddingVertical(Value),
    Position(PositionType),
    Right(Value),
    Start(Value),
    Top(Value),
    Width(Value),
}

#[derive(Debug)]
pub struct Context(Box<Any>);

impl Context {
    pub fn new<T: Any>(value: T) -> Self {
        Context(Box::new(value))
    }

    pub(crate) fn into_raw(self) -> *mut c_void {
        // https://users.rust-lang.org/t/ffi-boxes-and-returning-references-to-the-boxed-data
        Box::into_raw(Box::new(self.0)) as *mut c_void
    }

    pub(crate) fn get_inner_ref<'a>(raw: *mut c_void) -> Option<&'a Box<Any>> {
        let ptr = raw as *const Box<Any>;
        unsafe { ptr.as_ref() }
    }

    pub(crate) fn get_inner_mut<'a>(raw: *mut c_void) -> Option<&'a mut Box<Any>> {
        let ptr = raw as *mut Box<Any>;
        unsafe { ptr.as_mut() }
    }

    pub(crate) fn drop_raw(raw: *mut c_void) {
        let ptr = raw as *mut Box<Any>;
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }
}

impl Deref for Context {
    type Target = Box<Any>;
    fn deref(&self) -> &Box<Any> {
        &self.0
    }
}

#[macro_export]
macro_rules! unit {
    ($val:tt pt) => {
        $val.point()
    };
    ($val:tt %) => {
        $val.percent()
    };
    ($val:expr) => {
        $val
    };
}

#[macro_export]
macro_rules! flex_style {
	// Manually match on styles which require an OrderedFloat
	// This way the styles like
	//     Flex(1.0)
	// will be converted to:
	//     Flex(1.0.into())
	(AspectRatio($val:expr)) => (
		AspectRatio($val.into())
	);
	(BorderBottom($val:expr)) => (
		BorderBottom($val.into())
	);
	(BorderEnd($val:expr)) => (
		BorderEnd($val.into())
	);
	(BorderLeft($val:expr)) => (
		BorderLeft($val.into())
	);
	(BorderRight($val:expr)) => (
		BorderRight($val.into())
	);
	(BorderStart($val:expr)) => (
		BorderStart($val.into())
	);
	(BorderTop($val:expr)) => (
		BorderTop($val.into())
	);
	(Border($val:expr)) => (
		Border($val.into())
	);
	(Flex($val:expr)) => (
		Flex($val.into())
	);
	(FlexGrow($val:expr)) => (
		FlexGrow($val.into())
	);
	(FlexShrink($val:expr)) => (
		FlexShrink($val.into())
	);
	($s:ident($($unit:tt)*)) => (
		$s(unit!($($unit)*))
	);
}

#[macro_export]
macro_rules! style {
	( $x:expr, $($s:tt($($unit:tt)*)),* ) => {
		$x.apply_styles(&vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		))
	};
}

#[macro_export]
macro_rules! make_styles {
	( $($s:tt($($unit:tt)*)),* ) => {
		vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		)
	};
}
