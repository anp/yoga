prelude!();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum Edge {
    Left,
    Top,
    Right,
    Bottom,
    Start,
    End,
    Horizontal,
    Vertical,
    All,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum PhysicalEdge {
    Top,
    Bottom,
    // TODO(anp) pretty sure these should be left/right
    Start,
    End,
}

macro_rules! edges {
    ($a:ident: $aty:ty, $b:ident: $bty:ty) => {
        edges! {
            user: $a: $aty,
            resolved: $b: $bty,
            fields: [
                top (set_top),
                bottom (set_bottom),
                start (set_start),
                end (set_end),
                left (set_left),
                right (set_right),
                vertical (set_vertical),
                horizontal (set_horizontal),
                all (set_all)
            ],
            resolved_fields: [ top, bottom, start, end ]
        }
    };
    ($a:ident, $b:ident) => {
        edges! {
            user: $a: Value,
            resolved: $b: R32,
            fields: [
                top (set_top),
                bottom (set_bottom),
                start (set_start),
                end (set_end),
                left (set_left),
                right (set_right),
                vertical (set_vertical),
                horizontal (set_horizontal),
                all (set_all)
            ],
            resolved_fields: [ top, bottom, start, end ]
        }
    };
    (
        user: $mindlessoutlining:ident: $userty:ty,
        resolved: $mindlessresolution:ident: $resolvedty:ty,
        fields: [ $($field:ident ($set_fn:ident)),* ],
        resolved_fields: [ $( $resolvedfield:ident ),* ]
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
        pub(crate) struct $mindlessoutlining {
            $(
                $field: Option<$userty>,
            )*
        }


        default!($mindlessoutlining, $mindlessoutlining { $( $field: None, )* });

        impl ::std::ops::Index<Edge> for $mindlessoutlining {
            type Output = Option<$userty>;
            fn index(&self, edge: Edge) -> &Self::Output {
                use Edge::*;
                match edge {
                    // first we see if the literal value is filled in
                    Top if self.top.is_some() => &self.top,
                    Bottom if self.bottom.is_some() => &self.bottom,
                    Left if self.left.is_some() => &self.left,
                    Right if self.right.is_some() => &self.right,
                    Start if self.start.is_some() => &self.start,
                    End if self.end.is_some() => &self.end,
                    Vertical if self.vertical.is_some() => &self.vertical,
                    Horizontal if self.horizontal.is_some() => &self.horizontal,
                    All if self.all.is_some() => &self.all,

                    // now we try to make fallbacks work
                    Top |
                    Bottom
                    if self.vertical.is_some() => &self.vertical,

                    Left |
                    Right |
                    Start |
                    End
                    if self.horizontal.is_some() => &self.horizontal,

                    // one fallback to rule them all
                    _ if self.all.is_some() => &self.all,
                    _ => &None,
                }
            }
        }

        impl ::std::ops::Index<PhysicalEdge> for $mindlessoutlining {
            type Output = Option<$userty>;
            fn index(&self, edge: PhysicalEdge) -> &Self::Output {
                use PhysicalEdge::*;
                match edge {
                    Top if self.top.is_some() => &self.top,
                    Bottom if self.bottom.is_some() => &self.bottom,
                    Start if self.start.is_some() => &self.start,
                    End if self.end.is_some() => &self.end,

                    Top |
                    Bottom
                    if self.vertical.is_some() => &self.vertical,

                    Start |
                    End
                    if self.horizontal.is_some() => &self.horizontal,

                    // one fallback to rule them all
                    _ if self.all.is_some() => &self.all,
                    _ => &None,
                }
            }
        }

        impl $mindlessoutlining {
            pub(crate) fn get(&self, edge: Edge) -> Option<$userty> {
                self[edge]
            }

            pub(crate) fn set(&mut self, edge: Edge, new: $userty) -> Updated {
                use Edge::*;

                let field = match edge {
                    Top => &mut self.top,
                    Bottom => &mut self.bottom,
                    Left => &mut self.left,
                    Right => &mut self.right,
                    Start => &mut self.start,
                    End => &mut self.end,
                    Vertical => &mut self.vertical,
                    Horizontal => &mut self.horizontal,
                    All => &mut self.all,
                };

                if *field == Some(new) {
                    Updated::Clean
                } else {
                    *field = Some(new);
                    Updated::Dirty
                }
            }
        }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
        pub(crate) struct $mindlessresolution {
            $(
                pub(crate) $resolvedfield: $resolvedty,
            )*
        }

        // TODO(anp): take this away and require full initialization of layout fields
        default!(
            $mindlessresolution,
            $mindlessresolution {
                start: R32::default(),
                end: R32::default(),
                top: R32::default(),
                bottom: R32::default(),
            }
        );

        impl ::std::ops::Index<PhysicalEdge> for $mindlessresolution {
            type Output = $resolvedty;
            fn index(&self, edge: PhysicalEdge) -> &Self::Output {
                &self.get(edge)
            }
        }

        impl $mindlessresolution {
            pub(crate) fn get(&self, edge: PhysicalEdge) -> &$resolvedty {
                use PhysicalEdge::*;
                match edge {
                    Top => &self.top,
                    Bottom => &self.bottom,
                    Start => &self.start,
                    End => &self.end,
                }
            }

            pub(crate) fn add(&mut self, edge: PhysicalEdge, new: $resolvedty) {
                use PhysicalEdge::*;

                let field = match edge {
                    Top => &mut self.top,
                    Bottom => &mut self.bottom,
                    Start => &mut self.start,
                    End => &mut self.end,
                };

                *field = *field + new;
            }

            pub(crate) fn set(&mut self, edge: PhysicalEdge, new: $resolvedty) {
                use PhysicalEdge::*;

                let field = match edge {
                    Top => &mut self.top,
                    Bottom => &mut self.bottom,
                    Start => &mut self.start,
                    End => &mut self.end,
                };

                *field = new;
            }
        }
    };
}

edges! { Border: R32, BorderResolved: R32 }
edges! { Margin, MarginResolved }
edges! { Padding, PaddingResolved }
edges! { Position, PositionResolved }

impl Border {
    pub(crate) fn resolve(&self, row_dir: FlexDirection, col_dir: FlexDirection) -> BorderResolved {
        BorderResolved {
            start: self.leading(row_dir),
            end: self.trailing(row_dir),
            top: self.leading(col_dir),
            bottom: self.trailing(col_dir),
        }
    }

    pub(crate) fn leading(&self, axis: FlexDirection) -> R32 {
        match (axis.is_row(), self[Edge::Start]) {
            (true, Some(b)) if b >= 0.0 => b,
            _ => self[axis.leading_edge()].unwrap_or(r32(0.0)),
        }.max(r32(0.0))
    }

    pub(crate) fn trailing(&self, axis: FlexDirection) -> R32 {
        match (axis.is_row(), self[Edge::End]) {
            (true, Some(b)) if b >= 0.0 => b,
            _ => self[axis.trailing_edge()].unwrap_or(r32(0.0)),
        }.max(r32(0.0))
    }
}

impl Margin {
    pub(crate) fn leading_value(&self, axis: FlexDirection) -> Option<Value> {
        match (axis.is_row(), self[Edge::Start], self[axis.leading_edge()]) {
            (true, Some(m), _) | (_, _, Some(m)) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn trailing_value(&self, axis: FlexDirection) -> Option<Value> {
        match (axis.is_row(), self[Edge::End], self[axis.trailing_edge()]) {
            (true, Some(m), _) | (_, _, Some(m)) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn trailing(&self, axis: FlexDirection, width_size: R32) -> R32 {
        match (axis.is_row(), self[Edge::End]) {
            (true, Some(v)) => Some(v),
            _ => self[axis.trailing_edge()],
        }.into_iter()
            .flat_map(|m| m.resolve(width_size))
            .next()
            .unwrap_or(r32(0.0))
    }

    pub(crate) fn for_axis(&self, axis: FlexDirection, width_size: R32) -> R32 {
        self.leading(axis, width_size) + self.trailing(axis, width_size)
    }

    pub(crate) fn leading(&self, axis: FlexDirection, width_size: R32) -> R32 {
        match (axis.is_row(), self[Edge::Start]) {
            (true, Some(m)) => Some(m),
            _ => self[axis.leading_edge()],
        }.into_iter()
            .flat_map(|m| m.resolve(width_size))
            .next()
            .unwrap_or(r32(0.0))
    }

    pub(crate) fn resolve(
        &self,
        row_dir: FlexDirection,
        col_dir: FlexDirection,
        parent_width: R32,
    ) -> MarginResolved {
        MarginResolved {
            start: self.leading(row_dir, parent_width),
            end: self.trailing(row_dir, parent_width),
            top: self.leading(col_dir, parent_width),
            bottom: self.trailing(col_dir, parent_width),
        }
    }
}

impl Padding {
    pub(crate) fn trailing(&self, axis: FlexDirection, parent_width: R32) -> R32 {
        let existing = self[Edge::End];
        let resolved = existing.map(|v| v.resolve(parent_width));

        match (axis.is_row(), resolved) {
            (true, Some(Some(p))) if p >= 0.0 => p,
            _ => self[axis.trailing_edge()]
                .into_iter()
                .flat_map(|p| p.resolve(parent_width))
                .next()
                .unwrap_or(r32(0.0)),
        }.max(r32(0.0))
    }

    pub(crate) fn leading(&self, axis: FlexDirection, parent_width: R32) -> R32 {
        let existing = self[Edge::Start];
        let resolved = existing.map(|v| v.resolve(parent_width));

        match (axis.is_row(), resolved) {
            (true, Some(Some(p))) if p >= 0.0 => p,
            _ => self[axis.leading_edge()]
                .into_iter()
                .flat_map(|p| p.resolve(parent_width))
                .next()
                .unwrap_or(r32(0.0)),
        }.max(r32(0.0))
    }

    pub(crate) fn resolve(
        &self,
        row_dir: FlexDirection,
        col_dir: FlexDirection,
        parent_width: R32,
    ) -> PaddingResolved {
        PaddingResolved {
            start: self.leading(row_dir, parent_width),
            end: self.trailing(row_dir, parent_width),
            top: self.leading(col_dir, parent_width),
            bottom: self.trailing(col_dir, parent_width),
        }
    }
}

impl Position {
    /// If both left and right are defined, then use left. Otherwise return
    /// +left or -right depending on which is defined.
    // was YGNodeRelativePosition
    pub(crate) fn relative(&self, axis: FlexDirection, axis_size: R32) -> Option<R32> {
        if let Some(pos) = self.leading(axis, axis_size) {
            Some(pos)
        } else {
            self.trailing(axis, axis_size).map(|p| -p)
        }
    }

    pub(crate) fn leading(&self, axis: FlexDirection, axis_size: R32) -> Option<R32> {
        let leading_edge = if axis.is_row() {
            PhysicalEdge::Start
        } else {
            axis.leading_edge()
        };

        self[leading_edge]
            .into_iter()
            .flat_map(|p| p.resolve(axis_size))
            .next()
    }

    pub(crate) fn trailing(&self, axis: FlexDirection, axis_size: R32) -> Option<R32> {
        let trailing_edge = if axis.is_row() {
            PhysicalEdge::End
        } else {
            axis.trailing_edge()
        };

        self[trailing_edge]
            .into_iter()
            .flat_map(|p| p.resolve(axis_size))
            .next()
    }

    pub(crate) fn resolve(
        &self,
        margin: &Margin,
        main_axis: FlexDirection,
        main_size: R32,
        cross_axis: FlexDirection,
        cross_size: R32,
        parent_width: R32,
    ) -> PositionResolved {
        let relative_position_main = self.relative(main_axis, main_size).unwrap_or(r32(0.0));
        let relative_position_cross = self.relative(cross_axis, cross_size).unwrap_or(r32(0.0));

        PositionBuilder::new()
            .set(
                main_axis.leading_edge(),
                margin.leading(main_axis, parent_width) + relative_position_main,
            )
            .set(
                main_axis.trailing_edge(),
                margin.trailing(main_axis, parent_width) + relative_position_main,
            )
            .set(
                cross_axis.leading_edge(),
                margin.leading(cross_axis, parent_width) + relative_position_cross,
            )
            .set(
                cross_axis.trailing_edge(),
                margin.trailing(cross_axis, parent_width) + relative_position_cross,
            )
            .build()
    }
}

struct PositionBuilder {
    start: Option<R32>,
    end: Option<R32>,
    top: Option<R32>,
    bottom: Option<R32>,
}

impl PositionBuilder {
    fn new() -> Self {
        PositionBuilder {
            start: None,
            end: None,
            top: None,
            bottom: None,
        }
    }

    fn set(&mut self, edge: PhysicalEdge, value: R32) -> &mut Self {
        use PhysicalEdge::*;
        match edge {
            Top => self.top = Some(value),
            Bottom => self.bottom = Some(value),
            Start => self.start = Some(value),
            End => self.end = Some(value),
        }

        self
    }

    fn build(&mut self) -> PositionResolved {
        match self {
            PositionBuilder {
                start: Some(start),
                end: Some(end),
                top: Some(top),
                bottom: Some(bottom),
            } => PositionResolved {
                start: *start,
                end: *end,
                top: *top,
                bottom: *bottom,
            },
            _ => panic!("not all position fields have been set"),
        }
    }
}
