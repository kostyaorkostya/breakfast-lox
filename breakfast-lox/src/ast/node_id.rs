use super::Node;
use nutype::nutype;
use std::ops::Range;

// LALRPOP @L/@R are `usize`, and that's the upper bound on the number of AST nodes, so make it
// `usize` too.
#[nutype(derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsRef, Deref, Into, From, Hash
))]
pub struct NodeId(usize);

pub trait NodeIdGen {
    fn next(&mut self) -> NodeId;

    fn new_node<T>(&mut self, loc: Range<usize>, kind: T) -> Node<T>
    where
        Self: Sized,
    {
        Node::new(self.next(), false, Some(loc), kind)
    }

    fn new_synth_node<T>(&mut self, kind: T) -> Node<T>
    where
        Self: Sized,
    {
        Node::new(self.next(), true, None, kind)
    }

    fn new_synth_with_loc_node<T>(&mut self, loc: Range<usize>, kind: T) -> Node<T>
    where
        Self: Sized,
    {
        Node::new(self.next(), true, Some(loc), kind)
    }
}

#[derive(Debug, Default)]
pub struct SeqNodeIdGen(usize);

impl SeqNodeIdGen {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NodeIdGen for SeqNodeIdGen {
    fn next(&mut self) -> NodeId {
        let ret: NodeId = self.0.into();
        self.0 = self.0 + 1;
        ret
    }
}
