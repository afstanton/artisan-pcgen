//! ABILITY entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesability.html`
//!
//! Ability files define the individual class/racial abilities that make up each
//! character. The first field is the Ability Name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static ASPECT_SLOTS: &[&str] = &["name", "value", "formula"];

static ABILITY_TOKENS: &[TokenDef] = &[
    // Required: every ability must belong to a category
    TokenDef::text_required("CATEGORY", "category"),
    // Optional entity-specific tokens (doc order)
    TokenDef::integer("ADDSPELLLEVEL", "pcgen_addspelllevel"),
    TokenDef::pipe_positional_repeatable("ASPECT", ASPECT_SLOTS, "aspects"),
    TokenDef::text("BENEFIT", "benefit"),
    TokenDef::text("COST", "cost"),
    TokenDef::text("SPELLLEVEL", "spell_level"),
    TokenDef::text("NEWCATEGORY", "newcategory"),
    TokenDef::text("INFO", "pcgen_info"),
    TokenDef::text("MULT", "pcgen_mult"),
    TokenDef::text("STACK", "pcgen_stack"),
    TokenDef {
        key: "TEMPVALUE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_tempvalue"),
        required: false,
    },
    TokenDef {
        key: "TEMPLATE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Effect,
        required: false,
    },
    TokenDef::text("VISIBLE", "visible"),
    // Spell-like abilities: SPELLS:mode|TIMES=formula|CASTERLEVEL=formula|spell,...
    TokenDef {
        key: "SPELLS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spells"),
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
        key: "MOVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("move"),
        required: false,
    },
    TokenDef {
        key: "MOVECLONE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_moveclone"),
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
    // Equipment modification grant: EQMOD:name|key=value...
    TokenDef {
        key: "EQMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_eqmods"),
        required: false,
    },
    TokenDef::pipe_list_repeatable("COMPANIONLIST", "companion_list"),
    TokenDef::pipe_list_repeatable("CCSKILL", "pcgen_ccskill"),
    TokenDef::pipe_list_repeatable("UNENCUMBEREDMOVE", "pcgen_unencumberedmove"),
    TokenDef::pipe_positional_repeatable("FOLLOWERS", &["type", "limit"], "pcgen_followers"),
    // .pcg sub-token: the choice(s) this feat/ability was applied to
    TokenDef::text("APPLIEDTO", "pcgen_appliedto"),
];

static ABILITY_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::ChangeProf,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::SourceMeta,
];

pub static ABILITY_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:ability",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: ABILITY_TOKENS,
    globals: ABILITY_GLOBALS,
};
