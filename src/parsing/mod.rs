use lalrpop_util::lalrpop_mod;

pub(crate) mod line_codec;
pub(crate) mod parser_tokens;
pub(crate) mod modify_parser;
pub(crate) mod modify_lexer;

lalrpop_mod!(pub(crate) clause_grammar, "/parsing/clause_grammar.rs");
lalrpop_mod!(pub(crate) line_grammar, "/parsing/line_grammar.rs");

pub use modify_parser::{parse_modify, ModifyExpression, ModifyOp};
