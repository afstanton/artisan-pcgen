//! TEMPLATE entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilestemplates.html`
//!
//! Template files define character templates that modify base creatures.
//! The head is the template name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, LineGrammar, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static ADDLEVEL_SLOTS: &[&str] = &["class", "count"];

static TEMPLATE_TOKENS: &[TokenDef] = &[
    // Core template properties
    TokenDef::integer("LEVELADJUSTMENT", "level_adjustment"),
    TokenDef::text("RACETYPE", "racetype"),
    TokenDef {
        key: "RACESUBTYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_racesubtype"),
        required: false,
    },
    TokenDef::text("SUBRACE", "pcgen_subrace"),
    TokenDef::text("SIZE", "size"),
    TokenDef::text("FACE", "pcgen_face"),
    TokenDef {
        key: "VISION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("vision"),
        required: false,
    },
    TokenDef::text("LEGS", "pcgen_legs"),
    TokenDef::text("HANDS", "pcgen_hands"),
    TokenDef::text("DR", "pcgen_dr"),
    TokenDef::text("SR", "pcgen_sr"),
    TokenDef::text("CR", "cr"),
    TokenDef::text("REGION", "region"),
    TokenDef::text("SUBREGION", "pcgen_subregion"),
    TokenDef::text("VISIBLE", "pcgen_visible"),
    TokenDef::text("REMOVABLE", "pcgen_removable"),
    TokenDef::text("HITDIE", "hitdie"),
    TokenDef {
        key: "MOVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_move"),
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
        artisan_mapping: ArtisanMapping::Field("pcgen_naturalattacks"),
        required: false,
    },
    TokenDef::text("GENDERLOCK", "pcgen_genderlock"),
    TokenDef::integer("BONUSSKILLPOINTS", "pcgen_bonusskillpoints"),
    // Non-party points cost: how many party-tracking points this template does NOT consume
    TokenDef::integer("NONPP", "pcgen_nonpp"),
    TokenDef {
        key: "TEMPVALUE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_tempvalue"),
        required: false,
    },
    // Level-based
    TokenDef::pipe_positional("ADDLEVEL", ADDLEVEL_SLOTS, "pcgen_addlevel"),
    TokenDef {
        key: "REPEATLEVEL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_repeatlevel"),
        required: false,
    },
    // Favored class
    TokenDef {
        key: "FAVOREDCLASS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_favoredclass"),
        required: false,
    },
    // Feats and weapons
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
        artisan_mapping: ArtisanMapping::Field("pcgen_weaponbonus"),
        required: false,
    },
    TokenDef::pipe_list_repeatable("COMPANIONLIST", "pcgen_companionlist"),
    TokenDef::pipe_list_repeatable("CCSKILL", "pcgen_ccskill"),
    TokenDef::pipe_list_repeatable("UNENCUMBEREDMOVE", "pcgen_unencumberedmove"),
    TokenDef::pipe_positional_repeatable("FOLLOWERS", &["type", "limit"], "pcgen_followers"),
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

pub static TEMPLATE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:template",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: TEMPLATE_TOKENS,
    globals: TEMPLATE_GLOBALS,
};
