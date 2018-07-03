#![feature(specialization)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unknown_lints)]
#![warn(clippy)]

// TODO(anp): look at `gPrintChanges` variable in Yoga.c and add logging statements here
// TODO(anp): excise unwrap/expect/panic!
// TODO(anp): check out the inline annotations from the c code
// TODO(anp): revist raph's continuation-based layout stuff, in case you forget, june 2018 meetup at mozilla

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
    pub(crate) use super::enums::*;
    pub(crate) use super::hacks::ApproxEqHackForReals;
    pub(crate) use super::layout::{CachedMeasurement, Layout};
    pub(crate) use super::style::{Property, Style};
    pub(crate) use super::Node;
    pub(crate) use super::POINT_SCALE_FACTOR;
    pub(crate) use itertools::Itertools;
    pub(crate) use noisy_float::prelude::*;
    pub(crate) use std::ops::{Index, IndexMut};
}

#[macro_use]
macro_rules! prelude {
    () => {
        #[allow(unused_imports)]
        use $crate::prelude::*;
    };
}

pub mod enums;
pub(crate) mod hacks;
pub mod layout;
pub mod style;

prelude!();

pub(crate) const POINT_SCALE_FACTOR: f32 = 1.0;

// FIXME(anp): this seems...wrong
// static mut gDepth: uint32_t = 0i32 as uint32_t;

pub trait Node<CHILDREN>
where
    CHILDREN: Iterator<Item = Self>,
    Self: 'static + std::fmt::Debug + Sized,
{
    fn parent(&mut self) -> Option<&mut Self>;
    fn child(&mut self, index: usize) -> Option<&mut Self>;
    fn children(&mut self) -> CHILDREN;
    fn style(&mut self) -> &mut Style;
    fn layout(&mut self) -> &mut Layout;
    fn line(&mut self) -> &mut usize;
    // TODO(anp): can this be done without dynamic dispatch?
    fn measure_fn(&self) -> Option<&Fn(&Self, f32, MeasureMode, f32, MeasureMode) -> Size>;
    fn baseline_fn(&self) -> Option<&Fn(&Self, f32, f32) -> f32>;
    fn dirty(&mut self) -> &mut bool;
    fn new_layout(&mut self) -> &mut bool;
    fn node_type(&self) -> NodeType;
    fn resolved(&mut self) -> &mut ResolvedDimensions;

    fn increment_generation();
    fn current_generation(&self) -> u32;

    fn is_style_dim_defined(&mut self, axis: FlexDirection, parent_size: R32) -> bool {
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
                self.resolved()[dim] = Some(style.max_dimensions[dim]);
            } else {
                self.resolved()[dim] = Some(style.dimensions[dim]);
            };
        }
    }

    /// If both left and right are defined, then use left. Otherwise return
    /// +left or -right depending on which is defined.
    // was YGNodeRelativePosition
    fn relative_position(&mut self, axis: FlexDirection, axis_size: R32) -> Option<R32> {
        if let Some(pos) = self.leading_position(axis, axis_size) {
            Some(pos)
        } else {
            self.trailing_position(axis, axis_size).map(|p| -p)
        }
    }

    fn leading_position(&mut self, axis: FlexDirection, axis_size: R32) -> Option<R32> {
        let leading_edge = if axis.is_row() {
            Edge::Start
        } else {
            axis.leading_edge()
        };

        self.style()
            .position
            .computed(leading_edge)
            .into_iter()
            .flat_map(|p| p.resolve(axis_size))
            .next()
    }

    fn trailing_position(&mut self, axis: FlexDirection, axis_size: R32) -> Option<R32> {
        let trailing_edge = if axis.is_row() {
            Edge::End
        } else {
            axis.trailing_edge()
        };

        self.style()
            .position
            .computed(trailing_edge)
            .into_iter()
            .flat_map(|p| p.resolve(axis_size))
            .next()
    }

    fn margin_for_axis(&mut self, axis: FlexDirection, width_size: R32) -> R32 {
        self.leading_margin(axis, width_size) + self.trailing_margin(axis, width_size)
    }

    fn leading_margin(&mut self, axis: FlexDirection, width_size: R32) -> R32 {
        let leading_edge = match (self.style().margin[Edge::Start], axis.is_row()) {
            (Some(_), true) => Edge::Start,
            _ => axis.leading_edge(),
        };

        self.style()
            .margin
            .computed(leading_edge)
            .into_iter()
            .flat_map(|m| m.resolve(width_size))
            .next()
            .unwrap_or(r32(0.0))
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
                        .map(|v| v + self.margin_for_axis(FlexDirection::Row, parent_width))
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
                        .map(|v| v + self.margin_for_axis(FlexDirection::Column, parent_height))
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
            true,
            "initial",
        );

        if did_something_wat {
            let dir = self.layout().direction;
            self.set_position(dir, parent_width, parent_height, parent_width);

            // FIXME(anp): uncomment
            // YGRoundToPixelGrid(node, (*(*node).config).pointScaleFactor, 0.0f32, 0.0f32);
        };
    }

    /// This is a wrapper around the YGNodelayoutImpl function. It determines whether the layout
    /// request is redundant and can be skipped. Input parameters are the same as `YGNodelayoutImpl`
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
            || self.layout().last_parent_direction != parent_direction;

        if need_to_visit_node {
            // Invalidate the cached results.
            self.layout().cached_layout = None;
            self.layout().next_cached_measurements_index = 0;
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
                let margin_axis_row = self.margin_for_axis(FlexDirection::Row, parent_width);
                let margin_axis_column = self.margin_for_axis(FlexDirection::Column, parent_height);
                // First, try to use the layout cache.
                if CachedMeasurement::usable(
                    Some(cached),
                    width_measure_mode,
                    available_width,
                    height_measure_mode,
                    available_height,
                    margin_axis_row,
                    margin_axis_column,
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
            self.layout().measured_dimensions = Some(cached.computed);
        } else {
            YGNodelayoutImpl(
                node,
                availableWidth,
                availableHeight,
                parent_direction,
                width_measure_mode,
                height_measure_mode,
                parentWidth,
                parentHeight,
                performLayout,
            );

            self.layout().last_parent_direction = parent_direction;
            if cached_results.is_none() {
                if self.layout().next_cached_measurements_index == 16 {
                    self.layout().next_cached_measurements_index = 0;
                };

                let computed = self.layout().measured_dimensions.unwrap();

                let mut new_cache_entry = if perform_layout {
                    // Use the single layout cache entry.
                    &mut self.layout().cached_layout
                } else {
                    self.layout().next_cached_measurements_index += 1;
                    let idx = self.layout().next_cached_measurements_index;
                    &mut self.layout().cached_measurements[idx]
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

        self.layout().generation_count = self.current_generation();

        if perform_layout {
            self.layout().dimensions = self.layout().measured_dimensions.map(|d| d.into());
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
            self.layout().computed_flex_basis = None;

            if let Some(p) = self.parent() {
                p.mark_dirty();
            }
        };
    }

    fn apply_style<P: Property>(&mut self, new_style: P) {
        if Updated::Dirty == new_style.apply(self.style()) {
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
            .relative_position(main_axis, main_size)
            .unwrap_or(r32(0.0));
        let relative_position_cross = self
            .relative_position(cross_axis, cross_size)
            .unwrap_or(r32(0.0));

        *self.layout().index_mut(main_axis.leading_edge()) =
            self.leading_margin(main_axis, parent_width) + relative_position_main;

        *self.layout().index_mut(main_axis.trailing_edge()) =
            self.trailing_margin(main_axis, parent_width) + relative_position_main;

        *self.layout().index_mut(cross_axis.leading_edge()) =
            self.leading_margin(cross_axis, parent_width) + relative_position_cross;

        *self.layout().index_mut(cross_axis.trailing_edge()) =
            self.trailing_margin(cross_axis, parent_width);
    }

    fn trailing_margin(&mut self, axis: FlexDirection, width_size: R32) -> R32 {
        match (self.style().margin[Edge::End], axis.is_row()) {
            (Some(v), true) => Some(v),
            _ => self.style().margin.computed(axis.trailing_edge()),
        }.into_iter()
            .flat_map(|v| v.resolve(width_size))
            .next()
            .unwrap_or(r32(0.0))
    }

    // fn YGNodeSetChildTrailingPosition(&mut self, child: Node, axis: FlexDirection) -> () {
    //     let size: R32 = (*child).layout.measured_dimensions[DIM[axis as usize]];
    //     (*child).layout.position[trailing[axis as usize] as usize] =
    //         (*node).layout.measured_dimensions[DIM[axis as usize]]
    //             - size
    //             - (*child).layout.position[pos[axis as usize] as usize];
    // }

    // fn YGNodePaddingAndBorderForAxis(&mut self, axis: FlexDirection, widthSize: R32) -> R32 {
    //     return YGNodeLeadingPaddingAndBorder(node, axis, widthSize)
    //         + YGNodeTrailingPaddingAndBorder(node, axis, widthSize);
    // }

    // fn YGNodeTrailingPaddingAndBorder(&mut self, axis: FlexDirection, widthSize: R32) -> R32 {
    //     return YGNodeTrailingPadding(node, axis, widthSize) + YGNodeTrailingBorder(node, axis);
    // }

    // fn YGNodeTrailingBorder(&mut self, axis: FlexDirection) -> R32 {
    //     if FlexDirectionIsRow(axis) {
    //         match (*node).style.border[Edge::End] {
    //             Some(Value::Point(v)) | Some(Value::Percent(v)) if v >= 0.0 => {
    //                 return v.into();
    //             }
    //         }
    //     };
    //     return (*YGComputedEdgeValue(
    //         (*node).style.border.as_mut_ptr() as *const Value,
    //         trailing[axis as usize],
    //         &mut ValueZero as *mut Value,
    //     )).value
    //         .max(0.0f32);
    // }

    // fn YGNodeTrailingPadding(&mut self, axis: FlexDirection, widthSize: R32) -> R32 {
    //     if FlexDirectionIsRow(axis) && (*node).style.padding[Edge::End].is_some()
    //         && YGResolveValue(
    //             &mut (*node).style.padding[Edge::End as usize] as *mut Value,
    //             widthSize,
    //         ) >= 0.0f32
    //     {
    //         return YGResolveValue(
    //             &mut (*node).style.padding[Edge::End as usize] as *mut Value,
    //             widthSize,
    //         );
    //     };
    //     return YGResolveValue(
    //         YGComputedEdgeValue(
    //             (*node).style.padding.as_mut_ptr() as *const Value,
    //             trailing[axis as usize],
    //             &mut ValueZero as *mut Value,
    //         ),
    //         widthSize,
    //     ).max(0.0f32);
    // }

    // fn YGNodeLeadingPaddingAndBorder(&mut self, axis: FlexDirection, widthSize: R32) -> R32 {
    //     return YGNodeLeadingPadding(node, axis, widthSize) + YGNodeLeadingBorder(node, axis);
    // }

    // fn YGNodeLeadingBorder(&mut self, axis: FlexDirection) -> R32 {
    //     if FlexDirectionIsRow(axis)
    //         && (*node).style.border[Edge::Start].is_some()
    //         && (*node).style.border[Edge::Start as usize].value >= 0.0f32
    //     {
    //         return (*node).style.border[Edge::Start as usize].value;
    //     };
    //     return (*YGComputedEdgeValue(
    //         (*node).style.border.as_mut_ptr() as *const Value,
    //         leading[axis as usize],
    //         &mut ValueZero as *mut Value,
    //     )).value
    //         .max(0.0f32);
    // }

    // fn YGNodeLeadingPadding(&mut self, axis: FlexDirection, widthSize: R32) -> R32 {
    //     if FlexDirectionIsRow(axis) && (*node).style.padding[Edge::Start].is_some()
    //         && YGResolveValue(
    //             &mut (*node).style.padding[Edge::Start as usize] as *mut Value,
    //             widthSize,
    //         ) >= 0.0f32
    //     {
    //         return YGResolveValue(
    //             &mut (*node).style.padding[Edge::Start as usize] as *mut Value,
    //             widthSize,
    //         );
    //     };
    //     return YGResolveValue(
    //         YGComputedEdgeValue(
    //             (*node).style.padding.as_mut_ptr() as *const Value,
    //             leading[axis as usize],
    //             &mut ValueZero as *mut Value,
    //         ),
    //         widthSize,
    //     ).max(0.0f32);
    // }

    // fn YGNodeAbsoluteLayoutChild(
    //     &mut self,
    //     child: Node,
    //     width: R32,
    //     width_mode: MeasureMode,
    //     height: R32,
    //     direction: Direction,
    // ) -> () {
    //     let mainAxis: FlexDirection =
    //         YGResolveFlexDirection((*node).style.flex_direction, direction);
    //     let crossAxis: FlexDirection = FlexDirectionCross(mainAxis, direction);
    //     let isMainAxisRow: bool = FlexDirectionIsRow(mainAxis);
    //     let mut childWidth: R32 = ::std::f32::NAN;
    //     let mut childHeight: R32 = ::std::f32::NAN;
    //     let mut childWidthMeasureMode: MeasureMode;
    //     let mut childHeightMeasureMode: MeasureMode;
    //     let margin_row: R32 = YGNodeMarginForAxis(child, FlexDirection::Row, width);
    //     let margin_column: R32 = YGNodeMarginForAxis(child, FlexDirection::Column, width);
    //     if YGNodeIsStyleDimDefined(child, FlexDirection::Row, width) {
    //         childWidth = YGResolveValue((*child).resolvedDimensions.width, width) + margin_row;
    //     } else {
    //         // If the child doesn't have a specified width, compute the width based
    //         // on the left/right
    //         // offsets if they're defined.
    //         if YGNodeIsLeadingPosDefined(child, FlexDirection::Row)
    //             && YGNodeIsTrailingPosDefined(child, FlexDirection::Row)
    //         {
    //             childWidth = (*node).layout.measured_dimensions.width
    //                 - (YGNodeLeadingBorder(node, FlexDirection::Row)
    //                     + YGNodeTrailingBorder(node, FlexDirection::Row))
    //                 - (YGNodeLeadingPosition(child, FlexDirection::Row, width)
    //                     + YGNodeTrailingPosition(child, FlexDirection::Row, width));
    //             childWidth = YGNodeBoundAxis(child, FlexDirection::Row, childWidth, width, width);
    //         };
    //     };
    //     if YGNodeIsStyleDimDefined(child, FlexDirection::Column, height) {
    //         childHeight = YGResolveValue((*child).resolvedDimensions.height, height) + margin_column;
    //     } else {
    //         // If the child doesn't have a specified height, compute the height
    //         // based on the top/bottom
    //         // offsets if they're defined.
    //         if YGNodeIsLeadingPosDefined(child, FlexDirection::Column)
    //             && YGNodeIsTrailingPosDefined(child, FlexDirection::Column)
    //         {
    //             childHeight = (*node).layout.measured_dimensions.height
    //                 - (YGNodeLeadingBorder(node, FlexDirection::Column)
    //                     + YGNodeTrailingBorder(node, FlexDirection::Column))
    //                 - (YGNodeLeadingPosition(child, FlexDirection::Column, height)
    //                     + YGNodeTrailingPosition(child, FlexDirection::Column, height));
    //             childHeight =
    //                 YGNodeBoundAxis(child, FlexDirection::Column, childHeight, height, width);
    //         };
    //     };
    //     // Exactly one dimension needs to be defined for us to be able to do aspect ratio
    //     // calculation. One dimension being the anchor and the other being flexible.
    //     if childWidth.is_nan() || childHeight.is_nan() {
    //         if !(*child).style.aspect_ratio.is_nan() {
    //             if childWidth.is_nan() {
    //                 childWidth =
    //                     margin_row + (childHeight - margin_column) * (*child).style.aspect_ratio;
    //             } else {
    //                 if childHeight.is_nan() {
    //                     childHeight =
    //                         margin_column + (childWidth - margin_row) / (*child).style.aspect_ratio;
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
    //         childWidth = (*child).layout.measured_dimensions.width
    //             + YGNodeMarginForAxis(child, FlexDirection::Row, width);
    //         childHeight = (*child).layout.measured_dimensions.height
    //             + YGNodeMarginForAxis(child, FlexDirection::Column, width);
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
    //     if YGNodeIsTrailingPosDefined(child, mainAxis)
    //         && !YGNodeIsLeadingPosDefined(child, mainAxis)
    //     {
    //         (*child).layout.position[leading[mainAxis as usize] as usize] =
    //             (*node).layout.measured_dimensions[DIM[mainAxis as usize]]
    //                 - (*child).layout.measured_dimensions[DIM[mainAxis as usize]]
    //                 - YGNodeTrailingBorder(node, mainAxis)
    //                 - YGNodeTrailingMargin(child, mainAxis, width)
    //                 - YGNodeTrailingPosition(
    //                     child,
    //                     mainAxis,
    //                     if isMainAxisRow { width } else { height },
    //                 );
    //     } else {
    //         if !YGNodeIsLeadingPosDefined(child, mainAxis)
    //             && (*node).style.justify_content == Justify::Center
    //         {
    //             (*child).layout.position[leading[mainAxis as usize] as usize] =
    //                 ((*node).layout.measured_dimensions[DIM[mainAxis as usize]]
    //                     - (*child).layout.measured_dimensions[DIM[mainAxis as usize]])
    //                     / 2.0f32;
    //         } else {
    //             if !YGNodeIsLeadingPosDefined(child, mainAxis)
    //                 && (*node).style.justify_content == Justify::FlexEnd
    //             {
    //                 (*child).layout.position[leading[mainAxis as usize] as usize] =
    //                     (*node).layout.measured_dimensions[DIM[mainAxis as usize]]
    //                         - (*child).layout.measured_dimensions[DIM[mainAxis as usize]];
    //             };
    //         };
    //     };
    //     if YGNodeIsTrailingPosDefined(child, crossAxis)
    //         && !YGNodeIsLeadingPosDefined(child, crossAxis)
    //     {
    //         (*child).layout.position[leading[crossAxis as usize] as usize] =
    //             (*node).layout.measured_dimensions[DIM[crossAxis as usize]]
    //                 - (*child).layout.measured_dimensions[DIM[crossAxis as usize]]
    //                 - YGNodeTrailingBorder(node, crossAxis)
    //                 - YGNodeTrailingMargin(child, crossAxis, width)
    //                 - YGNodeTrailingPosition(
    //                     child,
    //                     crossAxis,
    //                     if isMainAxisRow { height } else { width },
    //                 );
    //     } else {
    //         if !YGNodeIsLeadingPosDefined(child, crossAxis)
    //             && YGNodeAlignItem(node, child) == Align::Center
    //         {
    //             (*child).layout.position[leading[crossAxis as usize] as usize] =
    //                 ((*node).layout.measured_dimensions[DIM[crossAxis as usize]]
    //                     - (*child).layout.measured_dimensions[DIM[crossAxis as usize]])
    //                     / 2.0f32;
    //         } else {
    //             if !YGNodeIsLeadingPosDefined(child, crossAxis)
    //                 && (YGNodeAlignItem(node, child) == Align::FlexEnd)
    //                     ^ ((*node).style.flex_wrap == YGWrapWrapReverse)
    //             {
    //                 (*child).layout.position[leading[crossAxis as usize] as usize] =
    //                     (*node).layout.measured_dimensions[DIM[crossAxis as usize]]
    //                         - (*child).layout.measured_dimensions[DIM[crossAxis as usize]];
    //             };
    //         };
    //     };
    // }

    // fn YGNodeAlignItem(&mut self, child: Self) -> Align {
    //     let align: Align = if (*child).style.align_self == Align::Auto {
    //         (*node).style.align_items
    //     } else {
    //         (*child).style.align_self
    //     };

    //     if align == Align::Baseline && self.style().flex_direction.is_column() {
    //         return Align::FlexStart;
    //     };
    //     return align;
    // }

    // fn YGNodeIsTrailingPosDefined(&mut self, axis: FlexDirection) -> bool {
    //     FlexDirectionIsRow(axis)
    //         && ((*node).style.position.computed(Edge::End).is_some()
    //             || (*node)
    //                 .style
    //                 .position
    //                 .computed(trailing[axis as usize])
    //                 .is_some())
    // }

    // /// Like bound_axis_within_min_and_max but also ensures that the value doesn't go below the
    // /// padding and border amount.
    // fn YGNodeBoundAxis(
    //     &mut self,
    //     axis: FlexDirection,
    //     value: R32,
    //     axisSize: R32,
    //     widthSize: R32,
    // ) -> R32 {
    //     return YGNodeBoundAxisWithinMinAndMax(node, axis, value, axisSize)
    //         .max(YGNodePaddingAndBorderForAxis(node, axis, widthSize));
    // }
    // fn YGNodeBoundAxisWithinMinAndMax(
    //     &mut self,
    //     axis: FlexDirection,
    //     value: R32,
    //     axisSize: R32,
    // ) -> R32 {
    //     let mut min: R32 = ::std::f32::NAN;
    //     let mut max: R32 = ::std::f32::NAN;
    //     if FlexDirectionIsColumn(axis) {
    //         min = YGResolveValue(
    //             &mut (*node).style.min_dimensions[Dimension::Height] as *mut Value,
    //             axisSize,
    //         );
    //         max = YGResolveValue(
    //             &mut (*node).style.max_dimensions[Dimension::Height] as *mut Value,
    //             axisSize,
    //         );
    //     } else {
    //         if FlexDirectionIsRow(axis) {
    //             min = YGResolveValue(
    //                 &mut (*node).style.min_dimensions[Dimension::Width] as *mut Value,
    //                 axisSize,
    //             );
    //             max = YGResolveValue(
    //                 &mut (*node).style.max_dimensions[Dimension::Width] as *mut Value,
    //                 axisSize,
    //             );
    //         };
    //     };
    //     let mut boundValue: R32 = value;
    //     if !max.is_nan() && max >= 0.0f32 && boundValue > max {
    //         boundValue = max;
    //     };
    //     if !min.is_nan() && min >= 0.0f32 && boundValue < min {
    //         boundValue = min;
    //     };
    //     return boundValue;
    // }

    // fn YGBaseline(&mut self) -> R32 {
    //     if (*node).baseline.is_some() {
    //         let baseline: R32 = (*node).baseline.expect("non-null function pointer")(
    //             node,
    //             (*node).layout.measured_dimensions.width,
    //             (*node).layout.measured_dimensions.height,
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
    //     let childCount = YGNodeGetChildCount(node);
    //     {
    //         let mut i = 0usize;
    //         'loop5: while i < childCount {
    //             'body3: loop {
    //                 {
    //                     let child: Node = YGNodeGetChild(node, i);
    //                     if (*child).lineIndex > 0 {
    //                         break 'loop5;
    //                     };
    //                     if (*child).style.position_type == PositionType::Absolute {
    //                         break 'body3;
    //                     };
    //                     if YGNodeAlignItem(node, child) == Align::Baseline {
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
    //         return (*node).layout.measured_dimensions.height;
    //     };
    //     let baseline: R32 = YGBaseline(baselineChild);
    //     return baseline + (*baselineChild).layout.position[Edge::Top as usize];
    // }

    // fn YGNodeIsLayoutDimDefined(&mut self, axis: FlexDirection) -> bool {
    //     let value: R32 = (*node).layout.measured_dimensions[DIM[axis as usize]];
    //     return !value.is_nan() && value >= 0.0f32;
    // }

    // fn YGIsBaselineLayout(&mut self) -> bool {
    //     if FlexDirectionIsColumn((*node).style.flex_direction) {
    //         return false;
    //     };
    //     if (*node).style.align_items == Align::Baseline {
    //         return true;
    //     };
    //     let childCount = YGNodeGetChildCount(node);
    //     {
    //         let mut i = 0;
    //         while i < childCount {
    //             {
    //                 let child: Node = YGNodeGetChild(node, i);
    //                 if (*child).style.position_type == PositionType::Relative
    //                     && (*child).style.align_self == Align::Baseline
    //                 {
    //                     return true;
    //                 };
    //             }
    //             i = i.wrapping_add(1);
    //         }
    //     }
    //     return false;
    // }

    // fn YGNodeDimWithMargin(&mut self, axis: FlexDirection, widthSize: R32) -> R32 {
    //     return (*node).layout.measured_dimensions[DIM[axis as usize]]
    //         + YGNodeLeadingMargin(node, axis, widthSize)
    //         + YGNodeTrailingMargin(node, axis, widthSize);
    // }

    // fn YGMarginLeadingValue(&mut self, axis: FlexDirection) -> *mut Value {
    //     if FlexDirectionIsRow(axis) && (*node).style.margin[Edge::Start].is_some() {
    //         return &mut (*node).style.margin[Edge::Start as usize] as *mut Value;
    //     } else {
    //         return &mut (*node).style.margin[leading[axis as usize] as usize] as *mut Value;
    //     };
    // }

    // fn YGMarginTrailingValue(&mut self, axis: FlexDirection) -> *mut Value {
    //     if FlexDirectionIsRow(axis) && (*node).style.margin[Edge::End].is_some() {
    //         return &mut (*node).style.margin[Edge::End as usize] as *mut Value;
    //     } else {
    //         return &mut (*node).style.margin[trailing[axis as usize] as usize] as *mut Value;
    //     };
    // }

    // fn YGResolveFlexGrow(&mut self) -> R32 {
    //     // Root nodes flexGrow should always be 0
    //     if (*node).parent.is_null() {
    //         return 0.0;
    //     };
    //     if !(*node).style.flex_grow.is_nan() {
    //         return (*node).style.flex_grow;
    //     };
    //     if !(*node).style.flex.is_nan() && (*node).style.flex > 0.0 {
    //         return (*node).style.flex;
    //     };
    //     return kDefaultFlexGrow;
    // }

    // fn YGNodeResolveFlexShrink(&mut self) -> R32 {
    //     // Root nodes flexShrink should always be 0
    //     if (*node).parent.is_null() {
    //         return 0.0;
    //     };
    //     if !(*node).style.flex_shrink.is_nan() {
    //         return (*node).style.flex_shrink;
    //     };
    //     if !(*(*node).config).useWebDefaults
    //         && !(*node).style.flex.is_nan()
    //         && (*node).style.flex < 0.0f32
    //     {
    //         return -(*node).style.flex;
    //     };
    //     return if (*(*node).config).useWebDefaults {
    //         kWebDefaultFlexShrink
    //     } else {
    //         kDefaultFlexShrink
    //     };
    // }

    // fn YGNodeIsFlex(&mut self) -> bool {
    //     return (*node).style.position_type == PositionType::Relative
    //         && (YGResolveFlexGrow(node) != 0.0 || YGNodeResolveFlexShrink(node) != 0.0);
    // }

    // fn YGNodeComputeFlexBasisForChild(
    //     &mut self,
    //     child: Self,
    //     width: R32,
    //     width_mode: MeasureMode,
    //     height: R32,
    //     parentWidth: R32,
    //     parentHeight: R32,
    //     height_mode: MeasureMode,
    //     direction: Direction,
    // ) -> () {
    //     let mainAxis: FlexDirection =
    //         YGResolveFlexDirection((*node).style.flex_direction, direction);
    //     let isMainAxisRow: bool = FlexDirectionIsRow(mainAxis);
    //     let mainAxisSize: R32 = if isMainAxisRow { width } else { height };
    //     let mainAxisParentSize: R32 = if isMainAxisRow {
    //         parentWidth
    //     } else {
    //         parentHeight
    //     };
    //     let mut childWidth: R32;
    //     let mut childHeight: R32;
    //     let mut childWidthMeasureMode: MeasureMode;
    //     let mut childHeightMeasureMode: MeasureMode;
    //     let resolvedFlexBasis: R32 =
    //         YGResolveValue(YGNodeResolveFlexBasisPtr(child), mainAxisParentSize);
    //     let isRowStyleDimDefined: bool =
    //         YGNodeIsStyleDimDefined(child, FlexDirection::Row, parentWidth);
    //     let isColumnStyleDimDefined: bool =
    //         YGNodeIsStyleDimDefined(child, FlexDirection::Column, parentHeight);
    //     if !resolvedFlexBasis.is_nan() && !mainAxisSize.is_nan() {
    //         if (*child).layout.computed_flex_basis.is_nan()
    //             || YGConfigIsExperimentalFeatureEnabled(
    //                 (*child).config,
    //                 YGExperimentalFeatureWebFlexBasis,
    //             )
    //                 && (*child).layout.computed_flex_basis_generation != gCurrentGenerationCount
    //         {
    //             (*child).layout.computed_flex_basis = resolvedFlexBasis
    //                 .max(YGNodePaddingAndBorderForAxis(child, mainAxis, parentWidth));
    //         };
    //     } else {
    //         if isMainAxisRow && isRowStyleDimDefined {
    //             // The width is definite, so use that as the flex basis.
    //             (*child).layout.computed_flex_basis =
    //                 YGResolveValue((*child).resolvedDimensions.width, parentWidth).max(
    //                     YGNodePaddingAndBorderForAxis(child, FlexDirection::Row, parentWidth),
    //                 );
    //         } else {
    //             if !isMainAxisRow && isColumnStyleDimDefined {
    //                 // The height is definite, so use that as the flex basis.
    //                 (*child).layout.computed_flex_basis = YGResolveValue(
    //                     (*child).resolvedDimensions.height,
    //                     parentHeight,
    //                 ).max(
    //                     YGNodePaddingAndBorderForAxis(child, FlexDirection::Column, parentWidth),
    //                 );
    //             } else {
    //                 // Compute the flex basis and hypothetical main size (i.e. the clamped
    //                 // flex basis).
    //                 childWidth = ::std::f32::NAN;
    //                 childHeight = ::std::f32::NAN;
    //                 childWidthMeasureMode = MeasureMode::Undefined;
    //                 childHeightMeasureMode = MeasureMode::Undefined;
    //                 let margin_row: R32 =
    //                     YGNodeMarginForAxis(child, FlexDirection::Row, parentWidth);
    //                 let margin_column: R32 =
    //                     YGNodeMarginForAxis(child, FlexDirection::Column, parentWidth);
    //                 if isRowStyleDimDefined {
    //                     childWidth = YGResolveValue((*child).resolvedDimensions.width, parentWidth)
    //                         + margin_row;
    //                     childWidthMeasureMode = MeasureMode::Exactly;
    //                 };
    //                 if isColumnStyleDimDefined {
    //                     childHeight =
    //                         YGResolveValue((*child).resolvedDimensions.height, parentHeight)
    //                             + margin_column;
    //                     childHeightMeasureMode = MeasureMode::Exactly;
    //                 };
    //                 // The W3C spec doesn't say anything about the 'overflow' property,
    //                 // but all major browsers appear to implement the following logic.
    //                 if !isMainAxisRow && (*node).style.overflow == Overflow::Scroll
    //                     || (*node).style.overflow != Overflow::Scroll
    //                 {
    //                     if childWidth.is_nan() && !width.is_nan() {
    //                         childWidth = width;
    //                         childWidthMeasureMode = MeasureMode::AtMost;
    //                     };
    //                 };
    //                 if isMainAxisRow && (*node).style.overflow == Overflow::Scroll
    //                     || (*node).style.overflow != Overflow::Scroll
    //                 {
    //                     if childHeight.is_nan() && !height.is_nan() {
    //                         childHeight = height;
    //                         childHeightMeasureMode = MeasureMode::AtMost;
    //                     };
    //                 };
    //                 if !(*child).style.aspect_ratio.is_nan() {
    //                     if !isMainAxisRow && childWidthMeasureMode == MeasureMode::Exactly {
    //                         childHeight = (childWidth - margin_row) / (*child).style.aspect_ratio;
    //                         childHeightMeasureMode = MeasureMode::Exactly;
    //                     } else {
    //                         if isMainAxisRow && childHeightMeasureMode == MeasureMode::Exactly {
    //                             childWidth =
    //                                 (childHeight - margin_column) * (*child).style.aspect_ratio;
    //                             childWidthMeasureMode = MeasureMode::Exactly;
    //                         };
    //                     };
    //                 };
    //                 // If child has no defined size in the cross axis and is set to stretch,
    //                 // set the cross
    //                 // axis to be measured exactly with the available inner width
    //                 let hasExactWidth: bool = !width.is_nan() && width_mode == MeasureMode::Exactly;
    //                 let childWidthStretch: bool = YGNodeAlignItem(node, child) == Align::Stretch
    //                     && childWidthMeasureMode != MeasureMode::Exactly;
    //                 if !isMainAxisRow && !isRowStyleDimDefined && hasExactWidth && childWidthStretch
    //                 {
    //                     childWidth = width;
    //                     childWidthMeasureMode = MeasureMode::Exactly;
    //                     if !(*child).style.aspect_ratio.is_nan() {
    //                         childHeight = (childWidth - margin_row) / (*child).style.aspect_ratio;
    //                         childHeightMeasureMode = MeasureMode::Exactly;
    //                     };
    //                 };
    //                 let hasExactHeight: bool =
    //                     !height.is_nan() && height_mode == MeasureMode::Exactly;
    //                 let childHeightStretch: bool = YGNodeAlignItem(node, child) == Align::Stretch
    //                     && childHeightMeasureMode != MeasureMode::Exactly;
    //                 if isMainAxisRow
    //                     && !isColumnStyleDimDefined
    //                     && hasExactHeight
    //                     && childHeightStretch
    //                 {
    //                     childHeight = height;
    //                     childHeightMeasureMode = MeasureMode::Exactly;
    //                     if !(*child).style.aspect_ratio.is_nan() {
    //                         childWidth = (childHeight - margin_column) * (*child).style.aspect_ratio;
    //                         childWidthMeasureMode = MeasureMode::Exactly;
    //                     };
    //                 };
    //                 YGConstrainMaxSizeForMode(
    //                     child,
    //                     FlexDirection::Row,
    //                     parentWidth,
    //                     parentWidth,
    //                     &mut childWidthMeasureMode as *mut MeasureMode,
    //                     &mut childWidth as *mut R32,
    //                 );
    //                 YGConstrainMaxSizeForMode(
    //                     child,
    //                     FlexDirection::Column,
    //                     parentHeight,
    //                     parentWidth,
    //                     &mut childHeightMeasureMode as *mut MeasureMode,
    //                     &mut childHeight as *mut R32,
    //                 );
    //                 YGLayoutNodeInternal(
    //                     child,
    //                     childWidth,
    //                     childHeight,
    //                     direction,
    //                     childWidthMeasureMode,
    //                     childHeightMeasureMode,
    //                     parentWidth,
    //                     parentHeight,
    //                     false,
    //                     "measure",
    //                     config,
    //                 );
    //                 (*child).layout.computed_flex_basis = (*child).layout.measured_dimensions
    //                     [DIM[mainAxis as usize]]
    //                     .max(YGNodePaddingAndBorderForAxis(child, mainAxis, parentWidth));
    //             };
    //         };
    //     };
    //     (*child).layout.computed_flex_basis_generation = gCurrentGenerationCount;
    // }

    // fn YGNodeResolveFlexBasisPtr(&mut self) -> *const Value {
    //     if (*node).style.flex_basis.unit != UnitType::Auto
    //         && (*node).style.flex_basis.unit != UnitType::Undefined
    //     {
    //         return &mut (*node).style.flex_basis as *mut Value;
    //     };
    //     if !(*node).style.flex.is_nan() && (*node).style.flex > 0.0f32 {
    //         return if (*(*node).config).useWebDefaults {
    //             &Value::Auto as *const Value
    //         } else {
    //             &mut ValueZero as *mut Value
    //         };
    //     };
    //     return &Value::Auto as *const Value;
    // }

    // fn YGNodeFixedSizeSetMeasuredDimensions(
    //     &mut self,
    //     availableWidth: R32,
    //     availableHeight: R32,
    //     width_measure_mode: MeasureMode,
    //     height_measure_mode: MeasureMode,
    //     parentWidth: R32,
    //     parentHeight: R32,
    // ) -> bool {
    //     if width_measure_mode == MeasureMode::AtMost && availableWidth <= 0.0f32
    //         || height_measure_mode == MeasureMode::AtMost && availableHeight <= 0.0f32
    //         || width_measure_mode == MeasureMode::Exactly && height_measure_mode == MeasureMode::Exactly
    //     {
    //         let marginAxisColumn: R32 =
    //             YGNodeMarginForAxis(node, FlexDirection::Column, parentWidth);
    //         let marginAxisRow: R32 = YGNodeMarginForAxis(node, FlexDirection::Row, parentWidth);
    //         (*node).layout.measured_dimensions.width = YGNodeBoundAxis(
    //             node,
    //             FlexDirection::Row,
    //             if availableWidth.is_nan()
    //                 || width_measure_mode == MeasureMode::AtMost && availableWidth < 0.0f32
    //             {
    //                 0.0f32
    //             } else {
    //                 availableWidth - marginAxisRow
    //             },
    //             parentWidth,
    //             parentWidth,
    //         );
    //         (*node).layout.measured_dimensions.height = YGNodeBoundAxis(
    //             node,
    //             FlexDirection::Column,
    //             if availableHeight.is_nan()
    //                 || height_measure_mode == MeasureMode::AtMost && availableHeight < 0.0f32
    //             {
    //                 0.0f32
    //             } else {
    //                 availableHeight - marginAxisColumn
    //             },
    //             parentHeight,
    //             parentWidth,
    //         );
    //         return true;
    //     };
    //     return false;
    // }

    // /// For nodes with no children, use the available values if they were provided,
    // /// or the minimum size as indicated by the padding and border sizes.
    // fn YGNodeEmptyContainerSetMeasuredDimensions(
    //     &mut self,
    //     availableWidth: R32,
    //     availableHeight: R32,
    //     width_measure_mode: MeasureMode,
    //     height_measure_mode: MeasureMode,
    //     parentWidth: R32,
    //     parentHeight: R32,
    // ) -> () {
    //     let paddingAndBorderAxisRow: R32 =
    //         YGNodePaddingAndBorderForAxis(node, FlexDirection::Row, parentWidth);
    //     let paddingAndBorderAxisColumn: R32 =
    //         YGNodePaddingAndBorderForAxis(node, FlexDirection::Column, parentWidth);
    //     let marginAxisRow: R32 = YGNodeMarginForAxis(node, FlexDirection::Row, parentWidth);
    //     let marginAxisColumn: R32 = YGNodeMarginForAxis(node, FlexDirection::Column, parentWidth);
    //     (*node).layout.measured_dimensions.width = YGNodeBoundAxis(
    //         node,
    //         FlexDirection::Row,
    //         if width_measure_mode == MeasureMode::Undefined || width_measure_mode == MeasureMode::AtMost
    //         {
    //             paddingAndBorderAxisRow
    //         } else {
    //             availableWidth - marginAxisRow
    //         },
    //         parentWidth,
    //         parentWidth,
    //     );
    //     (*node).layout.measured_dimensions.height = YGNodeBoundAxis(
    //         node,
    //         FlexDirection::Column,
    //         if height_measure_mode == MeasureMode::Undefined
    //             || height_measure_mode == MeasureMode::AtMost
    //         {
    //             paddingAndBorderAxisColumn
    //         } else {
    //             availableHeight - marginAxisColumn
    //         },
    //         parentHeight,
    //         parentWidth,
    //     );
    // }

    // fn YGNodeWithMeasureFuncSetMeasuredDimensions(
    //     &mut self,
    //     availableWidth: R32,
    //     availableHeight: R32,
    //     width_measure_mode: MeasureMode,
    //     height_measure_mode: MeasureMode,
    //     parentWidth: R32,
    //     parentHeight: R32,
    // ) -> () {
    //     YGAssertWithNode(
    //         node,
    //         (*node).measure.is_some(),
    //         b"Expected node to have custom measure function\x00" as *const u8 as *const c_char,
    //     );
    //     let paddingAndBorderAxisRow: R32 =
    //         YGNodePaddingAndBorderForAxis(node, FlexDirection::Row, availableWidth);
    //     let paddingAndBorderAxisColumn: R32 =
    //         YGNodePaddingAndBorderForAxis(node, FlexDirection::Column, availableWidth);
    //     let marginAxisRow: R32 = YGNodeMarginForAxis(node, FlexDirection::Row, availableWidth);
    //     let marginAxisColumn: R32 =
    //         YGNodeMarginForAxis(node, FlexDirection::Column, availableWidth);
    //     // We want to make sure we don't call measure with negative size
    //     let innerWidth: R32 = if availableWidth.is_nan() {
    //         availableWidth
    //     } else {
    //         (0.0f32).max(availableWidth - marginAxisRow - paddingAndBorderAxisRow)
    //     };
    //     let innerHeight: R32 = if availableHeight.is_nan() {
    //         availableHeight
    //     } else {
    //         (0.0f32).max(availableHeight - marginAxisColumn - paddingAndBorderAxisColumn)
    //     };
    //     if width_measure_mode == MeasureMode::Exactly && height_measure_mode == MeasureMode::Exactly {
    //         // Don't bother sizing the text if both dimensions are already defined.
    //         (*node).layout.measured_dimensions.width = YGNodeBoundAxis(
    //             node,
    //             FlexDirection::Row,
    //             availableWidth - marginAxisRow,
    //             parentWidth,
    //             parentWidth,
    //         );
    //         (*node).layout.measured_dimensions.height = YGNodeBoundAxis(
    //             node,
    //             FlexDirection::Column,
    //             availableHeight - marginAxisColumn,
    //             parentHeight,
    //             parentWidth,
    //         );
    //     } else {
    //         // Measure the text under the current constraints.
    //         let measuredSize: Size = (*node).measure.expect("non-null function pointer")(
    //             node,
    //             innerWidth,
    //             width_measure_mode,
    //             innerHeight,
    //             height_measure_mode,
    //         );
    //         (*node).layout.measured_dimensions.width = YGNodeBoundAxis(
    //             node,
    //             FlexDirection::Row,
    //             if width_measure_mode == MeasureMode::Undefined
    //                 || width_measure_mode == MeasureMode::AtMost
    //             {
    //                 measuredSize.width + paddingAndBorderAxisRow
    //             } else {
    //                 availableWidth - marginAxisRow
    //             },
    //             availableWidth,
    //             availableWidth,
    //         );
    //         (*node).layout.measured_dimensions.height = YGNodeBoundAxis(
    //             node,
    //             FlexDirection::Column,
    //             if height_measure_mode == MeasureMode::Undefined
    //                 || height_measure_mode == MeasureMode::AtMost
    //             {
    //                 measuredSize.height + paddingAndBorderAxisColumn
    //             } else {
    //                 availableHeight - marginAxisColumn
    //             },
    //             availableHeight,
    //             availableWidth,
    //         );
    //     };
    // }

    // fn copy_style(&mut self, mut from: Self) {
    //     let mut src = from.style();
    //     let mut dst = self.style();
    //     if src != dst {
    //         *dst = *src;
    //         self.mark_dirty();
    //     }
    // }

    // fn YGNodeSetMeasureFunc(&mut self, mut measureFunc: YGMeasureFunc) -> () {
    //     if measureFunc.is_none() {
    //         (*node).measure = None;
    //         (*node).nodeType = NodeType::Default;
    //     } else {
    //         YGAssertWithNode(
    //         node,
    //         YGNodeGetChildCount(node) == 0,
    //         b"Cannot set measure function: Nodes with measure functions cannot have children.\x00"
    //             as *const u8 as *const c_char,
    //     );
    //         (*node).measure = measureFunc;
    //         (*node).nodeType = NodeType::Text;
    //     };
    // }

    // fn YGNodeSetNodeType(&mut self, mut nodeType: NodeType) -> () {
    //     (*node).nodeType = nodeType;
    // }

    // fn YGNodeGetNodeType(&mut self) -> NodeType {
    //     return (*node).nodeType;
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

    //     let nodeLeft = (*node).layout.position[Edge::Left as usize];
    //     let nodeTop = (*node).layout.position[Edge::Top as usize];

    //     let nodeWidth = (*node).layout.dimensions[Dimension::Width as usize];
    //     let nodeHeight = (*node).layout.dimensions[Dimension::Height as usize];

    //     let absoluteNodeLeft = absoluteLeft + nodeLeft;
    //     let absoluteNodeTop = absoluteTop + nodeTop;

    //     let absoluteNodeRight = absoluteNodeLeft + nodeWidth;
    //     let absoluteNodeBottom = absoluteNodeTop + nodeHeight;

    //     // If a node has a custom measure function we never want to round down its size as this could
    //     // lead to unwanted text truncation.
    //     let textRounding = (*node).nodeType == NodeType::Text;

    //     (*node).layout.position[Edge::Left as usize] =
    //         round_value_to_pixel_grid(nodeLeft, pointScaleFactor, false, textRounding);
    //     (*node).layout.position[Edge::Top as usize] =
    //         round_value_to_pixel_grid(nodeTop, pointScaleFactor, false, textRounding);

    //     // We multiply dimension by scale factor and if the result is close to the whole number, we don't
    //     // have any fraction
    //     // To verify if the result is close to whole number we want to check both floor and ceil numbers
    //     let hasFractionalWidth = !YGFloatsEqual(nodeWidth * pointScaleFactor % 1.0, 0.0)
    //         && !YGFloatsEqual(nodeWidth * pointScaleFactor % 1.0, 1.0);
    //     let hasFractionalHeight = !YGFloatsEqual(nodeHeight * pointScaleFactor % 1.0, 0.0)
    //         && !YGFloatsEqual(nodeHeight * pointScaleFactor % 1.0, 1.0);

    //     (*node).layout.dimensions[Dimension::Width as usize] = round_value_to_pixel_grid(
    //         absoluteNodeRight,
    //         pointScaleFactor,
    //         textRounding && hasFractionalWidth,
    //         textRounding && !hasFractionalWidth,
    //     )
    //         - round_value_to_pixel_grid(absoluteNodeLeft, pointScaleFactor, false, textRounding);
    //     (*node).layout.dimensions[Dimension::Height as usize] = round_value_to_pixel_grid(
    //         absoluteNodeBottom,
    //         pointScaleFactor,
    //         textRounding && hasFractionalHeight,
    //         textRounding && !hasFractionalHeight,
    //     )
    //         - round_value_to_pixel_grid(absoluteNodeTop, pointScaleFactor, false, textRounding);

    //     for i in 0..YGNodeListCount((*node).children) {
    //         YGRoundToPixelGrid(
    //             YGNodeGetChild(node, i),
    //             pointScaleFactor,
    //             absoluteNodeLeft,
    //             absoluteNodeTop,
    //         );
    //     }
    // }

    // fn YGConstrainMaxSizeForMode(
    //     &mut self,
    //     axis: FlexDirection,
    //     parentAxisSize: R32,
    //     parentWidth: R32,
    //     mode: *mut MeasureMode,
    //     size: *mut R32,
    // ) {
    //     let maxSize = YGResolveValue(
    //         &(*node).style.max_dimensions[axis.dimension()],
    //         parentAxisSize,
    //     ) + YGNodeMarginForAxis(node, axis, parentWidth);
    //     match *mode {
    //         MeasureMode::Exactly | MeasureMode::AtMost => {
    //             *size = if maxSize.is_nan() || *size < maxSize {
    //                 *size
    //             } else {
    //                 maxSize
    //             }
    //         }
    //         MeasureMode::Undefined => if !maxSize.is_nan() {
    //             *mode = MeasureMode::AtMost;
    //             *size = maxSize;
    //         },
    //     }
    // }

    // TODO(anp): do these even need to exist?
    // static mut firstAbsoluteChild: Node = ::std::ptr::null_mut();
    // static mut currentAbsoluteChild: Node = ::std::ptr::null_mut();
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    // from here to the end of the comments is the main layout impl
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //

    //
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
    //    When calling YGNodelayoutImpl and YGLayoutNodeInternal, if the caller passes
    //    an available size of
    //    undefined then it must also pass a measure mode of MeasureMode::Undefined
    //    in that dimension.
    //
    unsafe fn YGNodelayoutImpl(
        &mut self,
        availableWidth: R32,
        availableHeight: R32,
        parent_direction: Direction,
        width_measure_mode: MeasureMode,
        height_measure_mode: MeasureMode,
        parentWidth: R32,
        parentHeight: R32,
        performLayout: bool,
    ) {
        assert!(
            if availableWidth.is_nan() {
                width_measure_mode == MeasureMode::Undefined
            } else {
                true
            },
            "availableWidth is indefinite so width_measure_mode must be MeasureMode::Undefined"
        );

        assert!(
            if availableHeight.is_nan() {
                height_measure_mode == MeasureMode::Undefined
            } else {
                true
            },
            "availableHeight is indefinite so height_measure_mode must be MeasureMode::Undefined"
        );

        // // Set the resolved resolution in the node's layout.
        let direction = YGNodeResolveDirection(node, parent_direction);
        (*node).layout.direction = direction;

        let flexRowDirection = YGResolveFlexDirection(FlexDirection::Row, direction);
        let flexColumnDirection = YGResolveFlexDirection(FlexDirection::Column, direction);

        (*node).layout.margin[Edge::Start as usize] =
            YGNodeLeadingMargin(node, flexRowDirection, parentWidth);
        (*node).layout.margin[Edge::End as usize] =
            YGNodeTrailingMargin(node, flexRowDirection, parentWidth);
        (*node).layout.margin[Edge::Top as usize] =
            YGNodeLeadingMargin(node, flexColumnDirection, parentWidth);
        (*node).layout.margin[Edge::Bottom as usize] =
            YGNodeTrailingMargin(node, flexColumnDirection, parentWidth);

        (*node).layout.border[Edge::Start as usize] = YGNodeLeadingBorder(node, flexRowDirection);
        (*node).layout.border[Edge::End as usize] = YGNodeTrailingBorder(node, flexRowDirection);
        (*node).layout.border[Edge::Top as usize] = YGNodeLeadingBorder(node, flexColumnDirection);
        (*node).layout.border[Edge::Bottom as usize] =
            YGNodeTrailingBorder(node, flexColumnDirection);

        (*node).layout.padding[Edge::Start as usize] =
            YGNodeLeadingPadding(node, flexRowDirection, parentWidth);
        (*node).layout.padding[Edge::End as usize] =
            YGNodeTrailingPadding(node, flexRowDirection, parentWidth);
        (*node).layout.padding[Edge::Top as usize] =
            YGNodeLeadingPadding(node, flexColumnDirection, parentWidth);
        (*node).layout.padding[Edge::Bottom as usize] =
            YGNodeTrailingPadding(node, flexColumnDirection, parentWidth);

        if (*node).measure.is_some() {
            YGNodeWithMeasureFuncSetMeasuredDimensions(
                node,
                availableWidth,
                availableHeight,
                width_measure_mode,
                height_measure_mode,
                parentWidth,
                parentHeight,
            );
            return;
        }

        let childCount = YGNodeListCount((*node).children);
        if childCount == 0 {
            YGNodeEmptyContainerSetMeasuredDimensions(
                node,
                availableWidth,
                availableHeight,
                width_measure_mode,
                height_measure_mode,
                parentWidth,
                parentHeight,
            );
            return;
        }

        // If we're not being asked to perform a full layout we can skip the algorithm if we already know
        // the size
        if !performLayout
            && YGNodeFixedSizeSetMeasuredDimensions(
                node,
                availableWidth,
                availableHeight,
                width_measure_mode,
                height_measure_mode,
                parentWidth,
                parentHeight,
            ) {
            return;
        }

        // At this point we know we're going to perform work. Ensure that each child has a mutable copy.
        YGCloneChildrenIfNeeded(node);

        // Reset layout flags, as they could have changed.
        (*node).layout.had_overflow = false;

        // STEP 1: CALCULATE VALUES FOR REMAINDER OF ALGORITHM
        let mainAxis = YGResolveFlexDirection((*node).style.flex_direction, direction);
        let crossAxis = FlexDirectionCross(mainAxis, direction);
        let isMainAxisRow = FlexDirectionIsRow(mainAxis);
        let justify_content = (*node).style.justify_content;
        let isNodeFlexWrap = (*node).style.flex_wrap != YGWrapNoWrap;

        let mainAxisParentSize = if isMainAxisRow {
            parentWidth
        } else {
            parentHeight
        };
        let crossAxisParentSize = if isMainAxisRow {
            parentHeight
        } else {
            parentWidth
        };

        let leadingPaddingAndBorderMain =
            YGNodeLeadingPaddingAndBorder(node, mainAxis, parentWidth);
        let trailingPaddingAndBorderMain =
            YGNodeTrailingPaddingAndBorder(node, mainAxis, parentWidth);
        let leadingPaddingAndBorderCross =
            YGNodeLeadingPaddingAndBorder(node, crossAxis, parentWidth);
        let paddingAndBorderAxisMain = YGNodePaddingAndBorderForAxis(node, mainAxis, parentWidth);
        let paddingAndBorderAxisCross = YGNodePaddingAndBorderForAxis(node, crossAxis, parentWidth);

        let mut measureModeMainDim = if isMainAxisRow {
            width_measure_mode
        } else {
            height_measure_mode
        };
        let measureModeCrossDim = if isMainAxisRow {
            height_measure_mode
        } else {
            width_measure_mode
        };

        let paddingAndBorderAxisRow = if isMainAxisRow {
            paddingAndBorderAxisMain
        } else {
            paddingAndBorderAxisCross
        };
        let paddingAndBorderAxisColumn = if isMainAxisRow {
            paddingAndBorderAxisCross
        } else {
            paddingAndBorderAxisMain
        };

        let marginAxisRow = YGNodeMarginForAxis(node, FlexDirection::Row, parentWidth);
        let marginAxisColumn = YGNodeMarginForAxis(node, FlexDirection::Column, parentWidth);

        // STEP 2: DETERMINE AVAILABLE SIZE IN MAIN AND CROSS DIRECTIONS
        let minInnerWidth = YGResolveValue(&(*node).style.min_dimensions.width, parentWidth)
            - marginAxisRow
            - paddingAndBorderAxisRow;
        let maxInnerWidth = YGResolveValue(&(*node).style.max_dimensions.width, parentWidth)
            - marginAxisRow
            - paddingAndBorderAxisRow;
        let minInnerHeight = YGResolveValue(&(*node).style.min_dimensions.height, parentHeight)
            - marginAxisColumn
            - paddingAndBorderAxisColumn;
        let maxInnerHeight = YGResolveValue(&(*node).style.max_dimensions.height, parentHeight)
            - marginAxisColumn
            - paddingAndBorderAxisColumn;
        let minInnerMainDim = if isMainAxisRow {
            minInnerWidth
        } else {
            minInnerHeight
        };
        let maxInnerMainDim = if isMainAxisRow {
            maxInnerWidth
        } else {
            maxInnerHeight
        };

        // Max dimension overrides predefined dimension value; Min dimension in turn overrides both of the
        // above
        let mut availableInnerWidth = availableWidth - marginAxisRow - paddingAndBorderAxisRow;
        if !availableInnerWidth.is_nan() {
            // We want to make sure our available width does not violate min and max constraints
            availableInnerWidth = availableInnerWidth.min(maxInnerWidth).max(minInnerWidth);
        }

        let mut availableInnerHeight =
            availableHeight - marginAxisColumn - paddingAndBorderAxisColumn;
        if !availableInnerHeight.is_nan() {
            // We want to make sure our available height does not violate min and max constraints
            availableInnerHeight = availableInnerHeight.min(maxInnerHeight).max(minInnerHeight);
        }

        let mut availableInnerMainDim = if isMainAxisRow {
            availableInnerWidth
        } else {
            availableInnerHeight
        };
        let mut availableInnerCrossDim = if isMainAxisRow {
            availableInnerHeight
        } else {
            availableInnerWidth
        };

        // If there is only one child with flex_grow + flex_shrink it means we can set the
        // computed_flex_basis to 0 instead of measuring and shrinking / flexing the child to exactly
        // match the remaining space
        let mut singleFlexChild: Node = ::std::ptr::null_mut();
        if measureModeMainDim == MeasureMode::Exactly {
            for i in 0..childCount {
                let child = YGNodeGetChild(node, i);
                if !singleFlexChild.is_null() {
                    if YGNodeIsFlex(child) {
                        // There is already a flexible child, abort.
                        singleFlexChild = ::std::ptr::null_mut();
                        break;
                    }
                } else if YGResolveFlexGrow(child) > 0.0 && YGNodeResolveFlexShrink(child) > 0.0 {
                    singleFlexChild = child;
                }
            }
        }

        let mut totalOuterFlexBasis = 0.0;

        // // STEP 3: DETERMINE FLEX BASIS FOR EACH ITEM
        for i in 0..childCount {
            {
                let child = YGNodeListGet((*node).children, i);
                if (*child).style.display == Display::None {
                    YGZeroOutLayoutRecursivly(child);
                    (*child).hasNewLayout = true;
                    (*child).isDirty = false;
                    continue;
                }
                YGResolveDimensions(child);
                if performLayout {
                    // Set the initial position (relative to the parent).
                    let childDirection = YGNodeResolveDirection(child, direction);
                    YGNodeSetPosition(
                        child,
                        childDirection,
                        availableInnerMainDim,
                        availableInnerCrossDim,
                        availableInnerWidth,
                    );
                }

                // Absolute-positioned children don't participate in flex layout. Add them
                // to a list that we can process later.
                if (*child).style.position_type == PositionType::Absolute {
                    // Store a private linked list of absolutely positioned children
                    // so that we can efficiently traverse them later.
                    if firstAbsoluteChild.is_null() {
                        firstAbsoluteChild = child;
                    }
                    if currentAbsoluteChild.is_null() {
                        (*currentAbsoluteChild).nextChild = child;
                    }
                    currentAbsoluteChild = child;
                    (*child).nextChild = ::std::ptr::null_mut();
                } else {
                    if child == singleFlexChild {
                        (*child).layout.computed_flex_basis_generation = gCurrentGenerationCount;
                        (*child).layout.computed_flex_basis = 0.0;
                    } else {
                        YGNodeComputeFlexBasisForChild(
                            node,
                            child,
                            availableInnerWidth,
                            width_measure_mode,
                            availableInnerHeight,
                            availableInnerWidth,
                            availableInnerHeight,
                            height_measure_mode,
                            direction,
                            config,
                        );
                    }
                }

                totalOuterFlexBasis += (*child).layout.computed_flex_basis;
                totalOuterFlexBasis += YGNodeMarginForAxis(child, mainAxis, availableInnerWidth);
            }

            let flexBasisOverflows = if measureModeMainDim == MeasureMode::Undefined {
                false
            } else {
                totalOuterFlexBasis > availableInnerMainDim
            };
            if isNodeFlexWrap && flexBasisOverflows && measureModeMainDim == MeasureMode::AtMost {
                measureModeMainDim = MeasureMode::Exactly;
            }

            // STEP 4: COLLECT FLEX ITEMS INTO FLEX LINES

            // Indexes of children that represent the first and last items in the line.
            let mut startOfLineIndex = 0;
            let mut endOfLineIndex = 0;

            // Number of lines.
            let mut lineCount = 0;

            // Accumulated cross dimensions of all lines so far.
            let mut totalLineCrossDim = 0.0f32;

            // Max main dimension of all the lines.
            let mut maxLineMainDim = 0.0f32;

            while endOfLineIndex < childCount {
                // Number of items on the currently line. May be different than the
                // difference
                // between start and end indicates because we skip over absolute-positioned
                // items.
                let mut itemsOnLine = 0;

                // sizeConsumedOnCurrentLine is accumulation of the dimensions and margin
                // of all the children on the current line. This will be used in order to
                // either set the dimensions of the node if none already exist or to compute
                // the remaining space left for the flexible children.
                let mut sizeConsumedOnCurrentLine = 0.0f32;
                let mut sizeConsumedOnCurrentLineIncludingMinConstraint = 0.0f32;

                let mut totalFlexGrowFactors = 0.0f32;
                let mut totalFlexShrinkScaledFactors = 0.0f32;

                // Maintain a linked list of the child nodes that can shrink and/or grow.
                let mut firstRelativeChild: Node = ::std::ptr::null_mut();
                let mut currentRelativeChild: Node = ::std::ptr::null_mut();

                // Add items to the current line until it's full or we run out of items.
                let mut i = startOfLineIndex;
                while i < childCount {
                    let child = YGNodeListGet((*node).children, i);
                    if (*child).style.display == Display::None {
                        continue;
                    }
                    (*child).lineIndex = lineCount;

                    if (*child).style.position_type != PositionType::Absolute {
                        let childMarginMainAxis =
                            YGNodeMarginForAxis(child, mainAxis, availableInnerWidth);
                        let flexBasisWithMaxConstraints =
                            YGResolveValue(
                                &(*child).style.max_dimensions[DIM[mainAxis as usize]],
                                mainAxisParentSize,
                            ).min((*child).layout.computed_flex_basis);
                        let flexBasisWithMinAndMaxConstraints = YGResolveValue(
                            &(*child).style.min_dimensions[DIM[mainAxis as usize]],
                            mainAxisParentSize,
                        ).max(flexBasisWithMaxConstraints);

                        // If this is a multi-line flow and this item pushes us over the
                        // available size, we've
                        // hit the end of the current line. Break out of the loop and lay out
                        // the current line.
                        if sizeConsumedOnCurrentLineIncludingMinConstraint
                            + flexBasisWithMinAndMaxConstraints
                            + childMarginMainAxis > availableInnerMainDim
                            && isNodeFlexWrap && itemsOnLine > 0
                        {
                            break;
                        }

                        sizeConsumedOnCurrentLineIncludingMinConstraint +=
                            flexBasisWithMinAndMaxConstraints + childMarginMainAxis;
                        sizeConsumedOnCurrentLine +=
                            flexBasisWithMinAndMaxConstraints + childMarginMainAxis;
                        itemsOnLine += 1;

                        if YGNodeIsFlex(child) {
                            totalFlexGrowFactors += YGResolveFlexGrow(child);

                            // Unlike the grow factor, the shrink factor is scaled relative to the child dimension.
                            totalFlexShrinkScaledFactors += -YGNodeResolveFlexShrink(child)
                                * (*child).layout.computed_flex_basis;
                        }

                        // Store a private linked list of children that need to be layed out.
                        if firstRelativeChild.is_null() {
                            firstRelativeChild = child;
                        }
                        if currentRelativeChild.is_null() {
                            (*currentRelativeChild).nextChild = child;
                        }
                        currentRelativeChild = child;
                        (*child).nextChild = ::std::ptr::null_mut();
                    }
                    i += 1;
                    endOfLineIndex += 1;
                }

                // The total flex factor needs to be floored to 1.
                if totalFlexGrowFactors > 0.0 && totalFlexGrowFactors < 1.0 {
                    totalFlexGrowFactors = 1.0;
                }

                // The total flex shrink factor needs to be floored to 1.
                if totalFlexShrinkScaledFactors > 0.0 && totalFlexShrinkScaledFactors < 1.0 {
                    totalFlexShrinkScaledFactors = 1.0;
                }

                // If we don't need to measure the cross axis, we can skip the entire flex
                // step.
                let canSkipFlex = !performLayout && measureModeCrossDim == MeasureMode::Exactly;

                // In order to position the elements in the main axis, we have two
                // controls. The space between the beginning and the first element
                // and the space between each two elements.
                let mut leadingMainDim = 0.0;
                let mut betweenMainDim = 0.0;

                // STEP 5: RESOLVING FLEXIBLE LENGTHS ON MAIN AXIS
                // Calculate the remaining available space that needs to be allocated.
                // If the main dimension size isn't known, it is computed based on
                // the line length, so there's no more space left to distribute.

                // If we don't measure with exact main dimension we want to ensure we don't violate min and max
                if measureModeMainDim != MeasureMode::Exactly {
                    if !minInnerMainDim.is_nan() && sizeConsumedOnCurrentLine < minInnerMainDim {
                        availableInnerMainDim = minInnerMainDim;
                    } else if !maxInnerMainDim.is_nan()
                        && sizeConsumedOnCurrentLine > maxInnerMainDim
                    {
                        availableInnerMainDim = maxInnerMainDim;
                    } else if !(*(*node).config).useLegacyStretchBehaviour
                        && (totalFlexGrowFactors == 0.0 || YGResolveFlexGrow(node) == 0.0)
                    {
                        // If we don't have any children to flex or we can't flex the node itself,
                        // space we've used is all space we need. Root node also should be shrunk to minimum
                        availableInnerMainDim = sizeConsumedOnCurrentLine;
                    }
                }

                let mut remainingFreeSpace = 0.0;
                if !availableInnerMainDim.is_nan() {
                    remainingFreeSpace = availableInnerMainDim - sizeConsumedOnCurrentLine;
                } else if sizeConsumedOnCurrentLine < 0.0 {
                    // availableInnerMainDim is indefinite which means the node is being sized based on its
                    // content.
                    // sizeConsumedOnCurrentLine is negative which means the node will allocate 0 points for
                    // its content. Consequently, remainingFreeSpace is 0 - sizeConsumedOnCurrentLine.
                    remainingFreeSpace = -sizeConsumedOnCurrentLine;
                }

                let mut originalRemainingFreeSpace = remainingFreeSpace;
                let mut deltaFreeSpace = 0.0;

                if !canSkipFlex {
                    let mut childFlexBasis: f32;
                    let mut flexShrinkScaledFactor: f32;
                    let mut flexGrowFactor: f32;
                    let mut baseMainSize: f32;
                    let mut boundMainSize: f32;

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
                    let mut deltaFlexShrinkScaledFactors = 0.0;
                    let mut deltaFlexGrowFactors = 0.0;
                    currentRelativeChild = firstRelativeChild;
                    while !currentRelativeChild.is_null() {
                        childFlexBasis = YGResolveValue(
                            &(*currentRelativeChild).style.max_dimensions[DIM[mainAxis as usize]],
                            mainAxisParentSize,
                        ).min(
                            YGResolveValue(
                                &(*currentRelativeChild).style.min_dimensions
                                    [DIM[mainAxis as usize]],
                                mainAxisParentSize,
                            ).max((*currentRelativeChild).layout.computed_flex_basis),
                        );

                        if remainingFreeSpace < 0.0 {
                            flexShrinkScaledFactor =
                                -YGNodeResolveFlexShrink(currentRelativeChild) * childFlexBasis;

                            // Is this child able to shrink?
                            if flexShrinkScaledFactor != 0.0 {
                                baseMainSize = childFlexBasis
                                    + remainingFreeSpace / totalFlexShrinkScaledFactors
                                        * flexShrinkScaledFactor;
                                boundMainSize = YGNodeBoundAxis(
                                    currentRelativeChild,
                                    mainAxis,
                                    baseMainSize,
                                    availableInnerMainDim,
                                    availableInnerWidth,
                                );
                                if baseMainSize != boundMainSize {
                                    // By excluding this item's size and flex factor from remaining,
                                    // this item's
                                    // min/max constraints should also trigger in the second pass
                                    // resulting in the
                                    // item's size calculation being identical in the first and second
                                    // passes.
                                    deltaFreeSpace -= boundMainSize - childFlexBasis;
                                    deltaFlexShrinkScaledFactors -= flexShrinkScaledFactor;
                                }
                            }
                        } else if remainingFreeSpace > 0.0 {
                            flexGrowFactor = YGResolveFlexGrow(currentRelativeChild);

                            // Is this child able to grow?
                            if flexGrowFactor != 0.0 {
                                baseMainSize = childFlexBasis
                                    + remainingFreeSpace / totalFlexGrowFactors * flexGrowFactor;
                                boundMainSize = YGNodeBoundAxis(
                                    currentRelativeChild,
                                    mainAxis,
                                    baseMainSize,
                                    availableInnerMainDim,
                                    availableInnerWidth,
                                );

                                if baseMainSize != boundMainSize {
                                    // By excluding this item's size and flex factor from remaining,
                                    // this item's
                                    // min/max constraints should also trigger in the second pass
                                    // resulting in the
                                    // item's size calculation being identical in the first and second
                                    // passes.
                                    deltaFreeSpace -= boundMainSize - childFlexBasis;
                                    deltaFlexGrowFactors -= flexGrowFactor;
                                }
                            }
                        }

                        currentRelativeChild = (*currentRelativeChild).nextChild;
                    }

                    totalFlexShrinkScaledFactors += deltaFlexShrinkScaledFactors;
                    totalFlexGrowFactors += deltaFlexGrowFactors;
                    remainingFreeSpace += deltaFreeSpace;

                    // Second pass: resolve the sizes of the flexible items
                    deltaFreeSpace = 0.0;
                    currentRelativeChild = firstRelativeChild;
                    while !currentRelativeChild.is_null() {
                        childFlexBasis = YGResolveValue(
                            &(*currentRelativeChild).style.max_dimensions[DIM[mainAxis as usize]],
                            mainAxisParentSize,
                        ).min(
                            YGResolveValue(
                                &(*currentRelativeChild).style.min_dimensions
                                    [DIM[mainAxis as usize]],
                                mainAxisParentSize,
                            ).max((*currentRelativeChild).layout.computed_flex_basis),
                        );
                        let mut updatedMainSize = childFlexBasis;

                        if remainingFreeSpace < 0.0 {
                            flexShrinkScaledFactor =
                                -YGNodeResolveFlexShrink(currentRelativeChild) * childFlexBasis;
                            // Is this child able to shrink?
                            if flexShrinkScaledFactor != 0.0 {
                                let mut childSize: f32;

                                if totalFlexShrinkScaledFactors == 0.0 {
                                    childSize = childFlexBasis + flexShrinkScaledFactor;
                                } else {
                                    childSize = childFlexBasis
                                        + (remainingFreeSpace / totalFlexShrinkScaledFactors)
                                            * flexShrinkScaledFactor;
                                }

                                updatedMainSize = YGNodeBoundAxis(
                                    currentRelativeChild,
                                    mainAxis,
                                    childSize,
                                    availableInnerMainDim,
                                    availableInnerWidth,
                                );
                            }
                        } else if remainingFreeSpace > 0.0 {
                            flexGrowFactor = YGResolveFlexGrow(currentRelativeChild);

                            // Is this child able to grow?
                            if flexGrowFactor != 0.0 {
                                updatedMainSize = YGNodeBoundAxis(
                                    currentRelativeChild,
                                    mainAxis,
                                    childFlexBasis
                                        + remainingFreeSpace / totalFlexGrowFactors
                                            * flexGrowFactor,
                                    availableInnerMainDim,
                                    availableInnerWidth,
                                );
                            }
                        }

                        deltaFreeSpace -= updatedMainSize - childFlexBasis;

                        let marginMain = YGNodeMarginForAxis(
                            currentRelativeChild,
                            mainAxis,
                            availableInnerWidth,
                        );
                        let marginCross = YGNodeMarginForAxis(
                            currentRelativeChild,
                            crossAxis,
                            availableInnerWidth,
                        );

                        let mut childCrossSize: f32;
                        let mut childMainSize = updatedMainSize + marginMain;
                        let mut childCrossMeasureMode: MeasureMode;
                        let mut childMainMeasureMode: MeasureMode = MeasureMode::Exactly;

                        // TODO(anp) check for bug on the C side -- this was an != NULL check
                        if !(*currentRelativeChild).style.aspect_ratio.is_nan() {
                            childCrossSize = if isMainAxisRow {
                                (childMainSize - marginMain)
                                    / (*currentRelativeChild).style.aspect_ratio
                            } else {
                                (childMainSize - marginMain)
                                    * (*currentRelativeChild).style.aspect_ratio
                            };
                            childCrossMeasureMode = MeasureMode::Exactly;

                            childCrossSize += marginCross;
                        } else if !availableInnerCrossDim.is_nan()
                            && !YGNodeIsStyleDimDefined(
                                currentRelativeChild,
                                crossAxis,
                                availableInnerCrossDim,
                            )
                            && measureModeCrossDim == MeasureMode::Exactly
                            && !(isNodeFlexWrap && flexBasisOverflows)
                            && YGNodeAlignItem(node, currentRelativeChild) == Align::Stretch
                        {
                            childCrossSize = availableInnerCrossDim;
                            childCrossMeasureMode = MeasureMode::Exactly;
                        } else if !YGNodeIsStyleDimDefined(
                            currentRelativeChild,
                            crossAxis,
                            availableInnerCrossDim,
                        ) {
                            childCrossSize = availableInnerCrossDim;
                            childCrossMeasureMode = if childCrossSize.is_nan() {
                                MeasureMode::Undefined
                            } else {
                                MeasureMode::AtMost
                            };
                        } else {
                            childCrossSize = YGResolveValue(
                                (*currentRelativeChild).resolvedDimensions[DIM[crossAxis as usize]],
                                availableInnerCrossDim,
                            ) + marginCross;
                            let isLoosePercentageMeasurement = (*(*currentRelativeChild)
                                .resolvedDimensions[DIM[crossAxis as usize]])
                                .unit
                                == UnitType::Percent
                                && measureModeCrossDim != MeasureMode::Exactly;
                            childCrossMeasureMode =
                                if childCrossSize.is_nan() || isLoosePercentageMeasurement {
                                    MeasureMode::Undefined
                                } else {
                                    MeasureMode::Exactly
                                };
                        }

                        YGConstrainMaxSizeForMode(
                            currentRelativeChild,
                            mainAxis,
                            availableInnerMainDim,
                            availableInnerWidth,
                            &mut childMainMeasureMode,
                            &mut childMainSize,
                        );
                        YGConstrainMaxSizeForMode(
                            currentRelativeChild,
                            crossAxis,
                            availableInnerCrossDim,
                            availableInnerWidth,
                            &mut childCrossMeasureMode,
                            &mut childCrossSize,
                        );

                        let requiresStretchLayout = !YGNodeIsStyleDimDefined(
                            currentRelativeChild,
                            crossAxis,
                            availableInnerCrossDim,
                        )
                            && YGNodeAlignItem(node, currentRelativeChild) == Align::Stretch;

                        let childWidth = if isMainAxisRow {
                            childMainSize
                        } else {
                            childCrossSize
                        };
                        let childHeight = if !isMainAxisRow {
                            childMainSize
                        } else {
                            childCrossSize
                        };

                        let childWidthMeasureMode = if isMainAxisRow {
                            childMainMeasureMode
                        } else {
                            childCrossMeasureMode
                        };
                        let childHeightMeasureMode = if !isMainAxisRow {
                            childMainMeasureMode
                        } else {
                            childCrossMeasureMode
                        };

                        // Recursively call the layout algorithm for this child with the updated
                        // main size.
                        YGLayoutNodeInternal(
                            currentRelativeChild,
                            childWidth,
                            childHeight,
                            direction,
                            childWidthMeasureMode,
                            childHeightMeasureMode,
                            availableInnerWidth,
                            availableInnerHeight,
                            performLayout && !requiresStretchLayout,
                            "flex",
                            config,
                        );
                        (*node).layout.had_overflow |= (*currentRelativeChild).layout.had_overflow;

                        currentRelativeChild = (*currentRelativeChild).nextChild;
                    }
                }

                remainingFreeSpace = originalRemainingFreeSpace + deltaFreeSpace;
                (*node).layout.had_overflow |= remainingFreeSpace < 0.0;

                // STEP 6: MAIN-AXIS JUSTIFICATION & CROSS-AXIS SIZE DETERMINATION

                // At this point, all the children have their dimensions set in the main
                // axis.
                // Their dimensions are also set in the cross axis with the exception of
                // items
                // that are aligned "stretch". We need to compute these stretch values and
                // set the final positions.

                // If we are using "at most" rules in the main axis. Calculate the remaining space when
                // constraint by the min size defined for the main axis.

                if measureModeMainDim == MeasureMode::AtMost && remainingFreeSpace > 0.0 {
                    if (*node).style.min_dimensions[DIM[mainAxis as usize]] != None
                        && YGResolveValue(
                            &(*node).style.min_dimensions[DIM[mainAxis as usize]],
                            mainAxisParentSize,
                        ) >= 0.0
                    {
                        remainingFreeSpace = (0.0f32).max(
                            YGResolveValue(
                                &(*node).style.min_dimensions[DIM[mainAxis as usize]],
                                mainAxisParentSize,
                            )
                                - (availableInnerMainDim - remainingFreeSpace),
                        );
                    } else {
                        remainingFreeSpace = 0.0;
                    }
                }

                let mut numberOfAutoMarginsOnCurrentLine = 0;
                for i in startOfLineIndex..endOfLineIndex {
                    let child = YGNodeListGet((*node).children, i);
                    if (*child).style.position_type == PositionType::Relative {
                        if (*YGMarginLeadingValue(child, mainAxis)) == Value::Auto {
                            numberOfAutoMarginsOnCurrentLine += 1;
                        }
                        if (*YGMarginTrailingValue(child, mainAxis)) == Value::Auto {
                            numberOfAutoMarginsOnCurrentLine += 1;
                        }
                    }
                }

                if numberOfAutoMarginsOnCurrentLine == 0 {
                    match justify_content {
                        Justify::Center => leadingMainDim = remainingFreeSpace / 2.0,
                        Justify::FlexEnd => leadingMainDim = remainingFreeSpace,
                        Justify::SpaceBetween => {
                            if itemsOnLine > 1 {
                                betweenMainDim =
                                    remainingFreeSpace.max(0.0) / (itemsOnLine - 1) as f32;
                            } else {
                                betweenMainDim = 0.0;
                            }
                        }
                        Justify::SpaceAround => {
                            // Space on the edges is half of the space between elements
                            betweenMainDim = remainingFreeSpace / itemsOnLine as f32;
                            leadingMainDim = betweenMainDim / 2.0;
                        }
                        _ => (),
                    }
                }

                let mut mainDim = leadingPaddingAndBorderMain + leadingMainDim;
                let mut crossDim = 0.0;

                for i in startOfLineIndex..endOfLineIndex {
                    let child = YGNodeListGet((*node).children, i);
                    if (*child).style.display == Display::None {
                        continue;
                    }
                    if (*child).style.position_type == PositionType::Absolute
                        && YGNodeIsLeadingPosDefined(child, mainAxis)
                    {
                        if performLayout {
                            // In case the child is position absolute and has left/top being
                            // defined, we override the position to whatever the user said
                            // (and margin/border).
                            (*child).layout.position[pos[mainAxis as usize] as usize] =
                                YGNodeLeadingPosition(child, mainAxis, availableInnerMainDim)
                                    + YGNodeLeadingBorder(node, mainAxis)
                                    + YGNodeLeadingMargin(child, mainAxis, availableInnerWidth);
                        }
                    } else {
                        // Now that we placed the element, we need to update the variables.
                        // We need to do that only for relative elements. Absolute elements
                        // do not take part in that phase.
                        if (*child).style.position_type == PositionType::Relative {
                            if (*YGMarginLeadingValue(child, mainAxis)) == Value::Auto {
                                mainDim +=
                                    remainingFreeSpace / numberOfAutoMarginsOnCurrentLine as f32;
                            }

                            if performLayout {
                                (*child).layout.position[pos[mainAxis as usize] as usize] +=
                                    mainDim;
                            }

                            if (*YGMarginTrailingValue(child, mainAxis)) == Value::Auto {
                                mainDim +=
                                    remainingFreeSpace / numberOfAutoMarginsOnCurrentLine as f32;
                            }

                            if canSkipFlex {
                                // If we skipped the flex step, then we can't rely on the
                                // measuredDims because
                                // they weren't computed. This means we can't call YGNodeDimWithMargin.
                                mainDim += betweenMainDim
                                    + YGNodeMarginForAxis(child, mainAxis, availableInnerWidth)
                                    + (*child).layout.computed_flex_basis;
                                crossDim = availableInnerCrossDim;
                            } else {
                                // The main dimension is the sum of all the elements dimension plus the spacing.
                                mainDim += betweenMainDim
                                    + YGNodeDimWithMargin(child, mainAxis, availableInnerWidth);

                                // The cross dimension is the max of the elements dimension since
                                // there can only be one element in that cross dimension.
                                crossDim = crossDim.max(YGNodeDimWithMargin(
                                    child,
                                    crossAxis,
                                    availableInnerWidth,
                                ));
                            }
                        } else if performLayout {
                            (*child).layout.position[pos[mainAxis as usize] as usize] +=
                                YGNodeLeadingBorder(node, mainAxis) + leadingMainDim;
                        }
                    }
                }

                mainDim += trailingPaddingAndBorderMain;

                let mut containerCrossAxis = availableInnerCrossDim;
                if measureModeCrossDim == MeasureMode::Undefined
                    || measureModeCrossDim == MeasureMode::AtMost
                {
                    // Compute the cross axis from the max cross dimension of the children.
                    containerCrossAxis = YGNodeBoundAxis(
                        node,
                        crossAxis,
                        crossDim + paddingAndBorderAxisCross,
                        crossAxisParentSize,
                        parentWidth,
                    ) - paddingAndBorderAxisCross;
                }

                // If there's no flex wrap, the cross dimension is defined by the container.
                if !isNodeFlexWrap && measureModeCrossDim == MeasureMode::Exactly {
                    crossDim = availableInnerCrossDim;
                }

                // Clamp to the min/max size specified on the container.
                crossDim = YGNodeBoundAxis(
                    node,
                    crossAxis,
                    crossDim + paddingAndBorderAxisCross,
                    crossAxisParentSize,
                    parentWidth,
                ) - paddingAndBorderAxisCross;

                // STEP 7: CROSS-AXIS ALIGNMENT
                // We can skip child alignment if we're just measuring the container.
                if performLayout {
                    for i in startOfLineIndex..endOfLineIndex {
                        let child = YGNodeListGet((*node).children, i);
                        if (*child).style.display == Display::None {
                            continue;
                        }
                        if (*child).style.position_type == PositionType::Absolute {
                            // If the child is absolutely positioned and has a
                            // top/left/bottom/right
                            // set, override all the previously computed positions to set it
                            // correctly.
                            let isChildLeadingPosDefined =
                                YGNodeIsLeadingPosDefined(child, crossAxis);
                            if isChildLeadingPosDefined {
                                (*child).layout.position[pos[crossAxis as usize] as usize] =
                                    YGNodeLeadingPosition(child, crossAxis, availableInnerCrossDim)
                                        + YGNodeLeadingBorder(node, crossAxis)
                                        + YGNodeLeadingMargin(
                                            child,
                                            crossAxis,
                                            availableInnerWidth,
                                        );
                            }
                            // If leading position is not defined or calculations result in Nan, default to border + margin
                            if !isChildLeadingPosDefined
                                || (*child).layout.position[pos[crossAxis as usize] as usize]
                                    .is_nan()
                            {
                                (*child).layout.position[pos[crossAxis as usize] as usize] =
                                    YGNodeLeadingBorder(node, crossAxis)
                                        + YGNodeLeadingMargin(
                                            child,
                                            crossAxis,
                                            availableInnerWidth,
                                        );
                            }
                        } else {
                            let mut leadingCrossDim = leadingPaddingAndBorderCross;

                            // For a relative children, we're either using align_items (parent) or
                            // align_self (child) in order to determine the position in the cross
                            // axis
                            let alignItem = YGNodeAlignItem(node, child);

                            // If the child uses align stretch, we need to lay it out one more
                            // time, this time
                            // forcing the cross-axis size to be the computed cross size for the
                            // current line.
                            if alignItem == Align::Stretch
                                && (*YGMarginLeadingValue(child, crossAxis)) != Value::Auto
                                && (*YGMarginTrailingValue(child, crossAxis)) != Value::Auto
                            {
                                // If the child defines a definite size for its cross axis, there's
                                // no need to stretch.
                                if !YGNodeIsStyleDimDefined(
                                    child,
                                    crossAxis,
                                    availableInnerCrossDim,
                                ) {
                                    let mut childMainSize =
                                        (*child).layout.measured_dimensions[DIM[mainAxis as usize]];
                                    let mut childCrossSize = if !(*child)
                                        .style
                                        .aspect_ratio
                                        .is_nan()
                                    {
                                        (YGNodeMarginForAxis(child, crossAxis, availableInnerWidth)
                                            + (if isMainAxisRow {
                                                childMainSize / (*child).style.aspect_ratio
                                            } else {
                                                childMainSize * (*child).style.aspect_ratio
                                            }))
                                    } else {
                                        crossDim
                                    };

                                    childMainSize +=
                                        YGNodeMarginForAxis(child, mainAxis, availableInnerWidth);

                                    let mut childMainMeasureMode = MeasureMode::Exactly;
                                    let mut childCrossMeasureMode = MeasureMode::Exactly;
                                    YGConstrainMaxSizeForMode(
                                        child,
                                        mainAxis,
                                        availableInnerMainDim,
                                        availableInnerWidth,
                                        &mut childMainMeasureMode,
                                        &mut childMainSize,
                                    );
                                    YGConstrainMaxSizeForMode(
                                        child,
                                        crossAxis,
                                        availableInnerCrossDim,
                                        availableInnerWidth,
                                        &mut childCrossMeasureMode,
                                        &mut childCrossSize,
                                    );

                                    let childWidth = if isMainAxisRow {
                                        childMainSize
                                    } else {
                                        childCrossSize
                                    };
                                    let childHeight = if !isMainAxisRow {
                                        childMainSize
                                    } else {
                                        childCrossSize
                                    };

                                    let childWidthMeasureMode = if childWidth.is_nan() {
                                        MeasureMode::Undefined
                                    } else {
                                        MeasureMode::Exactly
                                    };
                                    let childHeightMeasureMode = if childHeight.is_nan() {
                                        MeasureMode::Undefined
                                    } else {
                                        MeasureMode::Exactly
                                    };

                                    YGLayoutNodeInternal(
                                        child,
                                        childWidth,
                                        childHeight,
                                        direction,
                                        childWidthMeasureMode,
                                        childHeightMeasureMode,
                                        availableInnerWidth,
                                        availableInnerHeight,
                                        true,
                                        "stretch",
                                        config,
                                    );
                                }
                            } else {
                                let remainingCrossDim = containerCrossAxis
                                    - YGNodeDimWithMargin(child, crossAxis, availableInnerWidth);

                                if (*YGMarginLeadingValue(child, crossAxis)) == Value::Auto
                                    && (*YGMarginTrailingValue(child, crossAxis)) == Value::Auto
                                {
                                    leadingCrossDim += (remainingCrossDim / 2.0).max(0.0);
                                } else if (*YGMarginTrailingValue(child, crossAxis)) == Value::Auto
                                {
                                    // No-Op
                                } else if (*YGMarginLeadingValue(child, crossAxis)) == Value::Auto {
                                    leadingCrossDim += 0.0f32.max(remainingCrossDim);
                                } else if alignItem == Align::FlexStart {
                                    // No-Op
                                } else if alignItem == Align::Center {
                                    leadingCrossDim += remainingCrossDim / 2.0;
                                } else {
                                    leadingCrossDim += remainingCrossDim;
                                }
                            }
                            // And we apply the position
                            (*child).layout.position[pos[crossAxis as usize] as usize] +=
                                totalLineCrossDim + leadingCrossDim;
                        }
                    }
                }

                totalLineCrossDim += crossDim;
                maxLineMainDim = maxLineMainDim.max(mainDim);
                lineCount += 1;
                startOfLineIndex = endOfLineIndex;
            }

            // STEP 8: MULTI-LINE CONTENT ALIGNMENT
            if performLayout
                && (lineCount > 1 || YGIsBaselineLayout(node))
                && !availableInnerCrossDim.is_nan()
            {
                let remainingAlignContentDim = availableInnerCrossDim - totalLineCrossDim;

                let mut crossDimLead = 0.0;
                let mut currentLead = leadingPaddingAndBorderCross;

                match (*node).style.align_content {
                    Align::FlexEnd => currentLead += remainingAlignContentDim,
                    Align::Center => currentLead += remainingAlignContentDim / 2.0,
                    Align::Stretch => if availableInnerCrossDim > totalLineCrossDim {
                        crossDimLead = remainingAlignContentDim / lineCount as f32;
                    },
                    Align::SpaceAround => if availableInnerCrossDim > totalLineCrossDim {
                        currentLead += remainingAlignContentDim / (2.0 * lineCount as f32);
                        if lineCount > 1 {
                            crossDimLead = remainingAlignContentDim / lineCount as f32;
                        }
                    } else {
                        currentLead += remainingAlignContentDim / 2.0;
                    },
                    Align::SpaceBetween => {
                        if availableInnerCrossDim > totalLineCrossDim && lineCount > 1 {
                            crossDimLead = remainingAlignContentDim / (lineCount as f32 - 1.0);
                        }
                    }
                    _ => (),
                }

                let mut endIndex = 0;
                for i in 0..lineCount {
                    let startIndex = endIndex;

                    // compute the line's height and find the endIndex
                    let mut lineHeight = 0.0f32;
                    let mut maxAscentForCurrentLine = 0.0f32;
                    let mut maxDescentForCurrentLine = 0.0f32;

                    for ii in startIndex..childCount {
                        endIndex = ii;
                        let child = YGNodeListGet((*node).children, ii);
                        if (*child).style.display == Display::None {
                            continue;
                        }
                        if (*child).style.position_type == PositionType::Relative {
                            if (*child).lineIndex != i {
                                break;
                            }
                            if YGNodeIsLayoutDimDefined(child, crossAxis) {
                                lineHeight = lineHeight.max(
                                    (*child).layout.measured_dimensions[DIM[crossAxis as usize]]
                                        + YGNodeMarginForAxis(
                                            child,
                                            crossAxis,
                                            availableInnerWidth,
                                        ),
                                );
                            }
                            if YGNodeAlignItem(node, child) == Align::Baseline {
                                let ascent = YGBaseline(child)
                                    + YGNodeLeadingMargin(
                                        child,
                                        FlexDirection::Column,
                                        availableInnerWidth,
                                    );
                                let descent = (*child).layout.measured_dimensions.height
                                    + YGNodeMarginForAxis(
                                        child,
                                        FlexDirection::Column,
                                        availableInnerWidth,
                                    ) - ascent;
                                maxAscentForCurrentLine = maxAscentForCurrentLine.max(ascent);
                                maxDescentForCurrentLine = maxDescentForCurrentLine.max(descent);
                                lineHeight = lineHeight
                                    .max(maxAscentForCurrentLine + maxDescentForCurrentLine);
                            }
                        }
                    }
                    lineHeight += crossDimLead;

                    if performLayout {
                        for ii in startIndex..endIndex {
                            let child = YGNodeListGet((*node).children, ii);
                            if (*child).style.display == Display::None {
                                continue;
                            }
                            if (*child).style.position_type == PositionType::Relative {
                                match YGNodeAlignItem(node, child) {
                                    Align::FlexStart => {
                                        (*child).layout.position
                                            [pos[crossAxis as usize] as usize] = currentLead
                                            + YGNodeLeadingMargin(
                                                child,
                                                crossAxis,
                                                availableInnerWidth,
                                            );
                                    }
                                    Align::FlexEnd => {
                                        (*child).layout.position
                                            [pos[crossAxis as usize] as usize] = currentLead
                                            + lineHeight
                                            - YGNodeTrailingMargin(
                                                child,
                                                crossAxis,
                                                availableInnerWidth,
                                            )
                                            - (*child).layout.measured_dimensions
                                                [DIM[crossAxis as usize]];
                                    }
                                    Align::Center => {
                                        let mut childHeight = (*child).layout.measured_dimensions
                                            [DIM[crossAxis as usize]];
                                        (*child).layout.position
                                            [pos[crossAxis as usize] as usize] =
                                            currentLead + (lineHeight - childHeight) / 2.0;
                                    }
                                    Align::Stretch => {
                                        (*child).layout.position
                                            [pos[crossAxis as usize] as usize] = currentLead
                                            + YGNodeLeadingMargin(
                                                child,
                                                crossAxis,
                                                availableInnerWidth,
                                            );

                                        // Remeasure child with the line height as it as been only measured with the
                                        // parents height yet.
                                        if !YGNodeIsStyleDimDefined(
                                            child,
                                            crossAxis,
                                            availableInnerCrossDim,
                                        ) {
                                            let childWidth = if isMainAxisRow {
                                                ((*child).layout.measured_dimensions.width
                                                    + YGNodeMarginForAxis(
                                                        child,
                                                        mainAxis,
                                                        availableInnerWidth,
                                                    ))
                                            } else {
                                                lineHeight
                                            };

                                            let childHeight = if !isMainAxisRow {
                                                ((*child).layout.measured_dimensions.height
                                                    + YGNodeMarginForAxis(
                                                        child,
                                                        crossAxis,
                                                        availableInnerWidth,
                                                    ))
                                            } else {
                                                lineHeight
                                            };

                                            if !(YGFloatsEqual(
                                                childWidth,
                                                (*child).layout.measured_dimensions.width,
                                            )
                                                && YGFloatsEqual(
                                                    childHeight,
                                                    (*child).layout.measured_dimensions.height,
                                                )) {
                                                YGLayoutNodeInternal(
                                                    child,
                                                    childWidth,
                                                    childHeight,
                                                    direction,
                                                    MeasureMode::Exactly,
                                                    MeasureMode::Exactly,
                                                    availableInnerWidth,
                                                    availableInnerHeight,
                                                    true,
                                                    "multiline-stretch",
                                                    config,
                                                );
                                            }
                                        }
                                    }
                                    Align::Baseline => {
                                        (*child).layout.position[Edge::Top as usize] = currentLead
                                            + maxAscentForCurrentLine
                                            - YGBaseline(child)
                                            + YGNodeLeadingPosition(
                                                child,
                                                FlexDirection::Column,
                                                availableInnerCrossDim,
                                            );
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }

                    currentLead += lineHeight;
                }
            }

            // STEP 9: COMPUTING FINAL DIMENSIONS
            (*node).layout.measured_dimensions.width = YGNodeBoundAxis(
                node,
                FlexDirection::Row,
                availableWidth - marginAxisRow,
                parentWidth,
                parentWidth,
            );
            (*node).layout.measured_dimensions.height = YGNodeBoundAxis(
                node,
                FlexDirection::Column,
                availableHeight - marginAxisColumn,
                parentHeight,
                parentWidth,
            );

            // If the user didn't specify a width or height for the node, set the
            // dimensions based on the children.
            if measureModeMainDim == MeasureMode::Undefined
                || ((*node).style.overflow != Overflow::Scroll
                    && measureModeMainDim == MeasureMode::AtMost)
            {
                // Clamp the size to the min/max size, if specified, and make sure it
                // doesn't go below the padding and border amount.
                (*node).layout.measured_dimensions[DIM[mainAxis as usize]] = YGNodeBoundAxis(
                    node,
                    mainAxis,
                    maxLineMainDim,
                    mainAxisParentSize,
                    parentWidth,
                );
            } else if measureModeMainDim == MeasureMode::AtMost
                && (*node).style.overflow == Overflow::Scroll
            {
                (*node).layout.measured_dimensions[DIM[mainAxis as usize]] = (availableInnerMainDim
                    + paddingAndBorderAxisMain)
                    .min(YGNodeBoundAxisWithinMinAndMax(
                        node,
                        mainAxis,
                        maxLineMainDim,
                        mainAxisParentSize,
                    ))
                    .max(paddingAndBorderAxisMain);
            }

            if measureModeCrossDim == MeasureMode::Undefined
                || ((*node).style.overflow != Overflow::Scroll
                    && measureModeCrossDim == MeasureMode::AtMost)
            {
                // Clamp the size to the min/max size, if specified, and make sure it
                // doesn't go below the padding and border amount.
                (*node).layout.measured_dimensions[DIM[crossAxis as usize]] = YGNodeBoundAxis(
                    node,
                    crossAxis,
                    totalLineCrossDim + paddingAndBorderAxisCross,
                    crossAxisParentSize,
                    parentWidth,
                );
            } else if measureModeCrossDim == MeasureMode::AtMost
                && (*node).style.overflow == Overflow::Scroll
            {
                (*node).layout.measured_dimensions[DIM[crossAxis as usize]] =
                    (availableInnerCrossDim + paddingAndBorderAxisCross)
                        .max(YGNodeBoundAxisWithinMinAndMax(
                            node,
                            crossAxis,
                            totalLineCrossDim + paddingAndBorderAxisCross,
                            crossAxisParentSize,
                        ))
                        .max(paddingAndBorderAxisCross);
            }

            // As we only wrapped in normal direction yet, we need to reverse the positions on wrap-reverse.
            if performLayout && (*node).style.flex_wrap == YGWrapWrapReverse {
                for i in 0..childCount {
                    let child = YGNodeGetChild(node, i);
                    if (*child).style.position_type == PositionType::Relative {
                        (*child).layout.position[pos[crossAxis as usize] as usize] =
                            (*node).layout.measured_dimensions[DIM[crossAxis as usize]]
                                - (*child).layout.position[pos[crossAxis as usize] as usize]
                                - (*child).layout.measured_dimensions[DIM[crossAxis as usize]];
                    }
                }
            }

            if performLayout {
                // STEP 10: SIZING AND POSITIONING ABSOLUTE CHILDREN
                currentAbsoluteChild = firstAbsoluteChild;
                while !currentAbsoluteChild.is_null() {
                    YGNodeAbsoluteLayoutChild(
                        node,
                        currentAbsoluteChild,
                        availableInnerWidth,
                        if isMainAxisRow {
                            measureModeMainDim
                        } else {
                            measureModeCrossDim
                        },
                        availableInnerHeight,
                        direction,
                        config,
                    );
                    currentAbsoluteChild = (*currentAbsoluteChild).nextChild;
                }

                // STEP 11: SETTING TRAILING POSITIONS FOR CHILDREN
                let needsMainTrailingPos = mainAxis == FlexDirection::RowReverse
                    || mainAxis == FlexDirection::ColumnReverse;
                let needsCrossTrailingPos = crossAxis == FlexDirection::RowReverse
                    || crossAxis == FlexDirection::ColumnReverse;

                // Set trailing position if necessary.
                if needsMainTrailingPos || needsCrossTrailingPos {
                    for i in 0..childCount {
                        let child = YGNodeListGet((*node).children, i);
                        if (*child).style.display == Display::None {
                            continue;
                        }
                        if needsMainTrailingPos {
                            YGNodeSetChildTrailingPosition(node, child, mainAxis);
                        }

                        if needsCrossTrailingPos {
                            YGNodeSetChildTrailingPosition(node, child, crossAxis);
                        }
                    }
                }
            }
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
