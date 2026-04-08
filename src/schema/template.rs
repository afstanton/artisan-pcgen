//! TEMPLATE entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilestemplates.html`
//!
//! Template files define character templates that modify base creatures.
//! The head is the template name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static ADDLEVEL_SLOTS: &[&str] = &["class", "count"];

static TEMPLATE_TOKENS: &[TokenDef] = &[
    // Core template properties
    TokenDef::integer("LEVELADJUSTMENT", "pcgen_leveladjustment"),
    TokenDef::text("RACETYPE", "pcgen_racetype"),
    TokenDef {
        key: "RACESUBTYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_racesubtype"),
        required: false,
    },
    TokenDef::text("SIZE", "pcgen_size"),
    TokenDef::text("HITDIE", "pcgen_hitdie"),
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
    TokenDef::text("GENDERLOCK", "pcgen_genderlock"),
    TokenDef::integer("BONUSSKILLPOINTS", "pcgen_bonusskillpoints"),
    // Level-based
    TokenDef::pipe_positional("ADDLEVEL", ADDLEVEL_SLOTS, "pcgen_addlevel"),
    TokenDef {
        key: "REPEATLEVEL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_repeatlevel"),
        required: false,
    },
    // Favored class
    TokenDef {
        key: "FAVOREDCLASS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_favoredclass"),
        required: false,
    },
    // Feats and weapons
    TokenDef {
        key: "FEAT",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_feats"),
        required: false,
    },
    TokenDef {
        key: "WEAPONBONUS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_weaponbonus"),
        required: false,
    },
];

static TEMPLATE_GLOBALS: &[GlobalGroup] = &[
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

pub static TEMPLATE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:template",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: TEMPLATE_TOKENS,
    globals: TEMPLATE_GLOBALS,
};
