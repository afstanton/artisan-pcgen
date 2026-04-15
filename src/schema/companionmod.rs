//! Companion modifier schemas.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilescompanionmodifiers.html`
//!
//! Companion modifier files define two line types using token-prefixed heads:
//! `FOLLOWER:x=y` and `MASTERBONUSRACE:x`.

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static FOLLOWER_TOKENS: &[TokenDef] = &[
    TokenDef::text("HD", "hitdie"),
    TokenDef::text("RACETYPE", "racetype"),
    TokenDef::text("COPYMASTERBAB", "pcgen_copymasterbab"),
    TokenDef::text("COPYMASTERCHECK", "pcgen_copymastercheck"),
    TokenDef::text("COPYMASTERHP", "pcgen_copymasterhp"),
    TokenDef::yesno("USEMASTERSKILL", "pcgen_usemasterskill"),
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    TokenDef::pipe_positional_repeatable("KIT", &["count", "kit"], "pcgen_kits"),
];

static MASTERBONUSRACE_TOKENS: &[TokenDef] = &[TokenDef {
    key: "ABILITY",
    grammar: TokenGrammar::PipeList,
    cardinality: Cardinality::Repeatable,
    artisan_mapping: ArtisanMapping::Field("abilities"),
    required: false,
}];

pub static FOLLOWER_COMPANIONMOD_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:companionmod-follower",
    head_token: Some("FOLLOWER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: FOLLOWER_TOKENS,
    globals: &[
        GlobalGroup::Type,
        GlobalGroup::Bonus,
        GlobalGroup::Add,
        GlobalGroup::Choose,
        GlobalGroup::Auto,
        GlobalGroup::Define,
        GlobalGroup::Modify,
        GlobalGroup::Prerequisites,
        GlobalGroup::Template,
        GlobalGroup::Sab,
        GlobalGroup::OutputName,
        GlobalGroup::SourcePage,
        GlobalGroup::SourceLink,
        GlobalGroup::SourceMeta,
    ],
};

pub static MASTERBONUSRACE_COMPANIONMOD_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:companionmod-masterbonusrace",
    head_token: Some("MASTERBONUSRACE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: MASTERBONUSRACE_TOKENS,
    globals: &[
        GlobalGroup::Type,
        GlobalGroup::Bonus,
        GlobalGroup::Add,
        GlobalGroup::Choose,
        GlobalGroup::Auto,
        GlobalGroup::Define,
        GlobalGroup::Modify,
        GlobalGroup::Prerequisites,
        GlobalGroup::Template,
        GlobalGroup::Sab,
        GlobalGroup::OutputName,
        GlobalGroup::SourcePage,
        GlobalGroup::SourceLink,
        GlobalGroup::SourceMeta,
    ],
};
