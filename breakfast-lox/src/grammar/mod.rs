use lalrpop_util::lalrpop_mod;

mod parse_error;

lalrpop_mod!(grammar, "/grammar/grammar.rs");

#[cfg(test)]
mod tests;
