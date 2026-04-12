//! MODIFY entity schema
//!
//! MODIFY tokens are directives that modify variable values and appear primarily
//! on stat lines (PF2e size adjustments) and race entities.
//!
//! Format: MODIFY:VarName|Operation|Value
//! where Operation is ADD, SET, or SOLVE

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static MODIFY_TOKENS: &[TokenDef] = &[
    // Note: MODIFY values are complex and can contain formulas. They are stored as-is.
    // The three components (variable, operation, value) are project as separate attributes.
];

static MODIFY_GLOBALS: &[GlobalGroup] = &[GlobalGroup::SourceMeta];

pub static MODIFY_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:modify",
    head_token: Some("MODIFY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: MODIFY_TOKENS,
    globals: MODIFY_GLOBALS,
};
