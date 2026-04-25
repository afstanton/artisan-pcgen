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
    GlobalGroup::Sab,
    GlobalGroup::CSkill,
    GlobalGroup::ServesAs,
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
    // INFO: free-text annotation for the spell (appears in some Pathfinder datasets).
    TokenDef::text("INFO", "pcgen_info"),
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
    // VISIBLE controls whether the spell appears in the spell list UI.
    TokenDef::text("VISIBLE", "visible"),
    // SPELLBASEDC: base difficulty class formula (e.g. SPELLBASEDC:10+SPELLLEVEL+WIS).
    TokenDef::text("SPELLBASEDC", "spell_base_dc"),
    // NAMEOPT: name-display option (e.g. NAMEOPT:NONAME or NAMEOPT:SPELL).
    TokenDef::text("NAMEOPT", "pcgen_nameopt"),
    // SPROP: special property text displayed on spell detail (seen in some datasets).
    TokenDef {
        key: "SPROP",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("sprop"),
        required: false,
    },
    // DR: damage reduction grant (some datasets attach DR to spell-like effects).
    TokenDef {
        key: "DR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_dr"),
        required: false,
    },
    // ITYPE: item type filter (some 3rd-party datasets use on spell entities).
    TokenDef {
        key: "ITYPE",
        grammar: TokenGrammar::DotList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_itype"),
        required: false,
    },
    // VISION: vision grant on spell-like entities.
    TokenDef {
        key: "VISION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("vision"),
        required: false,
    },
    // HITDIE: hit-die expression (some datasets apply HD on spell-like entities).
    TokenDef::text("HITDIE", "hitdie"),
    // NOTE: free-text annotation (seen in some power/spell datasets).
    TokenDef::text("NOTE", "note"),
];

pub static SPELL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:spell",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: SPELL_TOKENS,
    globals: SKILL_GLOBALS,
};
