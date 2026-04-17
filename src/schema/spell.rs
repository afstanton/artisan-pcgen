//! SPELL entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesspells.html`
//!
//! Spell files define individual spells. The head is the spell name only
//! (no token prefix). Spells have a rich set of descriptor tokens.

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
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
    GlobalGroup::Modify,
    GlobalGroup::Prerequisites,
    GlobalGroup::SourcePage,
    GlobalGroup::SourceLink,
    GlobalGroup::OutputName,
    GlobalGroup::SortKey,
    GlobalGroup::SourceMeta,
];

static SPELL_TOKENS: &[TokenDef] = &[
    // CATEGORY:SPELL is written by some publishers (notably 5e/Pathfinder data)
    // to mark an entity as a spell. It must be emitted on roundtrip or the
    // attribute is silently dropped for those spell records.
    TokenDef::text("CATEGORY", "category"),
    // Core spell descriptor tokens (from datafilesspells.html)
    TokenDef::text("SCHOOL", "school"),
    TokenDef::text("SUBSCHOOL", "subschool"),
    TokenDef {
        key: "DESCRIPTOR",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("descriptors"),
        required: false,
    },
    // Class/domain spell levels.
    // Cardinality is Repeatable because some spell files carry multiple
    // CLASSES: tokens on a single spell line (one per caster group), and
    // joining them into one token would merge distinct groups.
    TokenDef {
        key: "CLASSES",
        grammar: TokenGrammar::PipeGroups,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("classes"),
        required: false,
    },
    // DOMAINS can also appear multiple times (though rarely).
    TokenDef {
        key: "DOMAINS",
        grammar: TokenGrammar::PipeGroups,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("domains"),
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
    TokenDef {
        key: "TEMPVALUE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_tempvalue"),
        required: false,
    },
    // Cost and variants
    TokenDef::text("COST", "cost"),
    TokenDef::text("XPCOST", "pcgen_xpcost"),
    TokenDef::integer("PPCOST", "pcgen_ppcost"),
    TokenDef {
        key: "SPELLPOINTCOST",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_spellpointcost"),
        required: false,
    },
    TokenDef {
        key: "ITEM",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("items"),
        required: false,
    },
    TokenDef {
        key: "VARIANTS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_variants"),
        required: false,
    },
];

pub static SPELL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:spell",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: SPELL_TOKENS,
    globals: SKILL_GLOBALS,
};
