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
    TokenDef {
        key: "RACESUBTYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("race_subtype"),
        required: false,
    },
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
    TokenDef::pipe_positional_repeatable("KIT", &["count", "kit"], "kits"),
    // MOVE, NATURALATTACKS, VISION, DR, SR — mechanical properties of companion creature type.
    TokenDef {
        key: "MOVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("move"),
        required: false,
    },
    TokenDef {
        key: "NATURALATTACKS",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("natural_attacks"),
        required: false,
    },
    TokenDef {
        key: "VISION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("vision"),
        required: false,
    },
    TokenDef::text("DR", "pcgen_dr"),
    TokenDef::text("SR", "pcgen_sr"),
    TokenDef::text("FACE", "face"),
    TokenDef::text("LEGS", "legs"),
    TokenDef::text("HANDS", "hands"),
    TokenDef::text("CR", "cr"),
    TokenDef::text("SIZE", "size"),
    // SPELLS: spell-like abilities of the companion.
    TokenDef {
        key: "SPELLS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spells"),
        required: false,
    },
];

static MASTERBONUSRACE_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    // SR: spell resistance granted by the master bonus race.
    TokenDef::text("SR", "pcgen_sr"),
];

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
        GlobalGroup::CSkill,
        GlobalGroup::OutputName,
        GlobalGroup::SortKey,
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
        GlobalGroup::SortKey,
        GlobalGroup::SourcePage,
        GlobalGroup::SourceLink,
        GlobalGroup::SourceMeta,
    ],
};
