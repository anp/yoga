internal_prelude!();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Style {
    pub(crate) direction: Direction,
    pub(crate) flex_direction: FlexDirection,
    pub(crate) justify_content: Justify,
    pub(crate) align_content: Align,
    pub(crate) align_items: Align,
    pub(crate) align_self: Align,
    pub(crate) position_type: PositionType,
    pub(crate) flex_wrap: Wrap,
    pub(crate) overflow: Overflow,
    pub(crate) display: Display,
    pub(crate) flex: Option<R32>,
    pub(crate) flex_grow: R32,
    pub(crate) flex_shrink: R32,
    pub(crate) flex_basis: Value,
    pub(crate) margin: Margin,
    pub(crate) position: Position,
    pub(crate) padding: Padding,
    pub(crate) border: Border,
    pub(crate) dimensions: Dimensions,
    pub(crate) min_dimensions: Dimensions,
    pub(crate) max_dimensions: Dimensions,
    // Yoga specific properties, not compatible with flexbox specification
    pub(crate) aspect_ratio: Option<R32>,
}

impl Style {
    pub(crate) const DEFAULT_FLEX_GROW: f32 = 0.0;

    #[cfg(feature = "web-default")]
    pub(crate) const DEFAULT_FLEX_SHRINK: f32 = 1.0;

    #[cfg(not(feature = "web-default"))]
    pub(crate) const DEFAULT_FLEX_SHRINK: f32 = 0.0;

    #[inline]
    pub(crate) fn padding_and_border_for_axis(&self, axis: FlexDirection, width_size: R32) -> R32 {
        self.leading_padding_and_border(axis, width_size)
            + self.trailing_padding_and_border(axis, width_size)
    }

    #[inline]
    pub(crate) fn leading_padding_and_border(&self, axis: FlexDirection, width_size: R32) -> R32 {
        self.padding.leading(axis, width_size) + self.border.leading(axis)
    }

    #[inline]
    pub(crate) fn trailing_padding_and_border(&self, axis: FlexDirection, width_size: R32) -> R32 {
        self.padding.trailing(axis, width_size) + self.border.trailing(axis)
    }

    pub(crate) fn resolve_flex_basis(&self, parent_size: R32) -> Option<R32> {
        if let Value::Point(p) = self.flex_basis {
            Value::Point(p)
        } else if let Value::Percent(p) = self.flex_basis {
            Value::Percent(p)
        } else if let (Some((_, true)), false) = (
            self.flex.map(|f| (f, f > 0.0)),
            cfg!(feature = "web-default"),
        ) {
            Value::Point(r32(0.0))
        } else {
            Value::Auto
        }.resolve(parent_size)
    }
}

default!(
    Style,
    Style {
        direction: Direction::default(),
        flex_direction: FlexDirection::default(),
        justify_content: Justify::default(),
        align_content: Align::FlexStart,
        align_items: Align::Stretch,
        align_self: Align::Auto,
        position_type: PositionType::default(),
        flex_wrap: Wrap::default(),
        overflow: Overflow::default(),
        display: Display::default(),
        flex: None,
        flex_grow: r32(Self::DEFAULT_FLEX_GROW),
        flex_shrink: r32(Self::DEFAULT_FLEX_SHRINK),
        flex_basis: Value::Auto,
        margin: Margin::default(),
        position: Position::default(),
        padding: Padding::default(),
        border: Border::default(),
        // FIXME(anp): i seem to recall there being specific logic in the setter  to prevent
        // these values from being assigned here...
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
);

pub trait Property
where
    Self: Sized,
{
    type Target: Copy + Clone + Eq + PartialEq;

    fn prep(self) -> Self::Target;
    fn value(style: &Style) -> Self::Target;
    fn apply(self, style: &mut Style) -> Updated;
}

macro_rules! property_impl {
    {
        @edge_apply
        struct: $struct:ty,
        target: $target:ty,
        prep: |$inner:ident| $prep:expr,
        style: $style:ident,
        edge: $edge:expr,
        field: $field:expr,
    } => {
        impl Property for $struct {
            type Target = Option<$target>;

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn prep(self) -> Self::Target {
                Some(|$inner: $struct| -> $target { $prep }(self))
            }

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn value(style: &Style) -> Self::Target {
                let $style = style;
                $field.get($edge)
            }

            #[inline]
            fn apply(self, style: &mut Style) -> Updated {
                let $style = style;
                let to_apply = self.prep();

                match ($field[$edge] == to_apply, to_apply) {
                    (false, Some(a)) => $field.set($edge, a),
                    _ => Updated::Clean,
                }
            }
        }
    };
    {
        @trait
        struct: $struct:ty,
        target: $target:ty,
        prep: |$inner:ident| $prep:expr,
        field: |$style:ident| $field:expr
    } => {
        impl Property for $struct {
            type Target = $target;

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn prep(self) -> Self::Target {
                |$inner: $struct| -> $target { $prep }(self)
            }

            // inline attribute necessary for cross-crate inlining
            #[inline]
            fn value(style: &Style) -> Self::Target {
                let $style = style;
                $field
            }

            #[inline]
            fn apply(self, style: &mut Style) -> Updated {
                let $style = style;
                let field = &mut $field;

                let to_apply = self.prep();

                if *field == to_apply {
                    Updated::Clean
                } else {
                    *field = to_apply;
                    Updated::Dirty
                }
            }
        }
    };
    (@trait |$style:ident| $field:expr, $struct:ident) => {
        property_impl! {
            @trait
            struct: $struct,
            target: $struct,
            prep: |v| v,
            field: |$style| $field
        }
    };
    (
        |
        $style:ident |
        $field:expr,
        $struct:ident($contained:ty), |
        $inner:ident | ->
        $target:ty { $prep:expr }
    ) => {
        pub struct $struct(pub $contained);

        property_impl! {
            @trait
            struct: $struct,
            target: $target,
            prep: |$inner| $prep,
            field: |$style| $field
        }
    };
    (| $style:ident | $field:expr, $struct:ident(optional $target:ty)) => {
        property_impl!(
            |$style| $field,
            $struct($target),
            |v| -> Option<$target> { Some(v.0) }
        );
    };
    (| $style:ident | $field:expr, $edge:expr, $struct:ident(optional $target:ty) ) => {
        pub struct $struct(pub $target);

        property_impl!(
            @edge_apply
            struct: $struct,
            target: $target,
            prep: |v| { v.0 },
            style: $style,
            edge: $edge,
            field: $field,
        );
    };
    (| $style:ident | $field:expr, $struct:ident($target:ty)) => {
        property_impl!(|$style| $field, $struct($target), |v| -> $target { v.0 });
    };
}

property_impl!(@trait |s| s.display, Display);
property_impl!(@trait |s| s.flex_direction, FlexDirection);
property_impl!(@trait |s| s.flex_wrap, Wrap);
property_impl!(@trait |s| s.justify_content, Justify);
property_impl!(@trait |s| s.overflow, Overflow);
property_impl!(@trait |s| s.position_type, PositionType);
property_impl!(@trait |s| s.border, Border);
property_impl!(@trait |s| s.margin, Margin);
property_impl!(@trait |s| s.padding, Padding);

property_impl!(|s| s.align_content, AlignContent(Align));
property_impl!(|s| s.align_items, AlignItems(Align));
property_impl!(|s| s.align_self, AlignSelf(Align));
property_impl!(|s| s.aspect_ratio, AspectRatio(optional R32));
property_impl!(|s| s.flex_basis, FlexBasis(Value));
property_impl!(|s| s.flex_grow, FlexGrow(R32));
property_impl!(|s| s.flex_shrink, FlexShrink(R32));
property_impl!(|s| s.flex, Flex(optional R32));
property_impl!(|s| s.max_dimensions.height, MaxHeight(Value));
property_impl!(|s| s.max_dimensions.width, MaxWidth(Value));
property_impl!(|s| s.min_dimensions.height, MinHeight(Value));
property_impl!(|s| s.min_dimensions.width, MinWidth(Value));

property_impl!(|s| s.border, Edge::Bottom, BorderBottom(optional R32));
property_impl!(|s| s.border, Edge::End, BorderEnd(optional R32));
property_impl!(|s| s.border, Edge::Left, BorderLeft(optional R32));
property_impl!(|s| s.border, Edge::Right, BorderRight(optional R32));
property_impl!(|s| s.border, Edge::Start, BorderStart(optional R32));
property_impl!(|s| s.border, Edge::Top, BorderTop(optional R32));
property_impl!(|s| s.margin, Edge::Bottom, MarginBottom(optional Value));
property_impl!(|s| s.margin, Edge::End, MarginEnd(optional Value));
property_impl!(|s| s.margin, Edge::Horizontal, MarginHorizontal(optional Value));
property_impl!(|s| s.margin, Edge::Left, MarginLeft(optional Value));
property_impl!(|s| s.margin, Edge::Right, MarginRight(optional Value));
property_impl!(|s| s.margin, Edge::Start, MarginStart(optional Value));
property_impl!(|s| s.margin, Edge::Top, MarginTop(optional Value));
property_impl!(|s| s.margin, Edge::Vertical, MarginVertical(optional Value));
property_impl!(|s| s.padding, Edge::Bottom, PaddingBottom(optional Value));
property_impl!(|s| s.padding, Edge::End, PaddingEnd(optional Value));
property_impl!(|s| s.padding, Edge::Horizontal, PaddingHorizontal(optional Value));
property_impl!(|s| s.padding, Edge::Left, PaddingLeft(optional Value));
property_impl!(|s| s.padding, Edge::Right, PaddingRight(optional Value));
property_impl!(|s| s.padding, Edge::Start, PaddingStart(optional Value));
property_impl!(|s| s.padding, Edge::Top, PaddingTop(optional Value));
property_impl!(|s| s.padding, Edge::Vertical, PaddingVertical(optional Value));
property_impl!(|s| s.position, Edge::Bottom, Bottom(optional Value));
property_impl!(|s| s.position, Edge::End, End(optional Value));
property_impl!(|s| s.position, Edge::Left, Left(optional Value));
property_impl!(|s| s.position, Edge::Right, Right(optional Value));
property_impl!(|s| s.position, Edge::Start, Start(optional Value));
property_impl!(|s| s.position, Edge::Top, Top(optional Value));

property_impl!(
    |s| s.dimensions.height,
    Height(Value),
    |h| -> Value {
        match h.0 {
            Value::Auto => panic!("invalid to set an auto height!"),
            rest @ _ => rest,
        }
    }
);
property_impl!(
    |s| s.dimensions.width,
    Width(Value),
    |h| -> Value {
        match h.0 {
            Value::Auto => panic!("invalid to set an auto width!"),
            rest @ _ => rest,
        }
    }
);
