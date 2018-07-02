prelude!();

// TODO(anp): figure out how to rule out Value::Auto for height/width

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Style {
    pub direction: Direction,
    pub flex_direction: FlexDirection,
    pub justify_content: Justify,
    pub align_content: Align,
    pub align_items: Align,
    pub align_self: Align,
    pub position_type: PositionType,
    pub flex_wrap: Wrap,
    pub overflow: Overflow,
    pub display: Display,
    pub flex: Option<R32>,
    pub flex_grow: R32,
    pub flex_shrink: R32,
    pub flex_basis: Value,
    pub margin: Edges<Value>,
    pub position: Edges<Value>,
    pub padding: Edges<Value>,
    pub border: Edges<R32>,
    pub dimensions: Dimensions,
    pub min_dimensions: Dimensions,
    pub max_dimensions: Dimensions,
    pub aspect_ratio: Option<R32>,
}

impl ::std::default::Default for Style {
    fn default() -> Self {
        Style {
            direction: Direction::Inherit,
            flex_direction: FlexDirection::Column,
            justify_content: Justify::FlexStart,
            align_content: Align::FlexStart,
            align_items: Align::Stretch,
            align_self: Align::Auto,
            position_type: PositionType::Relative,
            flex_wrap: Wrap::NoWrap,
            overflow: Overflow::Visible,
            display: Display::Flex,
            flex: None,
            flex_grow: r32(0.0),
            flex_shrink: r32(if cfg!(feature = "web-defaults") {
                1.0
            } else {
                0.0
            }),
            flex_basis: Value::Auto,
            margin: Edges::empty(),
            position: Edges::empty(),
            padding: Edges::empty(),
            border: Edges::empty(),
            dimensions: Dimensions {
                width: Value::Auto,
                height: Value::Auto,
            },
            min_dimensions: Dimensions {
                width: Value::Auto,
                height: Value::Auto,
            },
            max_dimensions: Dimensions {
                width: Value::Auto,
                height: Value::Auto,
            },
            aspect_ratio: None,
        }
    }
}

pub trait Property
where
    Self: Sized,
{
    type Target: Eq + PartialEq;

    fn prep(self) -> Self::Target;
    fn field(style: &Style) -> &Self::Target;
    fn field_mut(style: &mut Style) -> &mut Self::Target;

    // inline attribute necessary for cross-crate inlining
    #[inline]
    fn apply(self, style: &mut Style) -> Updated {
        let apply = self.prep();
        let mut field = Self::field_mut(style);
        if *field == apply {
            Updated::Clean
        } else {
            *field = apply;
            Updated::Dirty
        }
    }
}

macro_rules! property_impl {
    (
        |
        $style:ident |
        $field:expr,
        $struct:ident($contained:ty), |
        $inner:ident | ->
        $target:ty { $prep:expr }
    ) => {
        pub struct $struct($contained);

        impl Property for $struct {
            type Target = $target;

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn prep(self) -> Self::Target {
                |$inner: $struct| -> $target { $prep }(self)
            }

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn field<'a>(style: &'a Style) -> &'a Self::Target {
                let $style = style;
                &$field
            }

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn field_mut(style: &mut Style) -> &mut Self::Target {
                let $style = style;
                &mut $field
            }
        }
    };
    (| $style:ident | $field:expr, $struct:ident(optional $target:ty)) => {
        property_impl!(
            |$style| $field,
            $struct($target),
            |v| -> Option<$target> { Some(v.0) }
        );
    };
    (| $style:ident | $field:expr, $struct:ident($target:ty)) => {
        property_impl!(|$style| $field, $struct($target), |v| -> $target { v.0 });
    };
}

// TODO(anp): impl Property without a newtype?
// pub struct Display(Display);
// pub struct FlexDirection(FlexDirection);
// pub struct Overflow(Overflow);
// property_impl!(|s| s.justify_content, JustifyContent(Justify));

// TODO(anp): splat trait? custom impl?
// property_impl!(|s| s.TODO, Border(R32));
// property_impl!(|s| s.TODO, Margin(Value));
// property_impl!(|s| s.TODO, Padding(Value));
// property_impl!(|s| s.TODO, Position(PositionType));

// TODO(anp): consider storing the newtypes in the layout struct directly
property_impl!(|s| s.align_content, AlignContent(Align));
property_impl!(|s| s.align_items, AlignItems(Align));
property_impl!(|s| s.align_self, AlignSelf(Align));
property_impl!(|s| s.aspect_ratio, AspectRatio(optional R32));
property_impl!(|s| s.border[Edge::Bottom], BorderBottom(optional R32));
property_impl!(|s| s.border[Edge::End], BorderEnd(optional R32));
property_impl!(|s| s.border[Edge::Left], BorderLeft(optional R32));
property_impl!(|s| s.border[Edge::Right], BorderRight(optional R32));
property_impl!(|s| s.border[Edge::Start], BorderStart(optional R32));
property_impl!(|s| s.border[Edge::Top], BorderTop(optional R32));
property_impl!(|s| s.position[Edge::Bottom], Bottom(optional Value));
property_impl!(|s| s.position[Edge::End], End(optional Value));
property_impl!(|s| s.flex, Flex(optional R32));
property_impl!(|s| s.flex_basis, FlexBasis(Value));
property_impl!(|s| s.flex_grow, FlexGrow(R32));
property_impl!(|s| s.flex_shrink, FlexShrink(R32));
property_impl!(|s| s.flex_wrap, FlexWrap(Wrap));
property_impl!(|s| s.dimensions.height, Height(Value));
property_impl!(|s| s.position[Edge::Left], Left(optional Value));
property_impl!(|s| s.margin[Edge::Bottom], MarginBottom(optional Value));
property_impl!(|s| s.margin[Edge::End], MarginEnd(optional Value));
property_impl!(|s| s.margin[Edge::Horizontal], MarginHorizontal(optional Value));
property_impl!(|s| s.margin[Edge::Left], MarginLeft(optional Value));
property_impl!(|s| s.margin[Edge::Right], MarginRight(optional Value));
property_impl!(|s| s.margin[Edge::Start], MarginStart(optional Value));
property_impl!(|s| s.margin[Edge::Top], MarginTop(optional Value));
property_impl!(|s| s.margin[Edge::Vertical], MarginVertical(optional Value));
property_impl!(|s| s.max_dimensions.height, MaxHeight(Value));
property_impl!(|s| s.max_dimensions.width, MaxWidth(Value));
property_impl!(|s| s.min_dimensions.height, MinHeight(Value));
property_impl!(|s| s.min_dimensions.width, MinWidth(Value));
property_impl!(|s| s.padding[Edge::Bottom], PaddingBottom(optional Value));
property_impl!(|s| s.padding[Edge::End], PaddingEnd(optional Value));
property_impl!(|s| s.padding[Edge::Horizontal], PaddingHorizontal(optional Value));
property_impl!(|s| s.padding[Edge::Left], PaddingLeft(optional Value));
property_impl!(|s| s.padding[Edge::Right], PaddingRight(optional Value));
property_impl!(|s| s.padding[Edge::Start], PaddingStart(optional Value));
property_impl!(|s| s.padding[Edge::Top], PaddingTop(optional Value));
property_impl!(|s| s.padding[Edge::Vertical], PaddingVertical(optional Value));
property_impl!(|s| s.position[Edge::Right], Right(optional Value));
property_impl!(|s| s.position[Edge::Start], Start(optional Value));
property_impl!(|s| s.position[Edge::Top], Top(optional Value));
// TODO(anp): mirror the height custom setter
property_impl!(|s| s.dimensions.width, Width(Value));
