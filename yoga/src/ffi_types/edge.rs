use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Edge {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Start = 4,
    End = 5,
    Horizontal = 6,
    Vertical = 7,
    All = 8,
}

impl From<Edge> for internal::YGEdge {
    fn from(e: Edge) -> internal::YGEdge {
        match e {
            Edge::Left => internal::YGEdgeLeft,
            Edge::Top => internal::YGEdgeTop,
            Edge::Right => internal::YGEdgeRight,
            Edge::Bottom => internal::YGEdgeBottom,
            Edge::Start => internal::YGEdgeStart,
            Edge::End => internal::YGEdgeEnd,
            Edge::Horizontal => internal::YGEdgeHorizontal,
            Edge::Vertical => internal::YGEdgeVertical,
            Edge::All => internal::YGEdgeAll,
        }
    }
}
