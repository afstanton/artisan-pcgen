//! System-file schemas for game mode and registry records.
//!
//! Sources:
//! - `docs/listfilepages/systemfilestagpages/gamemodestatsandcheckslist.html`
//! - `docs/listfilepages/systemfilestagpages/gamemodemiscinfolist.html`
//! - `docs/listfilepages/systemfilestagpages/equipiconslst.html`

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static BONUSSPELLLEVEL_TOKENS: &[TokenDef] = &[
    TokenDef::integer("BASESTATSCORE", "pcgen_basestatscore"),
    TokenDef::integer("STATRANGE", "pcgen_statrange"),
];

pub static BONUSSPELLLEVEL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:bonusspelllevel",
    head_token: Some("BONUSSPELLLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: BONUSSPELLLEVEL_TOKENS,
    globals: &[],
};

pub static BONUSSTACKS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:bonusstacks",
    head_token: Some("BONUSSTACKS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static BONUSFEATLEVELSTARTINTERVAL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:bonusfeatlevelstartinterval",
    head_token: Some("BONUSFEATLEVELSTARTINTERVAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static BONUSSTATLEVELSTARTINTERVAL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:bonusstatlevelstartinterval",
    head_token: Some("BONUSSTATLEVELSTARTINTERVAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static PREVIEWDIR_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:previewdir",
    head_token: Some("PREVIEWDIR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static PREVIEWSHEET_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:previewsheet",
    head_token: Some("PREVIEWSHEET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static LOAD_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:load",
    head_token: Some("LOAD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

static METHOD_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef::integer("POINTS", "pcgen_points")];

pub static METHOD_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:method",
    head_token: Some("METHOD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: METHOD_SYSTEM_TOKENS,
    globals: &[],
};

pub static SIZEMULT_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:sizemult",
    head_token: Some("SIZEMULT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static ENCUMBRANCE_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:encumbrance",
    head_token: Some("ENCUMBRANCE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static DEFAULTVARIABLEVALUE_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:defaultvariablevalue",
    head_token: Some("DEFAULTVARIABLEVALUE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

static ACTYPE_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef {
    key: "REMOVE",
    grammar: TokenGrammar::Text,
    cardinality: Cardinality::Repeatable,
    artisan_mapping: ArtisanMapping::Attribute("pcgen_remove"),
    required: false,
}];

pub static ACTYPE_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:actype",
    head_token: Some("ACTYPE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ACTYPE_SYSTEM_TOKENS,
    globals: &[GlobalGroup::Add],
};

static BASEDICE_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("UP", "pcgen_up"),
    TokenDef::text("DOWN", "pcgen_down"),
];

pub static BASEDICE_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:basedice",
    head_token: Some("BASEDICE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: BASEDICE_SYSTEM_TOKENS,
    globals: &[],
};

static WIELDCATEGORY_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("SWITCH", "pcgen_switch"),
    TokenDef::text("SIZEDIFF", "pcgen_sizediff"),
    TokenDef::yesno("FINESSABLE", "pcgen_finessable"),
];

pub static WIELDCATEGORY_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:wieldcategory",
    head_token: Some("WIELDCATEGORY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: WIELDCATEGORY_SYSTEM_TOKENS,
    globals: &[GlobalGroup::Prerequisites],
};

static TAB_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef::text("CONTEXT", "pcgen_context")];

pub static TAB_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:tab",
    head_token: Some("TAB"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: TAB_SYSTEM_TOKENS,
    globals: &[],
};

static EQSLOT_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("CONTAINS", "pcgen_contains"),
    TokenDef::text("NUMBER", "pcgen_number"),
];

pub static EQSLOT_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:eqslot",
    head_token: Some("EQSLOT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: EQSLOT_SYSTEM_TOKENS,
    globals: &[],
};

pub static AGESET_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:ageset",
    head_token: Some("AGESET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

static LEVEL_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("MINXP", "pcgen_minxp"),
    TokenDef::text("CSKILLMAX", "pcgen_cskillmax"),
    TokenDef::text("CCSKILLMAX", "pcgen_ccskillmax"),
];

pub static LEVEL_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:level",
    head_token: Some("LEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: LEVEL_SYSTEM_TOKENS,
    globals: &[],
};

pub static ICON_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:icon",
    head_token: Some("ICON"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static ALIGN_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:align",
    head_token: Some("ALIGN"),
    head_format: HeadFormat::NameOnly,
    tokens: &[
        TokenDef::text("ABB", "pcgen_abbreviation"),
        TokenDef::text("VALIDFORDEITY", "pcgen_validfordeity"),
        TokenDef::text("VALIDFORFOLLOWER", "pcgen_validforfollower"),
    ],
    globals: &[GlobalGroup::Key, GlobalGroup::SortKey],
};

pub static STAT_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:stat",
    head_token: Some("STAT"),
    head_format: HeadFormat::NameOnly,
    tokens: &[
        TokenDef::text("ABB", "pcgen_abbreviation"),
        TokenDef::text("STATMOD", "pcgen_statmod"),
    ],
    globals: &[GlobalGroup::Key, GlobalGroup::SortKey],
};

pub static SIZEADJUSTMENT_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:sizeadjustment",
    head_token: Some("SIZENAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[
        TokenDef::text("SIZENUM", "pcgen_sizenum"),
        TokenDef::text("ISDEFAULTSIZE", "pcgen_isdefaultsize"),
    ],
    globals: &[],
};

pub static RACE_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:race",
    head_token: Some("RACE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static NAME_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:name",
    head_token: Some("NAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[
        TokenDef::text("PARM", "pcgen_parm"),
        TokenDef::text("VAR", "pcgen_var"),
        TokenDef::yesno("DEFAULT", "pcgen_default"),
    ],
    globals: &[],
};

pub static WEAPONTYPE_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:weapontype",
    head_token: Some("WEAPONTYPE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

static TABLE_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef {
    key: "VALUES",
    grammar: TokenGrammar::Text,
    cardinality: Cardinality::Repeatable,
    artisan_mapping: ArtisanMapping::Attribute("pcgen_values"),
    required: false,
}];

pub static TABLE_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:table",
    head_token: Some("TABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: TABLE_SYSTEM_TOKENS,
    globals: &[],
};

static SUBSTITUTION_CLASS_GLOBALS: &[GlobalGroup] = &[
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
];

pub static SUBCLASSLEVEL_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:subclasslevel",
    head_token: Some("SUBCLASSLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: SUBSTITUTION_CLASS_GLOBALS,
};

pub static SUBSTITUTIONCLASS_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:substitutionclass",
    head_token: Some("SUBSTITUTIONCLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: SUBSTITUTION_CLASS_GLOBALS,
};

pub static SUBSTITUTIONLEVEL_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:substitutionlevel",
    head_token: Some("SUBSTITUTIONLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: SUBSTITUTION_CLASS_GLOBALS,
};
