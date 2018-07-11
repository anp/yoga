prelude!();

// TODO(anp): validate this comment from the original c
/// This value was chosen based on data. Even the most complicated layouts should not require more
/// than 16 entries to fit within the cache.
const MAX_CACHED_RESULTS: usize = 16;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub position: PositionResolved,
    // pub left: R32,
    // pub right: R32,
    // pub top: R32,
    // pub bottom: R32,
    pub dimensions: Option<Dimensions>,
    pub direction: Direction,
    pub margin: MarginResolved,
    pub border: BorderResolved,
    pub padding: PaddingResolved,
    pub computed_flex_basis_generation: u32,
    pub computed_flex_basis: Option<R32>,
    pub had_overflow: bool,
    // Instead of recomputing the entire layout every single time, we
    // cache some information to break early when nothing changed:
    pub generation_count: u32,
    pub last_parent_direction: Option<Direction>,
    // TODO(anp): use arrayvec or an LRU crate for these
    // pub(crate) next_cached_measurements_index: usize,
    // pub(crate) cached_measurements: [Option<CachedMeasurement>; MAX_CACHED_RESULTS],
    pub measured_dimensions: MeasuredDimensions,
    // pub(crate) cached_layout: Option<CachedMeasurement>,
}

impl ::std::ops::Index<PhysicalEdge> for Layout {
    type Output = R32;
    fn index(&self, edge: PhysicalEdge) -> &Self::Output {
        self.position.index(edge)
    }
}

default!(
    Layout,
    Layout {
        position: PositionResolved::default(),
        dimensions: None,
        margin: MarginResolved::default(),
        border: BorderResolved::default(),
        padding: PaddingResolved::default(),
        direction: Direction::default(),
        computed_flex_basis_generation: 0,
        computed_flex_basis: None,
        had_overflow: false,
        generation_count: 0,
        // RIIR(anp): this is not technically correct, it was uninit  before
        last_parent_direction: None,
        // next_cached_measurements_index: 0,
        // cached_measurements: [None; 16],
        measured_dimensions: MeasuredDimensions::default(),
        // cached_layout: None,
    }
);

impl Layout {
    pub(crate) fn set_position(
        &mut self,
        style: Style,
        direction: Direction,
        main_size: R32,
        cross_size: R32,
        parent_width: R32,
        has_parent: bool,
    ) {
        // Root nodes should be always layouted as LTR, so we don't return negative values.
        let direction_respecting_root: Direction = if has_parent {
            direction
        } else {
            Direction::LTR
        };

        let main_axis: FlexDirection = style
            .flex_direction
            .resolve_direction(direction_respecting_root);

        let cross_axis: FlexDirection = main_axis.cross(direction_respecting_root);

        self.position = style.position.resolve(
            &style.margin,
            main_axis,
            main_size,
            cross_axis,
            cross_size,
            parent_width,
        );
    }

    pub fn is_dim_defined(&self, axis: FlexDirection) -> bool {
        self.measured_dimensions[axis.dimension()] >= 0.0
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
        point_scale_factor: R32,
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

        if last_computed_height < 0.0 || last_computed_width < 0.0 {
            return false;
        };

        let (effective_width, effective_height, effective_last_width, effective_last_height) =
            if point_scale_factor != 0.0 {
                let rounder = |v| round_value_to_pixel_grid(v, point_scale_factor, false, false);
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
