//! RACE entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesraces.html`
//!
//! Race files define playable and monster races. The head is the race name
//! (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static RACE_TOKENS: &[TokenDef] = &[
    // Core racial properties
    TokenDef::text("RACETYPE", "racetype"),
    TokenDef {
        key: "RACESUBTYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("race_subtype"),
        required: false,
    },
    TokenDef::text("SIZE", "size"),
    TokenDef::text("FACE", "face"),
    TokenDef {
        key: "VISION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("vision"),
        required: false,
    },
    TokenDef {
        key: "LANGAUTO",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("lang_auto"),
        required: false,
    },
    TokenDef::pipe_positional_repeatable("GRANT", &["category", "value"], "pcgen_grant"),
    TokenDef::text("LEGS", "legs"),
    TokenDef::text("HANDS", "hands"),
    TokenDef::text("DR", "pcgen_dr"),
    TokenDef::text("SR", "pcgen_sr"),
    TokenDef::text("CR", "cr"),
    TokenDef::pipe_positional_repeatable("CRMOD", &["class_types", "modifier"], "pcgen_crmod"),
    TokenDef::text("REACH", "reach"),
    TokenDef::integer("LEVELADJUSTMENT", "level_adjustment"),
    TokenDef::integer("STARTFEATS", "pcgen_startfeats"),
    TokenDef::integer("XTRASKILLPTSPERLVL", "pcgen_xtraskillptsperlvl"),
    TokenDef {
        key: "SKILLMULT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_skillmult"),
        required: false,
    },
    TokenDef::text("HITDIE", "hitdie"),
    TokenDef::text("HITDICEADVANCEMENT", "pcgen_hitdiceadvancement"),
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
    // Favored class
    TokenDef {
        key: "FAVCLASS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("favored_class"),
        required: false,
    },
    // Monster-specific
    TokenDef::text("MONSTERCLASS", "monsterclass"),
    TokenDef {
        key: "MONCSKILL",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_moncskill"),
        required: false,
    },
    TokenDef {
        key: "MONCCSKILL",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_monccskill"),
        required: false,
    },
    // Feats and weapon bonuses
    TokenDef {
        key: "FEAT",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_feats"),
        required: false,
    },
    TokenDef {
        key: "WEAPONBONUS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("weapon_bonus"),
        required: false,
    },
    TokenDef {
        key: "GROUP",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("group"),
        required: false,
    },
    TokenDef::pipe_list_repeatable("CCSKILL", "pcgen_ccskill"),
    TokenDef::pipe_list_repeatable("UNENCUMBEREDMOVE", "pcgen_unencumberedmove"),
    // Adult-content flag. YES marks a race as mature content.
    TokenDef::yesno("ISMATURE", "pcgen_ismature"),
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

pub static RACE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:race",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: RACE_TOKENS,
    globals: RACE_GLOBALS,
};
