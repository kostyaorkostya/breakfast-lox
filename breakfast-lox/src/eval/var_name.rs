use crate::ast;
use nutype::nutype;

#[nutype(derive(Debug, Display, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, FromStr))]
pub struct VarName(String);

impl From<ast::VarName> for VarName {
    fn from(value: ast::VarName) -> Self {
        Self::new(value.into_inner())
    }
}
