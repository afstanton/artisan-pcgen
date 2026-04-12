//! BIOS settings schema.
//!
//! Source: `docs/listfilepages/systemfilestagpages/systemfilesbiosettingslist.html`
//!
//! BIOS settings lines are keyed by race selector with `RACENAME:x` heads.

use crate::schema::{
    ArtisanMapping, Cardinality, LineGrammar, HeadFormat, TokenDef, TokenGrammar,
};

static BIOSETTINGS_TOKENS: &[TokenDef] = &[
    TokenDef::text("BASEAGE", "base_age"),
    TokenDef::text("BASEAGEADD", "base_age_add"),
    TokenDef::text("MAXAGE", "max_age"),
    TokenDef::text("AGEDIEROLL", "age_die_roll"),
    TokenDef {
        key: "SEX",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("sex"),
        required: false,
    },
    TokenDef {
        key: "HAIR",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_hair"),
        required: false,
    },
    TokenDef {
        key: "EYES",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_eyes"),
        required: false,
    },
    TokenDef {
        key: "SKINTONE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_skintone"),
        required: false,
    },
];

pub static BIOSETTINGS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:biosettings",
    head_token: Some("RACENAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: BIOSETTINGS_TOKENS,
    globals: &[],
};
