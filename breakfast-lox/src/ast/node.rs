use super::NodeId;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: NodeId,
    pub synthetic: bool,
    // `None` if lost due to desugaring
    pub loc: Option<Range<usize>>,
    pub kind: T,
}

impl<T> Node<T> {
    pub fn new(id: NodeId, synthetic: bool, loc: Option<Range<usize>>, kind: T) -> Self {
        Self {
            id,
            synthetic,
            loc,
            kind,
        }
    }
}
