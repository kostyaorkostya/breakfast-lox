use nutype::nutype;

// LALRPOP @L/@R are `usize`, and that's the upper bound on the number of AST nodes, so make it
// `usize` too.
#[nutype(derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsRef, Deref, Into, From, Hash
))]
pub struct NodeId(usize);

pub trait NodeIdGen {
    fn next(&mut self) -> NodeId;
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
