use nutype::nutype;

#[nutype(derive(Debug, Display, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, FromStr))]
pub struct VarName(String);
