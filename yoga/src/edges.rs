prelude!();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Edge {
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

macro_rules! edges {
    ($a:ident: $aty:ty, $b:ident: $bty:ty) => {
        edges! {
            user: $a: $aty,
            resolved: $b: $bty,
            fields: [
                left (set_left),
                top (set_top),
                right (set_right),
                bottom (set_bottom),
                start (set_start),
                end (set_end),
                vertical (set_vertical),
                horizontal (set_horizontal),
                all (set_all)
            ]
        }
    };
    ($a:ident, $b:ident) => {
        edges! {
            user: $a: Value,
            resolved: $b: R32,
            fields: [
                left (set_left),
                top (set_top),
                right (set_right),
                bottom (set_bottom),
                start (set_start),
                end (set_end),
                vertical (set_vertical),
                horizontal (set_horizontal),
                all (set_all)
            ]
        }
    };
    (
        user: $mindlessoutlining:ident: $userty:ty,
        resolved: $mindlessresolution:ident: $resolvedty:ty,
        fields: [ $($field:ident ($set_fn:ident)),* ]
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
        pub struct $mindlessoutlining {
            $(
                $field: Option<$userty>,
            )*
        }

        edges! { @both $mindlessoutlining [ $($field)* ] $userty }

        impl $mindlessoutlining {
            pub fn get(&self, edge: Edge) -> Option<$userty> {
                self[edge]
            }

            $(
                pub(crate) fn $set_fn(&mut self, new: $userty) {
                    self.$field = Some(new);
                }
            )*
        }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
        pub struct $mindlessresolution {
            // TODO(anp): the conversion process from Border -> BorderResolved should obviate
            // the need for Option here
            $(
                $field: Option<$resolvedty>,
            )*
        }

        edges! { @both $mindlessresolution [ $($field)* ] $resolvedty }

        impl $mindlessresolution {
            pub fn get(&self, edge: Edge) -> Option<$resolvedty> {
                self[edge]
            }

            $(
                pub fn $set_fn(&mut self, new: $resolvedty) {
                    self.$field = Some(new);
                }
            )*
        }
    };

    (@both $struct:ident [ $($field:ident)* ] $field_ty:ty) => {
        default!($struct, $struct { $( $field: None, )* });

        impl $struct {
            pub(crate) fn set(&mut self, edge: Edge, new: $field_ty) -> Updated {
                use Edge::*;

                let mut field = match edge {
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

        impl ::std::ops::Index<Edge> for $struct {
            type Output = Option<$field_ty>;
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
    };
}

edges! { Border: R32, BorderResolved: R32 }
edges! { Margin, MarginResolved }
edges! { Padding, PaddingResolved }
edges! { Position, PositionResolved }

impl Border {
    pub fn resolve(&self, row_dir: FlexDirection, col_dir: FlexDirection) -> BorderResolved {
        BorderResolved {
            start: Some(self.leading(row_dir)),
            end: Some(self.trailing(row_dir)),
            top: Some(self.leading(col_dir)),
            bottom: Some(self.trailing(col_dir)),
            // FIXME(anp): pretty sure we shouldn't even have these, right?
            ..Default::default()
        }
    }

    pub fn leading(&self, axis: FlexDirection) -> R32 {
        match (axis.is_row(), self[Edge::Start]) {
            (true, Some(b)) if b >= 0.0 => b,
            _ => self[axis.leading_edge()].unwrap_or(r32(0.0)),
        }.max(r32(0.0))
    }

    pub fn trailing(&self, axis: FlexDirection) -> R32 {
        match (axis.is_row(), self[Edge::End]) {
            (true, Some(b)) if b >= 0.0 => b,
            _ => self[axis.trailing_edge()].unwrap_or(r32(0.0)),
        }.max(r32(0.0))
    }
}

impl Margin {
    pub fn trailing(&self, axis: FlexDirection, width_size: R32) -> Option<R32> {
        match (axis.is_row(), self[Edge::End]) {
            (true, Some(v)) => Some(v),
            _ => self[axis.trailing_edge()],
        }.into_iter()
            .flat_map(|m| m.resolve(width_size))
            .next()
    }

    pub fn for_axis(&self, axis: FlexDirection, width_size: R32) -> R32 {
        self.leading(axis, width_size).unwrap_or(r32(0.0))
            + self.trailing(axis, width_size).unwrap_or(r32(0.0))
    }

    pub fn leading(&self, axis: FlexDirection, width_size: R32) -> Option<R32> {
        match (axis.is_row(), self[Edge::Start]) {
            (true, Some(m)) => Some(m),
            _ => self[axis.leading_edge()],
        }.into_iter()
            .flat_map(|m| m.resolve(width_size))
            .next()
    }

    pub fn resolve(
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
            // FIXME(anp): pretty sure we shouldn't even have these, right?
            ..Default::default()
        }
    }
}

impl Padding {
    pub fn trailing(&self, axis: FlexDirection, parent_width: R32) -> R32 {
        let existing = self[Edge::End];
        let resolved = existing.map(|v| v.resolve(parent_width));

        // TODO(anp): why does leading have a max(0.0) call but this doesn't? my mistake?
        match (axis.is_row(), resolved) {
            (true, Some(Some(p))) if p >= 0.0 => p,
            _ => self[axis.trailing_edge()]
                .into_iter()
                .flat_map(|p| p.resolve(parent_width))
                .next()
                .unwrap_or(r32(0.0)),
        }
    }

    pub fn leading(&self, axis: FlexDirection, parent_width: R32) -> R32 {
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

    pub fn resolve(
        &self,
        row_dir: FlexDirection,
        col_dir: FlexDirection,
        parent_width: R32,
    ) -> PaddingResolved {
        PaddingResolved {
            // FIXME(anp): these don't need to be optional, needs change to macro
            start: Some(self.leading(row_dir, parent_width)),
            end: Some(self.trailing(row_dir, parent_width)),
            top: Some(self.leading(col_dir, parent_width)),
            bottom: Some(self.trailing(col_dir, parent_width)),
            // FIXME(anp): pretty sure we dont actually need these anymore
            ..Default::default()
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
            Edge::Start
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
            Edge::End
        } else {
            axis.trailing_edge()
        };

        self[trailing_edge]
            .into_iter()
            .flat_map(|p| p.resolve(axis_size))
            .next()
    }
}
