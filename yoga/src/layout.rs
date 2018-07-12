internal_prelude!();

// TODO(anp): validate this comment from the original c
/// This value was chosen based on data. Even the most complicated layouts should not require more
/// than 16 entries to fit within the cache.
const MAX_CACHED_RESULTS: usize = 16;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Layout {}

// impl ::std::ops::Index<PhysicalEdge> for Layout {
//     type Output = R32;
//     fn index(&self, edge: PhysicalEdge) -> &Self::Output {
//         self.position.index(edge)
//     }
// }

// default!(
//     Layout,
//     Layout {
//         position: PositionResolved::default(),
//         dimensions: Dimensions {
//             width: Value::Auto,
//             height: Value::Auto,
//         },
//         margin: MarginResolved::default(),
//         border: BorderResolved::default(),
//         padding: PaddingResolved::default(),
//         direction: Direction::default(),
//         computed_flex_basis_generation: 0,
//         computed_flex_basis: None,
//         had_overflow: false,
//         generation_count: 0,
//         // RIIR(anp): this is not technically correct, it was uninit  before
//         last_parent_direction: None,
//         // next_cached_measurements_index: 0,
//         // cached_measurements: [None; 16],
//         measured_dimensions: MeasuredDimensions::default(),
//         // cached_layout: None,
//     }
// );

// CACHING(anp): this needs to be rethought in the rust context
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub(crate) struct CachedMeasurement {
    pub(crate) available_width: R32,
    pub(crate) available_height: R32,
    pub(crate) width_measure_mode: Option<MeasureMode>,
    pub(crate) height_measure_mode: Option<MeasureMode>,
    pub(crate) computed: MeasuredDimensions,
}

impl CachedMeasurement {
    pub(crate) fn usable(
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
