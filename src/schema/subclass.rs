//! SUBCLASS entity schema.
//!
//! SUBCLASS lines define class specialization options. The head is
//! token-prefixed: `SUBCLASS:name`.

use crate::schema::{ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar};

static SUBCLASS_TOKENS: &[TokenDef] = &[
    TokenDef::text("COST", "cost"),
    // ABILITY grants abilities at the subclass level (e.g. bonus proficiencies).
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    TokenDef::pipe_list_repeatable("CHOICE", "pcgen_choice"),
    TokenDef::pipe_list_repeatable("CCSKILL", "pcgen_ccskill"),
    TokenDef::pipe_list_repeatable("UNENCUMBEREDMOVE", "pcgen_unencumberedmove"),
    TokenDef::pipe_positional_repeatable("SPELLLIST", &["level", "list"], "pcgen_spelllist"),
    TokenDef::text("KNOWNSPELLSFROMSPECIALTY", "known_spells_from_specialty"),
    TokenDef::integer("PROHIBITCOST", "pcgen_prohibitcost"),
    // SPELLSTAT: ability score used for spell DCs and bonus spells (same as class-level SPELLSTAT).
    TokenDef::text("SPELLSTAT", "spellstat"),
    // STARTSKILLPTS: skill points at first subclass level.
    TokenDef::text("STARTSKILLPTS", "pcgen_startskillpts"),
    // DOMAIN: domain list for divine subclasses.
    TokenDef {
        key: "DOMAIN",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("domains"),
        required: false,
    },
    // PROHIBITED: prohibited spell school(s).
    TokenDef {
        key: "PROHIBITED",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_prohibited"),
        required: false,
    },
];

static SUBCLASS_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::LangBonus,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::ChangeProf,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::Template,
    GlobalGroup::SourceMeta,
];

pub static SUBCLASS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:subclass",
    head_token: Some("SUBCLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SUBCLASS_TOKENS,
    globals: SUBCLASS_GLOBALS,
};
