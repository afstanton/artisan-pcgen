//! BIOS settings schema.
//!
//! Source: `docs/listfilepages/systemfilestagpages/systemfilesbiosettingslist.html`
//!
//! BIOS settings lines are keyed by race selector with `RACENAME:x` heads.

use crate::schema::{ArtisanMapping, Cardinality, EntitySchema, HeadFormat, TokenDef, TokenGrammar};

static BIOSETTINGS_TOKENS: &[TokenDef] = &[
    TokenDef::text("BASEAGE", "pcgen_baseage"),
    TokenDef::text("BASEAGEADD", "pcgen_baseageadd"),
    TokenDef::text("MAXAGE", "pcgen_maxage"),
    TokenDef::text("AGEDIEROLL", "pcgen_agedieroll"),
    TokenDef {
        key: "SEX",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_sex"),
        required: false,
    },
    TokenDef {
        key: "HAIR",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_hair"),
        required: false,
    },
    TokenDef {
        key: "EYES",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_eyes"),
        required: false,
    },
    TokenDef {
        key: "SKINTONE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_skintone"),
        required: false,
    },
];

pub static BIOSETTINGS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:biosettings",
    head_token: Some("RACENAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: BIOSETTINGS_TOKENS,
    globals: &[],
};
