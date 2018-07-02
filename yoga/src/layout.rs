prelude!();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Layout {
    left: R32,
    right: R32,
    top: R32,
    bottom: R32,
    pub dimensions: Option<Dimensions>,
    pub direction: Direction,
    margin: Edges<R32>,
    border: Edges<R32>,
    padding: Edges<R32>,
    computedFlexBasisGeneration: u32,
    pub computedFlexBasis: Option<R32>,
    pub hadOverflow: bool,
    generationCount: u32,
    lastParentDirection: Direction,
    nextCachedMeasurementsIndex: usize,
    cachedMeasurements: [Option<CachedMeasurement>; 16],
    measuredDimensions: Option<MeasuredDimensions>,
    cachedLayout: Option<CachedMeasurement>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
struct CachedMeasurement {
    availableWidth: R32,
    availableHeight: R32,
    widthMeasureMode: Option<MeasureMode>,
    heightMeasureMode: Option<MeasureMode>,
    computedWidth: R32,
    computedHeight: R32,
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
            computedFlexBasisGeneration: 0,
            computedFlexBasis: None,
            hadOverflow: false,
            generationCount: 0,
            // RIIR(anp): this is not technically correct, it was uninit  before
            lastParentDirection: Direction::Inherit,
            nextCachedMeasurementsIndex: 0,
            cachedMeasurements: [None; 16],
            measuredDimensions: None,
            cachedLayout: None,
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
