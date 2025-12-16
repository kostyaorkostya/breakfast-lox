use lalrpop_util::lalrpop_mod;

mod parse_error;
pub use parse_error::ParseError;

lalrpop_mod!(grammar, "/grammar/grammar.rs");
pub use grammar::ExprParser;

#[cfg(test)]
mod tests;
