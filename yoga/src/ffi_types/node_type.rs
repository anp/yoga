use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Default = 0,
    Text = 1,
}

impl From<NodeType> for internal::YGNodeType {
    fn from(n: NodeType) -> internal::YGNodeType {
        match n {
            NodeType::Default => internal::YGNodeTypeDefault,
            NodeType::Text => internal::YGNodeTypeText,
        }
    }
}
