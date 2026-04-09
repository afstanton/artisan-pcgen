//! DEITY entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesdeities.html`
//!
//! Deity files define gods and similar divine patrons. The head is the deity
//! name with no token prefix.

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static DEITY_TOKENS: &[TokenDef] = &[
    TokenDef::text("NAMEISPI", "pcgen_nameispi"),
    TokenDef::text("ALIGN", "pcgen_align"),
    TokenDef {
        key: "DOMAINS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_domains"),
        required: false,
    },
    TokenDef {
        key: "DEITYWEAP",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_deityweap"),
        required: false,
    },
    TokenDef {
        key: "GROUP",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_group"),
        required: false,
    },
];

static DEITY_GLOBALS: &[GlobalGroup] = &[
    GlobalGroup::Type,
    GlobalGroup::Key,
    GlobalGroup::Desc,
    GlobalGroup::Fact,
    GlobalGroup::Bonus,
    GlobalGroup::Add,
    GlobalGroup::Choose,
    GlobalGroup::Auto,
    GlobalGroup::Define,
    GlobalGroup::Modify,
    GlobalGroup::Prerequisites,
    GlobalGroup::SourcePage,
    GlobalGroup::SourceLink,
    GlobalGroup::OutputName,
    GlobalGroup::SourceMeta,
];

pub static DEITY_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:deity",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: DEITY_TOKENS,
    globals: DEITY_GLOBALS,
};
