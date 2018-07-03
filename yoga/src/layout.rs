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
    pub next_cached_measurements_index: usize,
    pub cached_measurements: [Option<CachedMeasurement>; MAX_CACHED_RESULTS],
    pub measured_dimensions: Option<MeasuredDimensions>,
    pub cached_layout: Option<CachedMeasurement>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
struct CachedMeasurement {
    available_width: R32,
    available_weight: R32,
    width_measure_mode: Option<MeasureMode>,
    height_measure_mode: Option<MeasureMode>,
    computed_width: R32,
    computed_height: R32,
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
