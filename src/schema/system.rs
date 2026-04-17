//! System-file schemas for game mode and registry records.
//!
//! Sources:
//! - `docs/listfilepages/systemfilestagpages/gamemodestatsandcheckslist.html`
//! - `docs/listfilepages/systemfilestagpages/gamemodemiscinfolist.html`
//! - `docs/listfilepages/systemfilestagpages/equipiconslst.html`

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static BONUSSPELLLEVEL_TOKENS: &[TokenDef] = &[
    TokenDef::integer("BASESTATSCORE", "base_stat_score"),
    TokenDef::integer("STATRANGE", "stat_range"),
];

pub static BONUSSPELLLEVEL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:bonusspelllevel",
    head_token: Some("BONUSSPELLLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: BONUSSPELLLEVEL_TOKENS,
    globals: &[],
};

pub static BONUSSTACKS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:bonusstacks",
    head_token: Some("BONUSSTACKS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static BONUSFEATLEVELSTARTINTERVAL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:bonusfeatlevelstartinterval",
    head_token: Some("BONUSFEATLEVELSTARTINTERVAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static BONUSSTATLEVELSTARTINTERVAL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:bonusstatlevelstartinterval",
    head_token: Some("BONUSSTATLEVELSTARTINTERVAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static ALIGNMENTFEATURE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:alignmentfeature",
    head_token: Some("ALIGNMENTFEATURE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno(
        "ALIGNMENTFEATURE",
        "alignment_feature",
    )],
    globals: &[],
};

pub static CURRENCYUNITABBREV_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:currencyunitabbrev",
    head_token: Some("CURRENCYUNITABBREV"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "CURRENCYUNITABBREV",
        "currency_unit_abbrev",
    )],
    globals: &[],
};

pub static MENUENTRY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:menuentry",
    head_token: Some("MENUENTRY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("MENUENTRY", "menu_entry")],
    globals: &[],
};

pub static DISPLAYORDER_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:displayorder",
    head_token: Some("DISPLAYORDER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "DISPLAYORDER",
        "display_order",
    )],
    globals: &[],
};

pub static DIESIZES_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:diesizes",
    head_token: Some("DIESIZES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("DIESIZES", "die_sizes")],
    globals: &[],
};

pub static DEFAULTUNITSET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:defaultunitset",
    head_token: Some("DEFAULTUNITSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "DEFAULTUNITSET",
        "default_unit_set",
    )],
    globals: &[],
};

pub static DEFAULTDATASET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:defaultdataset",
    head_token: Some("DEFAULTDATASET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "DEFAULTDATASET",
        "default_dataset",
    )],
    globals: &[],
};

pub static ALLOWEDMODES_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:allowedmodes",
    head_token: Some("ALLOWEDMODES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "ALLOWEDMODES",
        "allowed_modes",
    )],
    globals: &[],
};

pub static GAMEMODEKEY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:gamemodekey",
    head_token: Some("GAMEMODEKEY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("GAMEMODEKEY", "game_mode_key")],
    globals: &[],
};

pub static BABMAXATT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:babmaxatt",
    head_token: Some("BABMAXATT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("BABMAXATT", "bab_max_att")],
    globals: &[],
};

pub static BABMINVAL_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:babminval",
    head_token: Some("BABMINVAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("BABMINVAL", "bab_min_val")],
    globals: &[],
};

pub static BABATTCYC_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:babattcyc",
    head_token: Some("BABATTCYC"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("BABATTCYC", "bab_att_cyc")],
    globals: &[],
};

pub static ACNAME_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:acname",
    head_token: Some("ACNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("ACNAME", "ac_name")],
    globals: &[],
};

pub static DOMAINFEATURE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:domainfeature",
    head_token: Some("DOMAINFEATURE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("DOMAINFEATURE", "domain_feature")],
    globals: &[],
};

pub static LEVELMSG_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:levelmsg",
    head_token: Some("LEVELMSG"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("LEVELMSG", "level_msg")],
    globals: &[],
};

pub static SHORTRANGE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:shortrange",
    head_token: Some("SHORTRANGE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("SHORTRANGE", "short_range")],
    globals: &[],
};

pub static RANGEPENALTY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:rangepenalty",
    head_token: Some("RANGEPENALTY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("RANGEPENALTY", "range_penalty")],
    globals: &[],
};

pub static SQUARESIZE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:squaresize",
    head_token: Some("SQUARESIZE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("SQUARESIZE", "square_size")],
    globals: &[],
};

pub static SKILLMULTIPLIER_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:skillmultiplier",
    head_token: Some("SKILLMULTIPLIER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "SKILLMULTIPLIER",
        "skill_multiplier",
    )],
    globals: &[],
};

pub static SPELLBASEDC_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:spellbasedc",
    head_token: Some("SPELLBASEDC"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("SPELLBASEDC", "spell_base_dc")],
    globals: &[],
};

pub static WEAPONNONPROFPENALTY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:weaponnonprofpenalty",
    head_token: Some("WEAPONNONPROFPENALTY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "WEAPONNONPROFPENALTY",
        "weapon_nonprof_penalty",
    )],
    globals: &[],
};

pub static WEAPONREACH_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:weaponreach",
    head_token: Some("WEAPONREACH"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("WEAPONREACH", "weapon_reach")],
    globals: &[],
};

pub static CHARACTERTYPE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:charactertype",
    head_token: Some("CHARACTERTYPE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef {
        key: "CHARACTERTYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("character_type"),
        required: true,
    }],
    globals: &[],
};

pub static CRTHRESHOLD_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:crthreshold",
    head_token: Some("CRTHRESHOLD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("CRTHRESHOLD", "cr_threshold")],
    globals: &[],
};

pub static CRSTEPS_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:crsteps",
    head_token: Some("CRSTEPS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("CRSTEPS", "cr_steps")],
    globals: &[],
};

pub static MONSTERROLES_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:monsterroles",
    head_token: Some("MONSTERROLES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef {
        key: "MONSTERROLES",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("monster_roles"),
        required: true,
    }],
    globals: &[],
};

pub static MONSTERROLEDEFAULT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:monsterroledefault",
    head_token: Some("MONSTERROLEDEFAULT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "MONSTERROLEDEFAULT",
        "monster_role_default",
    )],
    globals: &[],
};

pub static XPTABLE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:xptable",
    head_token: Some("XPTABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("XPTABLE", "xp_table")],
    globals: &[],
};

pub static EQSIZEPENALTY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:eqsizepenalty",
    head_token: Some("EQSIZEPENALTY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "EQSIZEPENALTY",
        "pcgen_eqsizepenalty",
    )],
    globals: &[GlobalGroup::Bonus, GlobalGroup::Prerequisites],
};

pub static RESIZABLEEQUIPTYPE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:resizableequiptype",
    head_token: Some("RESIZABLEEQUIPTYPE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef {
        key: "RESIZABLEEQUIPTYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("resizable_equip_type"),
        required: true,
    }],
    globals: &[],
};

pub static SKILLCOST_CROSSCLASS_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:skillcost_crossclass",
    head_token: Some("SKILLCOST_CROSSCLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "SKILLCOST_CROSSCLASS",
        "skill_cost_cross_class",
    )],
    globals: &[],
};

pub static SKILLCOST_CLASS_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:skillcost_class",
    head_token: Some("SKILLCOST_CLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "SKILLCOST_CLASS",
        "skill_cost_class",
    )],
    globals: &[],
};

pub static SKILLCOST_EXCLUSIVE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:skillcost_exclusive",
    head_token: Some("SKILLCOST_EXCLUSIVE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "SKILLCOST_EXCLUSIVE",
        "skill_cost_exclusive",
    )],
    globals: &[],
};

pub static SPELLBASECONCENTRATION_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:spellbaseconcentration",
    head_token: Some("SPELLBASECONCENTRATION"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "SPELLBASECONCENTRATION",
        "pcgen_spellbaseconcentration",
    )],
    globals: &[],
};

pub static XPAWARD_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:xpaward",
    head_token: Some("XPAWARD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("XPAWARD", "pcgen_xpaward")],
    globals: &[],
};

pub static STATINPUT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:statinput",
    head_token: Some("STATINPUT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("STATINPUT", "pcgen_statinput")],
    globals: &[],
};

pub static MAXNONEPICLEVEL_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:maxnonepiclevel",
    head_token: Some("MAXNONEPICLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "MAXNONEPICLEVEL",
        "max_non_epic_level",
    )],
    globals: &[],
};

pub static PLUSCOST_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:pluscost",
    head_token: Some("PLUSCOST"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::pipe_positional(
        "PLUSCOST",
        &["equipment_type", "formula"],
        "plus_cost",
    )],
    globals: &[],
};

pub static PREVIEWDIR_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:previewdir",
    head_token: Some("PREVIEWDIR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static PREVIEWSHEET_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:previewsheet",
    head_token: Some("PREVIEWSHEET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static LOAD_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:load",
    head_token: Some("LOAD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static LOADMULT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:loadmult",
    head_token: Some("LOADMULT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("LOADMULT", "load_mult")],
    globals: &[],
};

static NUMSLOTS_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("NUMSLOTS", "pcgen_numslots"),
    TokenDef::integer("HEAD", "pcgen_headslots"),
    TokenDef::integer("HANDS", "hands"),
    TokenDef::integer("TORSO", "pcgen_torsoslots"),
    TokenDef::integer("LEGS", "legs"),
    TokenDef::integer("SHIELD", "pcgen_shieldslots"),
];

pub static NUMSLOTS_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:numslots",
    head_token: Some("NUMSLOTS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: NUMSLOTS_SYSTEM_TOKENS,
    globals: &[],
};

static METHOD_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef::integer("POINTS", "points")];

pub static METHOD_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:method",
    head_token: Some("METHOD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: METHOD_SYSTEM_TOKENS,
    globals: &[],
};

pub static SIZEMULT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:sizemult",
    head_token: Some("SIZEMULT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static ENCUMBRANCE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:encumbrance",
    head_token: Some("ENCUMBRANCE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static DEFAULTVARIABLEVALUE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:defaultvariablevalue",
    head_token: Some("DEFAULTVARIABLEVALUE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static SPELLRANGE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:spellrange",
    head_token: Some("SPELLRANGE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("SPELLRANGE", "spell_range")],
    globals: &[],
};

pub static OUTPUTSHEET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:outputsheet",
    head_token: Some("OUTPUTSHEET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("OUTPUTSHEET", "output_sheet")],
    globals: &[],
};

pub static INFOSHEET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:infosheet",
    head_token: Some("INFOSHEET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("INFOSHEET", "info_sheet")],
    globals: &[],
};

static UNITSET_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("UNITSET", "unit_set"),
    TokenDef::text("DISTANCEUNIT", "distance_unit"),
    TokenDef::text("DISTANCEFACTOR", "distance_factor"),
    TokenDef::text("DISTANCEPATTERN", "distance_pattern"),
    TokenDef::text("HEIGHTUNIT", "height_unit"),
    TokenDef::text("HEIGHTFACTOR", "height_factor"),
    TokenDef::text("HEIGHTPATTERN", "height_pattern"),
    TokenDef::text("WEIGHTUNIT", "weight_unit"),
    TokenDef::text("WEIGHTFACTOR", "weight_factor"),
    TokenDef::text("WEIGHTPATTERN", "weight_pattern"),
];

pub static UNITSET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:unitset",
    head_token: Some("UNITSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: UNITSET_SYSTEM_TOKENS,
    globals: &[],
};

pub static WEAPONCATEGORY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:weaponcategory",
    head_token: Some("WEAPONCATEGORY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

static ROLLMETHOD_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("METHOD", "pcgen_method"),
    // EXPRESSION is a sub-token in roll formulas: ROLLMETHOD:1|EXPRESSION:10
    TokenDef::integer("EXPRESSION", "pcgen_rollmethod_expression"),
];

pub static ROLLMETHOD_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:rollmethod",
    head_token: Some("ROLLMETHOD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ROLLMETHOD_SYSTEM_TOKENS,
    globals: &[GlobalGroup::SortKey],
};

// codeControl.lst: maps save-bonus variable name, e.g. STATMODSAVE:Save_StatBonus
pub static STATMODSAVE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:statmodsave",
    head_token: Some("STATMODSAVE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("STATMODSAVE", "pcgen_statmodsave")],
    globals: &[],
};

// codeControl.lst: maps alternate HP variable, e.g. ALTHP:HP_Vitality
pub static ALTHP_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:althp",
    head_token: Some("ALTHP"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("ALTHP", "pcgen_althp")],
    globals: &[],
};

// miscinfo.lst: pipe-separated list of equipment types hidden from UI
pub static HIDDENEQUIPTYPES_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:hiddenequiptypes",
    head_token: Some("HIDDENEQUIPTYPES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef {
        key: "HIDDENEQUIPTYPES",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_hiddenequiptypes"),
        required: true,
    }],
    globals: &[],
};

// miscinfo.lst: pipe-separated list of feat types hidden from UI
pub static HIDDENFEATTYPES_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:hiddenfeattypes",
    head_token: Some("HIDDENFEATTYPES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef {
        key: "HIDDENFEATTYPES",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_hiddenfeattypes"),
        required: true,
    }],
    globals: &[],
};

static CLASSTYPE_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("CRMOD", "pcgen_crmod"),
    TokenDef::integer("CRMODPRIORITY", "pcgen_crmodpriority"),
    TokenDef::text("CRFORMULA", "cr_formula"),
    TokenDef::yesno("ISMONSTER", "is_monster"),
    TokenDef::yesno("XPPENALTY", "xp_penalty"),
];

pub static CLASSTYPE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:classtype",
    head_token: Some("CLASSTYPE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: CLASSTYPE_SYSTEM_TOKENS,
    globals: &[],
};

static ACTYPE_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef {
    key: "REMOVE",
    grammar: TokenGrammar::Text,
    cardinality: Cardinality::Repeatable,
    artisan_mapping: ArtisanMapping::Field("pcgen_remove"),
    required: false,
}];

pub static ACTYPE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:actype",
    head_token: Some("ACTYPE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ACTYPE_SYSTEM_TOKENS,
    globals: &[GlobalGroup::Add],
};

static BASEDICE_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("UP", "up"),
    TokenDef::text("DOWN", "down"),
];

pub static BASEDICE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
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

pub static WIELDCATEGORY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:wieldcategory",
    head_token: Some("WIELDCATEGORY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: WIELDCATEGORY_SYSTEM_TOKENS,
    globals: &[GlobalGroup::Prerequisites],
};

static TAB_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef::text("CONTEXT", "context")];

pub static TAB_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:tab",
    head_token: Some("TAB"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: TAB_SYSTEM_TOKENS,
    globals: &[],
};

static EQSLOT_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("CONTAINS", "contains"),
    TokenDef::text("NUMBER", "number"),
];

pub static EQSLOT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:eqslot",
    head_token: Some("EQSLOT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: EQSLOT_SYSTEM_TOKENS,
    globals: &[],
};

pub static AGESET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:ageset",
    head_token: Some("AGESET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

static LEVEL_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text("MINXP", "min_xp"),
    TokenDef::text("CSKILLMAX", "c_skill_max"),
    TokenDef::text("CCSKILLMAX", "cc_skill_max"),
];

pub static LEVEL_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:level",
    head_token: Some("LEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: LEVEL_SYSTEM_TOKENS,
    globals: &[],
};

pub static ICON_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:icon",
    head_token: Some("ICON"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static ALIGN_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:align",
    head_token: Some("ALIGN"),
    head_format: HeadFormat::NameOnly,
    tokens: &[
        TokenDef::text("ABB", "abbreviation"),
        TokenDef::text("VALIDFORDEITY", "pcgen_validfordeity"),
        TokenDef::text("VALIDFORFOLLOWER", "pcgen_validforfollower"),
    ],
    globals: &[GlobalGroup::Key, GlobalGroup::SortKey],
};

static STAT_TOKENS: &[TokenDef] = &[
    TokenDef::integer("SCORE", "score"),
    TokenDef::text("ABB", "abbreviation"),
    TokenDef::text("STATMOD", "pcgen_statmod"),
    // Some game modes (e.g. Pathfinder) grant internal abilities from stat entities.
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
];

pub static STAT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:stat",
    head_token: Some("STAT"),
    head_format: HeadFormat::NameOnly,
    tokens: STAT_TOKENS,
    globals: &[GlobalGroup::Key, GlobalGroup::SortKey, GlobalGroup::Bonus, GlobalGroup::Define],
};

pub static SIZEADJUSTMENT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:sizeadjustment",
    head_token: Some("SIZENAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[
        TokenDef::text("SIZENUM", "size_num"),
        TokenDef::text("ISDEFAULTSIZE", "pcgen_isdefaultsize"),
    ],
    globals: &[],
};

pub static RACE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:race",
    head_token: Some("RACE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static NAME_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:name",
    head_token: Some("NAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[
        TokenDef::text("PARM", "pcgen_parm"),
        TokenDef::text("VAR", "pcgen_var"),
        TokenDef::yesno("DEFAULT", "pcgen_default"),
        TokenDef::text("EXCLUDE", "pcgen_exclude"),
    ],
    globals: &[],
};

pub static STARTTABLE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:starttable",
    head_token: Some("STARTTABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static ENDTABLE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:endtable",
    head_token: Some("ENDTABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("ENDTABLE", "pcgen_endtable")],
    globals: &[],
};

pub static MOVEMENT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:movement",
    head_token: Some("MOVEMENT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static WEAPONTYPE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
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
    artisan_mapping: ArtisanMapping::Field("values"),
    required: false,
}];

pub static TABLE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
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

// Ability grants shared by subclasslevel and substitutionlevel rows.
static SUBSTITUTION_LEVEL_TOKENS: &[TokenDef] = &[TokenDef {
    key: "ABILITY",
    grammar: TokenGrammar::Text,
    cardinality: Cardinality::Repeatable,
    artisan_mapping: ArtisanMapping::Field("abilities"),
    required: false,
}];

pub static SUBCLASSLEVEL_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:subclasslevel",
    head_token: Some("SUBCLASSLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SUBSTITUTION_LEVEL_TOKENS,
    globals: SUBSTITUTION_CLASS_GLOBALS,
};

pub static SUBSTITUTIONCLASS_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:substitutionclass",
    head_token: Some("SUBSTITUTIONCLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: SUBSTITUTION_CLASS_GLOBALS,
};

pub static SUBSTITUTIONLEVEL_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:substitutionlevel",
    head_token: Some("SUBSTITUTIONLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SUBSTITUTION_LEVEL_TOKENS,
    globals: SUBSTITUTION_CLASS_GLOBALS,
};
