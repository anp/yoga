prelude!();

use ffi_types::{dimension::MeasuredDimensions, direction::Direction, measure_mode::MeasureMode};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Layout {
    position: [F32; 4],
    dimensions: [Option<F32>; 2],
    margin: [F32; 6],
    border: [F32; 6],
    padding: [F32; 6],
    direction: Direction,
    computedFlexBasisGeneration: u32,
    computedFlexBasis: Option<F32>,
    hadOverflow: bool,
    generationCount: u32,
    lastParentDirection: Direction,
    nextCachedMeasurementsIndex: usize,
    cachedMeasurements: [Option<CachedMeasurement>; 16],
    measuredDimensions: Option<MeasuredDimensions>,
    cachedLayout: Option<CachedMeasurement>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
struct CachedMeasurement {
    availableWidth: F32,
    availableHeight: F32,
    widthMeasureMode: Option<MeasureMode>,
    heightMeasureMode: Option<MeasureMode>,
    computedWidth: F32,
    computedHeight: F32,
}

impl ::std::default::Default for Layout {
    fn default() -> Self {
        Layout {
            position: [*ZERO; 4],
            dimensions: [None, None],
            margin: [*ZERO; 6],
            border: [*ZERO; 6],
            padding: [*ZERO; 6],
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

pub unsafe fn YGNodeLayoutGetLeft(node: Node) -> c_float {
    return (*node).layout.position[Edge::Left as usize];
}

pub unsafe fn YGNodeLayoutGetTop(node: Node) -> c_float {
    return (*node).layout.position[Edge::Top as usize];
}

pub unsafe fn YGNodeLayoutGetRight(node: Node) -> c_float {
    return (*node).layout.position[Edge::Right as usize];
}

pub unsafe fn YGNodeLayoutGetBottom(node: Node) -> c_float {
    return (*node).layout.position[Edge::Bottom as usize];
}

pub unsafe fn YGNodeLayoutGetWidth(node: Node) -> c_float {
    return (*node).layout.dimensions[Dimension::Width as usize];
}

pub unsafe fn YGNodeLayoutGetHeight(node: Node) -> c_float {
    return (*node).layout.dimensions[Dimension::Height as usize];
}

pub unsafe fn YGNodeLayoutGetDirection(node: Node) -> Direction {
    return (*node).layout.direction;
}

pub unsafe fn YGNodeLayoutGetHadOverflow(node: Node) -> bool {
    return (*node).layout.hadOverflow;
}

pub unsafe fn YGNodeLayoutGetMargin(node: Node, edge: Edge) -> c_float {
    YGAssertWithNode(
        node,
        (edge) < Edge::End,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8 as *const c_char,
    );
    if edge == Edge::Left {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.margin[Edge::End as usize];
        } else {
            return (*node).layout.margin[Edge::Start as usize];
        };
    };
    if edge == Edge::Right {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.margin[Edge::Start as usize];
        } else {
            return (*node).layout.margin[Edge::End as usize];
        };
    };
    return (*node).layout.margin[edge as usize];
}

pub unsafe fn YGNodeLayoutGetBorder(node: Node, edge: Edge) -> c_float {
    YGAssertWithNode(
        node,
        (edge) < Edge::End,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8 as *const c_char,
    );
    if edge == Edge::Left {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.border[Edge::End as usize];
        } else {
            return (*node).layout.border[Edge::Start as usize];
        };
    };
    if edge == Edge::Right {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.border[Edge::Start as usize];
        } else {
            return (*node).layout.border[Edge::End as usize];
        };
    };
    return (*node).layout.border[edge as usize];
}

pub unsafe fn YGNodeLayoutGetPadding(node: Node, edge: Edge) -> c_float {
    YGAssertWithNode(
        node,
        (edge) < Edge::End,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8 as *const c_char,
    );
    if edge == Edge::Left {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.padding[Edge::End as usize];
        } else {
            return (*node).layout.padding[Edge::Start as usize];
        };
    };
    if edge == Edge::Right {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.padding[Edge::Start as usize];
        } else {
            return (*node).layout.padding[Edge::End as usize];
        };
    };
    return (*node).layout.padding[edge as usize];
}
