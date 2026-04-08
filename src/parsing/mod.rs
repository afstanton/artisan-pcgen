use lalrpop_util::lalrpop_mod;

pub(crate) mod line_codec;
pub(crate) mod parser_tokens;

lalrpop_mod!(pub(crate) clause_grammar, "/parsing/clause_grammar.rs");
lalrpop_mod!(pub(crate) line_grammar, "/parsing/line_grammar.rs");
