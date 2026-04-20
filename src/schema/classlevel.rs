//! CLASS level entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesclasses.html`
//!
//! Class level lines use the level number as the head with no token prefix.

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static CLASSLEVEL_TOKENS: &[TokenDef] = &[
    // DR: damage reduction granted at this class level.
    TokenDef::text("DR", "pcgen_dr"),
    // MOVE: movement speed granted or modified at this class level.
    TokenDef {
        key: "MOVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("move"),
        required: false,
    },
    TokenDef {
        key: "DONOTADD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("do_not_add"),
        required: false,
    },
    TokenDef::text("UDAM", "udam"),
    TokenDef::integer("UMULT", "pcgen_umult"),
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    TokenDef {
        key: "SPELLKNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("spell_known"),
        required: false,
    },
    TokenDef {
        key: "CAST",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("cast"),
        required: false,
    },
    TokenDef {
        key: "KNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("known"),
        required: false,
    },
    TokenDef {
        key: "SPECIALTYKNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_specialtyknown"),
        required: false,
    },
];

static CLASSLEVEL_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::LangBonus,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::Template,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::SourceMeta,
];

pub static CLASSLEVEL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:classlevel",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: CLASSLEVEL_TOKENS,
    globals: CLASSLEVEL_GLOBALS,
};
