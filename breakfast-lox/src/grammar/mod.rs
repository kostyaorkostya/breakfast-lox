use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar, "/grammar/grammar.rs");

#[cfg(test)]
mod tests;
