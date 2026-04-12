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
    TokenDef::integer("BASESTATSCORE", "pcgen_basestatscore"),
    TokenDef::integer("STATRANGE", "pcgen_statrange"),
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
        "pcgen_alignmentfeature",
    )],
    globals: &[],
};

pub static CURRENCYUNITABBREV_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:currencyunitabbrev",
    head_token: Some("CURRENCYUNITABBREV"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "CURRENCYUNITABBREV",
        "pcgen_currencyunitabbrev",
    )],
    globals: &[],
};

pub static MENUENTRY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:menuentry",
    head_token: Some("MENUENTRY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("MENUENTRY", "pcgen_menuentry")],
    globals: &[],
};

pub static DISPLAYORDER_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:displayorder",
    head_token: Some("DISPLAYORDER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "DISPLAYORDER",
        "pcgen_displayorder",
    )],
    globals: &[],
};

pub static DIESIZES_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:diesizes",
    head_token: Some("DIESIZES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("DIESIZES", "pcgen_diesizes")],
    globals: &[],
};

pub static DEFAULTUNITSET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:defaultunitset",
    head_token: Some("DEFAULTUNITSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "DEFAULTUNITSET",
        "pcgen_defaultunitset",
    )],
    globals: &[],
};

pub static DEFAULTDATASET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:defaultdataset",
    head_token: Some("DEFAULTDATASET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "DEFAULTDATASET",
        "pcgen_defaultdataset",
    )],
    globals: &[],
};

pub static ALLOWEDMODES_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:allowedmodes",
    head_token: Some("ALLOWEDMODES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "ALLOWEDMODES",
        "pcgen_allowedmodes",
    )],
    globals: &[],
};

pub static GAMEMODEKEY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:gamemodekey",
    head_token: Some("GAMEMODEKEY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("GAMEMODEKEY", "pcgen_gamemodekey")],
    globals: &[],
};

pub static BABMAXATT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:babmaxatt",
    head_token: Some("BABMAXATT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("BABMAXATT", "pcgen_babmaxatt")],
    globals: &[],
};

pub static BABMINVAL_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:babminval",
    head_token: Some("BABMINVAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("BABMINVAL", "pcgen_babminval")],
    globals: &[],
};

pub static BABATTCYC_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:babattcyc",
    head_token: Some("BABATTCYC"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("BABATTCYC", "pcgen_babattcyc")],
    globals: &[],
};

pub static ACNAME_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:acname",
    head_token: Some("ACNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("ACNAME", "pcgen_acname")],
    globals: &[],
};

pub static DOMAINFEATURE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:domainfeature",
    head_token: Some("DOMAINFEATURE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("DOMAINFEATURE", "pcgen_domainfeature")],
    globals: &[],
};

pub static LEVELMSG_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:levelmsg",
    head_token: Some("LEVELMSG"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("LEVELMSG", "pcgen_levelmsg")],
    globals: &[],
};

pub static SHORTRANGE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:shortrange",
    head_token: Some("SHORTRANGE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("SHORTRANGE", "pcgen_shortrange")],
    globals: &[],
};

pub static RANGEPENALTY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:rangepenalty",
    head_token: Some("RANGEPENALTY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("RANGEPENALTY", "pcgen_rangepenalty")],
    globals: &[],
};

pub static SQUARESIZE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:squaresize",
    head_token: Some("SQUARESIZE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("SQUARESIZE", "pcgen_squaresize")],
    globals: &[],
};

pub static SKILLMULTIPLIER_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:skillmultiplier",
    head_token: Some("SKILLMULTIPLIER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "SKILLMULTIPLIER",
        "pcgen_skillmultiplier",
    )],
    globals: &[],
};

pub static SPELLBASEDC_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:spellbasedc",
    head_token: Some("SPELLBASEDC"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("SPELLBASEDC", "pcgen_spellbasedc")],
    globals: &[],
};

pub static WEAPONNONPROFPENALTY_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:weaponnonprofpenalty",
    head_token: Some("WEAPONNONPROFPENALTY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "WEAPONNONPROFPENALTY",
        "pcgen_weaponnonprofpenalty",
    )],
    globals: &[],
};

pub static WEAPONREACH_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:weaponreach",
    head_token: Some("WEAPONREACH"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("WEAPONREACH", "pcgen_weaponreach")],
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
        artisan_mapping: ArtisanMapping::Field("pcgen_charactertype"),
        required: true,
    }],
    globals: &[],
};

pub static CRTHRESHOLD_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:crthreshold",
    head_token: Some("CRTHRESHOLD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("CRTHRESHOLD", "pcgen_crthreshold")],
    globals: &[],
};

pub static CRSTEPS_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:crsteps",
    head_token: Some("CRSTEPS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("CRSTEPS", "pcgen_crsteps")],
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
        artisan_mapping: ArtisanMapping::Field("pcgen_monsterroles"),
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
        "pcgen_monsterroledefault",
    )],
    globals: &[],
};

pub static XPTABLE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:xptable",
    head_token: Some("XPTABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("XPTABLE", "pcgen_xptable")],
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
        artisan_mapping: ArtisanMapping::Field("pcgen_resizableequiptype"),
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
        "pcgen_skillcost_crossclass",
    )],
    globals: &[],
};

pub static SKILLCOST_CLASS_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:skillcost_class",
    head_token: Some("SKILLCOST_CLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "SKILLCOST_CLASS",
        "pcgen_skillcost_class",
    )],
    globals: &[],
};

pub static SKILLCOST_EXCLUSIVE_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:skillcost_exclusive",
    head_token: Some("SKILLCOST_EXCLUSIVE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "SKILLCOST_EXCLUSIVE",
        "pcgen_skillcost_exclusive",
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
        "pcgen_maxnonepiclevel",
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
        "pcgen_pluscost",
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
    tokens: &[TokenDef::integer("LOADMULT", "pcgen_loadmult")],
    globals: &[],
};

static NUMSLOTS_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("NUMSLOTS", "pcgen_numslots"),
    TokenDef::integer("HEAD", "pcgen_headslots"),
    TokenDef::integer("HANDS", "pcgen_hands"),
    TokenDef::integer("TORSO", "pcgen_torsoslots"),
    TokenDef::integer("LEGS", "pcgen_legs"),
    TokenDef::integer("SHIELD", "pcgen_shieldslots"),
];

pub static NUMSLOTS_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:numslots",
    head_token: Some("NUMSLOTS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: NUMSLOTS_SYSTEM_TOKENS,
    globals: &[],
};

static METHOD_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef::integer("POINTS", "pcgen_points")];

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
    tokens: &[TokenDef::text_required("SPELLRANGE", "pcgen_spellrange")],
    globals: &[],
};

pub static OUTPUTSHEET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:outputsheet",
    head_token: Some("OUTPUTSHEET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("OUTPUTSHEET", "pcgen_outputsheet")],
    globals: &[],
};

pub static INFOSHEET_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:infosheet",
    head_token: Some("INFOSHEET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("INFOSHEET", "pcgen_infosheet")],
    globals: &[],
};

static UNITSET_SYSTEM_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("UNITSET", "pcgen_unitset"),
    TokenDef::text("DISTANCEUNIT", "pcgen_distanceunit"),
    TokenDef::text("DISTANCEFACTOR", "pcgen_distancefactor"),
    TokenDef::text("DISTANCEPATTERN", "pcgen_distancepattern"),
    TokenDef::text("HEIGHTUNIT", "pcgen_heightunit"),
    TokenDef::text("HEIGHTFACTOR", "pcgen_heightfactor"),
    TokenDef::text("HEIGHTPATTERN", "pcgen_heightpattern"),
    TokenDef::text("WEIGHTUNIT", "pcgen_weightunit"),
    TokenDef::text("WEIGHTFACTOR", "pcgen_weightfactor"),
    TokenDef::text("WEIGHTPATTERN", "pcgen_weightpattern"),
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
    TokenDef::text("UP", "pcgen_up"),
    TokenDef::text("DOWN", "pcgen_down"),
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

static TAB_SYSTEM_TOKENS: &[TokenDef] = &[TokenDef::text("CONTEXT", "pcgen_context")];

pub static TAB_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
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
    TokenDef::text("MINXP", "pcgen_minxp"),
    TokenDef::text("CSKILLMAX", "pcgen_cskillmax"),
    TokenDef::text("CCSKILLMAX", "pcgen_ccskillmax"),
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

pub static STAT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:stat",
    head_token: Some("STAT"),
    head_format: HeadFormat::NameOnly,
    tokens: &[
        TokenDef::integer("SCORE", "pcgen_score"),
        TokenDef::text("ABB", "abbreviation"),
        TokenDef::text("STATMOD", "pcgen_statmod"),
    ],
    globals: &[GlobalGroup::Key, GlobalGroup::SortKey],
};

pub static SIZEADJUSTMENT_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:sizeadjustment",
    head_token: Some("SIZENAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[
        TokenDef::text("SIZENUM", "pcgen_sizenum"),
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
    artisan_mapping: ArtisanMapping::Field("pcgen_values"),
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

pub static SUBCLASSLEVEL_SYSTEM_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:system:subclasslevel",
    head_token: Some("SUBCLASSLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
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
    tokens: &[],
    globals: SUBSTITUTION_CLASS_GLOBALS,
};
