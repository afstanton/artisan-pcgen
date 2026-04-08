//! STARTPACK entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesstartingkits.html`
//!
//! Starting kit files use `STARTPACK:name` heads and mostly consist of global
//! effect and prerequisite tags.

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static STARTPACK_TOKENS: &[TokenDef] = &[
    TokenDef::text("APPLY", "pcgen_apply"),
    TokenDef {
        key: "LOOKUP",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_lookup"),
        required: false,
    },
];

pub static STARTPACK_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:startpack",
    head_token: Some("STARTPACK"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: STARTPACK_TOKENS,
    globals: &[
        GlobalGroup::Prerequisites,
        GlobalGroup::Bonus,
        GlobalGroup::Add,
        GlobalGroup::Choose,
        GlobalGroup::Auto,
        GlobalGroup::Define,
        GlobalGroup::LangBonus,
        GlobalGroup::SourceMeta,
    ],
};
