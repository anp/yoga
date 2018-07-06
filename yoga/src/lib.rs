#![feature(specialization)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unknown_lints)]
// TODO(anp): what is itertools going to do once flatten stabilizes?
#![allow(unstable_name_collisions)]
#![warn(clippy)]

// TODO(anp): look at `gPrintChanges` variable in Yoga.c and add logging statements here
// TODO(anp): excise unwrap/expect/panic!
// TODO(anp): excise unsafe!
// TODO(anp): check out the inline annotations from the c code
// TODO(anp): revist raph's continuation-based layout stuff, in case you forget, june 2018 meetup at mozilla
// TODO(anp): pub/pub(crate)/private audit
// TODO(anp): #![deny(missing_docs)]
// TODO(anp): mutability pass
// TODO(anp): create a style builder that can be constructed with some defaults
//   and used to churn out nodes
// TODO(anp): do a pass to remove .is_nan()

extern crate arrayvec;
extern crate float_cmp;
extern crate itertools;
extern crate libc;
#[macro_use]
extern crate log;
extern crate noisy_float;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::edges::*;
    pub(crate) use super::enums::*;
    pub(crate) use super::hacks::ApproxEqHackForReals;
    pub(crate) use super::layout::{CachedMeasurement, Layout};
    pub(crate) use super::style::{Property, Style};
    pub(crate) use super::Node;
    pub(crate) use itertools::Itertools;
    pub(crate) use noisy_float::prelude::*;
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use std::default::Default;
    pub(crate) use std::fmt::Debug;
    pub(crate) use std::hash::Hash;
    pub(crate) use std::ops::{Index, IndexMut};
}

#[macro_use]
macro_rules! prelude {
    () => {
        #[allow(unused_imports)]
        use $crate::prelude::*;
    };
}

#[macro_use]
pub(crate) mod hacks;

pub mod edges;
pub mod enums;
pub mod layout;
pub mod style;

prelude!();

// FIXME(anp): this seems...wrong
// static mut gDepth: uint32_t = 0i32 as uint32_t;

// TODO(anp): do these even need to exist?
// static mut firstAbsoluteChild: Node = ::std::ptr::null_mut();
// static mut currentAbsoluteChild: Node = ::std::ptr::null_mut();

pub trait Yggdrasil<N: Node<Self>>
where
    N: Node<Self>,
    Self: 'static + Debug + Sized,
{
    // TODO(anp): implement this as a linked list or smth i guess? probably needs to be done with
    //   handles
    fn add_absolute_child(&N);
}

pub trait Node<Y>
where
    Y: Yggdrasil<Self>,
    Self: 'static + Debug + Eq + PartialEq + Sized,
{
    // TODO(anp): should probably be runtime configurable in some ergonomic way that doesn't force
    // an extra field onto frequently-created structs
    const POINT_SCALE_FACTOR: f32 = 1.0;

    // TODO(anp): eliminate mutable methods and require all mutations to be passed back to caller
    fn parent(&self) -> Option<&Self>;
    fn parent_mut(&mut self) -> Option<&mut Self>;
    fn child(&self, index: usize) -> &Self;
    fn child_mut(&mut self, index: usize) -> &mut Self;
    // TODO(anp): abstract over this, children shouldn't have to be in a slice
    fn children(&self) -> &[Self];
    fn children_mut(&mut self) -> &mut [Self];
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn layout(&self) -> &Layout;
    fn layout_mut(&mut self) -> &mut Layout;
    fn line(&mut self) -> &mut usize;
    // TODO(anp): can this be easly done without dynamic dispatch?
    // thought from later: probably not, it would require dyanamic dispatch of all nodes, much less
    // this specific method
    fn measure_fn(
        &self,
    ) -> Option<&'static Fn(&Self, R32, Option<MeasureMode>, R32, Option<MeasureMode>) -> Size>;
    fn baseline_fn(&self) -> Option<&'static Fn(&Self, R32, R32) -> R32>;
    fn dirty(&mut self) -> &mut bool;
    fn new_layout(&mut self) -> &mut bool;
    fn node_type(&self) -> NodeType;
    fn resolved(&self) -> &ResolvedDimensions;
    fn resolved_mut(&mut self) -> &mut ResolvedDimensions;

    fn increment_generation();
    fn current_generation(&self) -> u32;

    fn is_style_dim_defined(&self, axis: FlexDirection, parent_size: R32) -> bool {
        parent_size.is_nan() || match self.resolved()[axis.dimension()] {
            Some(Value::Percent(r)) | Some(Value::Point(r)) => r < 0.0,
            Some(Value::Auto) => false,
            None => true,
        }
    }

    fn resolve_dimensions(&mut self) {
        for &dim in [Dimension::Width, Dimension::Height].into_iter() {
            let style = *self.style();

            if style.max_dimensions[dim] != style.min_dimensions[dim] {
                self.resolved_mut()[dim] = Some(style.max_dimensions[dim]);
            } else {
                self.resolved_mut()[dim] = Some(style.dimensions[dim]);
            };
        }
    }

    fn calculate_layout(
        &mut self,
        parent_width: R32,
        parent_height: R32,
        parent_direction: Direction,
    ) {
        // Increment the generation count. This will force the recursive routine to
        // visit
        // all dirty nodes at least once. Subsequent visits will be skipped if the
        // input
        // parameters don't change.
        Self::increment_generation();

        self.resolve_dimensions();

        let (width, width_measure_mode): (Option<R32>, Option<MeasureMode>) =
            if self.is_style_dim_defined(FlexDirection::Row, parent_width) {
                (
                    self.resolved()[FlexDirection::Row.dimension()]
                        .into_iter()
                        .flat_map(|v| v.resolve(parent_width))
                        .map(|v| {
                            v + self
                                .style()
                                .margin
                                .for_axis(FlexDirection::Row, parent_width)
                        })
                        .next(),
                    Some(MeasureMode::Exactly),
                )
            } else {
                if self.style().max_dimensions.width.resolve(parent_width) >= Some(r32(0.0)) {
                    (
                        self.style().max_dimensions.width.resolve(parent_width),
                        Some(MeasureMode::AtMost),
                    )
                } else {
                    (Some(parent_width), None)
                }
            };

        let (height, height_measure_mode): (Option<R32>, Option<MeasureMode>) =
            if self.is_style_dim_defined(FlexDirection::Column, parent_height) {
                (
                    self.resolved()[FlexDirection::Column.dimension()]
                        .into_iter()
                        .flat_map(|v| v.resolve(parent_height))
                        .map(|v| {
                            v + self
                                .style()
                                .margin
                                .for_axis(FlexDirection::Column, parent_height)
                        })
                        .next(),
                    Some(MeasureMode::Exactly),
                )
            } else {
                if self.style().max_dimensions.height.resolve(parent_height) >= Some(r32(0.0)) {
                    (
                        self.style().max_dimensions.height.resolve(parent_height),
                        Some(MeasureMode::AtMost),
                    )
                } else {
                    (Some(parent_height), None)
                }
            };

        let did_something_wat = self.layout_node_internal(
            width.unwrap(),
            height.unwrap(),
            parent_direction,
            width_measure_mode,
            height_measure_mode,
            parent_width,
            parent_height,
            r32(Self::POINT_SCALE_FACTOR),
            true,
            "initial",
        );

        if did_something_wat {
            let dir = self.layout().direction;
            self.set_position(dir, parent_width, parent_height, parent_width);

            // FIXME(anp): uncomment
            // YGRoundToPixelGrid(node, (*self.config).pointScaleFactor, 0.0f32, 0.0f32);
        };
    }

    /// This is a wrapper around the layoutImpl function. It determines whether the layout
    /// request is redundant and can be skipped. Input parameters are the same as `layoutImpl`
    /// (see above). Return parameter is true if layout was performed, false if skipped.
    fn layout_node_internal(
        &mut self,
        available_width: R32,
        available_height: R32,
        parent_direction: Direction,
        width_measure_mode: Option<MeasureMode>,
        height_measure_mode: Option<MeasureMode>,
        parent_width: R32,
        parent_height: R32,
        point_scale_factor: R32,
        perform_layout: bool,
        reason: &str,
        // TODO(anp): make the return type an enum!!!!
    ) -> bool {
        trace!("layout for reason {} on node {:?}", reason, self);

        // FIXME(anp): make this non-static wtf
        // gDepth = gDepth.wrapping_add(1);

        let current_generation = self.current_generation();
        let need_to_visit_node = *self.dirty()
            && self.layout().generation_count != current_generation
            || self.layout().last_parent_direction != Some(parent_direction);

        if need_to_visit_node {
            // Invalidate the cached results.
            self.layout_mut().cached_layout = None;
            self.layout_mut().next_cached_measurements_index = 0;
        };

        // Determine whether the results are already cached. We maintain a separate
        // cache for layouts and measurements. A layout operation modifies the
        // positions
        // and dimensions for nodes in the subtree. The algorithm assumes that each
        // node
        // gets layed out a maximum of one time per tree layout, but multiple
        // measurements
        // may be required to resolve all of the flex dimensions.
        // We handle nodes with measure functions specially here because they are the
        // most
        // expensive to measure, so it's worth avoiding redundant measurements if at
        // all possible.
        let cached_results = if let Some(cached) = self.layout().cached_layout {
            if let Some(_) = self.measure_fn() {
                let margin_axis_row = self
                    .style()
                    .margin
                    .for_axis(FlexDirection::Row, parent_width);
                let margin_axis_column = self
                    .style()
                    .margin
                    .for_axis(FlexDirection::Column, parent_height);
                // First, try to use the layout cache.
                if CachedMeasurement::usable(
                    Some(cached),
                    width_measure_mode,
                    available_width,
                    height_measure_mode,
                    available_height,
                    margin_axis_row,
                    margin_axis_column,
                    point_scale_factor,
                ) {
                    Some(cached)
                } else {
                    // Try to use the measurement cache.
                    let idx = self.layout().next_cached_measurements_index;
                    self.layout().cached_measurements[0..idx]
                        .into_iter()
                        .find(|c| {
                            CachedMeasurement::usable(
                                **c,
                                width_measure_mode,
                                available_width,
                                height_measure_mode,
                                available_height,
                                margin_axis_row,
                                margin_axis_column,
                                point_scale_factor,
                            )
                        })
                        .into_iter()
                        .flatten()
                        .map(|&v| v)
                        .next()
                }
            } else if perform_layout
                && cached.available_width.approx_eq(available_width)
                && cached.available_height.approx_eq(available_height)
                && cached.width_measure_mode == width_measure_mode
                && cached.height_measure_mode == height_measure_mode
            {
                Some(cached)
            } else {
                let idx = self.layout().next_cached_measurements_index;
                self.layout().cached_measurements[0..idx]
                    .into_iter()
                    .filter_map(|&s| s)
                    .filter(|c| {
                        c.available_width.approx_eq(available_width)
                            && c.available_height.approx_eq(available_height)
                            && c.width_measure_mode == width_measure_mode
                            && c.height_measure_mode == height_measure_mode
                    })
                    .next()
            }
        } else {
            None
        };

        if let (false, Some(cached)) = (need_to_visit_node, cached_results) {
            self.layout_mut().measured_dimensions = Some(cached.computed);
        } else {
            self.layout_impl(
                available_width,
                available_height,
                parent_direction,
                width_measure_mode,
                height_measure_mode,
                parent_width,
                parent_height,
                perform_layout,
            );

            self.layout_mut().last_parent_direction = Some(parent_direction);
            if cached_results.is_none() {
                if self.layout_mut().next_cached_measurements_index == 16 {
                    self.layout_mut().next_cached_measurements_index = 0;
                };

                let computed = self.layout_mut().measured_dimensions.unwrap();

                let mut new_cache_entry = if perform_layout {
                    // Use the single layout cache entry.
                    &mut self.layout_mut().cached_layout
                } else {
                    self.layout_mut().next_cached_measurements_index += 1;
                    let idx = self.layout_mut().next_cached_measurements_index;
                    &mut self.layout_mut().cached_measurements[idx]
                };

                *new_cache_entry = Some(CachedMeasurement {
                    available_width: available_width,
                    available_height: available_height,
                    width_measure_mode: width_measure_mode,
                    height_measure_mode: height_measure_mode,
                    computed,
                });
            }
        }

        self.layout_mut().generation_count = self.current_generation();

        if perform_layout {
            self.layout_mut().dimensions = self.layout_mut().measured_dimensions.map(|d| d.into());
            self.new_layout();
            *self.dirty() = false;
        };

        // FIXME(anp) make this not static wtf
        // gDepth = gDepth.wrapping_sub(1);

        return need_to_visit_node || cached_results.is_none();
    }

    fn mark_dirty(&mut self) {
        let dirty = *self.dirty();
        if !dirty {
            *self.dirty() = true;
            self.layout_mut().computed_flex_basis = None;

            if let Some(p) = self.parent_mut() {
                p.mark_dirty();
            }
        };
    }

    fn apply_style<P: Property>(&mut self, new_style: P) {
        if Updated::Dirty == new_style.apply(self.style_mut()) {
            self.mark_dirty();
        }
    }

    fn set_position(
        &mut self,
        direction: Direction,
        main_size: R32,
        cross_size: R32,
        parent_width: R32,
    ) {
        // Root nodes should be always layouted as LTR, so we don't return negative values.
        let direction_respecting_root: Direction = if self.parent().is_some() {
            direction
        } else {
            Direction::LTR
        };

        let main_axis: FlexDirection = self
            .style()
            .flex_direction
            .resolve_direction(direction_respecting_root);

        let cross_axis: FlexDirection = main_axis.cross(direction_respecting_root);
        let relative_position_main = self
            .style()
            .position
            .relative(main_axis, main_size)
            .unwrap_or(r32(0.0));
        let relative_position_cross = self
            .style()
            .position
            .relative(cross_axis, cross_size)
            .unwrap_or(r32(0.0));

        *self.layout_mut().index_mut(main_axis.leading_edge()) =
            self.style()
                .margin
                .leading(main_axis, parent_width)
                .unwrap_or(r32(0.0)) + relative_position_main;

        *self.layout_mut().index_mut(main_axis.trailing_edge()) =
            self.style()
                .margin
                .trailing(main_axis, parent_width)
                .unwrap_or(r32(0.0)) + relative_position_main;

        *self.layout_mut().index_mut(cross_axis.leading_edge()) =
            self.style()
                .margin
                .leading(cross_axis, parent_width)
                .unwrap_or(r32(0.0)) + relative_position_cross;

        // FIXME(anp): this looks like a bug
        *self.layout_mut().index_mut(cross_axis.trailing_edge()) = self
            .style()
            .margin
            .trailing(cross_axis, parent_width)
            .unwrap_or(r32(0.0));
    }

    fn with_measure_func_set_measured_dimensions(
        &mut self,
        available_width: R32,
        available_height: R32,
        width_measure_mode: Option<MeasureMode>,
        height_measure_mode: Option<MeasureMode>,
        parent_width: R32,
        parent_height: R32,
    ) -> MeasuredDimensions {
        // TODO(anp): guarantee this statically i think
        let measure = self
            .measure_fn()
            .expect("expected node to have custom measure function");

        let padding_and_border_axis_row = self
            .style()
            .padding_and_border_for_axis(FlexDirection::Row, available_width);
        let padding_and_border_axis_column = self
            .style()
            .padding_and_border_for_axis(FlexDirection::Column, available_width);
        let margin_axis_row = self
            .style()
            .margin
            .for_axis(FlexDirection::Row, available_width);
        let margin_axis_column = self
            .style()
            .margin
            .for_axis(FlexDirection::Column, available_width);

        // We want to make sure we don't call measure with negative size
        // TODO(anp): presumably this is supposed to end up being NaN under some conditions?
        //   let inner_width = if YGFloatIsUndefined(availableWidth) {
        //                                availableWidth } else {
        //                                (availableWidth - marginAxisRow - paddingAndBorderAxisRow).max(0.0)};
        let inner_width = available_width;

        // TODO(anp): these types will panic if this were to be true
        // let inner_height = if YGFloatIsUndefined(availableHeight) {
        //     availableHeight
        // } else {
        //     fmaxf(
        //         0,
        //         availableHeight - marginAxisColumn - paddingAndBorderAxisColumn,
        //     )
        // };
        let inner_height = available_height;

        if width_measure_mode == Some(MeasureMode::Exactly)
            && height_measure_mode == Some(MeasureMode::Exactly)
        {
            // Don't bother sizing the text if both dimensions are already defined.
            let width = self.bound_axis(
                FlexDirection::Row,
                available_width - margin_axis_row,
                parent_width,
                parent_width,
            );

            let height = self.bound_axis(
                FlexDirection::Column,
                available_height - margin_axis_column,
                parent_height,
                parent_width,
            );

            MeasuredDimensions {
                // TODO(anp): the original source said parentWidth 2x here, not sure why
                width,
                height,
            }
        } else {
            // Measure the text under the current constraints.
            let measured_size = measure(
                self,
                inner_width,
                width_measure_mode,
                inner_height,
                height_measure_mode,
            );

            let mut bound = |dir,
                             measure_mode: Option<MeasureMode>,
                             measured_size,
                             axis_margin,
                             available1,
                             available2| {
                self.bound_axis(
                    dir,
                    if measure_mode.is_none() || measure_mode == Some(MeasureMode::AtMost) {
                        measured_size + padding_and_border_axis_row
                    } else {
                        available_width - axis_margin
                    },
                    available1,
                    available2,
                )
            };

            MeasuredDimensions {
                width: bound(
                    FlexDirection::Row,
                    width_measure_mode,
                    measured_size.width,
                    margin_axis_row,
                    available_width,
                    available_width,
                ),
                height: bound(
                    FlexDirection::Column,
                    height_measure_mode,
                    measured_size.height,
                    padding_and_border_axis_column,
                    available_height,
                    available_width,
                ),
            }
        }
    }

    /// Like bound_axis_within_min_and_max but also ensures that the value doesn't go below the
    /// padding and border amount.
    fn bound_axis(&self, axis: FlexDirection, value: R32, axisSize: R32, widthSize: R32) -> R32 {
        self.bound_axis_within_min_and_max(axis, value, axisSize)
            .max(self.style().padding_and_border_for_axis(axis, widthSize))
    }

    fn bound_axis_within_min_and_max(
        &self,
        axis: FlexDirection,
        value: R32,
        axis_size: R32,
    ) -> R32 {
        let (min, max) = match axis {
            FlexDirection::Column | FlexDirection::ColumnReverse => (
                self.style().min_dimensions.height.resolve(axis_size),
                self.style().max_dimensions.height.resolve(axis_size),
            ),
            FlexDirection::Row | FlexDirection::RowReverse => (
                self.style().min_dimensions.width.resolve(axis_size),
                self.style().max_dimensions.width.resolve(axis_size),
            ),
        };

        let value = if let Some(min) = min {
            value.max(min)
        } else {
            value
        };

        if let Some(max) = max {
            value.min(max)
        } else {
            value
        }
    }

    /// For nodes with no children, use the available values if they were provided,
    /// or the minimum size as indicated by the padding and border sizes.
    fn empty_container_set_measured_dimensions(
        &mut self,
        available_width: R32,
        available_height: R32,
        width_measure_mode: Option<MeasureMode>,
        height_measure_mode: Option<MeasureMode>,
        parent_width: R32,
        parent_height: R32,
    ) -> MeasuredDimensions {
        let padding_and_border_axis_row = self
            .style()
            .padding_and_border_for_axis(FlexDirection::Row, parent_width);
        let padding_and_border_axis_column = self
            .style()
            .padding_and_border_for_axis(FlexDirection::Column, parent_width);

        let margin_axis_row = self
            .style()
            .margin
            .for_axis(FlexDirection::Row, parent_width);
        let margin_axis_column = self
            .style()
            .margin
            .for_axis(FlexDirection::Column, parent_width);

        MeasuredDimensions {
            width: self.bound_axis(
                FlexDirection::Row,
                if width_measure_mode == None || width_measure_mode == Some(MeasureMode::AtMost) {
                    padding_and_border_axis_row
                } else {
                    available_width - margin_axis_row
                },
                parent_width,
                parent_width,
            ),
            height: self.bound_axis(
                FlexDirection::Column,
                if height_measure_mode == None || height_measure_mode == Some(MeasureMode::AtMost) {
                    padding_and_border_axis_column
                } else {
                    available_height - margin_axis_column
                },
                parent_height,
                parent_width,
            ),
        }
    }

    fn fixed_size_set_measured_dimensions(
        &mut self,
        available_width: R32,
        available_height: R32,
        width_measure_mode: Option<MeasureMode>,
        height_measure_mode: Option<MeasureMode>,
        parent_width: R32,
        parent_height: R32,
        // TODO(anp): maybe a different type than bool?
    ) -> Option<MeasuredDimensions> {
        if width_measure_mode == Some(MeasureMode::AtMost) && available_width <= 0.0
            || height_measure_mode == Some(MeasureMode::AtMost) && available_height <= 0.0
            || width_measure_mode == Some(MeasureMode::Exactly)
                && height_measure_mode == Some(MeasureMode::Exactly)
        {
            let margin_axis_column = self
                .style()
                .margin
                .for_axis(FlexDirection::Column, parent_width);

            let margin_axis_row = self
                .style()
                .margin
                .for_axis(FlexDirection::Row, parent_width);

            Some(MeasuredDimensions {
                width: self.bound_axis(
                    FlexDirection::Row,
                    if available_width.is_nan()
                        || width_measure_mode == Some(MeasureMode::AtMost) && available_width < 0.0
                    {
                        r32(0.0)
                    } else {
                        available_width - margin_axis_row
                    },
                    parent_width,
                    parent_width,
                ),
                height: self.bound_axis(
                    FlexDirection::Column,
                    if available_height.is_nan()
                        || height_measure_mode == Some(MeasureMode::AtMost)
                            && available_height < 0.0
                    {
                        r32(0.0)
                    } else {
                        available_height - margin_axis_column
                    },
                    parent_height,
                    parent_width,
                ),
            })
        } else {
            None
        }
    }

    fn resolve_flex_grow(&self) -> R32 {
        // Root nodes flexGrow should always be 0
        let flex_grow = self.style().flex_grow;
        let flex = self.style().flex;
        match (self.parent(), flex_grow, flex) {
            (None, _, _) => r32(0.0),
            // FIXME(anp): flex_grow should probably be an option?
            (_, grow, _) if grow != Style::DEFAULT_FLEX_GROW => grow,
            (_, _, Some(flex)) if flex > 0.0 => flex,
            _ => r32(Style::DEFAULT_FLEX_GROW),
        }
    }

    fn resolve_flex_shrink(&self) -> R32 {
        match (
            self.parent(),
            self.style().flex_shrink,
            self.style().flex,
            cfg!(feature = "web-default"),
        ) {
            // Root nodes flexShrink should always be 0
            (None, _, _, _) => r32(0.0),
            // FIXME(anp): flex_shrink should probably be an option?
            (_, shrink, _, _) if shrink != Style::DEFAULT_FLEX_SHRINK => shrink,
            (_, _, Some(flex), false) if flex < 0.0 => -flex,
            _ => r32(Style::DEFAULT_FLEX_SHRINK),
        }
    }

    fn zero_layout_recursively(&mut self) {
        // FIXME(anp): this should be assigning None to a nullable field!!!
        *self.layout_mut() = Layout::default();
        *self.new_layout() = true;
        for child in self.children_mut() {
            child.zero_layout_recursively();
        }
    }

    // This is the main routine that implements a subset of the flexbox layout
    // algorithm
    // described in the W3C YG documentation: https://www.w3.org/TR/YG3-flexbox/.
    //
    // Limitations of this algorithm, compared to the full standard:
    //  * Display property is always assumed to be 'flex' except for Text nodes,
    //  which
    //    are assumed to be 'inline-flex'.
    //  * The 'zIndex' property (or any form of z ordering) is not supported. Nodes
    //  are
    //    stacked in document order.
    //  * The 'order' property is not supported. The order of flex items is always
    //  defined
    //    by document order.
    //  * The 'visibility' property is always assumed to be 'visible'. Values of
    //  'collapse'
    //    and 'hidden' are not supported.
    //  * There is no support for forced breaks.
    //  * It does not support vertical inline directions (top-to-bottom or
    //  bottom-to-top text).
    //
    // Deviations from standard:
    //  * Section 4.5 of the spec indicates that all flex items have a default
    //  minimum
    //    main size. For text blocks, for example, this is the width of the widest
    //    word.
    //    Calculating the minimum width is expensive, so we forego it and assume a
    //    default
    //    minimum main size of 0.
    //  * Min/Max sizes in the main axis are not honored when resolving flexible
    //  lengths.
    //  * The spec indicates that the default value for 'flex_direction' is 'row',
    //  but
    //    the algorithm below assumes a default of 'column'.
    //
    // Input parameters:
    //    - node: current node to be sized and layed out
    //    - availableWidth & availableHeight: available size to be used for sizing
    //    the node
    //      or YGUndefined if the size is not available; interpretation depends on
    //      layout
    //      flags
    //    - parent_direction: the inline (text) direction within the parent
    //    (left-to-right or
    //      right-to-left)
    //    - width_measure_mode: indicates the sizing rules for the width (see below
    //    for explanation)
    //    - height_measure_mode: indicates the sizing rules for the height (see below
    //    for explanation)
    //    - performLayout: specifies whether the caller is interested in just the
    //    dimensions
    //      of the node or it requires the entire node and its subtree to be layed
    //      out
    //      (with final positions)
    //
    // Details:
    //    This routine is called recursively to lay out subtrees of flexbox
    //    elements. It uses the
    //    information in node.style, which is treated as a read-only input. It is
    //    responsible for
    //    setting the layout.direction and layout.measured_dimensions fields for the
    //    input node as well
    //    as the layout.position and layout.lineIndex fields for its child nodes.
    //    The
    //    layout.measured_dimensions field includes any border or padding for the
    //    node but does
    //    not include margins.
    //
    //    The spec describes four different layout modes: "fill available", "max
    //    content", "min
    //    content",
    //    and "fit content". Of these, we don't use "min content" because we don't
    //    support default
    //    minimum main sizes (see above for details). Each of our measure modes maps
    //    to a layout mode
    //    from the spec (https://www.w3.org/TR/YG3-sizing/#terms):
    //      - MeasureMode::Undefined: max content
    //      - MeasureMode::Exactly: fill available
    //      - MeasureMode::AtMost: fit content
    //
    //    When calling layoutImpl and YGLayoutNodeInternal, if the caller passes
    //    an available size of
    //    undefined then it must also pass a measure mode of MeasureMode::Undefined
    //    in that dimension.
    //
    fn layout_impl(
        &mut self,
        available_width: R32,
        available_height: R32,
        parent_direction: Direction,
        width_measure_mode: Option<MeasureMode>,
        height_measure_mode: Option<MeasureMode>,
        parent_width: R32,
        parent_height: R32,
        perform_layout: bool,
    ) {
        if available_width.is_nan() {
            assert!(
                width_measure_mode.is_none(),
                "availableWidth is indefinite so width_measure_mode must be None",
            );
        }

        if available_height.is_nan() {
            assert!(
                height_measure_mode.is_none(),
                "availableHeight is indefinite so height_measure_mode must be None",
            );
        }

        // Set the resolved resolution in the node's layout.
        let direction = self.layout().direction.resolve(parent_direction);
        let flex_row_direction = FlexDirection::Row.resolve_direction(direction);
        let flex_column_direction = FlexDirection::Column.resolve_direction(direction);
        self.layout_mut().direction = direction;

        self.layout_mut().margin =
            self.style()
                .margin
                .resolve(flex_row_direction, flex_column_direction, parent_width);

        self.layout_mut().border = self
            .style()
            .border
            .resolve(flex_row_direction, flex_column_direction);

        self.layout_mut().padding =
            self.style()
                .padding
                .resolve(flex_row_direction, flex_column_direction, parent_width);

        // TODO(anp): make this idempotent/typesafe/etc
        if self.measure_fn().is_some() {
            self.layout_mut().measured_dimensions =
                Some(self.with_measure_func_set_measured_dimensions(
                    available_width,
                    available_height,
                    width_measure_mode,
                    height_measure_mode,
                    parent_width,
                    parent_height,
                ));
            return;
        }

        if self.children().is_empty() {
            self.layout_mut().measured_dimensions =
                Some(self.empty_container_set_measured_dimensions(
                    available_width,
                    available_height,
                    width_measure_mode,
                    height_measure_mode,
                    parent_width,
                    parent_height,
                ));
            return;
        };

        // If we're not being asked to perform a full layout we can skip the algorithm if we already know
        // the size
        if let (false, Some(d)) = (
            perform_layout,
            self.fixed_size_set_measured_dimensions(
                available_width,
                available_height,
                width_measure_mode,
                height_measure_mode,
                parent_width,
                parent_height,
            ),
        ) {
            self.layout_mut().measured_dimensions = Some(d);
            return;
        }

        // Reset layout flags, as they could have changed.
        self.layout_mut().had_overflow = false;

        // STEP 1: CALCULATE VALUES FOR REMAINDER OF ALGORITHM
        let main_axis = self.style().flex_direction.resolve_direction(direction);
        let cross_axis = main_axis.cross(direction);
        let is_main_axis_row = main_axis.is_row();
        let justify_content = self.style().justify_content;
        let is_node_flex_wrap = self.style().flex_wrap != Wrap::NoWrap;

        let main_axis_parent_size = if is_main_axis_row {
            parent_width
        } else {
            parent_height
        };

        let cross_axis_parent_size = if is_main_axis_row {
            parent_height
        } else {
            parent_width
        };

        let leading_padding_and_border_main = self
            .style()
            .leading_padding_and_border(main_axis, parent_width);
        let trailing_padding_and_border_main = self
            .style()
            .trailing_padding_and_border(main_axis, parent_width);
        let leading_padding_and_border_cross = self
            .style()
            .leading_padding_and_border(cross_axis, parent_width);
        let padding_and_border_axis_main = self
            .style()
            .padding_and_border_for_axis(main_axis, parent_width);
        let padding_and_border_axis_cross = self
            .style()
            .padding_and_border_for_axis(cross_axis, parent_width);

        let mut measure_mode_main_dim = if is_main_axis_row {
            width_measure_mode
        } else {
            height_measure_mode
        };

        let measure_mode_cross_dim = if is_main_axis_row {
            height_measure_mode
        } else {
            width_measure_mode
        };

        let padding_and_border_axis_row = if is_main_axis_row {
            padding_and_border_axis_main
        } else {
            padding_and_border_axis_cross
        };
        let padding_and_border_axis_column = if is_main_axis_row {
            padding_and_border_axis_cross
        } else {
            padding_and_border_axis_main
        };

        let margin_axis_row = self
            .style()
            .margin
            .for_axis(FlexDirection::Row, parent_width);
        let margin_axis_column = self
            .style()
            .margin
            .for_axis(FlexDirection::Column, parent_width);

        // STEP 2: DETERMINE AVAILABLE SIZE IN MAIN AND CROSS DIRECTIONS
        let min_inner_width =
            self.style()
                .min_dimensions
                .width
                .resolve(parent_width)
                .unwrap_or(r32(0.0)) - margin_axis_row - padding_and_border_axis_row;
        let max_inner_width =
            self.style()
                .max_dimensions
                .width
                .resolve(parent_width)
                .unwrap_or(r32(0.0)) - margin_axis_row - padding_and_border_axis_row;
        let min_inner_height = self
            .style()
            .min_dimensions
            .height
            .resolve(parent_height)
            .unwrap_or(r32(0.0)) - margin_axis_column
            - padding_and_border_axis_column;
        let max_inner_height = self
            .style()
            .max_dimensions
            .height
            .resolve(parent_height)
            .unwrap_or(r32(0.0)) - margin_axis_column
            - padding_and_border_axis_column;

        let min_inner_main_dim = if is_main_axis_row {
            min_inner_width
        } else {
            min_inner_height
        };
        let max_inner_main_dim = if is_main_axis_row {
            max_inner_width
        } else {
            max_inner_height
        };

        // Max dimension overrides predefined dimension value;
        // Min dimension in turn overrides both of the above
        let mut available_inner_width =
            available_width - margin_axis_row - padding_and_border_axis_row;
        if !available_inner_width.is_nan() {
            // We want to make sure our available width does not violate min and max constraints
            available_inner_width = available_inner_width
                .min(max_inner_width)
                .max(min_inner_width);
        }

        let mut available_inner_height =
            available_height - margin_axis_column - padding_and_border_axis_column;
        if !available_inner_height.is_nan() {
            // We want to make sure our available height does not violate min and max constraints
            available_inner_height = available_inner_height
                .min(max_inner_height)
                .max(min_inner_height);
        }

        let mut available_inner_main_dim = if is_main_axis_row {
            available_inner_width
        } else {
            available_inner_height
        };
        let mut available_inner_cross_dim = if is_main_axis_row {
            available_inner_height
        } else {
            available_inner_width
        };

        // If there is only one child with flex_grow + flex_shrink it means we can set the
        // computed_flex_basis to 0 instead of measuring and shrinking / flexing the child to exactly
        // match the remaining space
        // FIXME(anp): figure out the borrowing bs here
        // let single_flex_child = if measure_mode_main_dim == Some(MeasureMode::Exactly) {
        //     self.children()
        //         .iter()
        //         .filter(|&child| {
        //             child.resolve_flex_grow() > 0.0 && child.resolve_flex_shrink() > 0.0
        //         })
        //         .next()
        // } else {
        //     None
        // };

        let mut total_outer_flex_basis = r32(0.0);

        // FIXME(anp): not sure whether this gets incremented later in the loop, check to see
        let current_generation = self.current_generation();
        let self_flex_direction = self.style().flex_direction;
        let self_overflow = self.style().overflow;

        let aligns = self
            .children()
            .into_iter()
            .map(|child| self.align_item(child))
            .collect::<Vec<_>>();

        // STEP 3: DETERMINE FLEX BASIS FOR EACH ITEM
        for (mut child, parent_align) in self.children_mut().into_iter().zip(aligns) {
            if child.style().display == Display::None {
                child.zero_layout_recursively();
                *child.new_layout() = true;
                child.mark_dirty();
                continue;
            }
            child.resolve_dimensions();
            if perform_layout {
                // Set the initial position (relative to the parent).
                let child_direction = child.style().direction.resolve(direction);
                child.set_position(
                    child_direction,
                    available_inner_main_dim,
                    available_inner_cross_dim,
                    available_inner_width,
                );
            }

            // Absolute-positioned children don't participate in flex layout. Add them
            // to a list that we can process later.
            if child.style().position_type == PositionType::Absolute {
                // Store a private linked list of absolutely positioned children
                // so that we can efficiently traverse them later.
                Y::add_absolute_child(&*child);
            // FIXME(anp): figure out the borrowing bs here (look for similar comment above)
            // } else if Some(&*child) == single_flex_child {
            //     child.layout_mut().computed_flex_basis_generation = current_generation;
            //     child.layout_mut().computed_flex_basis = Some(r32(0.0));
            } else {
                child.compute_flex_basis_from_parent(
                    available_inner_width,
                    width_measure_mode,
                    available_inner_height,
                    available_inner_width,
                    available_inner_height,
                    height_measure_mode,
                    direction,
                    self_flex_direction,
                    self_overflow,
                    parent_align,
                );
            }

            if let Some(basis) = child.layout().computed_flex_basis {
                total_outer_flex_basis += basis;
            }

            // FIXME(anp): pretty sure this should have been operating on layout, but i impld
            // it on the margin that style gets
            total_outer_flex_basis += child
                .style()
                .margin
                .for_axis(main_axis, available_inner_width);
        }

        let flex_basis_overflows = measure_mode_main_dim
            .map(|_| total_outer_flex_basis > available_inner_main_dim)
            .unwrap_or(false);
        if is_node_flex_wrap
            && flex_basis_overflows
            && measure_mode_main_dim == Some(MeasureMode::AtMost)
        {
            measure_mode_main_dim = Some(MeasureMode::Exactly);
        }

        // STEP 4: COLLECT FLEX ITEMS INTO FLEX LINES

        // Indexes of children that represent the first and last items in the line.
        let mut end_of_line_index = 0;

        // Number of lines.
        let line_count = 0;

        // Accumulated cross dimensions of all lines so far.
        let total_line_cross_dim = r32(0.0);

        // Max main dimension of all the lines.
        let max_line_main_dim = r32(0.0);

        while end_of_line_index < self.children().len() {
            // Number of items on the currently line. May be different than the
            // difference
            // between start and end indicates because we skip over absolute-positioned
            // items.
            let mut items_on_line = 0;

            // sizeConsumedOnCurrentLine is accumulation of the dimensions and margin
            // of all the children on the current line. This will be used in order to
            // either set the dimensions of the node if none already exist or to compute
            // the remaining space left for the flexible children.
            let mut size_consumed_on_current_line = r32(0.0);
            let mut size_consumed_on_current_line_including_min_constraint = r32(0.0);

            let mut total_flex_grow_factors = r32(0.0);
            let mut total_flex_shrink_scaled_factors = r32(0.0);

            // Add items to the current line until it's full or we run out of items.
            for child in self.children_mut() {
                if child.style().display == Display::None {
                    continue;
                }

                *child.line() = line_count;

                if child.style().position_type != PositionType::Absolute {
                    let child_margin_main_axis = child
                        .style()
                        .margin
                        .for_axis(main_axis, available_inner_width);

                    let flex_basis_with_max_constraints = child.style().max_dimensions
                        [main_axis.dimension()]
                        .resolve(main_axis_parent_size)
                        .min(child.layout().computed_flex_basis);

                    let flex_basis_with_min_and_max_constraints = child.style().min_dimensions
                        [main_axis.dimension()]
                        .resolve(main_axis_parent_size)
                        .max(flex_basis_with_max_constraints);

                    // If this is a multi-line flow and this item pushes us over the
                    // available size, we've
                    // hit the end of the current line. Break out of the loop and lay out
                    // the current line.
                    if size_consumed_on_current_line_including_min_constraint
                        + flex_basis_with_min_and_max_constraints.unwrap()
                        + child_margin_main_axis > available_inner_main_dim
                        && is_node_flex_wrap && items_on_line > 0
                    {
                        break;
                    }

                    size_consumed_on_current_line_including_min_constraint +=
                        flex_basis_with_min_and_max_constraints.unwrap() + child_margin_main_axis;
                    size_consumed_on_current_line +=
                        flex_basis_with_min_and_max_constraints.unwrap() + child_margin_main_axis;
                    items_on_line += 1;

                    if child.is_flex() {
                        total_flex_grow_factors += child.resolve_flex_grow();

                        // Unlike the grow factor, the shrink factor is scaled relative to the child dimension.
                        total_flex_shrink_scaled_factors += -child.resolve_flex_shrink()
                            * child.layout().computed_flex_basis.unwrap();
                    }

                    // FIXME(anp): see above in the commented-out decalarations of these vars
                    // // Store a private linked list of children that need to be layed out.
                    // if first_relative_child.is_none() {
                    //     first_relative_child = Some(child);
                    // }
                    // if curr_rel_child.is_none() {
                    //     (*curr_rel_child).next_child = Some(child);
                    // }
                    // curr_rel_child = Some(child);
                    // child.next_child = None;
                }
                // FIXME(anp): totally unsure if i broke this loop by  not having  the local linked
                // list
                end_of_line_index += 1;
            }

            // The total flex factor needs to be floored to 1.
            if total_flex_grow_factors > 0.0 && total_flex_grow_factors < 1.0 {
                total_flex_grow_factors = r32(1.0);
            }

            // The total flex shrink factor needs to be floored to 1.
            if total_flex_shrink_scaled_factors > 0.0 && total_flex_shrink_scaled_factors < 1.0 {
                total_flex_shrink_scaled_factors = r32(1.0);
            }

            // If we don't need to measure the cross axis, we can skip the entire flex
            // step.
            let can_skip_flex =
                !perform_layout && measure_mode_cross_dim == Some(MeasureMode::Exactly);

            // In order to position the elements in the main axis, we have two
            // controls. The space between the beginning and the first element
            // and the space between each two elements.
            let mut leading_main_dim = r32(0.0);
            let mut between_main_dim = r32(0.0);

            // STEP 5: RESOLVING FLEXIBLE LENGTHS ON MAIN AXIS
            // Calculate the remaining available space that needs to be allocated.
            // If the main dimension size isn't known, it is computed based on
            // the line length, so there's no more space left to distribute.

            // If we don't measure with exact main dimension we want to ensure we don't violate min and max
            if measure_mode_main_dim != Some(MeasureMode::Exactly) {
                if !min_inner_main_dim.is_nan()
                    && size_consumed_on_current_line < min_inner_main_dim
                {
                    available_inner_main_dim = min_inner_main_dim;
                } else if !max_inner_main_dim.is_nan()
                    && size_consumed_on_current_line > max_inner_main_dim
                {
                    available_inner_main_dim = max_inner_main_dim;
                }
            }

            let mut remaining_free_space = r32(0.0);
            if !available_inner_main_dim.is_nan() {
                remaining_free_space = available_inner_main_dim - size_consumed_on_current_line;
            } else if size_consumed_on_current_line < 0.0 {
                // availableInnerMainDim is indefinite which means the node is being sized based on its
                // content.
                // sizeConsumedOnCurrentLine is negative which means the node will allocate 0 points for
                // its content. Consequently, remainingFreeSpace is 0 - sizeConsumedOnCurrentLine.
                remaining_free_space = size_consumed_on_current_line;
            }

            let mut original_remaining_free_space = remaining_free_space;
            let mut delta_free_space = r32(0.0);

            // Maintain a linked list of the child node indices that can shrink and/or grow.
            let first_relative_child_idx: Option<usize> = Some(0);
            let mut curr_rel_child_idx: Option<usize> = Some(0);

            if !can_skip_flex {
                let mut child_flex_basis;
                let mut flex_shrink_scaled_factor;
                let mut flex_grow_factor;
                let mut base_main_size;
                let mut bound_main_size;

                // Do two passes over the flex items to figure out how to distribute the
                // remaining space.
                // The first pass finds the items whose min/max constraints trigger,
                // freezes them at those
                // sizes, and excludes those sizes from the remaining space. The second
                // pass sets the size
                // of each flexible item. It distributes the remaining space amongst the
                // items whose min/max
                // constraints didn't trigger in pass 1. For the other items, it sets
                // their sizes by forcing
                // their min/max constraints to trigger again.
                //
                // This two pass approach for resolving min/max constraints deviates from
                // the spec. The
                // spec (https://www.w3.org/TR/YG-flexbox-1/#resolve-flexible-lengths)
                // describes a process
                // that needs to be repeated a variable number of times. The algorithm
                // implemented here
                // won't handle all cases but it was simpler to implement and it mitigates
                // performance
                // concerns because we know exactly how many passes it'll do.

                // First pass: detect the flex items whose min/max constraints trigger
                let mut delta_flex_shrink_scaled_factors = r32(0.0);
                let mut delta_flex_grow_factors = r32(0.0);

                while let Some(idx) = curr_rel_child_idx {
                    let curr_rel_child = self.child(idx);
                    let child_flex_basis = curr_rel_child.style().max_dimensions
                        [main_axis.dimension()]
                        .resolve(main_axis_parent_size)
                        .min(
                            curr_rel_child.style().min_dimensions[main_axis.dimension()]
                                .resolve(main_axis_parent_size)
                                .max(curr_rel_child.layout().computed_flex_basis),
                        );

                    if remaining_free_space < 0.0 {
                        flex_shrink_scaled_factor =
                            -curr_rel_child.resolve_flex_shrink() * child_flex_basis.unwrap();

                        // Is this child able to shrink?
                        if flex_shrink_scaled_factor != 0.0 {
                            base_main_size = child_flex_basis.unwrap()
                                + remaining_free_space / total_flex_shrink_scaled_factors
                                    * flex_shrink_scaled_factor;
                            bound_main_size = curr_rel_child.bound_axis(
                                main_axis,
                                base_main_size,
                                available_inner_main_dim,
                                available_inner_width,
                            );
                            if base_main_size != bound_main_size {
                                // By excluding this item's size and flex factor from remaining,
                                // this item's
                                // min/max constraints should also trigger in the second pass
                                // resulting in the
                                // item's size calculation being identical in the first and second
                                // passes.
                                delta_free_space -= bound_main_size - child_flex_basis.unwrap();
                                delta_flex_shrink_scaled_factors -= flex_shrink_scaled_factor;
                            }
                        }
                    } else if remaining_free_space > 0.0 {
                        flex_grow_factor = curr_rel_child.resolve_flex_grow();

                        // Is this child able to grow?
                        if flex_grow_factor != 0.0 {
                            base_main_size = child_flex_basis.unwrap()
                                + remaining_free_space / total_flex_grow_factors * flex_grow_factor;
                            bound_main_size = curr_rel_child.bound_axis(
                                main_axis,
                                base_main_size,
                                available_inner_main_dim,
                                available_inner_width,
                            );

                            if base_main_size != bound_main_size {
                                // By excluding this item's size and flex factor from remaining,
                                // this item's
                                // min/max constraints should also trigger in the second pass
                                // resulting in the
                                // item's size calculation being identical in the first and second
                                // passes.
                                delta_free_space -= bound_main_size - child_flex_basis.unwrap();
                                delta_flex_grow_factors -= flex_grow_factor;
                            }
                        }
                    }

                    curr_rel_child_idx = Some(idx + 1);
                }

                total_flex_shrink_scaled_factors += delta_flex_shrink_scaled_factors;
                total_flex_grow_factors += delta_flex_grow_factors;
                remaining_free_space += delta_free_space;

                // Second pass: resolve the sizes of the flexible items
                delta_free_space = r32(0.0);

                curr_rel_child_idx = first_relative_child_idx;

                while let Some(curr_rel_child_idx_inner) = curr_rel_child_idx {
                    macro_rules! curr_rel_child {
                        () => {
                            self.child(curr_rel_child_idx_inner)
                        };
                        (mut) => {
                            self.child_mut(curr_rel_child_idx_inner)
                        };
                    }
                    child_flex_basis = curr_rel_child!().style().max_dimensions
                        [main_axis.dimension()]
                        .resolve(main_axis_parent_size)
                        .min(
                            curr_rel_child!().style().min_dimensions[main_axis.dimension()]
                                .resolve(main_axis_parent_size)
                                .max(curr_rel_child!().layout().computed_flex_basis),
                        );
                    let mut updated_main_size = child_flex_basis.unwrap();

                    if remaining_free_space < 0.0 {
                        flex_shrink_scaled_factor =
                            -curr_rel_child!().resolve_flex_shrink() * child_flex_basis.unwrap();
                        // Is this child able to shrink?
                        if flex_shrink_scaled_factor != 0.0 {
                            let mut child_size;

                            if total_flex_shrink_scaled_factors == 0.0 {
                                child_size = child_flex_basis.unwrap() + flex_shrink_scaled_factor;
                            } else {
                                child_size = child_flex_basis.unwrap()
                                    + (remaining_free_space / total_flex_shrink_scaled_factors)
                                        * flex_shrink_scaled_factor;
                            }

                            updated_main_size = curr_rel_child!().bound_axis(
                                main_axis,
                                child_size,
                                available_inner_main_dim,
                                available_inner_width,
                            );
                        }
                    } else if remaining_free_space > 0.0 {
                        flex_grow_factor = curr_rel_child!().resolve_flex_grow();

                        // Is this child able to grow?
                        if flex_grow_factor != 0.0 {
                            updated_main_size = curr_rel_child!().bound_axis(
                                main_axis,
                                child_flex_basis.unwrap()
                                    + remaining_free_space / total_flex_grow_factors
                                        * flex_grow_factor,
                                available_inner_main_dim,
                                available_inner_width,
                            );
                        }
                    }

                    delta_free_space -= updated_main_size - child_flex_basis.unwrap();

                    let margin_main = curr_rel_child!()
                        .style()
                        .margin
                        .for_axis(main_axis, available_inner_width);
                    let margin_cross = curr_rel_child!()
                        .style()
                        .margin
                        .for_axis(cross_axis, available_inner_width);

                    let mut child_cross_size;
                    let mut child_main_size = updated_main_size;
                    let mut child_cross_measure_mode;
                    let mut child_main_measure_mode = Some(MeasureMode::Exactly);

                    // TODO(anp) check for bug on the C side -- this was an != NULL check
                    if let Some(ar) = curr_rel_child!().style().aspect_ratio {
                        child_cross_size = if is_main_axis_row {
                            (child_main_size - margin_main) / ar
                        } else {
                            (child_main_size - margin_main) * ar
                        };
                        child_cross_measure_mode = Some(MeasureMode::Exactly);

                        child_cross_size += margin_cross;
                    } else if !available_inner_cross_dim.is_nan()
                        && !curr_rel_child!()
                            .is_style_dim_defined(cross_axis, available_inner_cross_dim)
                        && measure_mode_cross_dim == Some(MeasureMode::Exactly)
                        && !(is_node_flex_wrap && flex_basis_overflows)
                        && self.align_item(curr_rel_child!()) == Align::Stretch
                    {
                        child_cross_size = available_inner_cross_dim;
                        child_cross_measure_mode = Some(MeasureMode::Exactly);
                    } else if !curr_rel_child!()
                        .is_style_dim_defined(cross_axis, available_inner_cross_dim)
                    {
                        child_cross_size = available_inner_cross_dim;
                        child_cross_measure_mode = if child_cross_size.is_nan() {
                            None
                        } else {
                            Some(MeasureMode::AtMost)
                        };
                    } else {
                        child_cross_size = curr_rel_child!().resolved()[cross_axis.dimension()]
                            .map(|s| {
                                s.resolve(available_inner_cross_dim)
                                    .map(|s| s + margin_cross)
                            })
                            .unwrap()
                            .unwrap();

                        let is_loose_percentage_measurement =
                            match curr_rel_child!().resolved()[cross_axis.dimension()] {
                                Some(Value::Percent(_))
                                    if measure_mode_cross_dim != Some(MeasureMode::Exactly) =>
                                {
                                    true
                                }
                                _ => false,
                            };

                        child_cross_measure_mode =
                            if child_cross_size.is_nan() || is_loose_percentage_measurement {
                                None
                            } else {
                                Some(MeasureMode::Exactly)
                            };
                    }

                    let (child_main_size, child_main_measure_mode) = curr_rel_child!()
                        .constrained_max_size_for_mode(
                            main_axis,
                            available_inner_main_dim,
                            available_inner_width,
                            child_main_measure_mode,
                            child_main_size,
                        );

                    let (child_cross_size, child_cross_measure_mode) = curr_rel_child!()
                        .constrained_max_size_for_mode(
                            cross_axis,
                            available_inner_cross_dim,
                            available_inner_width,
                            child_cross_measure_mode,
                            child_cross_size,
                        );

                    let requires_stretch_layout = !curr_rel_child!()
                        .is_style_dim_defined(cross_axis, available_inner_cross_dim)
                        && self.align_item(curr_rel_child!()) == Align::Stretch;

                    let child_width = if is_main_axis_row {
                        child_main_size
                    } else {
                        child_cross_size
                    };
                    let child_height = if !is_main_axis_row {
                        child_main_size
                    } else {
                        child_cross_size
                    };

                    let child_width_measure_mode = if is_main_axis_row {
                        child_main_measure_mode
                    } else {
                        child_cross_measure_mode
                    };
                    let child_height_measure_mode = if !is_main_axis_row {
                        child_main_measure_mode
                    } else {
                        child_cross_measure_mode
                    };

                    // Recursively call the layout algorithm for this child with the updated
                    // main size.
                    // TODO(anp): pass a continuation here!
                    curr_rel_child!(mut).layout_node_internal(
                        child_width,
                        child_height,
                        direction,
                        child_width_measure_mode,
                        child_height_measure_mode,
                        available_inner_width,
                        available_inner_height,
                        r32(Self::POINT_SCALE_FACTOR),
                        perform_layout && !requires_stretch_layout,
                        "flex",
                    );

                    let current_had_overflow = curr_rel_child!().layout().had_overflow;
                    self.layout_mut().had_overflow |= current_had_overflow;
                    // TODO(anp): this is almost certainly a broken replacement for the linked list
                    let new_idx = curr_rel_child_idx_inner + 1;
                    curr_rel_child_idx = if new_idx < self.children().len() {
                        Some(new_idx)
                    } else {
                        None
                    };
                }

                remaining_free_space = original_remaining_free_space + delta_free_space;
                self.layout_mut().had_overflow |= remaining_free_space < 0.0;
            } //FIXME(anp): don't think this should be here

            //         // STEP 6: MAIN-AXIS JUSTIFICATION & CROSS-AXIS SIZE DETERMINATION

            //         // At this point, all the children have their dimensions set in the main
            //         // axis.
            //         // Their dimensions are also set in the cross axis with the exception of
            //         // items
            //         // that are aligned "stretch". We need to compute these stretch values and
            //         // set the final positions.

            //         // If we are using "at most" rules in the main axis. Calculate the remaining space when
            //         // constraint by the min size defined for the main axis.

            //         if measureModeMainDim == MeasureMode::AtMost && remainingFreeSpace > 0.0 {
            //             if self.style().min_dimensions[DIM[mainAxis as usize]] != None
            //                 && YGResolveValue(
            //                     &self.style().min_dimensions[DIM[mainAxis as usize]],
            //                     mainAxisParentSize,
            //                 ) >= 0.0
            //             {
            //                 remainingFreeSpace = (0.0f32).max(
            //                     YGResolveValue(
            //                         &self.style().min_dimensions[DIM[mainAxis as usize]],
            //                         mainAxisParentSize,
            //                     )
            //                         - (availableInnerMainDim - remainingFreeSpace),
            //                 );
            //             } else {
            //                 remainingFreeSpace = 0.0;
            //             }
            //         }

            //         let mut numberOfAutoMarginsOnCurrentLine = 0;
            //         for i in startOfLineIndex..endOfLineIndex {
            //             let child = ListGet(self.children, i);
            //             if child.style().position_type == PositionType::Relative {
            //                 if (*YGMarginLeadingValue(child, mainAxis)) == Value::Auto {
            //                     numberOfAutoMarginsOnCurrentLine += 1;
            //                 }
            //                 if (*YGMarginTrailingValue(child, mainAxis)) == Value::Auto {
            //                     numberOfAutoMarginsOnCurrentLine += 1;
            //                 }
            //             }
            //         }

            //         if numberOfAutoMarginsOnCurrentLine == 0 {
            //             match justify_content {
            //                 Justify::Center => leadingMainDim = remainingFreeSpace / 2.0,
            //                 Justify::FlexEnd => leadingMainDim = remainingFreeSpace,
            //                 Justify::SpaceBetween => {
            //                     if itemsOnLine > 1 {
            //                         betweenMainDim =
            //                             remainingFreeSpace.max(0.0) / (itemsOnLine - 1) as f32;
            //                     } else {
            //                         betweenMainDim = 0.0;
            //                     }
            //                 }
            //                 Justify::SpaceAround => {
            //                     // Space on the edges is half of the space between elements
            //                     betweenMainDim = remainingFreeSpace / itemsOnLine as f32;
            //                     leadingMainDim = betweenMainDim / 2.0;
            //                 }
            //                 _ => (),
            //             }
            //         }

            //         let mut mainDim = leadingPaddingAndBorderMain + leadingMainDim;
            //         let mut crossDim = 0.0;

            //         for i in startOfLineIndex..endOfLineIndex {
            //             let child = ListGet(self.children, i);
            //             if child.style().display == Display::None {
            //                 continue;
            //             }
            //             if child.style().position_type == PositionType::Absolute
            //                 && IsLeadingPosDefined(child, mainAxis)
            //             {
            //                 if performLayout {
            //                     // In case the child is position absolute and has left/top being
            //                     // defined, we override the position to whatever the user said
            //                     // (and margin/border).
            //                     child.layout.position[pos[mainAxis as usize] as usize] =
            //                         LeadingPosition(child, mainAxis, availableInnerMainDim)
            //                             + LeadingBorder(node, mainAxis)
            //                             + LeadingMargin(child, mainAxis, availableInnerWidth);
            //                 }
            //             } else {
            //                 // Now that we placed the element, we need to update the variables.
            //                 // We need to do that only for relative elements. Absolute elements
            //                 // do not take part in that phase.
            //                 if child.style().position_type == PositionType::Relative {
            //                     if (*YGMarginLeadingValue(child, mainAxis)) == Value::Auto {
            //                         mainDim +=
            //                             remainingFreeSpace / numberOfAutoMarginsOnCurrentLine as f32;
            //                     }

            //                     if performLayout {
            //                         child.layout.position[pos[mainAxis as usize] as usize] +=
            //                             mainDim;
            //                     }

            //                     if (*YGMarginTrailingValue(child, mainAxis)) == Value::Auto {
            //                         mainDim +=
            //                             remainingFreeSpace / numberOfAutoMarginsOnCurrentLine as f32;
            //                     }

            //                     if canSkipFlex {
            //                         // If we skipped the flex step, then we can't rely on the
            //                         // measuredDims because
            //                         // they weren't computed. This means we can't call DimWithMargin.
            //                         mainDim += betweenMainDim
            //                             + MarginForAxis(child, mainAxis, availableInnerWidth)
            //                             + child.layout.computed_flex_basis;
            //                         crossDim = availableInnerCrossDim;
            //                     } else {
            //                         // The main dimension is the sum of all the elements dimension plus the spacing.
            //                         mainDim += betweenMainDim
            //                             + DimWithMargin(child, mainAxis, availableInnerWidth);

            //                         // The cross dimension is the max of the elements dimension since
            //                         // there can only be one element in that cross dimension.
            //                         crossDim = crossDim.max(DimWithMargin(
            //                             child,
            //                             crossAxis,
            //                             availableInnerWidth,
            //                         ));
            //                     }
            //                 } else if performLayout {
            //                     child.layout.position[pos[mainAxis as usize] as usize] +=
            //                         LeadingBorder(node, mainAxis) + leadingMainDim;
            //                 }
            //             }
            //         }

            //         mainDim += trailingPaddingAndBorderMain;

            //         let mut containerCrossAxis = availableInnerCrossDim;
            //         if measureModeCrossDim == MeasureMode::Undefined
            //             || measureModeCrossDim == MeasureMode::AtMost
            //         {
            //             // Compute the cross axis from the max cross dimension of the children.
            //             containerCrossAxis = bound_axis(
            //                 node,
            //                 crossAxis,
            //                 crossDim + paddingAndBorderAxisCross,
            //                 crossAxisParentSize,
            //                 parentWidth,
            //             ) - paddingAndBorderAxisCross;
            //         }

            //         // If there's no flex wrap, the cross dimension is defined by the container.
            //         if !isNodeFlexWrap && measureModeCrossDim == MeasureMode::Exactly {
            //             crossDim = availableInnerCrossDim;
            //         }

            //         // Clamp to the min/max size specified on the container.
            //         crossDim = bound_axis(
            //             node,
            //             crossAxis,
            //             crossDim + paddingAndBorderAxisCross,
            //             crossAxisParentSize,
            //             parentWidth,
            //         ) - paddingAndBorderAxisCross;

            //         // STEP 7: CROSS-AXIS ALIGNMENT
            //         // We can skip child alignment if we're just measuring the container.
            //         if performLayout {
            //             for i in startOfLineIndex..endOfLineIndex {
            //                 let child = ListGet(self.children, i);
            //                 if child.style().display == Display::None {
            //                     continue;
            //                 }
            //                 if child.style().position_type == PositionType::Absolute {
            //                     // If the child is absolutely positioned and has a
            //                     // top/left/bottom/right
            //                     // set, override all the previously computed positions to set it
            //                     // correctly.
            //                     let isChildLeadingPosDefined =
            //                         IsLeadingPosDefined(child, crossAxis);
            //                     if isChildLeadingPosDefined {
            //                         child.layout.position[pos[crossAxis as usize] as usize] =
            //                             LeadingPosition(child, crossAxis, availableInnerCrossDim)
            //                                 + LeadingBorder(node, crossAxis)
            //                                 + LeadingMargin(
            //                                     child,
            //                                     crossAxis,
            //                                     availableInnerWidth,
            //                                 );
            //                     }
            //                     // If leading position is not defined or calculations result in Nan, default to border + margin
            //                     if !isChildLeadingPosDefined
            //                         || child.layout.position[pos[crossAxis as usize] as usize]
            //                             .is_nan()
            //                     {
            //                         child.layout.position[pos[crossAxis as usize] as usize] =
            //                             LeadingBorder(node, crossAxis)
            //                                 + LeadingMargin(
            //                                     child,
            //                                     crossAxis,
            //                                     availableInnerWidth,
            //                                 );
            //                     }
            //                 } else {
            //                     let mut leadingCrossDim = leadingPaddingAndBorderCross;

            //                     // For a relative children, we're either using align_items (parent) or
            //                     // align_self (child) in order to determine the position in the cross
            //                     // axis
            //                     let alignItem = align_item(node, child);

            //                     // If the child uses align stretch, we need to lay it out one more
            //                     // time, this time
            //                     // forcing the cross-axis size to be the computed cross size for the
            //                     // current line.
            //                     if alignItem == Align::Stretch
            //                         && (*YGMarginLeadingValue(child, crossAxis)) != Value::Auto
            //                         && (*YGMarginTrailingValue(child, crossAxis)) != Value::Auto
            //                     {
            //                         // If the child defines a definite size for its cross axis, there's
            //                         // no need to stretch.
            //                         if !IsStyleDimDefined(
            //                             child,
            //                             crossAxis,
            //                             availableInnerCrossDim,
            //                         ) {
            //                             let mut childMainSize =
            //                                 child.layout.measured_dimensions[DIM[mainAxis as usize]];
            //                             let mut childCrossSize = if !child
            //                                 .style
            //                                 .aspect_ratio
            //                                 .is_nan()
            //                             {
            //                                 (MarginForAxis(child, crossAxis, availableInnerWidth)
            //                                     + (if isMainAxisRow {
            //                                         childMainSize / child.style().aspect_ratio
            //                                     } else {
            //                                         childMainSize * child.style().aspect_ratio
            //                                     }))
            //                             } else {
            //                                 crossDim
            //                             };

            //                             childMainSize +=
            //                                 MarginForAxis(child, mainAxis, availableInnerWidth);

            //                             let mut childMainMeasureMode = MeasureMode::Exactly;
            //                             let mut childCrossMeasureMode = MeasureMode::Exactly;
            //                             YGConstrainMaxSizeForMode(
            //                                 child,
            //                                 mainAxis,
            //                                 availableInnerMainDim,
            //                                 availableInnerWidth,
            //                                 &mut childMainMeasureMode,
            //                                 &mut childMainSize,
            //                             );
            //                             YGConstrainMaxSizeForMode(
            //                                 child,
            //                                 crossAxis,
            //                                 availableInnerCrossDim,
            //                                 availableInnerWidth,
            //                                 &mut childCrossMeasureMode,
            //                                 &mut childCrossSize,
            //                             );

            //                             let childWidth = if isMainAxisRow {
            //                                 childMainSize
            //                             } else {
            //                                 childCrossSize
            //                             };
            //                             let childHeight = if !isMainAxisRow {
            //                                 childMainSize
            //                             } else {
            //                                 childCrossSize
            //                             };

            //                             let childWidthMeasureMode = if childWidth.is_nan() {
            //                                 MeasureMode::Undefined
            //                             } else {
            //                                 MeasureMode::Exactly
            //                             };
            //                             let childHeightMeasureMode = if childHeight.is_nan() {
            //                                 MeasureMode::Undefined
            //                             } else {
            //                                 MeasureMode::Exactly
            //                             };

            //                             YGLayoutNodeInternal(
            //                                 child,
            //                                 childWidth,
            //                                 childHeight,
            //                                 direction,
            //                                 childWidthMeasureMode,
            //                                 childHeightMeasureMode,
            //                                 availableInnerWidth,
            //                                 availableInnerHeight,
            //                                 true,
            //                                 "stretch",
            //                                 config,
            //                             );
            //                         }
            //                     } else {
            //                         let remainingCrossDim = containerCrossAxis
            //                             - DimWithMargin(child, crossAxis, availableInnerWidth);

            //                         if (*YGMarginLeadingValue(child, crossAxis)) == Value::Auto
            //                             && (*YGMarginTrailingValue(child, crossAxis)) == Value::Auto
            //                         {
            //                             leadingCrossDim += (remainingCrossDim / 2.0).max(0.0);
            //                         } else if (*YGMarginTrailingValue(child, crossAxis)) == Value::Auto
            //                         {
            //                             // No-Op
            //                         } else if (*YGMarginLeadingValue(child, crossAxis)) == Value::Auto {
            //                             leadingCrossDim += 0.0f32.max(remainingCrossDim);
            //                         } else if alignItem == Align::FlexStart {
            //                             // No-Op
            //                         } else if alignItem == Align::Center {
            //                             leadingCrossDim += remainingCrossDim / 2.0;
            //                         } else {
            //                             leadingCrossDim += remainingCrossDim;
            //                         }
            //                     }
            //                     // And we apply the position
            //                     child.layout.position[pos[crossAxis as usize] as usize] +=
            //                         totalLineCrossDim + leadingCrossDim;
            //                 }
            //             }
        }

        // total_line_cross_dim += cross_dim;
        // max_line_main_dim = max_line_main_dim.max(main_dim);
        // line_count += 1;
        // start_of_line_index = end_of_line_index;

        //     // STEP 8: MULTI-LINE CONTENT ALIGNMENT
        //     if performLayout
        //         && (lineCount > 1 || YGIsBaselineLayout(node))
        //         && !availableInnerCrossDim.is_nan()
        //     {
        //         let remainingAlignContentDim = availableInnerCrossDim - totalLineCrossDim;

        //         let mut crossDimLead = 0.0;
        //         let mut currentLead = leadingPaddingAndBorderCross;

        //         match self.style().align_content {
        //             Align::FlexEnd => currentLead += remainingAlignContentDim,
        //             Align::Center => currentLead += remainingAlignContentDim / 2.0,
        //             Align::Stretch => if availableInnerCrossDim > totalLineCrossDim {
        //                 crossDimLead = remainingAlignContentDim / lineCount as f32;
        //             },
        //             Align::SpaceAround => if availableInnerCrossDim > totalLineCrossDim {
        //                 currentLead += remainingAlignContentDim / (2.0 * lineCount as f32);
        //                 if lineCount > 1 {
        //                     crossDimLead = remainingAlignContentDim / lineCount as f32;
        //                 }
        //             } else {
        //                 currentLead += remainingAlignContentDim / 2.0;
        //             },
        //             Align::SpaceBetween => {
        //                 if availableInnerCrossDim > totalLineCrossDim && lineCount > 1 {
        //                     crossDimLead = remainingAlignContentDim / (lineCount as f32 - 1.0);
        //                 }
        //             }
        //             _ => (),
        //         }

        //         let mut endIndex = 0;
        //         for i in 0..lineCount {
        //             let startIndex = endIndex;

        //             // compute the line's height and find the endIndex
        //             let mut lineHeight = 0.0f32;
        //             let mut maxAscentForCurrentLine = 0.0f32;
        //             let mut maxDescentForCurrentLine = 0.0f32;

        //             for ii in startIndex..childCount {
        //                 endIndex = ii;
        //                 let child = ListGet(self.children, ii);
        //                 if child.style().display == Display::None {
        //                     continue;
        //                 }
        //                 if child.style().position_type == PositionType::Relative {
        //                     if child.lineIndex != i {
        //                         break;
        //                     }
        //                     if IsLayoutDimDefined(child, crossAxis) {
        //                         lineHeight = lineHeight.max(
        //                             child.layout.measured_dimensions[DIM[crossAxis as usize]]
        //                                 + MarginForAxis(
        //                                     child,
        //                                     crossAxis,
        //                                     availableInnerWidth,
        //                                 ),
        //                         );
        //                     }
        //                     if align_item(node, child) == Align::Baseline {
        //                         let ascent = baseline(child)
        //                             + LeadingMargin(
        //                                 child,
        //                                 FlexDirection::Column,
        //                                 availableInnerWidth,
        //                             );
        //                         let descent = child.layout.measured_dimensions.height
        //                             + MarginForAxis(
        //                                 child,
        //                                 FlexDirection::Column,
        //                                 availableInnerWidth,
        //                             ) - ascent;
        //                         maxAscentForCurrentLine = maxAscentForCurrentLine.max(ascent);
        //                         maxDescentForCurrentLine = maxDescentForCurrentLine.max(descent);
        //                         lineHeight = lineHeight
        //                             .max(maxAscentForCurrentLine + maxDescentForCurrentLine);
        //                     }
        //                 }
        //             }
        //             lineHeight += crossDimLead;

        //             if performLayout {
        //                 for ii in startIndex..endIndex {
        //                     let child = ListGet(self.children, ii);
        //                     if child.style().display == Display::None {
        //                         continue;
        //                     }
        //                     if child.style().position_type == PositionType::Relative {
        //                         match align_item(node, child) {
        //                             Align::FlexStart => {
        //                                 child.layout.position
        //                                     [pos[crossAxis as usize] as usize] = currentLead
        //                                     + LeadingMargin(
        //                                         child,
        //                                         crossAxis,
        //                                         availableInnerWidth,
        //                                     );
        //                             }
        //                             Align::FlexEnd => {
        //                                 child.layout.position
        //                                     [pos[crossAxis as usize] as usize] = currentLead
        //                                     + lineHeight
        //                                     - TrailingMargin(
        //                                         child,
        //                                         crossAxis,
        //                                         availableInnerWidth,
        //                                     )
        //                                     - child.layout.measured_dimensions
        //                                         [DIM[crossAxis as usize]];
        //                             }
        //                             Align::Center => {
        //                                 let mut childHeight = child.layout.measured_dimensions
        //                                     [DIM[crossAxis as usize]];
        //                                 child.layout.position
        //                                     [pos[crossAxis as usize] as usize] =
        //                                     currentLead + (lineHeight - childHeight) / 2.0;
        //                             }
        //                             Align::Stretch => {
        //                                 child.layout.position
        //                                     [pos[crossAxis as usize] as usize] = currentLead
        //                                     + LeadingMargin(
        //                                         child,
        //                                         crossAxis,
        //                                         availableInnerWidth,
        //                                     );

        //                                 // Remeasure child with the line height as it as been only measured with the
        //                                 // parents height yet.
        //                                 if !IsStyleDimDefined(
        //                                     child,
        //                                     crossAxis,
        //                                     availableInnerCrossDim,
        //                                 ) {
        //                                     let childWidth = if isMainAxisRow {
        //                                         (child.layout.measured_dimensions.width
        //                                             + MarginForAxis(
        //                                                 child,
        //                                                 mainAxis,
        //                                                 availableInnerWidth,
        //                                             ))
        //                                     } else {
        //                                         lineHeight
        //                                     };

        //                                     let childHeight = if !isMainAxisRow {
        //                                         (child.layout.measured_dimensions.height
        //                                             + MarginForAxis(
        //                                                 child,
        //                                                 crossAxis,
        //                                                 availableInnerWidth,
        //                                             ))
        //                                     } else {
        //                                         lineHeight
        //                                     };

        //                                     if !(YGFloatsEqual(
        //                                         childWidth,
        //                                         child.layout.measured_dimensions.width,
        //                                     )
        //                                         && YGFloatsEqual(
        //                                             childHeight,
        //                                             child.layout.measured_dimensions.height,
        //                                         )) {
        //                                         YGLayoutNodeInternal(
        //                                             child,
        //                                             childWidth,
        //                                             childHeight,
        //                                             direction,
        //                                             MeasureMode::Exactly,
        //                                             MeasureMode::Exactly,
        //                                             availableInnerWidth,
        //                                             availableInnerHeight,
        //                                             true,
        //                                             "multiline-stretch",
        //                                             config,
        //                                         );
        //                                     }
        //                                 }
        //                             }
        //                             Align::Baseline => {
        //                                 child.layout.position[Edge::Top as usize] = currentLead
        //                                     + maxAscentForCurrentLine
        //                                     - baseline(child)
        //                                     + LeadingPosition(
        //                                         child,
        //                                         FlexDirection::Column,
        //                                         availableInnerCrossDim,
        //                                     );
        //                             }
        //                             _ => (),
        //                         }
        //                     }
        //                 }
        //             }

        //             currentLead += lineHeight;
        //         }
        //     }

        //     // STEP 9: COMPUTING FINAL DIMENSIONS
        //     self.layout().measured_dimensions.width = bound_axis(
        //         node,
        //         FlexDirection::Row,
        //         availableWidth - marginAxisRow,
        //         parentWidth,
        //         parentWidth,
        //     );
        //     self.layout().measured_dimensions.height = bound_axis(
        //         node,
        //         FlexDirection::Column,
        //         availableHeight - marginAxisColumn,
        //         parentHeight,
        //         parentWidth,
        //     );

        //     // If the user didn't specify a width or height for the node, set the
        //     // dimensions based on the children.
        //     if measureModeMainDim == MeasureMode::Undefined
        //         || (self.style().overflow != Overflow::Scroll
        //             && measureModeMainDim == MeasureMode::AtMost)
        //     {
        //         // Clamp the size to the min/max size, if specified, and make sure it
        //         // doesn't go below the padding and border amount.
        //         self.layout().measured_dimensions[DIM[mainAxis as usize]] = bound_axis(
        //             node,
        //             mainAxis,
        //             maxLineMainDim,
        //             mainAxisParentSize,
        //             parentWidth,
        //         );
        //     } else if measureModeMainDim == MeasureMode::AtMost
        //         && self.style().overflow == Overflow::Scroll
        //     {
        //         self.layout().measured_dimensions[DIM[mainAxis as usize]] = (availableInnerMainDim
        //             + paddingAndBorderAxisMain)
        //             .min(bound_axis_within_min_and_max(
        //                 node,
        //                 mainAxis,
        //                 maxLineMainDim,
        //                 mainAxisParentSize,
        //             ))
        //             .max(paddingAndBorderAxisMain);
        //     }

        //     if measureModeCrossDim == MeasureMode::Undefined
        //         || (self.style().overflow != Overflow::Scroll
        //             && measureModeCrossDim == MeasureMode::AtMost)
        //     {
        //         // Clamp the size to the min/max size, if specified, and make sure it
        //         // doesn't go below the padding and border amount.
        //         self.layout().measured_dimensions[DIM[crossAxis as usize]] = bound_axis(
        //             node,
        //             crossAxis,
        //             totalLineCrossDim + paddingAndBorderAxisCross,
        //             crossAxisParentSize,
        //             parentWidth,
        //         );
        //     } else if measureModeCrossDim == MeasureMode::AtMost
        //         && self.style().overflow == Overflow::Scroll
        //     {
        //         self.layout().measured_dimensions[DIM[crossAxis as usize]] =
        //             (availableInnerCrossDim + paddingAndBorderAxisCross)
        //                 .max(bound_axis_within_min_and_max(
        //                     node,
        //                     crossAxis,
        //                     totalLineCrossDim + paddingAndBorderAxisCross,
        //                     crossAxisParentSize,
        //                 ))
        //                 .max(paddingAndBorderAxisCross);
        //     }

        //     // As we only wrapped in normal direction yet, we need to reverse the positions on wrap-reverse.
        //     if performLayout && self.style().flex_wrap == YGWrapWrapReverse {
        //         for i in 0..childCount {
        //             let child = GetChild(node, i);
        //             if child.style().position_type == PositionType::Relative {
        //                 child.layout.position[pos[crossAxis as usize] as usize] =
        //                     self.layout().measured_dimensions[DIM[crossAxis as usize]]
        //                         - child.layout.position[pos[crossAxis as usize] as usize]
        //                         - child.layout.measured_dimensions[DIM[crossAxis as usize]];
        //             }
        //         }
        //     }

        //     if performLayout {
        //         // STEP 10: SIZING AND POSITIONING ABSOLUTE CHILDREN
        //         currentAbsoluteChild = firstAbsoluteChild;
        //         while !currentAbsoluteChild.is_null() {
        //             absolute_layout_child((
        //                 node,
        //                 currentAbsoluteChild,
        //                 availableInnerWidth,
        //                 if isMainAxisRow {
        //                     measureModeMainDim
        //                 } else {
        //                     measureModeCrossDim
        //                 },
        //                 availableInnerHeight,
        //                 direction,
        //                 config,
        //             );
        //             currentAbsoluteChild = (*currentAbsoluteChild).nextChild;
        //         }

        //         // STEP 11: SETTING TRAILING POSITIONS FOR CHILDREN
        //         let needsMainTrailingPos = mainAxis == FlexDirection::RowReverse
        //             || mainAxis == FlexDirection::ColumnReverse;
        //         let needsCrossTrailingPos = crossAxis == FlexDirection::RowReverse
        //             || crossAxis == FlexDirection::ColumnReverse;

        //         // Set trailing position if necessary.
        //         if needsMainTrailingPos || needsCrossTrailingPos {
        //             for i in 0..childCount {
        //                 let child = ListGet(self.children, i);
        //                 if child.style().display == Display::None {
        //                     continue;
        //                 }
        //                 if needsMainTrailingPos {
        //                     set_child_trailing_position(node, child, mainAxis);
        //                 }

        //                 if needsCrossTrailingPos {
        //                     set_child_trailing_position(node, child, crossAxis);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    // fn set_child_trailing_position(&mut self, child: Node, axis: FlexDirection) -> () {
    //     let size: R32 = child.layout.measured_dimensions[DIM[axis as usize]];
    //     child.layout.position[trailing[axis as usize] as usize] =
    //         self.layout().measured_dimensions[DIM[axis as usize]]
    //             - size
    //             - child.layout.position[pos[axis as usize] as usize];
    // }

    // fn absolute_layout_child(
    //     &mut self,
    //     child: Node,
    //     width: R32,
    //     width_mode: MeasureMode,
    //     height: R32,
    //     direction: Direction,
    // ) -> () {
    //     let mainAxis: FlexDirection =
    //         YGResolveFlexDirection(self.style().flex_direction, direction);
    //     let crossAxis: FlexDirection = FlexDirectionCross(mainAxis, direction);
    //     let isMainAxisRow: bool = FlexDirectionIsRow(mainAxis);
    //     let mut childWidth: R32 = ::std::f32::NAN;
    //     let mut childHeight: R32 = ::std::f32::NAN;
    //     let mut childWidthMeasureMode: MeasureMode;
    //     let mut childHeightMeasureMode: MeasureMode;
    //     let margin_row: R32 = MarginForAxis(child, FlexDirection::Row, width);
    //     let margin_column: R32 = MarginForAxis(child, FlexDirection::Column, width);
    //     if IsStyleDimDefined(child, FlexDirection::Row, width) {
    //         childWidth = YGResolveValue(child.resolvedDimensions.width, width) + margin_row;
    //     } else {
    //         // If the child doesn't have a specified width, compute the width based
    //         // on the left/right
    //         // offsets if they're defined.
    //         if IsLeadingPosDefined(child, FlexDirection::Row)
    //             && is_trailing_pos_defined(child, FlexDirection::Row)
    //         {
    //             childWidth = self.layout().measured_dimensions.width
    //                 - (LeadingBorder(node, FlexDirection::Row)
    //                     + TrailingBorder(node, FlexDirection::Row))
    //                 - (LeadingPosition(child, FlexDirection::Row, width)
    //                     + TrailingPosition(child, FlexDirection::Row, width));
    //             childWidth = bound_axis(child, FlexDirection::Row, childWidth, width, width);
    //         };
    //     };
    //     if IsStyleDimDefined(child, FlexDirection::Column, height) {
    //         childHeight = YGResolveValue(child.resolvedDimensions.height, height) + margin_column;
    //     } else {
    //         // If the child doesn't have a specified height, compute the height
    //         // based on the top/bottom
    //         // offsets if they're defined.
    //         if IsLeadingPosDefined(child, FlexDirection::Column)
    //             && is_trailing_pos_defined(child, FlexDirection::Column)
    //         {
    //             childHeight = self.layout().measured_dimensions.height
    //                 - (LeadingBorder(node, FlexDirection::Column)
    //                     + TrailingBorder(node, FlexDirection::Column))
    //                 - (LeadingPosition(child, FlexDirection::Column, height)
    //                     + TrailingPosition(child, FlexDirection::Column, height));
    //             childHeight =
    //                 bound_axis(child, FlexDirection::Column, childHeight, height, width);
    //         };
    //     };
    //     // Exactly one dimension needs to be defined for us to be able to do aspect ratio
    //     // calculation. One dimension being the anchor and the other being flexible.
    //     if childWidth.is_nan() || childHeight.is_nan() {
    //         if !child.style().aspect_ratio.is_nan() {
    //             if childWidth.is_nan() {
    //                 childWidth =
    //                     margin_row + (childHeight - margin_column) * child.style().aspect_ratio;
    //             } else {
    //                 if childHeight.is_nan() {
    //                     childHeight =
    //                         margin_column + (childWidth - margin_row) / child.style().aspect_ratio;
    //                 };
    //             };
    //         };
    //     };
    //     // If we're still missing one or the other dimension, measure the content.
    //     if childWidth.is_nan() || childHeight.is_nan() {
    //         childWidthMeasureMode = if childWidth.is_nan() {
    //             MeasureMode::Undefined
    //         } else {
    //             MeasureMode::Exactly
    //         };
    //         childHeightMeasureMode = if childHeight.is_nan() {
    //             MeasureMode::Undefined
    //         } else {
    //             MeasureMode::Exactly
    //         };
    //         // If the size of the parent is defined then try to constrain the absolute child to that size
    //         // as well. This allows text within the absolute child to wrap to the size of its parent.
    //         // This is the same behavior as many browsers implement.
    //         if !isMainAxisRow
    //             && childWidth.is_nan()
    //             && width_mode != MeasureMode::Undefined
    //             && width > 0.0
    //         {
    //             childWidth = width;
    //             childWidthMeasureMode = MeasureMode::AtMost;
    //         };
    //         YGLayoutNodeInternal(
    //             child,
    //             childWidth,
    //             childHeight,
    //             direction,
    //             childWidthMeasureMode,
    //             childHeightMeasureMode,
    //             childWidth,
    //             childHeight,
    //             false,
    //             "abs-measure",
    //             config,
    //         );
    //         childWidth = child.layout.measured_dimensions.width
    //             + MarginForAxis(child, FlexDirection::Row, width);
    //         childHeight = child.layout.measured_dimensions.height
    //             + MarginForAxis(child, FlexDirection::Column, width);
    //     };
    //     YGLayoutNodeInternal(
    //         child,
    //         childWidth,
    //         childHeight,
    //         direction,
    //         MeasureMode::Exactly,
    //         MeasureMode::Exactly,
    //         childWidth,
    //         childHeight,
    //         true,
    //         "abs-layout",
    //         config,
    //     );
    //     if is_trailing_pos_defined(child, mainAxis)
    //         && !IsLeadingPosDefined(child, mainAxis)
    //     {
    //         child.layout.position[leading[mainAxis as usize] as usize] =
    //             self.layout().measured_dimensions[DIM[mainAxis as usize]]
    //                 - child.layout.measured_dimensions[DIM[mainAxis as usize]]
    //                 - TrailingBorder(node, mainAxis)
    //                 - TrailingMargin(child, mainAxis, width)
    //                 - TrailingPosition(
    //                     child,
    //                     mainAxis,
    //                     if isMainAxisRow { width } else { height },
    //                 );
    //     } else {
    //         if !IsLeadingPosDefined(child, mainAxis)
    //             && self.style().justify_content == Justify::Center
    //         {
    //             child.layout.position[leading[mainAxis as usize] as usize] =
    //                 (self.layout().measured_dimensions[DIM[mainAxis as usize]]
    //                     - child.layout.measured_dimensions[DIM[mainAxis as usize]])
    //                     / 2.0f32;
    //         } else {
    //             if !IsLeadingPosDefined(child, mainAxis)
    //                 && self.style().justify_content == Justify::FlexEnd
    //             {
    //                 child.layout.position[leading[mainAxis as usize] as usize] =
    //                     self.layout().measured_dimensions[DIM[mainAxis as usize]]
    //                         - child.layout.measured_dimensions[DIM[mainAxis as usize]];
    //             };
    //         };
    //     };
    //     if is_trailing_pos_defined(child, crossAxis)
    //         && !IsLeadingPosDefined(child, crossAxis)
    //     {
    //         child.layout.position[leading[crossAxis as usize] as usize] =
    //             self.layout().measured_dimensions[DIM[crossAxis as usize]]
    //                 - child.layout.measured_dimensions[DIM[crossAxis as usize]]
    //                 - TrailingBorder(node, crossAxis)
    //                 - TrailingMargin(child, crossAxis, width)
    //                 - TrailingPosition(
    //                     child,
    //                     crossAxis,
    //                     if isMainAxisRow { height } else { width },
    //                 );
    //     } else {
    //         if !IsLeadingPosDefined(child, crossAxis)
    //             && align_item(node, child) == Align::Center
    //         {
    //             child.layout.position[leading[crossAxis as usize] as usize] =
    //                 (self.layout().measured_dimensions[DIM[crossAxis as usize]]
    //                     - child.layout.measured_dimensions[DIM[crossAxis as usize]])
    //                     / 2.0f32;
    //         } else {
    //             if !IsLeadingPosDefined(child, crossAxis)
    //                 && (align_item(node, child) == Align::FlexEnd)
    //                     ^ (self.style().flex_wrap == YGWrapWrapReverse)
    //             {
    //                 child.layout.position[leading[crossAxis as usize] as usize] =
    //                     self.layout().measured_dimensions[DIM[crossAxis as usize]]
    //                         - child.layout.measured_dimensions[DIM[crossAxis as usize]];
    //             };
    //         };
    //     };
    // }

    fn align_item(&self, child: &Self) -> Align {
        let align: Align = if child.style().align_self == Align::Auto {
            self.style().align_items
        } else {
            child.style().align_self
        };

        if align == Align::Baseline && self.style().flex_direction.is_column() {
            return Align::FlexStart;
        };

        return align;
    }

    // fn baseline(&mut self) -> R32 {
    //     if self.baseline.is_some() {
    //         let baseline: R32 = self.baseline.expect("non-null function pointer")(
    //             node,
    //             self.layout().measured_dimensions.width,
    //             self.layout().measured_dimensions.height,
    //         );
    //         YGAssertWithNode(
    //             node,
    //             !baseline.is_nan(),
    //             b"Expect custom baseline function to not return NaN\x00" as *const u8
    //                 as *const c_char,
    //         );
    //         return baseline;
    //     };
    //     let mut baselineChild: Node = 0 as Node;
    //     let childCount = GetChildCount(node);
    //     {
    //         let mut i = 0usize;
    //         'loop5: while i < childCount {
    //             'body3: loop {
    //                 {
    //                     let child: Node = GetChild(node, i);
    //                     if child.lineIndex > 0 {
    //                         break 'loop5;
    //                     };
    //                     if child.style().position_type == PositionType::Absolute {
    //                         break 'body3;
    //                     };
    //                     if align_item(node, child) == Align::Baseline {
    //                         baselineChild = child;
    //                         break 'loop5;
    //                     };
    //                     if baselineChild.is_null() {
    //                         baselineChild = child;
    //                     };
    //                 }
    //                 break 'body3;
    //             }
    //             i = i.wrapping_add(1);
    //         }
    //     }
    //     if baselineChild.is_null() {
    //         return self.layout().measured_dimensions.height;
    //     };
    //     let baseline: R32 = baseline(baselineChild);
    //     return baseline + (*baselineChild).layout.position[Edge::Top as usize];
    // }

    // fn YGIsBaselineLayout(&mut self) -> bool {
    //     if FlexDirectionIsColumn(self.style().flex_direction) {
    //         return false;
    //     };
    //     if self.style().align_items == Align::Baseline {
    //         return true;
    //     };
    //     let childCount = GetChildCount(node);
    //     {
    //         let mut i = 0;
    //         while i < childCount {
    //             {
    //                 let child: Node = GetChild(node, i);
    //                 if child.style().position_type == PositionType::Relative
    //                     && child.style().align_self == Align::Baseline
    //                 {
    //                     return true;
    //                 };
    //             }
    //             i = i.wrapping_add(1);
    //         }
    //     }
    //     return false;
    // }

    fn is_flex(&self) -> bool {
        self.style().position_type == PositionType::Relative
            && (self.resolve_flex_grow() != 0.0 || self.resolve_flex_shrink() != 0.0)
    }

    fn compute_flex_basis_from_parent(
        &mut self,
        width: R32,
        width_mode: Option<MeasureMode>,
        height: R32,
        parent_width: R32,
        parent_height: R32,
        height_mode: Option<MeasureMode>,
        direction: Direction,
        parent_flex_direction: FlexDirection,
        parent_overflow: Overflow,
        self_align: Align,
    ) {
        let main_axis = parent_flex_direction.resolve_direction(direction);
        let is_main_axis_row = main_axis.is_row();
        let main_axis_size = if is_main_axis_row { width } else { height };
        let main_axis_parent_size = if is_main_axis_row {
            parent_width
        } else {
            parent_height
        };
        let resolved_flex_basis = self.style().resolve_flex_basis(main_axis_parent_size);

        let is_row_style_dim_defined = self.is_style_dim_defined(FlexDirection::Row, parent_width);
        let is_column_style_dim_defined =
            self.is_style_dim_defined(FlexDirection::Column, parent_height);
        if let Some(resolved_flex_basis) = resolved_flex_basis {
            // FIXME(anp): this should be sorted out!
            // if self.layout().computed_flex_basis.is_nan()
            //         && self.layout.computed_flex_basis_generation != gCurrentGenerationCount
            // {
            //     self.layout.computed_flex_basis =
            //         resolvedFlexBasis.max(PaddingAndBorderForAxis(self, mainAxis, parentWidth));
            // };
        } else if is_main_axis_row && is_row_style_dim_defined {
            // The width is definite, so use that as the flex basis.
            self.layout_mut().computed_flex_basis = self
                .resolved()
                .width
                .unwrap_or_else(Default::default)
                .resolve(parent_width)
                .map(|b| {
                    b.max(
                        self.style()
                            .padding_and_border_for_axis(FlexDirection::Row, parent_width),
                    )
                });
        } else if !is_main_axis_row && is_column_style_dim_defined {
            // The height is definite, so use that as the flex basis.
            self.layout_mut().computed_flex_basis = self
                .resolved()
                .height
                .unwrap_or_else(Default::default)
                .resolve(parent_height)
                .map(|b| {
                    b.max(
                        self.style()
                            .padding_and_border_for_axis(FlexDirection::Column, parent_width),
                    )
                });
        } else {
            let mut self_width = None;
            let mut self_height = None;
            let mut self_width_measure_mode = None;
            let mut self_height_measure_mode = None;
            // Compute the flex basis and hypothetical main size (i.e. the clamped flex basis).
            let margin_row = self
                .style()
                .margin
                .for_axis(FlexDirection::Row, parent_width);
            let margin_column = self
                .style()
                .margin
                .for_axis(FlexDirection::Column, parent_width);
            if is_row_style_dim_defined {
                self_width = self
                    .resolved()
                    .width
                    .map(|w| w.resolve(parent_width).map(|w| w + margin_row))
                    .into_iter()
                    .flatten()
                    .next();
                self_width_measure_mode = Some(MeasureMode::Exactly);
            };
            if is_column_style_dim_defined {
                self_height = self
                    .resolved()
                    .height
                    .map(|h| h.resolve(parent_height).map(|h| h + margin_column))
                    .into_iter()
                    .flatten()
                    .next();
                self_height_measure_mode = Some(MeasureMode::Exactly);
            };
            // The W3C spec doesn't say anything about the 'overflow' property,
            // but all major browsers appear to implement the following logic.
            if (!is_main_axis_row && parent_overflow == Overflow::Scroll)
                || parent_overflow != Overflow::Scroll
            {
                if self_width.is_none() {
                    self_width = Some(width);
                    self_width_measure_mode = Some(MeasureMode::AtMost);
                };
            }

            if (is_main_axis_row && parent_overflow == Overflow::Scroll)
                || parent_overflow != Overflow::Scroll
            {
                if self_height.is_none() {
                    self_height = Some(height);
                    self_height_measure_mode = Some(MeasureMode::AtMost);
                }
            }

            if !self.style().aspect_ratio.is_none() {
                if !is_main_axis_row && self_width_measure_mode == Some(MeasureMode::Exactly) {
                    self_height = self_width
                        // FIXME(anp): aspect_ratio should not be optional
                        .map(|w| (w - margin_row) / self.style().aspect_ratio.unwrap_or(r32(1.0)));
                    self_height_measure_mode = Some(MeasureMode::Exactly);
                } else {
                    if is_main_axis_row && self_height_measure_mode == Some(MeasureMode::Exactly) {
                        self_width = self_height.map(|h| {
                            (h - margin_column) * self.style().aspect_ratio.unwrap_or(r32(0.0))
                        });
                        self_width_measure_mode = Some(MeasureMode::Exactly);
                    };
                };
            };
            // If self has no defined size in the cross axis and is set to stretch,
            // set the cross
            // axis to be measured exactly with the available inner width
            let has_exact_width = !width.is_nan() && width_mode == Some(MeasureMode::Exactly);
            let self_width_stretch = self_align == Align::Stretch
                && self_width_measure_mode != Some(MeasureMode::Exactly);

            if !is_main_axis_row
                && !is_row_style_dim_defined
                && has_exact_width
                && self_width_stretch
            {
                self_width = Some(width);
                self_width_measure_mode = Some(MeasureMode::Exactly);
                if let Some(ar) = self.style().aspect_ratio {
                    self_height = self_width.map(|w| (w - margin_row) / ar);
                    self_height_measure_mode = Some(MeasureMode::Exactly);
                }
            }

            let has_exact_height = !height.is_nan() && height_mode == Some(MeasureMode::Exactly);
            let self_height_stretch = self.align_item(self) == Align::Stretch
                && self_height_measure_mode != Some(MeasureMode::Exactly);
            if is_main_axis_row
                && !is_column_style_dim_defined
                && has_exact_height
                && self_height_stretch
            {
                self_height = Some(height);
                self_height_measure_mode = Some(MeasureMode::Exactly);
                if let Some(ar) = self.style().aspect_ratio {
                    self_width = self_height.map(|h| (h - margin_column) * ar);
                    self_width_measure_mode = Some(MeasureMode::Exactly);
                }
            }

            let ((self_width, self_width_measure_mode), (self_height, self_height_measure_mode)) = (
                self.constrained_max_size_for_mode(
                    FlexDirection::Row,
                    parent_width,
                    parent_width,
                    self_width_measure_mode,
                    self_width.expect(
                        "anp hasn't had time yet to properly refactor the above into expressions",
                    ),
                ),
                self.constrained_max_size_for_mode(
                    FlexDirection::Column,
                    parent_height,
                    parent_width,
                    self_height_measure_mode,
                    self_height.expect(
                        "anp hasn't had time yet to properly refactor the above into expressions",
                    ),
                ),
            );

            self.layout_node_internal(
                self_width,
                self_height,
                direction,
                self_width_measure_mode,
                self_height_measure_mode,
                parent_width,
                parent_height,
                r32(Self::POINT_SCALE_FACTOR),
                false,
                "measure",
            );

            self.layout_mut().computed_flex_basis = self.layout().measured_dimensions.map(|m| {
                m[main_axis.dimension()].max(
                    self.style()
                        .padding_and_border_for_axis(main_axis, parent_width),
                )
            });
        }

        self.layout_mut().computed_flex_basis_generation = self.current_generation();
    }
    // fn copy_style(&mut self, mut from: Self) {
    //     let mut src = from.style();
    //     let mut dst = self.style();
    //     if src != dst {
    //         *dst = *src;
    //         self.mark_dirty();
    //     }
    // }

    // fn SetMeasureFunc(&mut self, mut measureFunc: YGMeasureFunc) -> () {
    //     if measureFunc.is_none() {
    //         self.measure = None;
    //         self.nodeType = NodeType::Default;
    //     } else {
    //         YGAssertWithNode(
    //         node,
    //         GetChildCount(node) == 0,
    //         b"Cannot set measure function: Nodes with measure functions cannot have children.\x00"
    //             as *const u8 as *const c_char,
    //     );
    //         self.measure = measureFunc;
    //         self.nodeType = NodeType::Text;
    //     };
    // }

    // fn SetNodeType(&mut self, mut nodeType: NodeType) -> () {
    //     self.nodeType = nodeType;
    // }

    // fn GetNodeType(&mut self) -> NodeType {
    //     return self.nodeType;
    // }

    // unsafe fn YGRoundToPixelGrid(
    //     &mut self,
    //     pointScaleFactor: R32,
    //     absoluteLeft: R32,
    //     absoluteTop: R32,
    // ) {
    //     if pointScaleFactor == 0.0 {
    //         return;
    //     }

    //     let nodeLeft = self.layout().position[Edge::Left as usize];
    //     let nodeTop = self.layout().position[Edge::Top as usize];

    //     let nodeWidth = self.layout().dimensions[Dimension::Width as usize];
    //     let nodeHeight = self.layout().dimensions[Dimension::Height as usize];

    //     let absoluteNodeLeft = absoluteLeft + nodeLeft;
    //     let absoluteNodeTop = absoluteTop + nodeTop;

    //     let absoluteNodeRight = absoluteNodeLeft + nodeWidth;
    //     let absoluteNodeBottom = absoluteNodeTop + nodeHeight;

    //     // If a node has a custom measure function we never want to round down its size as this could
    //     // lead to unwanted text truncation.
    //     let textRounding = self.nodeType == NodeType::Text;

    //     self.layout().position[Edge::Left as usize] =
    //         round_value_to_pixel_grid(nodeLeft, pointScaleFactor, false, textRounding);
    //     self.layout().position[Edge::Top as usize] =
    //         round_value_to_pixel_grid(nodeTop, pointScaleFactor, false, textRounding);

    //     // We multiply dimension by scale factor and if the result is close to the whole number, we don't
    //     // have any fraction
    //     // To verify if the result is close to whole number we want to check both floor and ceil numbers
    //     let hasFractionalWidth = !YGFloatsEqual(nodeWidth * pointScaleFactor % 1.0, 0.0)
    //         && !YGFloatsEqual(nodeWidth * pointScaleFactor % 1.0, 1.0);
    //     let hasFractionalHeight = !YGFloatsEqual(nodeHeight * pointScaleFactor % 1.0, 0.0)
    //         && !YGFloatsEqual(nodeHeight * pointScaleFactor % 1.0, 1.0);

    //     self.layout().dimensions[Dimension::Width as usize] = round_value_to_pixel_grid(
    //         absoluteNodeRight,
    //         pointScaleFactor,
    //         textRounding && hasFractionalWidth,
    //         textRounding && !hasFractionalWidth,
    //     )
    //         - round_value_to_pixel_grid(absoluteNodeLeft, pointScaleFactor, false, textRounding);
    //     self.layout().dimensions[Dimension::Height as usize] = round_value_to_pixel_grid(
    //         absoluteNodeBottom,
    //         pointScaleFactor,
    //         textRounding && hasFractionalHeight,
    //         textRounding && !hasFractionalHeight,
    //     )
    //         - round_value_to_pixel_grid(absoluteNodeTop, pointScaleFactor, false, textRounding);

    //     for i in 0..ListCount(self.children) {
    //         YGRoundToPixelGrid(
    //             GetChild(node, i),
    //             pointScaleFactor,
    //             absoluteNodeLeft,
    //             absoluteNodeTop,
    //         );
    //     }
    // }

    fn constrained_max_size_for_mode(
        &self,
        axis: FlexDirection,
        parent_axis_size: R32,
        parent_width: R32,
        mode: Option<MeasureMode>,
        existing_size: R32,
    ) -> (R32, Option<MeasureMode>) {
        let max_size = self.style().max_dimensions[axis.dimension()]
            .resolve(parent_axis_size)
            .map(|s| s + self.style().margin.for_axis(axis, parent_width));

        use MeasureMode::*;
        match (mode, max_size) {
            (Some(Exactly), Some(max_size)) | (Some(AtMost), Some(max_size))
                if existing_size < max_size =>
            {
                (max_size, mode)
            }
            (Some(Exactly), _) | (Some(AtMost), _) => (existing_size, mode),
            (None, Some(max_size)) => (max_size, Some(AtMost)),
            (None, None) => panic!("i dont think this case was covered by the original, TODO"),
        }
    }
}

pub trait Percent {
    fn percent(self) -> Value;
}

impl Percent for f32 {
    fn percent(self) -> Value {
        Value::Percent(r32(self))
    }
}

impl Percent for i32 {
    fn percent(self) -> Value {
        Value::Percent(r32(self as f32))
    }
}

pub trait Point {
    fn point(self) -> Value;
}

impl Point for f32 {
    fn point(self) -> Value {
        Value::Point(r32(self))
    }
}

impl Point for i32 {
    fn point(self) -> Value {
        Value::Point(r32(self as f32))
    }
}
