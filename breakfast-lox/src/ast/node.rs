use super::{NodeId, NodeIdGen};
use std::ops::{Fn, Range};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: NodeId,
    pub synthetic: bool,
    // `None` if lost due to desugaring
    pub loc: Option<Range<usize>>,
    pub kind: T,
}

pub fn node<T>(ids: &mut dyn NodeIdGen, loc: Range<usize>, kind: T) -> Node<T> {
    Node::new(ids.next(), false, Some(loc), kind)
}

pub fn synth_node<T>(ids: &mut dyn NodeIdGen, kind: T) -> Node<T> {
    Node::new(ids.next(), true, None, kind)
}

pub fn synth_with_loc_node<T>(ids: &mut dyn NodeIdGen, loc: Range<usize>, kind: T) -> Node<T> {
    Node::new(ids.next(), true, Some(loc), kind)
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

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Node<U> {
        let Self {
            id,
            synthetic,
            loc,
            kind,
        } = self;
        Node {
            id,
            synthetic,
            loc,
            kind: f(kind),
        }
    }
}
