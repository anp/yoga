use ordered_float::OrderedFloat;
pub type F32 = OrderedFloat<f32>;
use ffi_types::{
    align::Align, dimension::Dimensions, direction::Direction, display::Display,
    edge::{Edge, Edges}, flex_direction::FlexDirection, justify::Justify, overflow::Overflow,
    position_type::PositionType, value::Value, wrap::Wrap,
};

use internal::*;
use updated::Updated;

// TODO(anp): figure out how to rule out Value::Auto for height/width

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Style {
    pub direction: Direction,
    pub flexDirection: FlexDirection,
    pub justifyContent: Justify,
    pub alignContent: Align,
    pub alignItems: Align,
    pub alignSelf: Align,
    pub positionType: PositionType,
    pub flexWrap: YGWrap_0,
    pub overflow: Overflow,
    pub display: Display,
    pub flex: F32,
    pub flexGrow: F32,
    pub flexShrink: F32,
    pub flexBasis: Value,
    pub margin: Edges,
    pub position: Edges,
    pub padding: Edges,
    pub border: Edges,
    pub dimensions: Dimensions,
    pub minDimensions: Dimensions,
    pub maxDimensions: Dimensions,
    pub aspectRatio: Option<F32>,
}

pub trait Property
where
    Self: Sized,
{
    type Target: Eq + PartialEq;

    fn prep(self) -> Self::Target;
    fn field(style: &mut Style) -> &mut Self::Target;

    // inline attribute necessary for cross-crate inlining
    #[inline]
    fn apply(self, style: &mut Style) -> Updated {
        let apply = self.prep();
        let mut field = Self::field(style);
        if *field == apply {
            Updated::Clean
        } else {
            *field = apply;
            Updated::Dirty
        }
    }
}

macro_rules! simple_property_impl {
    (| $style:ident | $field:expr, $struct:ident($target:ty)) => {
        pub struct $struct($target);

        impl Property for $struct {
            type Target = $target;

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn prep(self) -> Self::Target {
                self.0
            }

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn field(style: &mut Style) -> &mut Self::Target {
                &mut |$style: &mut Style| { $field }(&mut style)
            }
        }
    };
}

// TODO(anp): impl Property without a newtype?
// pub struct Display(Display);
// pub struct FlexDirection(FlexDirection);
// pub struct Overflow(Overflow);

// TODO(anp): splat trait? custom impl?
// simple_property_impl!(|s| s.TODO, Border(F32));

simple_property_impl!(|s| s.alignContent, AlignContent(Align));
simple_property_impl!(|s| s.alignItems, AlignItems(Align));
simple_property_impl!(|s| s.alignSelf, AlignSelf(Align));
simple_property_impl!(|s| s.aspectRatio, AspectRatio(F32));
simple_property_impl!(|s| s.border[Edge::Bottom], BorderBottom(F32));
simple_property_impl!(|s| s.border[Edge::End], BorderEnd(F32));
simple_property_impl!(|s| s.border[Edge::Left], BorderLeft(F32));
simple_property_impl!(|s| s.border[Edge::Right], BorderRight(F32));
simple_property_impl!(|s| s.border[Edge::Start], BorderStart(F32));
simple_property_impl!(|s| s.border[Edge::Top], BorderTop(F32));
simple_property_impl!(|s| s.TODO, Bottom(Value));
simple_property_impl!(|s| s.TODO, End(Value));
simple_property_impl!(|s| s.flex, Flex(F32));
simple_property_impl!(|s| s.flex_basis, FlexBasis(Value));
simple_property_impl!(|s| s.flex_grow, FlexGrow(F32));
simple_property_impl!(|s| s.flex_shrink, FlexShrink(F32));
simple_property_impl!(|s| s.flex_wrap, FlexWrap(Wrap));
simple_property_impl!(|s| s.dimensions.height, Height(Value));
simple_property_impl!(|s| s.justifyContent, JustifyContent(Justify));
simple_property_impl!(|s| s.TODO, Left(Value));
simple_property_impl!(|s| s.TODO, Margin(Value));
simple_property_impl!(|s| s.margin[Edge::Bottom], MarginBottom(Value));
simple_property_impl!(|s| s.margin[Edge::End], MarginEnd(Value));
simple_property_impl!(|s| s.margin[Edge::Horizontal], MarginHorizontal(Value));
simple_property_impl!(|s| s.margin[Edge::Left], MarginLeft(Value));
simple_property_impl!(|s| s.margin[Edge::Right], MarginRight(Value));
simple_property_impl!(|s| s.margin[Edge::Start], MarginStart(Value));
simple_property_impl!(|s| s.margin[Edge::Top], MarginTop(Value));
simple_property_impl!(|s| s.margin[Edge::Vertical], MarginVertical(Value));
simple_property_impl!(|s| s.maxDimensions.height, MaxHeight(Value));
simple_property_impl!(|s| s.maxDimensions.width, MaxWidth(Value));
simple_property_impl!(|s| s.minDimensions.height, MinHeight(Value));
simple_property_impl!(|s| s.minDimensions.width, MinWidth(Value));
simple_property_impl!(|s| s.TODO, Padding(Value));
simple_property_impl!(|s| s.padding[Edge::Bottom], PaddingBottom(Value));
simple_property_impl!(|s| s.padding[Edge::End], PaddingEnd(Value));
simple_property_impl!(|s| s.padding[Edge::Horizontal], PaddingHorizontal(Value));
simple_property_impl!(|s| s.padding[Edge::Left], PaddingLeft(Value));
simple_property_impl!(|s| s.padding[Edge::Right], PaddingRight(Value));
simple_property_impl!(|s| s.padding[Edge::Start], PaddingStart(Value));
simple_property_impl!(|s| s.padding[Edge::Top], PaddingTop(Value));
simple_property_impl!(|s| s.padding[Edge::Vertical], PaddingVertical(Value));
simple_property_impl!(|s| s.TODO, Position(PositionType));
simple_property_impl!(|s| s.TODO, Right(Value));
simple_property_impl!(|s| s.TODO, Start(Value));
simple_property_impl!(|s| s.TODO, Top(Value));
simple_property_impl!(|s| s.dimensions.width, Width(Value));

impl ::std::default::Default for Style {
    fn default() -> Self {
        Style {
            direction: Direction::Inherit,
            flexDirection: FlexDirection::Column,
            justifyContent: Justify::FlexStart,
            alignContent: Align::FlexStart,
            alignItems: Align::Stretch,
            alignSelf: Align::Auto,
            positionType: PositionType::Relative,
            flexWrap: YGWrapNoWrap,
            overflow: Overflow::Visible,
            display: Display::Flex,
            flex: None,
            flexGrow: None,
            flexShrink: None,
            flexBasis: Value::Auto,
            margin: Edges::empty(),
            position: Edges::empty(),
            padding: Edges::empty(),
            border: Edges::empty(),
            dimensions: Dimensions {
                width: Value::Auto,
                height: Value::Auto,
            },
            minDimensions: Dimensions {
                width: Value::Auto,
                height: Value::Auto,
            },
            maxDimensions: Dimensions {
                width: Value::Auto,
                height: Value::Auto,
            },
            aspectRatio: None,
        }
    }
}

// style_getter_setter!(
//     self.direction,
//     get_direction,
//     set_direction,
//     direction: Direction
// );

// style_getter_setter!(
//     self.flexDirection,
//     get_flex_direction,
//     set_flex_direction,
//     flex_direction: FlexDirection
// );
// style_getter_setter!(
//     self.justifyContent,
//     get_justify_content,
//     set_justify_content,
//     justify_content: Justify
// );
// style_getter_setter!(
//     self.alignContent,
//     get_align_content,
//     set_align_content,
//     align_content: Align
// );
// style_getter_setter!(
//     self.alignItems,
//     get_align_items,
//     set_align_items,
//     align_items: Align
// );
// style_getter_setter!(
//     self.alignSelf,
//     get_align_self,
//     set_align_self,
//     align_self: Align
// );
// style_getter_setter!(
//     self.positionType,
//     get_position_type,
//     set_position_type,
//     position_type: PositionType
// );
// style_getter_setter!(
//     self.flexWrap,
//     get_flex_wrap,
//     set_flex_wrap,
//     flex_wrap: YGWrap_0
// );
// style_getter_setter!(
//     self.overflow,
//     get_overflow,
//     set_overflow,
//     overflow: Overflow
// );
// style_getter_setter!(self.display, get_display, set_display, display: Display);
// style_getter_setter!(self.flex, get_flex, set_flex, flex: c_float);
// style_getter_setter!(
//     path: self.flexGrow,
//     arg: flex_grow: f32,
//     getter: get_flex_grow,
//     setter: set_flex_grow,
//     // TODO(anp): handle this in a Default impl
//     getter_xform: |flex_grow| {
//         if flex_grow.is_nan() { kDefaultFlexGrow } else { flex_grow }
//     },
// );

// style_getter_setter! {
//     path: self.flexShrink,
//     arg: flex_shrink: f32,
//     getter: get_flex_shrink,
//     setter: set_flex_shrink,
//     getter_xform: |flex_shrink| {
//         if flex_shrink.is_nan() {
//             if cfg!(feature = "web-defaults") {
//                 kWebDefaultFlexShrink
//             } else {
//                 kDefaultFlexShrink
//             }
//         } else {
//             flex_shrink
//         }
//     },
// }

// style_getter_setter!(
//     self.flexBasis,
//     get_flex_basis,
//     set_flex_basis,
//     flex_basis: f32
// );

// style_getter_setter_edge!(
//     self.position,
//     get_position,
//     set_position,
//     position: Option<Value>
// );

// style_getter_setter_edge!(self.margin, get_margin, set_margin, margin: Option<Value>);

// style_getter_setter_edge!(
//     self.padding,
//     get_padding,
//     set_padding,
//     padding: Option<Value>
// );

// style_getter_setter_edge!(self.border, get_border, set_border, border: Option<Value>);

// style_getter_setter!(
//     self.dimensions,
//     get_dimensions,
//     set_dimensions,
//     dimensions: Option<Dimensions>
// );

// style_getter_setter!(
//     self.minDimensions,
//     get_min_dimensions,
//     set_min_dimensions,
//     min_dimensions: Option<Dimensions>
// );

// style_getter_setter!(
//     self.maxDimensions,
//     get_max_dimensions,
//     set_max_dimensions,
//     max_dimensions: Option<Dimensions>
// );

// style_getter_setter!(
//     path: self.maxDimensions.height,
//     arg: max_height: Option<Value>,
//     getter: get_max_height,
//     setter: set_max_height,
//     setter_validator: |max_height| {
//         match max_height {
//             Some(Value::Auto) => None,
//             this @ _ => this
//         }
//     },
// );

// style_getter_setter!(
//     self.aspectRatio,
//     get_aspect_ratio,
//     set_aspect_ratio,
//     aspect_ratio: F32
// );
