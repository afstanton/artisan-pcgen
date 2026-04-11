//! Variable-definition schemas.
//!
//! Sources:
//! - `docs/listfilepages/lstfileclass/lfc_lesson_variables.txt`
//! - corpus `*_variables.lst` files

use crate::schema::{EntitySchema, HeadFormat, TokenDef};

static LOCAL_VARIABLE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("LOCAL", "pcgen_local"),
    TokenDef::text("EXPLANATION", "pcgen_explanation"),
];

static GLOBAL_VARIABLE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("GLOBAL", "pcgen_global"),
    TokenDef::text("EXPLANATION", "pcgen_explanation"),
];

pub static LOCAL_VARIABLE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:variable-local",
    head_token: Some("LOCAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: LOCAL_VARIABLE_TOKENS,
    globals: &[],
};

pub static GLOBAL_VARIABLE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:variable-global",
    head_token: Some("GLOBAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: GLOBAL_VARIABLE_TOKENS,
    globals: &[],
};

// _variables.lst: channel variable definition, e.g. CHANNEL:PC.STAT|NUMBER=StatScore
static CHANNEL_VARIABLE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("CHANNEL", "pcgen_channel"),
    TokenDef::text("NUMBER", "pcgen_number"),
    TokenDef::text("EXPLANATION", "pcgen_explanation"),
];

pub static CHANNEL_VARIABLE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:variable-channel",
    head_token: Some("CHANNEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: CHANNEL_VARIABLE_TOKENS,
    globals: &[],
};
