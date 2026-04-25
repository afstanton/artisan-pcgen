//! TEMPLATE entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilestemplates.html`
//!
//! Template files define character templates that modify base creatures.
//! The head is the template name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
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
        artisan_mapping: ArtisanMapping::Field("race_subtype"),
        required: false,
    },
    TokenDef::text("SUBRACE", "subrace"),
    TokenDef::text("SIZE", "size"),
    TokenDef::text("FACE", "face"),
    TokenDef {
        key: "VISION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("vision"),
        required: false,
    },
    TokenDef::text("LEGS", "legs"),
    TokenDef::text("HANDS", "hands"),
    TokenDef {
        key: "DR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Effect,
        required: false,
    },
    TokenDef {
        key: "SR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Effect,
        required: false,
    },
    TokenDef::text("CR", "cr"),
    TokenDef::text("REGION", "region"),
    TokenDef::text("SUBREGION", "pcgen_subregion"),
    TokenDef::text("VISIBLE", "visible"),
    TokenDef::text("REMOVABLE", "pcgen_removable"),
    TokenDef::text("HITDIE", "hitdie"),
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
        artisan_mapping: ArtisanMapping::Field("favored_class"),
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
        artisan_mapping: ArtisanMapping::Field("weapon_bonus"),
        required: false,
    },
    TokenDef::pipe_list_repeatable("COMPANIONLIST", "companion_list"),
    TokenDef::pipe_list_repeatable("CCSKILL", "pcgen_ccskill"),
    TokenDef::pipe_list_repeatable("UNENCUMBEREDMOVE", "pcgen_unencumberedmove"),
    TokenDef::pipe_positional_repeatable("FOLLOWERS", &["type", "limit"], "pcgen_followers"),
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    // KIT grants a starting kit to characters that acquire this template.
    // Format: KIT:count|kit_name (raw string stored as array element).
    TokenDef {
        key: "KIT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("kits"),
        required: false,
    },
    // SPELLS: spell-like abilities granted by this template.
    TokenDef {
        key: "SPELLS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spells"),
        required: false,
    },
    // UDAM: unarmed damage progression granted by this template.
    TokenDef::text("UDAM", "udam"),
    // DONOTADD: prevent an inherited skill from being added as a class skill.
    TokenDef {
        key: "DONOTADD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("do_not_add"),
        required: false,
    },
    // COST: pool-point cost for acquiring this template.
    TokenDef::text("COST", "cost"),
    // LEVEL: virtual class level granted by this template (some 3rd-party data).
    TokenDef::integer("LEVEL", "level"),
    // SPELLLEVEL: spell level association for template-granted spell-like abilities.
    TokenDef::text("SPELLLEVEL", "spell_level"),
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
    GlobalGroup::SortKey,
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
