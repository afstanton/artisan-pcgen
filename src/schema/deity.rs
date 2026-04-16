//! DEITY entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesdeities.html`
//!
//! Deity files define gods and similar divine patrons. The head is the deity
//! name with no token prefix.

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static DEITY_TOKENS: &[TokenDef] = &[
    TokenDef::text("NAMEISPI", "pcgen_nameispi"),
    TokenDef::text("ALIGN", "alignment"),
    TokenDef {
        key: "DOMAINS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("domains"),
        required: false,
    },
    TokenDef {
        key: "DEITYWEAP",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("deity_weapon"),
        required: false,
    },
    TokenDef {
        key: "GROUP",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("group"),
        required: false,
    },
    TokenDef::text("SYMBOL", "pcgen_symbol"),
    TokenDef::text("PANTHEON", "pcgen_pantheon"),
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    TokenDef::text("VISIBLE", "visible"),
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
    GlobalGroup::SortKey,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::SourceMeta,
];

pub static DEITY_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:deity",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: DEITY_TOKENS,
    globals: DEITY_GLOBALS,
};
