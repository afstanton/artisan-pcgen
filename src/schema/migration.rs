//! Migration-file schemas for PCGen rename/update records.
//!
//! These records appear in game-mode migration files and use token-prefixed
//! heads such as `ABILITY:FEAT|Old Name` with metadata tokens like `MAXVER`,
//! `NEWKEY`, and sometimes `NEWCATEGORY`.

use crate::schema::{EntitySchema, HeadFormat, TokenDef};

pub static ABILITY_MIGRATION_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:ability-migration",
    head_token: Some("ABILITY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[
        TokenDef::text("MAXVER", "pcgen_maxver"),
        TokenDef::text("MAXDEVVER", "pcgen_maxdevver"),
        TokenDef::text("NEWKEY", "pcgen_newkey"),
        TokenDef::text("NEWCATEGORY", "pcgen_newcategory"),
    ],
    globals: &[],
};
