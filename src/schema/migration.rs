//! Migration-file schemas for PCGen rename/update records.
//!
//! These records appear in game-mode migration files and use token-prefixed
//! heads such as `ABILITY:FEAT|Old Name` with metadata tokens like `MAXVER`,
//! `NEWKEY`, and sometimes `NEWCATEGORY`.

use crate::schema::{HeadFormat, LineGrammar, TokenDef};

pub static ABILITY_MIGRATION_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:ability-migration",
    head_token: Some("ABILITY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[
        TokenDef::text("MAXVER", "max_version"),
        TokenDef::text("MAXDEVVER", "pcgen_maxdevver"),
        TokenDef::text("NEWKEY", "new_key"),
        TokenDef::text("NEWCATEGORY", "newcategory"),
    ],
    globals: &[],
};
