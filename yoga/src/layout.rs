prelude!();

// TODO(anp): validate this comment from the original c
/// This value was chosen based on data. Even the most complicated layouts should not require more
/// than 16 entries to fit within the cache.
const MAX_CACHED_RESULTS: usize = 16;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub left: R32,
    pub right: R32,
    pub top: R32,
    pub bottom: R32,
    pub dimensions: Option<Dimensions>,
    pub direction: Direction,
    pub margin: Edges<R32>,
    pub border: Edges<R32>,
    pub padding: Edges<R32>,
    pub computed_flex_basis_generation: u32,
    pub computed_flex_basis: Option<R32>,
    pub had_overflow: bool,
    // Instead of recomputing the entire layout every single time, we
    // cache some information to break early when nothing changed:
    pub generation_count: u32,
    pub last_parent_direction: Direction,
    // TODO(anp): use arrayvec or an LRU crate for these
    pub(crate) next_cached_measurements_index: usize,
    pub(crate) cached_measurements: [Option<CachedMeasurement>; MAX_CACHED_RESULTS],
    pub measured_dimensions: Option<MeasuredDimensions>,
    pub(crate) cached_layout: Option<CachedMeasurement>,
}

impl ::std::ops::Index<Edge> for Layout {
    type Output = R32;
    fn index(&self, edge: Edge) -> &Self::Output {
        match edge {
            Edge::Left => &self.left,
            Edge::Right => &self.right,
            Edge::Top => &self.top,
            Edge::Bottom => &self.bottom,
            _ => panic!("passed an invalid edge to index into the layout struct"),
        }
    }
}

impl ::std::ops::IndexMut<Edge> for Layout {
    fn index_mut(&mut self, edge: Edge) -> &mut Self::Output {
        match edge {
            Edge::Left => &mut self.left,
            Edge::Right => &mut self.right,
            Edge::Top => &mut self.top,
            Edge::Bottom => &mut self.bottom,
            _ => panic!("passed an invalid edge to index into the layout struct"),
        }
    }
}

impl ::std::default::Default for Layout {
    fn default() -> Self {
        Layout {
            left: r32(0.0),
            right: r32(0.0),
            top: r32(0.0),
            bottom: r32(0.0),
            dimensions: None,
            margin: Edges::empty(),
            border: Edges::empty(),
            padding: Edges::empty(),
            direction: Direction::Inherit,
            computed_flex_basis_generation: 0,
            computed_flex_basis: None,
            had_overflow: false,
            generation_count: 0,
            // RIIR(anp): this is not technically correct, it was uninit  before
            last_parent_direction: Direction::Inherit,
            next_cached_measurements_index: 0,
            cached_measurements: [None; 16],
            measured_dimensions: None,
            cached_layout: None,
        }
    }
}

impl Layout {
    fn edge_with_direction(&self, edge: Edge) -> Edge {
        match (edge, self.direction) {
            (Edge::Left, Direction::RTL) => Edge::End,
            (Edge::Left, _) => Edge::Start,
            (Edge::Right, Direction::RTL) => Edge::Start,
            (Edge::Right, _) => Edge::End,
            _ => edge,
        }
    }

    // pub unsafe fn YGNodeLayoutGetMargin(node: Node, edge: Edge) -> c_float {
    pub fn margin(&self, edge: Edge) -> Option<R32> {
        assert!(
            edge != Edge::Horizontal && edge != Edge::Vertical && edge != Edge::All,
            "cannot get layout properties of multi-edge shorthands. node: {:?}",
            self
        );

        self.margin[self.edge_with_direction(edge)]
    }

    pub fn border(&self, edge: Edge) -> Option<R32> {
        assert!(
            edge != Edge::Horizontal && edge != Edge::Vertical && edge != Edge::All,
            "cannot get layout properties of multi-edge shorthands. node: {:?}",
            self
        );

        self.border[self.edge_with_direction(edge)]
    }

    pub fn padding(&self, edge: Edge) -> Option<R32> {
        assert!(
            edge != Edge::Horizontal && edge != Edge::Vertical && edge != Edge::All,
            "cannot get layout properties of multi-edge shorthands. node: {:?}",
            self
        );

        self.padding[self.edge_with_direction(edge)]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct CachedMeasurement {
    pub available_width: R32,
    pub available_height: R32,
    pub width_measure_mode: Option<MeasureMode>,
    pub height_measure_mode: Option<MeasureMode>,
    pub computed: MeasuredDimensions,
}

impl CachedMeasurement {
    pub fn usable(
        this: Option<CachedMeasurement>,
        width_mode: Option<MeasureMode>,
        width: R32,
        height_mode: Option<MeasureMode>,
        height: R32,
        margin_row: R32,
        margin_column: R32,
    ) -> bool {
        let Self {
            available_width: last_width,
            available_height: last_height,
            width_measure_mode: last_width_mode,
            height_measure_mode: last_height_mode,
            computed:
                MeasuredDimensions {
                    width: last_computed_width,
                    height: last_computed_height,
                },
        } = match this {
            Some(t) => t,
            None => return false,
        };

        // TODO(anp): figure out how we'd get here? should we have positive float types for all of this?
        if last_computed_height < 0.0 || last_computed_width < 0.0 {
            return false;
        };

        let (effective_width, effective_height, effective_last_width, effective_last_height) =
            if POINT_SCALE_FACTOR != 0.0 {
                let rounder = |v| round_value_to_pixel_grid(v, POINT_SCALE_FACTOR, false, false);
                (
                    rounder(width),
                    rounder(height),
                    rounder(last_width),
                    rounder(last_height),
                )
            } else {
                (width, height, last_width, last_height)
            };

        let is_compatible =
            |has_same_spec, new_mode, new_space, margin, last_computed, last_space, last_mode| {
                has_same_spec
                    || MeasureMode::size_is_exact_and_matches_old_measured_size(
                        new_mode,
                        new_space - margin,
                        last_computed,
                    )
                    || MeasureMode::old_size_is_unspecified_and_still_fits(
                        new_mode,
                        new_space - margin,
                        last_mode,
                        last_computed,
                    )
                    || MeasureMode::new_measure_size_is_stricter_and_still_valid(
                        last_mode,
                        last_space,
                        last_computed,
                        new_mode,
                        new_space - margin,
                    )
            };

        is_compatible(
            last_width_mode == width_mode && effective_last_width.approx_eq(effective_width),
            width_mode,
            width,
            margin_row,
            last_computed_width,
            last_width,
            last_width_mode,
        )
            && is_compatible(
                last_height_mode == height_mode
                    && effective_last_height.approx_eq(effective_height),
                height_mode,
                height,
                margin_column,
                last_computed_height,
                last_height,
                last_height_mode,
            )
    }
}
