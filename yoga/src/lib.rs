// TODO(anp): excise unwrap/expect/panic!
// TODO(anp): check out the inline annotations from the c code
// TODO(anp): revist raph's continuation-based layout stuff, in case you forget, june 2018 meetup at mozilla
// TODO(anp): pub/pub(crate)/private audit
// TODO(anp): #![deny(missing_docs)]
// TODO(anp): mutability pass
// TODO(anp): create a style builder that can be constructed with some defaults
//   and used to churn out nodes
// TODO(anp): do a pass to remove .is_nan()
// TODO(anp): pass to remove unnecessary #[allow(...)]
// TODO(anp): let clippy loose once nightlies are blocked on it
// TODO(anp): disable pub on everything and do a public api pass
// TODO(anp): docs
// TODO(anp): sort out left/right vs start/end for PhysicalEdge
// TODO(anp): do a pass on node.0

extern crate arrayvec;
extern crate float_cmp;
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
    pub(crate) use super::enums::Value::*;
    pub(crate) use super::enums::*;
    pub(crate) use super::hacks::ApproxEqHackForReals;
    pub(crate) use super::layout::{CachedMeasurement, Layout};
    pub(crate) use super::style::{Property, Style};
    pub(crate) use super::*;
    pub(crate) use noisy_float::prelude::*;
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use std::collections::BTreeMap;
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

// pub trait Yggdrasil<N: Node<Self, I>, I>
// where
//     N: Node<Self, I>,
//     // FIXME(anp): this is going to be  broken until i can refactor the traits the use handles
//     I: Iterator<Item = N>,
//     Self: 'static + Debug + Sized,
// {
//     // TODO(anp): implement this as a linked list or smth i guess? probably needs to be done with
//     //   handles
//     fn add_absolute_child(&N);
//     fn absolute_children() -> I;
//     fn current_generation() -> u32;
//     fn increment_generation();
// }

// TODO(anp): include a generation count here to avoid issues
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Handle(usize);

#[derive(Clone)]
pub struct Wheel {
    current_gen: u32,
    // TODO(anp): cache layouts here instead of inline
    // TODO(anp): compact these between layouts?
    // TODO(anp): use an actual bitvec, this is unnecessarily large
    dirty: Vec<bool>,
    // TODO(anp): should this be inline with style or something else?
    types: Vec<NodeType>,
    styles: Vec<Style>,
    // FIXME(anp): probably needs to be eliminated completely?
    new_layout: Vec<bool>,
    layouts: Vec<Layout>,
    lines: Vec<u32>,
    parents: Vec<Option<Handle>>,
    children: Vec<Vec<Handle>>,
    // FIXME(anp): make this optional
    resolved: Vec<ResolvedDimensions>,
    // this one doesn't have the same indices!
    abs_children: Vec<Handle>,
    // measure_fns: BTreeMap<Handle, MEASURE_FN<(&Self, Handle)>>,
    // baseline_fns: BTreeMap<Handle, BASELINE_FN<(&Self, Handle)>>,
}

const POINT_SCALE_FACTOR: f32 = 1.0;
pub(crate) type BASELINE_FN = fn((&Wheel, Handle), R32, R32) -> R32;
pub(crate) type MEASURE_FN =
    fn((&Wheel, Handle), R32, Option<MeasureMode>, R32, Option<MeasureMode>) -> Size;

impl Wheel {
    // TODO(anp): eliminate mutable methods and require all mutations to be passed back to caller
    fn add_absolute_child(&mut self, child: Handle) {
        self.abs_children.push(child);
    }

    fn absolute_children(&self) -> &[Handle] {
        &self.abs_children
    }

    fn current_generation(&self) -> u32 {
        self.current_gen
    }

    fn increment_generation(&mut self) {
        self.current_gen += 1;
    }

    fn parent(&self, node: Handle) -> Option<Handle> {
        self.parents[node.0]
    }

    fn child(&self, node: Handle, index: usize) -> Handle {
        self.children[node.0][index]
    }

    fn children(&self, node: Handle) -> &[Handle] {
        self.children[node.0].as_slice()
    }

    fn style(&self, node: Handle) -> Style {
        self.styles[node.0]
    }

    fn style_mut(&mut self, node: Handle) -> &mut Style {
        self.styles.get_mut(node.0).unwrap()
    }

    fn layout(&self, node: Handle) -> Layout {
        self.layouts[node.0]
    }

    fn layout_mut(&mut self, node: Handle) -> &mut Layout {
        self.layouts.get_mut(node.0).unwrap()
    }

    fn measure_fn(&self, node: Handle) -> Option<MEASURE_FN> {
        unimplemented!()
    }

    fn baseline_fn(&self, node: Handle) -> Option<BASELINE_FN> {
        unimplemented!()
    }

    // fn dirty(&mut self) -> &mut bool;
    // fn new_layout(&mut self) -> &mut bool;
    // fn node_type(&self) -> NodeType;
    fn resolved(&self, node: Handle) -> ResolvedDimensions {
        self.resolved[node.0]
    }

    fn set_resolved(&mut self, node: Handle, edge: Dimension, resolved: Value) {
        unimplemented!()
    }

    fn is_style_dim_defined(&self, node: Handle, axis: FlexDirection, parent_size: R32) -> bool {
        parent_size.is_nan() || match self.resolved(node)[axis.dimension()] {
            Some(Value::Percent(r)) | Some(Value::Point(r)) => r < 0.0,
            Some(Value::Auto) => false,
            None => true,
        }
    }

    fn resolve_dimensions(&mut self, node: Handle) {
        for &dim in [Dimension::Width, Dimension::Height].into_iter() {
            let style = self.style(node);

            if style.max_dimensions[dim] != style.min_dimensions[dim] {
                self.set_resolved(node, dim, style.max_dimensions[dim]);
            } else {
                self.set_resolved(node, dim, style.dimensions[dim]);
            };
        }
    }

    fn calculate_layout(
        &mut self,
        node: Handle,
        parent_width: R32,
        parent_height: R32,
        parent_direction: Direction,
    ) {
        // Increment the generation count. This will force the recursive routine to
        // visit
        // all dirty nodes at least once. Subsequent visits will be skipped if the
        // input
        // parameters don't change.

        // FIXME(anp): reenable
        // Y::increment_generation();

        self.resolve_dimensions(node);

        let (width, width_measure_mode): (Option<R32>, Option<MeasureMode>) =
            if self.is_style_dim_defined(node, FlexDirection::Row, parent_width) {
                (
                    self.resolved(node)[FlexDirection::Row.dimension()]
                        .into_iter()
                        .flat_map(|v| v.resolve(parent_width))
                        .map(|v| {
                            v + self
                                .style(node)
                                .margin
                                .for_axis(FlexDirection::Row, parent_width)
                        })
                        .next(),
                    Some(MeasureMode::Exactly),
                )
            } else {
                if self.style(node).max_dimensions.width.resolve(parent_width) >= Some(r32(0.0)) {
                    (
                        self.style(node).max_dimensions.width.resolve(parent_width),
                        Some(MeasureMode::AtMost),
                    )
                } else {
                    (Some(parent_width), None)
                }
            };

        let (height, height_measure_mode): (Option<R32>, Option<MeasureMode>) =
            if self.is_style_dim_defined(node, FlexDirection::Column, parent_height) {
                (
                    self.resolved(node)[FlexDirection::Column.dimension()]
                        .into_iter()
                        .flat_map(|v| v.resolve(parent_height))
                        .map(|v| {
                            v + self
                                .style(node)
                                .margin
                                .for_axis(FlexDirection::Column, parent_height)
                        })
                        .next(),
                    Some(MeasureMode::Exactly),
                )
            } else {
                if self
                    .style(node)
                    .max_dimensions
                    .height
                    .resolve(parent_height) >= Some(r32(0.0))
                {
                    (
                        self.style(node)
                            .max_dimensions
                            .height
                            .resolve(parent_height),
                        Some(MeasureMode::AtMost),
                    )
                } else {
                    (Some(parent_height), None)
                }
            };

        let did_something_wat = self.layout_node_internal(
            node,
            width.unwrap(),
            height.unwrap(),
            parent_direction,
            width_measure_mode,
            height_measure_mode,
            parent_width,
            parent_height,
            r32(POINT_SCALE_FACTOR),
            true,
            "initial",
        );

        if did_something_wat {
            let dir = self.layout(node).direction;
            let has_parent = self.parent(node).is_some();
            let style = self.style(node);
            // unused: we know it's been updated, we're calculating it
            let _ = self.layout_mut(node).set_position(
                style,
                dir,
                parent_width,
                parent_height,
                parent_width,
                has_parent,
            );

            self.round_to_pixel_grid(node, r32(POINT_SCALE_FACTOR), r32(0.0), r32(0.0));
        };
    }

    /// This is a wrapper around the layoutImpl function. It determines whether the layout
    /// request is redundant and can be skipped. Input parameters are the same as `layoutImpl`
    /// (see above). Return parameter is true if layout was performed, false if skipped.
    fn layout_node_internal(
        &mut self,
        node: Handle,
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
        // FIXME(anp): figure out how to print these disaggregated nodes
        // trace!("layout for reason {} on node {:?}", reason, self);

        // FIXME(anp): reenable
        // let current_generation = Y::current_generation();
        let need_to_visit_node = self.dirty[node.0]
            // FIXME(anp): reenable
            // && self.layout(node).generation_count != current_generation
            || self.layout(node).last_parent_direction != Some(parent_direction);

        // CACHING(anp): if a node is marked as dirty, invalidate its cache results. i think
        // that it would be reasonable to move this to an entirely separate subsystem
        // if need_to_visit_node {
        //     // Invalidate the cached results.
        //     self.layout_mut(node).cached_layout = None;
        //     self.layout_mut(node).next_cached_measurements_index = 0;
        // };

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
        // CACHING(anp): this is where some cached values were fetched previously
        let cached_results: Option<CachedMeasurement> = None;
        // let cached_results = if let Some(cached) = self.layout(node).cached_layout {
        //     if let Some(_) = self.measure_fn(node) {
        //         let margin_axis_row = self
        //             .style(node)
        //             .margin
        //             .for_axis(FlexDirection::Row, parent_width);
        //         let margin_axis_column = self
        //             .style(node)
        //             .margin
        //             .for_axis(FlexDirection::Column, parent_height);
        //         // First, try to use the layout cache.
        //         if CachedMeasurement::usable(
        //             Some(cached),
        //             width_measure_mode,
        //             available_width,
        //             height_measure_mode,
        //             available_height,
        //             margin_axis_row,
        //             margin_axis_column,
        //             point_scale_factor,
        //         ) {
        //             Some(cached)
        //         } else {
        //             // Try to use the measurement cache.
        //             let idx = self.layout(node).next_cached_measurements_index;
        //             match self.layout(node).cached_measurements[0..idx]
        //                 .into_iter()
        //                 .find(|c| {
        //                     CachedMeasurement::usable(
        //                         **c,
        //                         width_measure_mode,
        //                         available_width,
        //                         height_measure_mode,
        //                         available_height,
        //                         margin_axis_row,
        //                         margin_axis_column,
        //                         point_scale_factor,
        //                     )
        //                 }) {
        //                 Some(Some(v)) => Some(*v),
        //                 _ => None,
        //             }
        //         }
        //     } else if perform_layout
        //         && cached.available_width.approx_eq(available_width)
        //         && cached.available_height.approx_eq(available_height)
        //         && cached.width_measure_mode == width_measure_mode
        //         && cached.height_measure_mode == height_measure_mode
        //     {
        //         Some(cached)
        //     } else {
        //         let idx = self.layout(node).next_cached_measurements_index;
        //         self.layout(node).cached_measurements[0..idx]
        //             .into_iter()
        //             .filter_map(|&s| s)
        //             .filter(|c| {
        //                 c.available_width.approx_eq(available_width)
        //                     && c.available_height.approx_eq(available_height)
        //                     && c.width_measure_mode == width_measure_mode
        //                     && c.height_measure_mode == height_measure_mode
        //             })
        //             .next()
        //     }
        // } else {
        //     None
        // };

        if let (false, Some(cached)) = (need_to_visit_node, cached_results) {
            self.layout_mut(node).measured_dimensions = cached.computed;
        } else {
            self.layout_impl(
                node,
                available_width,
                available_height,
                parent_direction,
                width_measure_mode,
                height_measure_mode,
                parent_width,
                parent_height,
                perform_layout,
            );

            self.layout_mut(node).last_parent_direction = Some(parent_direction);
            if cached_results.is_none() {
                // CACHING(anp): cache is populated here too
                // if self.layout_mut(node).next_cached_measurements_index == 16 {
                //     self.layout_mut(node).next_cached_measurements_index = 0;
                // };

                // let computed = self.layout_mut(node).measured_dimensions;

                // let mut new_cache_entry = if perform_layout {
                //     // Use the single layout cache entry.
                //     &mut self.layout_mut(node).cached_layout
                // } else {
                //     self.layout_mut(node).next_cached_measurements_index += 1;
                //     let idx = self.layout_mut(node).next_cached_measurements_index;
                //     &mut self.layout_mut(node).cached_measurements[idx]
                // };

                // *new_cache_entry = Some(CachedMeasurement {
                //     available_width: available_width,
                //     available_height: available_height,
                //     width_measure_mode: width_measure_mode,
                //     height_measure_mode: height_measure_mode,
                //     computed,
                // });
            }
        }

        // FIXME(anp): reenable
        // self.layout_mut(node).generation_count = Y::current_generation();

        if perform_layout {
            self.layout_mut(node).dimensions =
                Some(self.layout_mut(node).measured_dimensions.into());
            self.new_layout[node.0];
            self.dirty[node.0] = false;
        };

        return need_to_visit_node || cached_results.is_none();
    }

    fn mark_dirty(&mut self, node: Handle) {
        if !self.dirty[node.0] {
            self.dirty[node.0] = true;
            self.layout_mut(node).computed_flex_basis = None;

            if let Some(p) = self.parent(node) {
                self.mark_dirty(p);
            }
        };
    }

    fn apply_style<P: Property>(&mut self, node: Handle, new_style: P) {
        if Updated::Dirty == new_style.apply(&mut self.styles[node.0]) {
            self.mark_dirty(node);
        }
    }

    fn with_measure_func_set_measured_dimensions(
        &mut self,
        node: Handle,
        measure: MEASURE_FN,
        available_width: R32,
        available_height: R32,
        width_measure_mode: Option<MeasureMode>,
        height_measure_mode: Option<MeasureMode>,
        parent_width: R32,
        parent_height: R32,
    ) -> MeasuredDimensions {
        let padding_and_border_axis_row = self
            .style(node)
            .padding_and_border_for_axis(FlexDirection::Row, available_width);
        let padding_and_border_axis_column = self
            .style(node)
            .padding_and_border_for_axis(FlexDirection::Column, available_width);
        let margin_axis_row = self
            .style(node)
            .margin
            .for_axis(FlexDirection::Row, available_width);
        let margin_axis_column = self
            .style(node)
            .margin
            .for_axis(FlexDirection::Column, available_width);

        // We want to make sure we don't call measure with negative size
        let inner_width =
            (available_width - margin_axis_row - padding_and_border_axis_row).max(r32(0.0));

        let inner_height =
            (available_height - margin_axis_column - padding_and_border_axis_column).max(r32(0.0));

        if width_measure_mode == Some(MeasureMode::Exactly)
            && height_measure_mode == Some(MeasureMode::Exactly)
        {
            // Don't bother sizing the text if both dimensions are already defined.
            let width = self.bound_axis(
                node,
                FlexDirection::Row,
                available_width - margin_axis_row,
                // TODO(anp): the original source said parentWidth 2x here, not sure why
                parent_width,
                parent_width,
            );

            let height = self.bound_axis(
                node,
                FlexDirection::Column,
                available_height - margin_axis_column,
                parent_height,
                parent_width,
            );

            MeasuredDimensions { width, height }
        } else {
            // Measure the text under the current constraints.
            let measured_size = measure(
                (self, node),
                inner_width,
                width_measure_mode,
                inner_height,
                height_measure_mode,
            );

            let bound = |dir,
                         measure_mode: Option<MeasureMode>,
                         measured_size,
                         axis_margin,
                         available1,
                         available2| {
                self.bound_axis(
                    node,
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
    fn bound_axis(
        &self,
        node: Handle,
        axis: FlexDirection,
        value: R32,
        axis_size: R32,
        width_size: R32,
    ) -> R32 {
        self.bound_axis_within_min_and_max(node, axis, value, axis_size)
            .max(
                self.style(node)
                    .padding_and_border_for_axis(axis, width_size),
            )
    }

    fn bound_axis_within_min_and_max(
        &self,
        node: Handle,
        axis: FlexDirection,
        value: R32,
        axis_size: R32,
    ) -> R32 {
        let (min, max) = match axis {
            FlexDirection::Column | FlexDirection::ColumnReverse => (
                self.style(node).min_dimensions.height.resolve(axis_size),
                self.style(node).max_dimensions.height.resolve(axis_size),
            ),
            FlexDirection::Row | FlexDirection::RowReverse => (
                self.style(node).min_dimensions.width.resolve(axis_size),
                self.style(node).max_dimensions.width.resolve(axis_size),
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
        node: Handle,
        available_width: R32,
        available_height: R32,
        width_measure_mode: Option<MeasureMode>,
        height_measure_mode: Option<MeasureMode>,
        parent_width: R32,
        parent_height: R32,
    ) -> MeasuredDimensions {
        let padding_and_border_axis_row = self
            .style(node)
            .padding_and_border_for_axis(FlexDirection::Row, parent_width);
        let padding_and_border_axis_column = self
            .style(node)
            .padding_and_border_for_axis(FlexDirection::Column, parent_width);

        let margin_axis_row = self
            .style(node)
            .margin
            .for_axis(FlexDirection::Row, parent_width);
        let margin_axis_column = self
            .style(node)
            .margin
            .for_axis(FlexDirection::Column, parent_width);

        MeasuredDimensions {
            width: self.bound_axis(
                node,
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
                node,
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
        node: Handle,
        available_width: R32,
        available_height: R32,
        width_measure_mode: Option<MeasureMode>,
        height_measure_mode: Option<MeasureMode>,
        parent_width: R32,
        parent_height: R32,
    ) -> Option<MeasuredDimensions> {
        if width_measure_mode == Some(MeasureMode::AtMost) && available_width <= 0.0
            || height_measure_mode == Some(MeasureMode::AtMost) && available_height <= 0.0
            || width_measure_mode == Some(MeasureMode::Exactly)
                && height_measure_mode == Some(MeasureMode::Exactly)
        {
            let margin_axis_column = self
                .style(node)
                .margin
                .for_axis(FlexDirection::Column, parent_width);

            let margin_axis_row = self
                .style(node)
                .margin
                .for_axis(FlexDirection::Row, parent_width);

            Some(MeasuredDimensions {
                width: self.bound_axis(
                    node,
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
                    node,
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

    fn resolve_flex_grow(&self, node: Handle) -> R32 {
        // Root nodes flexGrow should always be 0
        let flex_grow = self.style(node).flex_grow;
        let flex = self.style(node).flex;
        match (self.parent(node), flex_grow, flex) {
            (None, _, _) => r32(0.0),
            (_, grow, _) if grow != Style::DEFAULT_FLEX_GROW => grow,
            (_, _, Some(flex)) if flex > 0.0 => flex,
            _ => r32(Style::DEFAULT_FLEX_GROW),
        }
    }

    fn resolve_flex_shrink(&self, node: Handle) -> R32 {
        match (
            self.parent(node),
            self.style(node).flex_shrink,
            self.style(node).flex,
            cfg!(feature = "web-default"),
        ) {
            // Root nodes flexShrink should always be 0
            (None, _, _, _) => r32(0.0),
            (_, shrink, _, _) if shrink != Style::DEFAULT_FLEX_SHRINK => shrink,
            (_, _, Some(flex), false) if flex < 0.0 => -flex,
            _ => r32(Style::DEFAULT_FLEX_SHRINK),
        }
    }

    fn zero_layout_recursively(&mut self, node: Handle) {
        // FIXME(anp): this should be assigning None to a nullable field!!!
        *self.layout_mut(node) = Layout::default();
        self.new_layout[node.0] = true;
        for idx in 0..self.children(node).len() {
            let child = self.child(node, idx);
            self.zero_layout_recursively(child);
        }
    }

    fn dim_with_margin(&self, node: Handle, axis: FlexDirection, width_size: R32) -> R32 {
        self.layout(node).measured_dimensions[axis.dimension()]
            + self.style(node).margin.leading(axis, width_size)
            + self.style(node).margin.trailing(axis, width_size)
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
        node: Handle,
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
        let direction = self.layout(node).direction.resolve(parent_direction);
        let flex_row_direction = FlexDirection::Row.resolve_direction(direction);
        let flex_column_direction = FlexDirection::Column.resolve_direction(direction);
        self.layout_mut(node).direction = direction;

        self.layout_mut(node).margin = self.style(node).margin.resolve(
            flex_row_direction,
            flex_column_direction,
            parent_width,
        );

        self.layout_mut(node).border = self
            .style(node)
            .border
            .resolve(flex_row_direction, flex_column_direction);

        self.layout_mut(node).padding = self.style(node).padding.resolve(
            flex_row_direction,
            flex_column_direction,
            parent_width,
        );

        // TODO(anp): make this idempotent/typesafe/etc
        if let Some(measure_fn) = self.measure_fn(node) {
            self.layout_mut(node).measured_dimensions = self
                .with_measure_func_set_measured_dimensions(
                    node,
                    measure_fn,
                    available_width,
                    available_height,
                    width_measure_mode,
                    height_measure_mode,
                    parent_width,
                    parent_height,
                );
            return;
        }

        if self.children(node).is_empty() {
            self.layout_mut(node).measured_dimensions = self
                .empty_container_set_measured_dimensions(
                    node,
                    available_width,
                    available_height,
                    width_measure_mode,
                    height_measure_mode,
                    parent_width,
                    parent_height,
                );
            return;
        };

        // If we're not being asked to perform a full layout we can skip the algorithm if we already know
        // the size
        if let (false, Some(d)) = (
            perform_layout,
            self.fixed_size_set_measured_dimensions(
                node,
                available_width,
                available_height,
                width_measure_mode,
                height_measure_mode,
                parent_width,
                parent_height,
            ),
        ) {
            self.layout_mut(node).measured_dimensions = d;
            return;
        }

        // Reset layout flags, as they could have changed.
        self.layout_mut(node).had_overflow = false;

        // STEP 1: CALCULATE VALUES FOR REMAINDER OF ALGORITHM
        let main_axis = self.style(node).flex_direction.resolve_direction(direction);
        let cross_axis = main_axis.cross(direction);
        let is_main_axis_row = main_axis.is_row();
        let justify_content = self.style(node).justify_content;
        let is_node_flex_wrap = self.style(node).flex_wrap != Wrap::NoWrap;

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
            .style(node)
            .leading_padding_and_border(main_axis, parent_width);
        let trailing_padding_and_border_main = self
            .style(node)
            .trailing_padding_and_border(main_axis, parent_width);
        let leading_padding_and_border_cross = self
            .style(node)
            .leading_padding_and_border(cross_axis, parent_width);
        let padding_and_border_axis_main = self
            .style(node)
            .padding_and_border_for_axis(main_axis, parent_width);
        let padding_and_border_axis_cross = self
            .style(node)
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
            .style(node)
            .margin
            .for_axis(FlexDirection::Row, parent_width);
        let margin_axis_column = self
            .style(node)
            .margin
            .for_axis(FlexDirection::Column, parent_width);

        // STEP 2: DETERMINE AVAILABLE SIZE IN MAIN AND CROSS DIRECTIONS
        let min_inner_width =
            self.style(node)
                .min_dimensions
                .width
                .resolve(parent_width)
                .unwrap_or(r32(0.0)) - margin_axis_row - padding_and_border_axis_row;
        let max_inner_width =
            self.style(node)
                .max_dimensions
                .width
                .resolve(parent_width)
                .unwrap_or(r32(0.0)) - margin_axis_row - padding_and_border_axis_row;
        let min_inner_height = self
            .style(node)
            .min_dimensions
            .height
            .resolve(parent_height)
            .unwrap_or(r32(0.0)) - margin_axis_column
            - padding_and_border_axis_column;
        let max_inner_height = self
            .style(node)
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
        let available_inner_cross_dim = if is_main_axis_row {
            available_inner_height
        } else {
            available_inner_width
        };

        // If there is only one child with flex_grow + flex_shrink it means we can set the
        // computed_flex_basis to 0 instead of measuring and shrinking / flexing the child to exactly
        // match the remaining space
        let has_single_flex_child = if measure_mode_main_dim == Some(MeasureMode::Exactly) {
            self.children(node)
                .iter()
                .filter(|&&child| {
                    self.resolve_flex_grow(child) > 0.0 && self.resolve_flex_shrink(child) > 0.0
                })
                .next()
                .is_some()
        } else {
            false
        };

        let mut total_outer_flex_basis = r32(0.0);

        let self_flex_direction = self.style(node).flex_direction;
        let self_overflow = self.style(node).overflow;

        let aligns = self
            .children(node)
            .into_iter()
            .map(|&child| self.align_item(node, child))
            .collect::<Vec<_>>();

        // STEP 3: DETERMINE FLEX BASIS FOR EACH ITEM
        let current_gen = self.current_generation();
        for (idx, parent_align) in (0..self.children(node).len()).zip(aligns) {
            let child = self.child(node, idx);
            if self.style(child).display == Display::None {
                self.zero_layout_recursively(child);
                self.new_layout[child.0] = true;
                self.mark_dirty(child);
                continue;
            }

            self.resolve_dimensions(child);

            if perform_layout {
                // Set the initial position (relative to the parent).
                let child_direction = self.style(child).direction.resolve(direction);
                let child_style = self.style(child);
                // unused: we know its getting mutated, we're doing layout now
                let _ = self.layout_mut(child).set_position(
                    child_style,
                    child_direction,
                    available_inner_main_dim,
                    available_inner_cross_dim,
                    available_inner_width,
                    true,
                );
            }

            // Absolute-positioned children don't participate in flex layout. Add them
            // to a list that we can process later.
            if self.style(child).position_type == PositionType::Absolute {
                // Store a private linked list of absolutely positioned children
                // so that we can efficiently traverse them later.
                self.add_absolute_child(child);
            } else if has_single_flex_child {
                self.layout_mut(child).computed_flex_basis_generation = current_gen;
                self.layout_mut(child).computed_flex_basis = Some(r32(0.0));
            } else {
                self.compute_flex_basis_from_parent(
                    child,
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

            if let Some(basis) = self.layout(child).computed_flex_basis {
                total_outer_flex_basis += basis;
            }

            total_outer_flex_basis += self
                .style(child)
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

        // Accumulated cross dimensions of all lines so far.
        let mut total_line_cross_dim = r32(0.0);

        // Max main dimension of all the lines.
        let mut max_line_main_dim = r32(0.0);

        let mut start_of_line_index = 0;
        let mut end_of_line_index = 0;
        let mut line_count = 0;

        while end_of_line_index < self.children(node).len() {
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
            for idx in start_of_line_index..self.children(node).len() {
                let child = self.child(node, idx);
                if self.style(child).display == Display::None {
                    continue;
                }

                self.lines[child.0] = line_count;

                if self.style(child).position_type != PositionType::Absolute {
                    let child_margin_main_axis = self
                        .style(child)
                        .margin
                        .for_axis(main_axis, available_inner_width);

                    let flex_basis_with_max_constraints = self.style(child).max_dimensions
                        [main_axis.dimension()]
                        .resolve(main_axis_parent_size)
                        .min(self.layout(child).computed_flex_basis);

                    let flex_basis_with_min_and_max_constraints = self.style(child).min_dimensions
                        [main_axis.dimension()]
                        .resolve(main_axis_parent_size)
                        .max(flex_basis_with_max_constraints);

                    // If this is a multi-line flow and this item pushes us over the available size,
                    // we've hit the end of the current line. Break out of the loop and lay out the
                    // current line.
                    if size_consumed_on_current_line_including_min_constraint
                        + flex_basis_with_min_and_max_constraints.unwrap()
                        + child_margin_main_axis > available_inner_main_dim
                        && is_node_flex_wrap && line_count > 0
                    {
                        break;
                    }

                    size_consumed_on_current_line_including_min_constraint +=
                        flex_basis_with_min_and_max_constraints.unwrap() + child_margin_main_axis;
                    size_consumed_on_current_line +=
                        flex_basis_with_min_and_max_constraints.unwrap() + child_margin_main_axis;
                    items_on_line += 1;

                    if self.is_flex(child) {
                        total_flex_grow_factors += self.resolve_flex_grow(child);

                        // Unlike the grow factor, the shrink factor is scaled relative to the child dimension.
                        total_flex_shrink_scaled_factors += -self.resolve_flex_shrink(child)
                            * self.layout(child).computed_flex_basis.unwrap();
                    }
                }
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

            let original_remaining_free_space = remaining_free_space;
            let mut delta_free_space = r32(0.0);

            // Maintain a linked list of the child node indices that can shrink and/or grow.
            let first_relative_child_idx = Some(0usize);
            let mut curr_rel_child_idx = Some(0usize);

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
                    let curr_rel_child = self.child(node, idx);
                    let child_flex_basis = self.style(curr_rel_child).max_dimensions
                        [main_axis.dimension()]
                        .resolve(main_axis_parent_size)
                        .min(
                            self.style(curr_rel_child).min_dimensions[main_axis.dimension()]
                                .resolve(main_axis_parent_size)
                                .max(self.layout(curr_rel_child).computed_flex_basis),
                        );

                    if remaining_free_space < 0.0 {
                        flex_shrink_scaled_factor =
                            -self.resolve_flex_shrink(curr_rel_child) * child_flex_basis.unwrap();

                        // Is this child able to shrink?
                        if flex_shrink_scaled_factor != 0.0 {
                            base_main_size = child_flex_basis.unwrap()
                                + remaining_free_space / total_flex_shrink_scaled_factors
                                    * flex_shrink_scaled_factor;
                            bound_main_size = self.bound_axis(
                                curr_rel_child,
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
                        flex_grow_factor = self.resolve_flex_grow(curr_rel_child);

                        // Is this child able to grow?
                        if flex_grow_factor != 0.0 {
                            base_main_size = child_flex_basis.unwrap()
                                + remaining_free_space / total_flex_grow_factors * flex_grow_factor;
                            bound_main_size = self.bound_axis(
                                curr_rel_child,
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

                while let Some(idx) = curr_rel_child_idx {
                    let curr_rel_child = self.child(node, idx);
                    child_flex_basis = self.style(curr_rel_child).max_dimensions
                        [main_axis.dimension()]
                        .resolve(main_axis_parent_size)
                        .min(
                            self.style(curr_rel_child).min_dimensions[main_axis.dimension()]
                                .resolve(main_axis_parent_size)
                                .max(self.layout(curr_rel_child).computed_flex_basis),
                        );
                    let mut updated_main_size = child_flex_basis.unwrap();

                    if remaining_free_space < 0.0 {
                        flex_shrink_scaled_factor =
                            -self.resolve_flex_shrink(curr_rel_child) * child_flex_basis.unwrap();
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

                            updated_main_size = self.bound_axis(
                                curr_rel_child,
                                main_axis,
                                child_size,
                                available_inner_main_dim,
                                available_inner_width,
                            );
                        }
                    } else if remaining_free_space > 0.0 {
                        flex_grow_factor = self.resolve_flex_grow(curr_rel_child);

                        // Is this child able to grow?
                        if flex_grow_factor != 0.0 {
                            updated_main_size = self.bound_axis(
                                curr_rel_child,
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

                    let margin_main = self
                        .style(curr_rel_child)
                        .margin
                        .for_axis(main_axis, available_inner_width);
                    let margin_cross = self
                        .style(curr_rel_child)
                        .margin
                        .for_axis(cross_axis, available_inner_width);

                    let mut child_cross_size;
                    let mut child_main_size = updated_main_size;
                    let mut child_cross_measure_mode;
                    let mut child_main_measure_mode = Some(MeasureMode::Exactly);

                    if let Some(ar) = self.style(curr_rel_child).aspect_ratio {
                        child_cross_size = if is_main_axis_row {
                            (child_main_size - margin_main) / ar
                        } else {
                            (child_main_size - margin_main) * ar
                        };
                        child_cross_measure_mode = Some(MeasureMode::Exactly);

                        child_cross_size += margin_cross;
                    } else if !available_inner_cross_dim.is_nan()
                        && !self.is_style_dim_defined(
                            curr_rel_child,
                            cross_axis,
                            available_inner_cross_dim,
                        )
                        && measure_mode_cross_dim == Some(MeasureMode::Exactly)
                        && !(is_node_flex_wrap && flex_basis_overflows)
                        && self.align_item(node, curr_rel_child) == Align::Stretch
                    {
                        child_cross_size = available_inner_cross_dim;
                        child_cross_measure_mode = Some(MeasureMode::Exactly);
                    } else if !self.is_style_dim_defined(
                        curr_rel_child,
                        cross_axis,
                        available_inner_cross_dim,
                    ) {
                        child_cross_size = available_inner_cross_dim;
                        child_cross_measure_mode = if child_cross_size.is_nan() {
                            None
                        } else {
                            Some(MeasureMode::AtMost)
                        };
                    } else {
                        child_cross_size = self.resolved(curr_rel_child)[cross_axis.dimension()]
                            .map(|s| {
                                s.resolve(available_inner_cross_dim)
                                    .map(|s| s + margin_cross)
                            })
                            .unwrap()
                            .unwrap();

                        let is_loose_percentage_measurement =
                            match self.resolved(curr_rel_child)[cross_axis.dimension()] {
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

                    let (child_main_size, child_main_measure_mode) = self
                        .constrained_max_size_for_mode(
                            curr_rel_child,
                            main_axis,
                            available_inner_main_dim,
                            available_inner_width,
                            child_main_measure_mode,
                            child_main_size,
                        );

                    let (child_cross_size, child_cross_measure_mode) = self
                        .constrained_max_size_for_mode(
                            curr_rel_child,
                            cross_axis,
                            available_inner_cross_dim,
                            available_inner_width,
                            child_cross_measure_mode,
                            child_cross_size,
                        );

                    let requires_stretch_layout = !self.is_style_dim_defined(
                        curr_rel_child,
                        cross_axis,
                        available_inner_cross_dim,
                    )
                        && self.align_item(node, curr_rel_child) == Align::Stretch;

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
                    self.layout_node_internal(
                        curr_rel_child,
                        child_width,
                        child_height,
                        direction,
                        child_width_measure_mode,
                        child_height_measure_mode,
                        available_inner_width,
                        available_inner_height,
                        r32(POINT_SCALE_FACTOR),
                        perform_layout && !requires_stretch_layout,
                        "flex",
                    );

                    let current_had_overflow = self.layout(curr_rel_child).had_overflow;
                    self.layout_mut(node).had_overflow |= current_had_overflow;
                    // TODO(anp): this is almost certainly a broken replacement for the linked list
                    let new_idx = idx + 1;
                    curr_rel_child_idx = if new_idx < self.children(node).len() {
                        Some(new_idx)
                    } else {
                        None
                    };
                }

                remaining_free_space = original_remaining_free_space + delta_free_space;
                self.layout_mut(node).had_overflow |= remaining_free_space < 0.0;
            }

            // STEP 6: MAIN-AXIS JUSTIFICATION & CROSS-AXIS SIZE DETERMINATION

            // At this point, all the children have their dimensions set in the main
            // axis.
            // Their dimensions are also set in the cross axis with the exception of
            // items
            // that are aligned "stretch". We need to compute these stretch values and
            // set the final positions.

            // If we are using "at most" rules in the main axis. Calculate the remaining space when
            // constraint by the min size defined for the main axis.

            if measure_mode_main_dim == Some(MeasureMode::AtMost) && remaining_free_space > 0.0 {
                remaining_free_space = match self.style(node).min_dimensions[main_axis.dimension()]
                    .resolve(main_axis_parent_size)
                {
                    Some(remaining) if remaining >= 0.0 => {
                        r32(0.0).max(remaining - (available_inner_main_dim - remaining_free_space))
                    }
                    _ => r32(0.0),
                };
            }

            let mut number_of_auto_margins_on_current_line = 0;
            for &child in &self.children(node)[start_of_line_index..end_of_line_index] {
                if self.style(child).position_type == PositionType::Relative {
                    if self.style(child).margin.leading_value(main_axis) == Some(Value::Auto) {
                        number_of_auto_margins_on_current_line += 1;
                    }
                    if self.style(child).margin.trailing_value(main_axis) == Some(Value::Auto) {
                        number_of_auto_margins_on_current_line += 1;
                    }
                }
            }

            if number_of_auto_margins_on_current_line == 0 {
                match justify_content {
                    Justify::Center => leading_main_dim = remaining_free_space / 2.0,
                    Justify::FlexEnd => leading_main_dim = remaining_free_space,
                    Justify::SpaceBetween => {
                        if items_on_line > 1 {
                            between_main_dim =
                                remaining_free_space.max(r32(0.0)) / (items_on_line - 1) as f32;
                        } else {
                            between_main_dim = r32(0.0);
                        }
                    }
                    Justify::SpaceAround => {
                        // Space on the edges is half of the space between elements
                        between_main_dim = remaining_free_space / items_on_line as f32;
                        leading_main_dim = between_main_dim / 2.0;
                    }
                    _ => (),
                }
            }

            let mut main_dim = leading_padding_and_border_main + leading_main_dim;
            let mut cross_dim = r32(0.0);

            for i in start_of_line_index..end_of_line_index {
                let child = self.child(node, i);
                if self.style(child).display == Display::None {
                    continue;
                }

                if let (PositionType::Absolute, Some(leading_pos), true) = (
                    self.style(child).position_type,
                    self.style(child)
                        .position
                        .leading(main_axis, available_inner_width),
                    perform_layout,
                ) {
                    // In case the child is position absolute and has left/top being
                    // defined, we override the position to whatever the user said
                    // (and margin/border).
                    let own_leading_border = self.style(node).border.leading(main_axis);
                    let child_leading_margin = self
                        .style(child)
                        .margin
                        .leading(main_axis, available_inner_width);
                    self.layout_mut(child).position.set(
                        main_axis.leading_edge(),
                        leading_pos + own_leading_border + child_leading_margin,
                    );
                } else {
                    // Now that we placed the element, we need to update the variables.
                    // We need to do that only for relative elements. Absolute elements
                    // do not take part in that phase.
                    if self.style(child).position_type == PositionType::Relative {
                        if self.style(child).margin.leading_value(main_axis) == Some(Value::Auto) {
                            main_dim += remaining_free_space
                                / number_of_auto_margins_on_current_line as f32;
                        }

                        if perform_layout {
                            self.layout_mut(child)
                                .position
                                .add(main_axis.leading_edge(), main_dim);
                        }

                        if self.style(child).margin.trailing_value(main_axis) == Some(Value::Auto) {
                            main_dim += remaining_free_space
                                / number_of_auto_margins_on_current_line as f32;
                        }

                        if can_skip_flex {
                            // If we skipped the flex step, then we can't rely on the
                            // measuredDims because
                            // they weren't computed. This means we can't call DimWithMargin.
                            main_dim += between_main_dim
                                + self
                                    .style(child)
                                    .margin
                                    .for_axis(main_axis, available_inner_width)
                                + self.layout(child).computed_flex_basis.unwrap_or(r32(0.0));
                            cross_dim = available_inner_cross_dim;
                        } else {
                            // The main dimension is the sum of all the elements dimension plus the spacing.
                            main_dim += between_main_dim
                                + self.dim_with_margin(child, main_axis, available_inner_width);

                            // The cross dimension is the max of the elements dimension since
                            // there can only be one element in that cross dimension.
                            cross_dim = cross_dim.max(self.dim_with_margin(
                                child,
                                cross_axis,
                                available_inner_width,
                            ));
                        }
                    } else if perform_layout {
                        let own_leading_border = self.style(node).border.leading(main_axis);
                        self.layout_mut(child).position.set(
                            main_axis.leading_edge(),
                            own_leading_border + leading_main_dim,
                        );
                    }
                }
            }

            main_dim += trailing_padding_and_border_main;

            let mut container_cross_axis = available_inner_cross_dim;
            if measure_mode_cross_dim == None || measure_mode_cross_dim == Some(MeasureMode::AtMost)
            {
                // Compute the cross axis from the max cross dimension of the children.
                container_cross_axis = self.bound_axis(
                    node,
                    cross_axis,
                    cross_dim + padding_and_border_axis_cross,
                    cross_axis_parent_size,
                    parent_width,
                ) - padding_and_border_axis_cross;
            }

            // If there's no flex wrap, the cross dimension is defined by the container.
            if !is_node_flex_wrap && measure_mode_cross_dim == Some(MeasureMode::Exactly) {
                cross_dim = available_inner_cross_dim;
            }

            // Clamp to the min/max size specified on the container.
            cross_dim = self.bound_axis(
                node,
                cross_axis,
                cross_dim + padding_and_border_axis_cross,
                cross_axis_parent_size,
                parent_width,
            ) - padding_and_border_axis_cross;

            // STEP 7: CROSS-AXIS ALIGNMENT
            // We can skip child alignment if we're just measuring the container.
            if perform_layout {
                for i in start_of_line_index..end_of_line_index {
                    let child = self.child(node, i);
                    if self.style(child).display == Display::None {
                        continue;
                    }

                    if self.style(child).position_type == PositionType::Absolute {
                        // If the child is absolutely positioned and has a top/left/bottom/right
                        // set, override all the previously computed positions to set it correctly.
                        if let Some(child_leading_pos) = self
                            .style(child)
                            .position
                            .leading(cross_axis, available_inner_cross_dim)
                        {
                            let own_leading_border = self.style(node).border.leading(cross_axis);
                            let child_leading_margin = self
                                .style(child)
                                .margin
                                .leading(cross_axis, available_inner_width);
                            let _ = self.layout_mut(child).position.set(
                                cross_axis.leading_edge(),
                                child_leading_pos + own_leading_border + child_leading_margin,
                            );
                        } else if self.style(child).position[cross_axis.leading_edge()].is_none() {
                            // If leading position is not defined or calculations result in Nan,
                            // default to border + margin
                            let own_leading_border = self.style(node).border.leading(cross_axis);
                            let child_leading_margin = self
                                .style(child)
                                .margin
                                .leading(cross_axis, available_inner_width);
                            let _ = self.layout_mut(child).position.set(
                                cross_axis.leading_edge(),
                                own_leading_border + child_leading_margin,
                            );
                        }
                    } else {
                        let mut leading_cross_dim = leading_padding_and_border_cross;

                        // For a relative children, we're either using align_items (parent) or
                        // align_self (child) in order to determine the position in the cross
                        // axis
                        let align_item = self.align_item(node, child);

                        // If the child uses align stretch, we need to lay it out one more
                        // time, this time
                        // forcing the cross-axis size to be the computed cross size for the
                        // current line.
                        let stretch_aligned = align_item == Align::Stretch;

                        let child_leading_margin =
                            self.style(child).margin.leading_value(cross_axis);
                        let child_trailing_margin =
                            self.style(child).margin.trailing_value(cross_axis);

                        if stretch_aligned
                            && child_leading_margin != Some(Value::Auto)
                            && child_trailing_margin != Some(Value::Auto)
                        {
                            // If the child defines a definite size for its cross axis, there's
                            // no need to stretch.
                            if !self.is_style_dim_defined(
                                child,
                                cross_axis,
                                available_inner_cross_dim,
                            ) {
                                let mut child_main_size =
                                    self.layout(child).measured_dimensions[main_axis.dimension()];

                                let mut child_cross_size =
                                    if let Some(ar) = self.style(child).aspect_ratio {
                                        self.style(child)
                                            .margin
                                            .for_axis(cross_axis, available_inner_width)
                                            + (if is_main_axis_row {
                                                child_main_size / ar
                                            } else {
                                                child_main_size * ar
                                            })
                                    } else {
                                        cross_dim
                                    };

                                // this is dumb
                                child_main_size += self
                                    .style(child)
                                    .margin
                                    .for_axis(main_axis, available_inner_width);

                                let mut child_main_measure_mode = Some(MeasureMode::Exactly);
                                let mut child_cross_measure_mode = Some(MeasureMode::Exactly);

                                let (
                                    (child_main_size, _child_main_measure_mode),
                                    (child_cross_size, _child_cross_measure_mode),
                                ) = (
                                    self.constrained_max_size_for_mode(
                                        child,
                                        main_axis,
                                        available_inner_main_dim,
                                        available_inner_width,
                                        child_main_measure_mode,
                                        child_main_size,
                                    ),
                                    self.constrained_max_size_for_mode(
                                        child,
                                        cross_axis,
                                        available_inner_cross_dim,
                                        available_inner_width,
                                        child_cross_measure_mode,
                                        child_cross_size,
                                    ),
                                );

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

                                let child_width_measure_mode = if child_width.is_nan() {
                                    None
                                } else {
                                    Some(MeasureMode::Exactly)
                                };
                                let child_height_measure_mode = if child_height.is_nan() {
                                    None
                                } else {
                                    Some(MeasureMode::Exactly)
                                };

                                self.layout_node_internal(
                                    child,
                                    child_width,
                                    child_height,
                                    direction,
                                    child_width_measure_mode,
                                    child_height_measure_mode,
                                    available_inner_width,
                                    available_inner_height,
                                    r32(POINT_SCALE_FACTOR),
                                    true,
                                    "stretch",
                                );
                            }
                        } else {
                            let remaining_cross_dim = container_cross_axis
                                - self.dim_with_margin(child, cross_axis, available_inner_width);

                            let child_leading_margin =
                                self.style(child).margin.leading_value(cross_axis);
                            let child_trailing_margin =
                                self.style(child).margin.trailing_value(cross_axis);

                            leading_cross_dim +=
                                match (child_leading_margin, child_trailing_margin, align_item) {
                                    (Some(Auto), Some(Auto), _) => {
                                        (remaining_cross_dim / 2.0).max(r32(0.0))
                                    }
                                    (_, Some(Auto), _) => r32(0.0),
                                    (Some(Auto), _, _) => r32(0.0).max(remaining_cross_dim),
                                    (_, _, Align::FlexStart) => r32(0.0),
                                    (_, _, Align::Center) => remaining_cross_dim / 2.0,
                                    _ => remaining_cross_dim,
                                };
                        }
                        // And we apply the position
                        let _ = self.layout_mut(child).position.set(
                            cross_axis.leading_edge(),
                            total_line_cross_dim + leading_cross_dim,
                        );
                    }
                }
            }

            total_line_cross_dim += cross_dim;
            max_line_main_dim = max_line_main_dim.max(main_dim);

            line_count += 1;
            start_of_line_index = end_of_line_index;
        }

        // STEP 8: MULTI-LINE CONTENT ALIGNMENT
        if perform_layout && (line_count > 1 || self.is_baseline_layout(node)) {
            let remaining_align_content_dim = available_inner_cross_dim - total_line_cross_dim;

            let mut cross_dim_lead = r32(0.0);
            let mut current_lead = leading_padding_and_border_cross;

            match self.style(node).align_content {
                Align::FlexEnd => current_lead += remaining_align_content_dim,
                Align::Center => current_lead += remaining_align_content_dim / 2.0,
                Align::Stretch => if available_inner_cross_dim > total_line_cross_dim {
                    cross_dim_lead = remaining_align_content_dim / line_count as f32;
                },
                Align::SpaceAround => if available_inner_cross_dim > total_line_cross_dim {
                    current_lead += remaining_align_content_dim / (2.0 * line_count as f32);
                    if line_count > 1 {
                        cross_dim_lead = remaining_align_content_dim / line_count as f32;
                    }
                } else {
                    current_lead += remaining_align_content_dim / 2.0;
                },
                Align::SpaceBetween => {
                    if available_inner_cross_dim > total_line_cross_dim && line_count > 1 {
                        cross_dim_lead = remaining_align_content_dim / (line_count as f32 - 1.0);
                    }
                }
                _ => (),
            }

            let mut end_index = 0;
            for i in 0..line_count {
                let start_index = end_index;

                // compute the line's height and find the endIndex
                let mut line_height = r32(0.0);
                let mut max_ascent_for_current_line = r32(0.0);
                let mut max_descent_for_current_line = r32(0.0);

                for ii in start_index..self.children(node).len() {
                    let child = self.child(node, ii);
                    end_index = ii;

                    if self.style(child).display == Display::None {
                        continue;
                    }

                    if self.style(child).position_type == PositionType::Relative {
                        if self.lines[child.0] != i {
                            break;
                        }

                        if self.layout(child).is_dim_defined(cross_axis) {
                            line_height = line_height.max(
                                self.layout(child).measured_dimensions[cross_axis.dimension()]
                                    + self
                                        .style(child)
                                        .margin
                                        .for_axis(cross_axis, available_inner_width),
                            );
                        }

                        if self.align_item(node, child) == Align::Baseline {
                            let ascent = self.baseline(child)
                                + self
                                    .style(child)
                                    .margin
                                    .leading(FlexDirection::Column, available_inner_width);
                            let descent = self.layout(child).measured_dimensions.height
                                + self
                                    .style(child)
                                    .margin
                                    .for_axis(FlexDirection::Column, available_inner_width)
                                - ascent;
                            max_ascent_for_current_line = max_ascent_for_current_line.max(ascent);
                            max_descent_for_current_line =
                                max_descent_for_current_line.max(descent);
                            line_height = line_height
                                .max(max_ascent_for_current_line + max_descent_for_current_line);
                        }
                    }
                }
                line_height += cross_dim_lead;

                if perform_layout {
                    for ii in start_index..end_index {
                        let child = self.child(node, ii);
                        if self.style(child).display == Display::None {
                            continue;
                        }
                        if self.style(child).position_type == PositionType::Relative {
                            match self.align_item(node, child) {
                                Align::FlexStart => {
                                    let child_leading_margin = self
                                        .style(child)
                                        .margin
                                        .leading(cross_axis, available_inner_width);
                                    self.layout_mut(child).position.set(
                                        cross_axis.leading_edge(),
                                        current_lead + child_leading_margin,
                                    );
                                }
                                Align::FlexEnd => {
                                    let child_trailing_margin = self
                                        .style(child)
                                        .margin
                                        .trailing(cross_axis, available_inner_width);
                                    let child_cross_measured = self
                                        .layout(child)
                                        .measured_dimensions[cross_axis.dimension()];

                                    self.layout_mut(child).position.set(
                                        cross_axis.leading_edge(),
                                        current_lead + line_height
                                            - child_trailing_margin
                                            - child_cross_measured,
                                    );
                                }
                                Align::Center => {
                                    let mut child_height = self.layout(child).measured_dimensions
                                        [cross_axis.dimension()];
                                    self.layout_mut(child).position.set(
                                        cross_axis.leading_edge(),
                                        current_lead + (line_height - child_height) / 2.0,
                                    );
                                }
                                Align::Stretch => {
                                    let child_leading_margin = self
                                        .style(child)
                                        .margin
                                        .leading(cross_axis, available_inner_width);
                                    self.layout_mut(child).position.set(
                                        cross_axis.leading_edge(),
                                        current_lead + child_leading_margin,
                                    );

                                    // Remeasure child with the line height as it as been only measured with the
                                    // parents height yet.
                                    if !self.is_style_dim_defined(
                                        child,
                                        cross_axis,
                                        available_inner_cross_dim,
                                    ) {
                                        let child_width = if is_main_axis_row {
                                            self.layout(child).measured_dimensions.width
                                                + self
                                                    .style(child)
                                                    .margin
                                                    .for_axis(main_axis, available_inner_width)
                                        } else {
                                            line_height
                                        };

                                        let child_height = if !is_main_axis_row {
                                            self.layout(child).measured_dimensions.height
                                                + self
                                                    .style(child)
                                                    .margin
                                                    .for_axis(cross_axis, available_inner_width)
                                        } else {
                                            line_height
                                        };

                                        if !child_width
                                            .approx_eq(self.layout(child).measured_dimensions.width)
                                            && child_height.approx_eq(
                                                self.layout(child).measured_dimensions.height,
                                            ) {
                                            self.layout_node_internal(
                                                child,
                                                child_width,
                                                child_height,
                                                direction,
                                                Some(MeasureMode::Exactly),
                                                Some(MeasureMode::Exactly),
                                                available_inner_width,
                                                available_inner_height,
                                                r32(POINT_SCALE_FACTOR),
                                                true,
                                                "multiline-stretch",
                                            );
                                        }
                                    }
                                }
                                Align::Baseline => {
                                    let child_baseline = self.baseline(child);
                                    let child_leading_position = self
                                        .style(child)
                                        .position
                                        .leading(FlexDirection::Column, available_inner_cross_dim)
                                        .unwrap();
                                    self.layout_mut(child).position.set(
                                        PhysicalEdge::Top,
                                        current_lead + max_ascent_for_current_line - child_baseline
                                            + child_leading_position,
                                    );
                                }
                                _ => (),
                            }
                        }
                    }
                }

                current_lead += line_height;
            }
        }

        // STEP 9: COMPUTING FINAL DIMENSIONS
        let measured = MeasuredDimensions {
            width: self.bound_axis(
                node,
                FlexDirection::Row,
                available_width - margin_axis_row,
                parent_width,
                parent_width,
            ),
            height: self.bound_axis(
                node,
                FlexDirection::Column,
                available_height - margin_axis_column,
                parent_height,
                parent_width,
            ),
        };

        {
            self.layout_mut(node).measured_dimensions = measured;
        }

        // If the user didn't specify a width or height for the node, set the
        // dimensions based on the children.
        if measure_mode_main_dim.is_none()
            || (self.style(node).overflow != Overflow::Scroll
                && measure_mode_main_dim == Some(MeasureMode::AtMost))
        {
            // Clamp the size to the min/max size, if specified, and make sure it
            // doesn't go below the padding and border amount.
            let new_dim = self.bound_axis(
                node,
                main_axis,
                max_line_main_dim,
                main_axis_parent_size,
                parent_width,
            );
            self.layout_mut(node).measured_dimensions[main_axis.dimension()] = new_dim;
        } else if measure_mode_main_dim == Some(MeasureMode::AtMost)
            && self.style(node).overflow == Overflow::Scroll
        {
            self.layout_mut(node).measured_dimensions[main_axis.dimension()] =
                (available_inner_main_dim + padding_and_border_axis_main)
                    .min(self.bound_axis_within_min_and_max(
                        node,
                        main_axis,
                        max_line_main_dim,
                        main_axis_parent_size,
                    ))
                    .max(padding_and_border_axis_main);
        }

        if measure_mode_cross_dim.is_none()
            || (self.style(node).overflow != Overflow::Scroll
                && measure_mode_cross_dim == Some(MeasureMode::AtMost))
        {
            // Clamp the size to the min/max size, if specified, and make sure it
            // doesn't go below the padding and border amount.
            self.layout_mut(node).measured_dimensions[cross_axis.dimension()] = self.bound_axis(
                node,
                cross_axis,
                total_line_cross_dim + padding_and_border_axis_cross,
                cross_axis_parent_size,
                parent_width,
            );
        } else if measure_mode_cross_dim == Some(MeasureMode::AtMost)
            && self.style(node).overflow == Overflow::Scroll
        {
            self.layout_mut(node).measured_dimensions[cross_axis.dimension()] =
                (available_inner_cross_dim + padding_and_border_axis_cross)
                    .max(self.bound_axis_within_min_and_max(
                        node,
                        cross_axis,
                        total_line_cross_dim + padding_and_border_axis_cross,
                        cross_axis_parent_size,
                    ))
                    .max(padding_and_border_axis_cross);
        }

        // As we only wrapped in normal direction yet, we need to reverse the positions on wrap-reverse.
        if perform_layout && self.style(node).flex_wrap == Wrap::WrapReverse {
            let thing_ill_name_above =
                self.layout(node).measured_dimensions[cross_axis.dimension()];

            for idx in 0..self.children(node).len() {
                let child = self.child(node, idx);
                if self.style(child).position_type == PositionType::Relative {
                    let child_cross_position =
                        self.layout(child).position[cross_axis.leading_edge()];
                    let child_cross_measured =
                        self.layout(child).measured_dimensions[cross_axis.dimension()];
                    self.layout_mut(child).position.set(
                        cross_axis.leading_edge(),
                        thing_ill_name_above - child_cross_position - child_cross_measured,
                    );
                }
            }
        }

        if perform_layout {
            // STEP 10: SIZING AND POSITIONING ABSOLUTE CHILDREN

            let abs_children = self.abs_children.clone();
            for current_absolute_child in abs_children {
                self.absolute_layout_child(
                    node,
                    current_absolute_child,
                    available_inner_width,
                    if is_main_axis_row {
                        measure_mode_main_dim
                    } else {
                        measure_mode_cross_dim
                    },
                    available_inner_height,
                    direction,
                );
            }

            // STEP 11: SETTING TRAILING POSITIONS FOR CHILDREN
            let needs_main_trailing_pos =
                main_axis == FlexDirection::RowReverse || main_axis == FlexDirection::ColumnReverse;
            let needs_cross_trailing_pos = cross_axis == FlexDirection::RowReverse
                || cross_axis == FlexDirection::ColumnReverse;

            // Set trailing position if necessary.
            if needs_main_trailing_pos || needs_cross_trailing_pos {
                for idx in 0..self.children(node).len() {
                    let child = self.child(node, idx);
                    if self.style(child).display == Display::None {
                        continue;
                    }

                    let mut set_child_trailing_position = |axis: FlexDirection| {
                        let size = self.layout(child).measured_dimensions[axis.dimension()];
                        let new_pos = self.layout(node).measured_dimensions[axis.dimension()]
                            - size
                            - self.layout(child).position[axis.leading_edge()];
                        self.layout_mut(child)
                            .position
                            .set(axis.trailing_edge(), new_pos);
                    };

                    if needs_main_trailing_pos {
                        set_child_trailing_position(main_axis);
                    }

                    if needs_cross_trailing_pos {
                        set_child_trailing_position(cross_axis);
                    }
                }
            }
        }
    }

    fn absolute_layout_child(
        &mut self,
        node: Handle,
        child: Handle,
        width: R32,
        width_mode: Option<MeasureMode>,
        height: R32,
        direction: Direction,
    ) -> () {
        let main_axis = self.style(node).flex_direction.resolve_direction(direction);
        let cross_axis = main_axis.cross(direction);
        let is_main_axis_row = main_axis.is_row();
        let mut child_width = None;
        let mut child_height = None;
        let mut child_width_measure_mode;
        let child_height_measure_mode;
        let margin_row = self.style(child).margin.for_axis(FlexDirection::Row, width);
        let margin_column = self
            .style(child)
            .margin
            .for_axis(FlexDirection::Column, width);
        if self.is_style_dim_defined(child, FlexDirection::Row, width) {
            child_width = self
                .resolved(child)
                .width
                .resolve(width)
                .map(|w| w + margin_row);
        } else {
            // If the child doesn't have a specified width, compute the width based
            // on the left/right
            // offsets if they're defined.
            if let (Some(leading_pos), Some(trailing_pos)) = (
                self.style(child)
                    .position
                    .leading(FlexDirection::Row, width),
                self.style(child)
                    .position
                    .trailing(FlexDirection::Row, width),
            ) {
                let new_child_width = self.layout(node).measured_dimensions.width
                    - (self.style(node).border.leading(FlexDirection::Row)
                        + self.style(node).border.trailing(FlexDirection::Row))
                    - (leading_pos + trailing_pos);
                child_width =
                    Some(self.bound_axis(child, FlexDirection::Row, new_child_width, width, width));
            };
        };
        if self.is_style_dim_defined(child, FlexDirection::Column, height) {
            child_height = self
                .resolved(child)
                .height
                .resolve(height)
                .map(|h| h + margin_column);
        } else {
            // If the child doesn't have a specified height, compute the height
            // based on the top/bottom
            // offsets if they're defined.
            if let (Some(leading_pos), Some(trailing_pos)) = (
                self.style(child)
                    .position
                    .leading(FlexDirection::Column, height),
                self.style(child)
                    .position
                    .trailing(FlexDirection::Column, height),
            ) {
                let new_child_height = self.layout(node).measured_dimensions.height
                    - (self.style(node).border.leading(FlexDirection::Column)
                        + self.style(node).border.trailing(FlexDirection::Column))
                    - (leading_pos + trailing_pos);

                child_height = Some(self.bound_axis(
                    child,
                    FlexDirection::Column,
                    new_child_height,
                    height,
                    width,
                ));
            };
        };

        // Exactly one dimension needs to be defined for us to be able to do aspect ratio
        // calculation. One dimension being the anchor and the other being flexible.
        match (child_width, child_height, self.style(child).aspect_ratio) {
            (Some(child_width), None, Some(ar)) => {
                child_height = Some(margin_column + (child_width - margin_row) / ar);
            }
            (None, Some(child_height), Some(ar)) => {
                child_width = Some(margin_row + (child_height - margin_column) * ar);
            }
            _ => (),
        }

        // If we're still missing one or the other dimension, measure the content.
        if child_width.is_none() || child_height.is_none() {
            child_width_measure_mode = if child_width.is_none() {
                None
            } else {
                Some(MeasureMode::Exactly)
            };
            child_height_measure_mode = if child_height.is_none() {
                None
            } else {
                Some(MeasureMode::Exactly)
            };

            // If the size of the parent is defined then try to constrain the absolute child to that size
            // as well. This allows text within the absolute child to wrap to the size of its parent.
            // This is the same behavior as many browsers implement.
            if !is_main_axis_row && child_width.is_none() && width_mode != None && width > 0.0 {
                child_width = Some(width);
                child_width_measure_mode = Some(MeasureMode::AtMost);
            };

            self.layout_node_internal(
                child,
                child_width.unwrap(),
                child_height.unwrap(),
                direction,
                child_width_measure_mode,
                child_height_measure_mode,
                child_width.unwrap(),
                child_height.unwrap(),
                r32(POINT_SCALE_FACTOR),
                false,
                "abs-measure",
            );

            child_width = Some(
                self.layout(child).measured_dimensions.width
                    + self.style(child).margin.for_axis(FlexDirection::Row, width),
            );
            child_height = Some(
                self.layout(child).measured_dimensions.height
                    + self
                        .style(child)
                        .margin
                        .for_axis(FlexDirection::Column, width),
            );
        };

        self.layout_node_internal(
            child,
            child_width.unwrap(),
            child_height.unwrap(),
            direction,
            Some(MeasureMode::Exactly),
            Some(MeasureMode::Exactly),
            child_width.unwrap(),
            child_height.unwrap(),
            r32(POINT_SCALE_FACTOR),
            true,
            "abs-layout",
        );

        if let (Some(trailing_pos), None) = (
            self.style(child)
                .position
                .trailing(main_axis, if is_main_axis_row { width } else { height }),
            self.style(child)
                .position
                .leading(main_axis, if is_main_axis_row { width } else { height }),
        ) {
            let new_pos = self.layout(node).measured_dimensions[main_axis.dimension()]
                - self.layout(child).measured_dimensions[main_axis.dimension()]
                - self.style(node).border.trailing(main_axis)
                - self.style(child).margin.trailing(main_axis, width)
                - trailing_pos;
            self.layout_mut(child)
                .position
                .set(main_axis.leading_edge(), new_pos);
        } else if let (None, Justify::Center) = (
            self.style(child)
                .position
                .leading(main_axis, if is_main_axis_row { width } else { height }),
            self.style(node).justify_content,
        ) {
            let new_pos = (self.layout(node).measured_dimensions[main_axis.dimension()]
                - self.layout(child).measured_dimensions[main_axis.dimension()])
                / 2.0;
            self.layout_mut(child)
                .position
                .set(main_axis.leading_edge(), new_pos);
        } else {
            if let (None, Justify::FlexEnd) = (
                self.style(child)
                    .position
                    .leading(main_axis, if is_main_axis_row { width } else { height }),
                self.style(node).justify_content,
            ) {
                let new_pos = self.layout(node).measured_dimensions[main_axis.dimension()]
                    - self.layout(child).measured_dimensions[main_axis.dimension()];
                self.layout_mut(child)
                    .position
                    .set(main_axis.leading_edge(), new_pos);
            };
        };

        if let (Some(trailing_cross_pos), None) = (
            self.style(child)
                .position
                .trailing(cross_axis, if is_main_axis_row { height } else { width }),
            self.style(child)
                .position
                .leading(cross_axis, if is_main_axis_row { height } else { width }),
        ) {
            let new_pos = self.layout(node).measured_dimensions[cross_axis.dimension()]
                - self.layout(child).measured_dimensions[cross_axis.dimension()]
                - self.style(node).border.trailing(cross_axis)
                - self.style(node).margin.trailing(cross_axis, width)
                - trailing_cross_pos;
            self.layout_mut(child)
                .position
                .set(cross_axis.leading_edge(), new_pos);
        } else if let (None, Align::Center) = (
            self.style(child)
                .position
                .leading(cross_axis, if is_main_axis_row { height } else { width }),
            self.align_item(node, child),
        ) {
            let own_cross_measured = self.layout(node).measured_dimensions[cross_axis.dimension()];
            let prev = self.layout(child).measured_dimensions[cross_axis.dimension()];
            self.layout_mut(child)
                .position
                .set(cross_axis.leading_edge(), (own_cross_measured - prev) / 2.0);
        } else {
            if self
                .style(child)
                .position
                .leading(cross_axis, if is_main_axis_row { height } else { width })
                .is_none()
                && (self.align_item(node, child) == Align::FlexEnd)
                    ^ (self.style(node).flex_wrap == Wrap::WrapReverse)
            {
                let new_pos = self.layout(node).measured_dimensions[cross_axis.dimension()]
                    - self.layout(child).measured_dimensions[cross_axis.dimension()];
                self.layout_mut(child)
                    .position
                    .set(cross_axis.leading_edge(), new_pos);
            };
        };
    }

    fn align_item(&self, node: Handle, child: Handle) -> Align {
        let align: Align = if self.style(child).align_self == Align::Auto {
            self.style(node).align_items
        } else {
            self.style(child).align_self
        };

        if align == Align::Baseline && self.style(node).flex_direction.is_column() {
            return Align::FlexStart;
        };

        return align;
    }

    fn baseline(&mut self, node: Handle) -> R32 {
        if let Some(baseline_fn) = self.baseline_fn(node) {
            baseline_fn(
                (self, node),
                self.layout(node).measured_dimensions.width,
                self.layout(node).measured_dimensions.height,
            )
        } else {
            let mut baseline_child = None;

            {
                let mut kerzy = || {
                    // TODO(anp): audit this, it seems a little too clean even though i still
                    // don't understand what it does
                    for &child in self.children(node) {
                        if self.lines[child.0] > 0 {
                            break;
                        };

                        if self.style(child).position_type == PositionType::Absolute {
                            continue;
                        }

                        if self.align_item(node, child) == Align::Baseline {
                            baseline_child = Some(child);
                            break;
                        }

                        if baseline_child.is_none() {
                            baseline_child = Some(child);
                        }
                    }
                };
                kerzy();
            }

            if let Some(baseline_child) = baseline_child {
                self.baseline(baseline_child)
                    + self.layout(baseline_child).position[PhysicalEdge::Top]
            } else {
                self.layout(node).measured_dimensions.height
            }
        }
    }

    fn is_baseline_layout(&self, node: Handle) -> bool {
        if self.style(node).flex_direction.is_column() {
            false
        } else if self.style(node).align_items == Align::Baseline {
            true
        } else {
            for &child in self.children(node) {
                if self.style(child).position_type == PositionType::Relative
                    && self.style(child).align_self == Align::Baseline
                {
                    return true;
                }
            }

            false
        }
    }

    fn is_flex(&self, node: Handle) -> bool {
        self.style(node).position_type == PositionType::Relative
            && (self.resolve_flex_grow(node) != 0.0 || self.resolve_flex_shrink(node) != 0.0)
    }

    fn compute_flex_basis_from_parent(
        &mut self,
        node: Handle,
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
        let current_gen = self.current_generation();
        let main_axis = parent_flex_direction.resolve_direction(direction);
        let is_main_axis_row = main_axis.is_row();
        let _main_axis_size = if is_main_axis_row { width } else { height };
        let main_axis_parent_size = if is_main_axis_row {
            parent_width
        } else {
            parent_height
        };
        let resolved_flex_basis = self.style(node).resolve_flex_basis(main_axis_parent_size);

        let is_row_style_dim_defined =
            self.is_style_dim_defined(node, FlexDirection::Row, parent_width);
        let is_column_style_dim_defined =
            self.is_style_dim_defined(node, FlexDirection::Column, parent_height);
        if let Some(resolved_flex_basis) = resolved_flex_basis {
            if self.layout(node).computed_flex_basis.is_none()
                && self.layout_mut(node).computed_flex_basis_generation != current_gen
            {
                self.layout_mut(node).computed_flex_basis = Some(
                    resolved_flex_basis.max(
                        self.style(node)
                            .padding_and_border_for_axis(main_axis, parent_width),
                    ),
                );
            };
        } else if is_main_axis_row && is_row_style_dim_defined {
            // The width is definite, so use that as the flex basis.
            self.layout_mut(node).computed_flex_basis = self
                .resolved(node)
                .width
                .unwrap_or_else(Default::default)
                .resolve(parent_width)
                .map(|b| {
                    b.max(
                        self.style(node)
                            .padding_and_border_for_axis(FlexDirection::Row, parent_width),
                    )
                });
        } else if !is_main_axis_row && is_column_style_dim_defined {
            // The height is definite, so use that as the flex basis.
            self.layout_mut(node).computed_flex_basis = self
                .resolved(node)
                .height
                .unwrap_or_else(Default::default)
                .resolve(parent_height)
                .map(|b| {
                    b.max(
                        self.style(node)
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
                .style(node)
                .margin
                .for_axis(FlexDirection::Row, parent_width);
            let margin_column = self
                .style(node)
                .margin
                .for_axis(FlexDirection::Column, parent_width);
            if is_row_style_dim_defined {
                self_width = match self
                    .resolved(node)
                    .width
                    .map(|w| w.resolve(parent_width).map(|w| w + margin_row))
                {
                    Some(Some(v)) => Some(v),
                    _ => None,
                };
                self_width_measure_mode = Some(MeasureMode::Exactly);
            };
            if is_column_style_dim_defined {
                self_height = match self
                    .resolved(node)
                    .height
                    .map(|h| h.resolve(parent_height).map(|h| h + margin_column))
                {
                    Some(Some(v)) => Some(v),
                    _ => None,
                };
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

            if let Some(ar) = self.style(node).aspect_ratio {
                if !is_main_axis_row && self_width_measure_mode == Some(MeasureMode::Exactly) {
                    self_height = self_width.map(|w| (w - margin_row) / ar);
                    self_height_measure_mode = Some(MeasureMode::Exactly);
                } else {
                    if is_main_axis_row && self_height_measure_mode == Some(MeasureMode::Exactly) {
                        self_width = self_height.map(|h| (h - margin_column) * ar);
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
                if let Some(ar) = self.style(node).aspect_ratio {
                    self_height = self_width.map(|w| (w - margin_row) / ar);
                    self_height_measure_mode = Some(MeasureMode::Exactly);
                }
            }

            let has_exact_height = !height.is_nan() && height_mode == Some(MeasureMode::Exactly);
            let self_height_stretch = self.align_item(node, node) == Align::Stretch
                && self_height_measure_mode != Some(MeasureMode::Exactly);
            if is_main_axis_row
                && !is_column_style_dim_defined
                && has_exact_height
                && self_height_stretch
            {
                self_height = Some(height);
                self_height_measure_mode = Some(MeasureMode::Exactly);
                if let Some(ar) = self.style(node).aspect_ratio {
                    self_width = self_height.map(|h| (h - margin_column) * ar);
                    self_width_measure_mode = Some(MeasureMode::Exactly);
                }
            }

            let ((self_width, self_width_measure_mode), (self_height, self_height_measure_mode)) = (
                self.constrained_max_size_for_mode(
                    node,
                    FlexDirection::Row,
                    parent_width,
                    parent_width,
                    self_width_measure_mode,
                    self_width.expect(
                        "anp hasn't had time yet to properly refactor the above into expressions",
                    ),
                ),
                self.constrained_max_size_for_mode(
                    node,
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
                node,
                self_width,
                self_height,
                direction,
                self_width_measure_mode,
                self_height_measure_mode,
                parent_width,
                parent_height,
                r32(POINT_SCALE_FACTOR),
                false,
                "measure",
            );

            self.layout_mut(node).computed_flex_basis = Some(
                self.layout(node).measured_dimensions[main_axis.dimension()].max(
                    self.style(node)
                        .padding_and_border_for_axis(main_axis, parent_width),
                ),
            );
        }

        //  FIXME(anp): reenable
        // self.layout_mut(node).computed_flex_basis_generation = Y::current_generation();
    }

    fn round_to_pixel_grid(
        &mut self,
        node: Handle,
        point_scale_factor: R32,
        absolute_left: R32,
        absolute_top: R32,
    ) {
        if point_scale_factor == 0.0 {
            return;
        }

        let node_left = self.layout(node).position.start;
        let node_top = self.layout(node).position.top;

        let node_width = match self.layout(node).dimensions.unwrap()[Dimension::Width] {
            Value::Point(nw) => nw,
            _ => panic!(
                // FIXME(anp): once we know how to do Debug for Wheel
                // "node_width had not been resolved before being rounded to pixel grid: {:?}",
                // self
            ),
        };

        let node_height = match self.layout(node).dimensions.unwrap()[Dimension::Height] {
            Value::Point(nh) => nh,
            _ => panic!(
                // FIXME(anp): once we know how to do Debug for Wheel
                // "node_height has not been resolved before being rounded to pixel grid: {:?}",
                // self
            ),
        };

        let absolute_node_left = absolute_left + node_left;
        let absolute_node_top = absolute_top + node_top;

        let absolute_node_right = absolute_node_left + node_width;
        let absolute_node_bottom = absolute_node_top + node_height;

        // If a node has a custom measure function we never want to round down its size as this could
        // lead to unwanted text truncation.
        let text_rounding = self.types[node.0] == NodeType::Text;

        self.layout_mut(node).position.set(
            PhysicalEdge::Start,
            round_value_to_pixel_grid(node_left, point_scale_factor, false, text_rounding),
        );
        self.layout_mut(node).position.set(
            PhysicalEdge::Top,
            round_value_to_pixel_grid(node_top, point_scale_factor, false, text_rounding),
        );

        // We multiply dimension by scale factor and if the result is close to the whole number, we don't
        // have any fraction
        // To verify if the result is close to whole number we want to check both floor and ceil numbers
        let has_fractional_width = !(node_width * point_scale_factor % 1.0).approx_eq(r32(0.0))
            && !(node_width * point_scale_factor % 1.0).approx_eq(r32(1.0));
        let has_fractional_height = !(node_height * point_scale_factor % 1.0).approx_eq(r32(0.0))
            && !(node_height * point_scale_factor % 1.0).approx_eq(r32(1.0));

        self.layout_mut(node).dimensions = Some(Dimensions {
            // TODO(anp): this type wrapping is silly
            width: Value::Point(
                round_value_to_pixel_grid(
                    absolute_node_right,
                    point_scale_factor,
                    text_rounding && has_fractional_width,
                    text_rounding && !has_fractional_width,
                )
                    - round_value_to_pixel_grid(
                        absolute_node_left,
                        point_scale_factor,
                        false,
                        text_rounding,
                    ),
            ),
            height: Value::Point(
                round_value_to_pixel_grid(
                    absolute_node_bottom,
                    point_scale_factor,
                    text_rounding && has_fractional_height,
                    text_rounding && !has_fractional_height,
                )
                    - round_value_to_pixel_grid(
                        absolute_node_top,
                        point_scale_factor,
                        false,
                        text_rounding,
                    ),
            ),
        });

        for idx in 0..self.children(node).len() {
            let child = self.child(node, idx);
            self.round_to_pixel_grid(
                child,
                point_scale_factor,
                absolute_node_left,
                absolute_node_top,
            );
        }
    }

    fn constrained_max_size_for_mode(
        &self,
        node: Handle,
        axis: FlexDirection,
        parent_axis_size: R32,
        parent_width: R32,
        mode: Option<MeasureMode>,
        existing_size: R32,
    ) -> (R32, Option<MeasureMode>) {
        let max_size = self.style(node).max_dimensions[axis.dimension()]
            .resolve(parent_axis_size)
            .map(|s| s + self.style(node).margin.for_axis(axis, parent_width));

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
