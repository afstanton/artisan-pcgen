//! RACE entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesraces.html`
//!
//! Race files define playable and monster races. The head is the race name
//! (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static RACE_TOKENS: &[TokenDef] = &[
    // Core racial properties
    TokenDef::text("RACETYPE", "pcgen_racetype"),
    TokenDef {
        key: "RACESUBTYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_racesubtype"),
        required: false,
    },
    TokenDef::text("SIZE", "pcgen_size"),
    TokenDef::text("FACE", "pcgen_face"),
    TokenDef {
        key: "VISION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_vision"),
        required: false,
    },
    TokenDef::text("LEGS", "pcgen_legs"),
    TokenDef::text("HANDS", "pcgen_hands"),
    TokenDef::text("DR", "pcgen_dr"),
    TokenDef::text("SR", "pcgen_sr"),
    TokenDef::text("CR", "pcgen_cr"),
    TokenDef::integer("LEVELADJUSTMENT", "pcgen_leveladjustment"),
    TokenDef::integer("STARTFEATS", "pcgen_startfeats"),
    TokenDef::integer("XTRASKILLPTSPERLVL", "pcgen_xtraskillptsperlvl"),
    TokenDef::text("HITDIE", "pcgen_hitdie"),
    TokenDef::text("HITDICEADVANCEMENT", "pcgen_hitdiceadvancement"),
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
    // Favored class
    TokenDef {
        key: "FAVCLASS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_favclass"),
        required: false,
    },
    // Monster-specific
    TokenDef::text("MONSTERCLASS", "pcgen_monsterclass"),
    TokenDef {
        key: "MONCSKILL",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_moncskill"),
        required: false,
    },
    TokenDef {
        key: "MONCCSKILL",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_monccskill"),
        required: false,
    },
    // Feats and weapon bonuses
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
    TokenDef {
        key: "GROUP",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_group"),
        required: false,
    },
];

static RACE_GLOBALS: &[GlobalGroup] = &[
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

pub static RACE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:race",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: RACE_TOKENS,
    globals: RACE_GLOBALS,
};
