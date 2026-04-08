//! SPELL entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesspells.html`
//!
//! Spell files define individual spells. The head is the spell name only
//! (no token prefix). Spells have a rich set of descriptor tokens.

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static SKILL_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::SortKey,
    GlobalGroup::SourceMeta,
];

static SPELL_TOKENS: &[TokenDef] = &[
    // Core spell descriptor tokens (from datafilesspells.html)
    TokenDef::text("SCHOOL", "pcgen_school"),
    TokenDef::text("SUBSCHOOL", "pcgen_subschool"),
    TokenDef {
        key: "DESCRIPTOR",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_descriptors"),
        required: false,
    },
    // Class/domain spell levels
    TokenDef {
        key: "CLASSES",
        grammar: TokenGrammar::PipeGroups,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_classes"),
        required: false,
    },
    TokenDef {
        key: "DOMAINS",
        grammar: TokenGrammar::PipeGroups,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_domains"),
        required: false,
    },
    // Casting parameters
    TokenDef::text("COMPS", "pcgen_comps"),
    TokenDef::text("CASTTIME", "pcgen_casttime"),
    TokenDef::text("CT", "pcgen_ct"),
    TokenDef::text("RANGE", "pcgen_range"),
    TokenDef::text("TARGETAREA", "pcgen_targetarea"),
    TokenDef::text("DURATION", "pcgen_duration"),
    TokenDef::text("SAVEINFO", "pcgen_saveinfo"),
    TokenDef::text("SPELLRES", "pcgen_spellres"),
    // Cost and variants
    TokenDef::text("COST", "pcgen_cost"),
    TokenDef::integer("PPCOST", "pcgen_ppcost"),
    TokenDef {
        key: "SPELLPOINTCOST",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_spellpointcost"),
        required: false,
    },
    TokenDef {
        key: "ITEM",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_items"),
        required: false,
    },
    TokenDef {
        key: "VARIANTS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_variants"),
        required: false,
    },
];

pub static SPELL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:spell",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: SPELL_TOKENS,
    globals: SKILL_GLOBALS,
};
