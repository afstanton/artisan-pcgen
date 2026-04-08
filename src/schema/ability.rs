//! ABILITY entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesability.html`
//!
//! Ability files define the individual class/racial abilities that make up each
//! character. The first field is the Ability Name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static ASPECT_SLOTS: &[&str] = &["name", "value", "formula"];

static ABILITY_TOKENS: &[TokenDef] = &[
    // Required: every ability must belong to a category
    TokenDef::text_required("CATEGORY", "pcgen_category"),
    // Optional entity-specific tokens (doc order)
    TokenDef::integer("ADDSPELLLEVEL", "pcgen_addspelllevel"),
    TokenDef::pipe_positional_repeatable("ASPECT", ASPECT_SLOTS, "pcgen_aspects"),
    TokenDef::text("BENEFIT", "pcgen_benefit"),
    TokenDef::text("COST", "pcgen_cost"),
    TokenDef::text("MULT", "pcgen_mult"),
    TokenDef::text("STACK", "pcgen_stack"),
    TokenDef {
        key: "TEMPLATE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Effect,
        required: false,
    },
    TokenDef::text("VISIBLE", "pcgen_visible"),
    // Spell-like abilities: SPELLS:mode|TIMES=formula|CASTERLEVEL=formula|spell,...
    TokenDef {
        key: "SPELLS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_spells"),
        required: false,
    },
    TokenDef {
        key: "SPELLKNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_spellknown"),
        required: false,
    },
    TokenDef {
        key: "MOVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_move"),
        required: false,
    },
    TokenDef {
        key: "NATURALATTACKS",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_naturalattacks"),
        required: false,
    },
    // Equipment modification grant: EQMOD:name|key=value...
    TokenDef {
        key: "EQMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_eqmods"),
        required: false,
    },
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
    GlobalGroup::Prerequisites,
    GlobalGroup::SourcePage,
    GlobalGroup::OutputName,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::Qualify,
    GlobalGroup::SourceMeta,
];

pub static ABILITY_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:ability",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: ABILITY_TOKENS,
    globals: ABILITY_GLOBALS,
};
