//! CLASS entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesclasses.html`
//!
//! Class files define character classes. The head is token-prefixed: `CLASS:name`.

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static SPELLLIST_SLOTS: &[&str] = &["count", "type", "class"];

static CLASS_TOKENS: &[TokenDef] = &[
    // Core class statistics
    TokenDef::integer("HITDIE", "pcgen_hitdie"),
    TokenDef::integer("HD", "pcgen_hitdie"),
    TokenDef::text("MAXLEVEL", "pcgen_maxlevel"),
    TokenDef::text("ABB", "pcgen_abbreviation"),
    TokenDef::integer("STARTSKILLPTS", "pcgen_startskillpts"),
    TokenDef::integer("LEVELSPERFEAT", "pcgen_levelsperfeat"),
    TokenDef::text("ATTACKCYCLE", "pcgen_attackcycle"),
    // Spell-related
    TokenDef::text("SPELLTYPE", "pcgen_spelltype"),
    TokenDef::text("SPELLSTAT", "pcgen_spellstat"),
    TokenDef::text("BONUSSPELLSTAT", "pcgen_bonusspellstat"),
    TokenDef {
        key: "CAST",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_cast"),
        required: false,
    },
    TokenDef {
        key: "KNOWN",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_known"),
        required: false,
    },
    TokenDef {
        key: "KNOWNSPELLS",
        grammar: TokenGrammar::PipeGroups,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_knownspells"),
        required: false,
    },
    TokenDef {
        key: "SPECIALTYKNOWN",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_specialtyknown"),
        required: false,
    },
    TokenDef::pipe_positional("SPELLLIST", SPELLLIST_SLOTS, "pcgen_spelllist"),
    TokenDef {
        key: "PROHIBITSPELL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_prohibitspell"),
        required: false,
    },
    TokenDef::text("KNOWNSPELLSFROMSPECIALTY", "pcgen_knownspellsfromspecialty"),
    // Domain
    TokenDef {
        key: "ADDDOMAINS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_adddomains"),
        required: false,
    },
    TokenDef {
        key: "DOMAIN",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_domains"),
        required: false,
    },
    // Class mechanics
    TokenDef::yesno("ALLOWBASECLASS", "pcgen_allowbaseclass"),
    TokenDef::text("COST", "pcgen_cost"),
    TokenDef::yesno("MODTOSKILLS", "pcgen_modtoskills"),
    TokenDef::integer("MONSKILL", "pcgen_monskill"),
    TokenDef::integer("MONNONSKILLHD", "pcgen_monnonskillhd"),
    TokenDef {
        key: "WEAPONBONUS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_weaponbonus"),
        required: false,
    },
    TokenDef::text("VISIBLE", "pcgen_visible"),
    // Sub-class / substitution
    TokenDef::integer("PROHIBITCOST", "pcgen_prohibitcost"),
    TokenDef::text("SUBCLASSLEVEL", "pcgen_subclasslevel"),
    TokenDef::text("SUBSTITUTIONCLASS", "pcgen_substitutionclass"),
    TokenDef::text("SUBSTITUTIONLEVEL", "pcgen_substitutionlevel"),
];

static CLASS_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::Template,
    GlobalGroup::SourceMeta,
];

pub static CLASS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:class",
    head_token: Some("CLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: CLASS_TOKENS,
    globals: CLASS_GLOBALS,
};
