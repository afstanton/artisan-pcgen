//! PCC (Campaign) file schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilespcc.html`
//!
//! PCC files define campaign/source metadata. Rather than entities, each
//! line is a key:value directive. This schema is registered primarily so
//! that metadata tokens are classified as `SemanticallyInterpreted` by
//! the global token policy.

use crate::schema::{
    ArtisanMapping, Cardinality, LineGrammar, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static PRECAMPAIGN_TOKEN: TokenDef = TokenDef {
    key: "PRECAMPAIGN",
    grammar: TokenGrammar::PipeList,
    cardinality: Cardinality::Repeatable,
    artisan_mapping: ArtisanMapping::Field("pcgen_precampaign"),
    required: false,
};

static PCC_TOKENS: &[TokenDef] = &[
    // Source identification
    TokenDef::text("CAMPAIGN", "pcgen_campaign"),
    TokenDef::text("SOURCELONG", "pcgen_source_long"),
    TokenDef::text("SOURCE", "pcgen_source"),
    TokenDef::text("SOURCESHORT", "pcgen_source_short"),
    TokenDef::text("SOURCEWEB", "pcgen_source_web"),
    TokenDef::text("SOURCEDATE", "pcgen_source_date"),
    // Publisher
    TokenDef::text("PUBNAMELONG", "pcgen_publisher_long"),
    TokenDef::text("PUBNAMESHORT", "pcgen_publisher_short"),
    TokenDef::text("PUBNAMEWEB", "pcgen_publisher_web"),
    // Game system
    TokenDef::text("GAMEMODE", "pcgen_gamemode"),
    TokenDef::text("SETTING", "pcgen_setting"),
    TokenDef::text("BOOKTYPE", "pcgen_booktype"),
    // Catalog entries (file references)
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_abilities"),
        required: false,
    },
    TokenDef {
        key: "ABILITYCATEGORY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_abilitycategories"),
        required: false,
    },
    TokenDef {
        key: "FEAT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_feats"),
        required: false,
    },
    TokenDef {
        key: "PCC",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_pcc"),
        required: false,
    },
    TokenDef {
        key: "EQUIPMENT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("equipment"),
        required: false,
    },
    TokenDef {
        key: "DATACONTROL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_datacontrol_catalog"),
        required: false,
    },
    TokenDef {
        key: "DYNAMIC",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_dynamic_catalog"),
        required: false,
    },
    TokenDef {
        key: "GLOBALMODIFIER",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_globalmodifier_catalog"),
        required: false,
    },
    TokenDef {
        key: "DATATABLE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_datatable_catalog"),
        required: false,
    },
    TokenDef {
        key: "COMPANIONMOD",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_companionmod_catalog"),
        required: false,
    },
    TokenDef {
        key: "EQUIPMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_equipmod_catalog"),
        required: false,
    },
    TokenDef {
        key: "LANGUAGE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_language_catalog"),
        required: false,
    },
    TokenDef {
        key: "ALIGNMENT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_alignment_catalog"),
        required: false,
    },
    TokenDef {
        key: "SAVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_save_catalog"),
        required: false,
    },
    TokenDef {
        key: "SPELL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spells"),
        required: false,
    },
    TokenDef {
        key: "WEAPONPROF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_weaponprof_catalog"),
        required: false,
    },
    TokenDef {
        key: "ARMORPROF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_armorprof_catalog"),
        required: false,
    },
    TokenDef {
        key: "SHIELDPROF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_shieldprof_catalog"),
        required: false,
    },
    // Legal
    TokenDef::text("ISLICENSED", "pcgen_islicensed"),
    TokenDef {
        key: "LICENSE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("license"),
        required: false,
    },
    TokenDef::text("STATUS", "pcgen_status"),
    TokenDef::text("GENRE", "pcgen_genre"),
    TokenDef::text("ISOGL", "pcgen_isogl"),
    TokenDef {
        key: "INFOTEXT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("infotext"),
        required: false,
    },
    TokenDef::text("RANK", "rank"),
    TokenDef::text("DESC", "description"),
    TokenDef {
        key: "COPYRIGHT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_copyright"),
        required: false,
    },
    TokenDef::text("COVER", "pcgen_cover"),
    TokenDef::text("LOGO", "pcgen_logo"),
    TokenDef::text("ALLOWDUPES", "pcgen_allowdupes"),
    TokenDef::text("DATAFORMAT", "pcgen_dataformat"),
    TokenDef::text("EXPLANATION", "explanation"),
    TokenDef::text("DISPLAYNAME", "pcgen_displayname"),
    TokenDef::text("VISIBLE", "pcgen_visible"),
    TokenDef::yesno("SHOWINMENU", "pcgen_showinmenu"),
    TokenDef::yesno("ISMATURE", "pcgen_ismature"),
    TokenDef::text("REQUIRED", "pcgen_required"),
    TokenDef::text("SELECTABLE", "pcgen_selectable"),
    TokenDef::text("NAMEISPI", "pcgen_nameispi"),
    TokenDef::text("DESCISPI", "pcgen_descispi"),
    TokenDef::text("MAXVER", "pcgen_maxver"),
    TokenDef::text("MAXDEVVER", "pcgen_maxdevver"),
    TokenDef::text("NEWKEY", "pcgen_newkey"),
    TokenDef {
        key: "FORWARDREF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_forwardref"),
        required: false,
    },
    TokenDef {
        key: "HIDETYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_hidetype"),
        required: false,
    },
    TokenDef {
        key: "PRECAMPAIGN",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_precampaign"),
        required: false,
    },
    TokenDef {
        key: "LSTEXCLUDE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_lstexclude"),
        required: false,
    },
    TokenDef {
        key: "URL",
        grammar: TokenGrammar::PipePositional(&["text", "url", "label"]),
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("url"),
        required: false,
    },
    TokenDef::text("OPTION", "pcgen_option"),
    // Path to a help HTML file bundled with the campaign (e.g. HELP:./help_spycraft.html)
    TokenDef::text("HELP", "pcgen_help"),
];

static GLOBALMODIFIER_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("GLOBALMODIFIER", "pcgen_globalmodifier_catalog"),
    PRECAMPAIGN_TOKEN,
];

pub static GLOBALMODIFIER_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-globalmodifier-include",
    head_token: Some("GLOBALMODIFIER"),
    head_format: HeadFormat::NameOnly,
    tokens: GLOBALMODIFIER_INCLUDE_TOKENS,
    globals: &[],
};

static PCC_GLOBALS: &[GlobalGroup] = &[GlobalGroup::SourceMeta];

pub static PCC_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: PCC_TOKENS,
    globals: PCC_GLOBALS,
};

static LANGUAGE_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("LANGUAGE", "pcgen_language_catalog"),
    PRECAMPAIGN_TOKEN,
];
static VARIABLE_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("VARIABLE", "pcgen_variable_catalog"),
    PRECAMPAIGN_TOKEN,
];
static ISMATURE_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::yesno("ISMATURE", "pcgen_ismature"),
    PRECAMPAIGN_TOKEN,
];
static ABILITY_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("ABILITY", "pcgen_abilities"),
    PRECAMPAIGN_TOKEN,
];
static ABILITYCATEGORY_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("ABILITYCATEGORY", "pcgen_abilitycategories"),
    PRECAMPAIGN_TOKEN,
];
static FEAT_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("FEAT", "pcgen_feats"),
    PRECAMPAIGN_TOKEN,
];
static EQUIPMENT_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("EQUIPMENT", "equipment"),
    PRECAMPAIGN_TOKEN,
];
static SPELL_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("SPELL", "pcgen_spells"),
    PRECAMPAIGN_TOKEN,
];
static LICENSE_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("LICENSE", "license"),
    PRECAMPAIGN_TOKEN,
];
static INFOTEXT_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("INFOTEXT", "infotext"),
    PRECAMPAIGN_TOKEN,
];
static FORWARDREF_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "FORWARDREF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_forwardref"),
        required: true,
    },
    PRECAMPAIGN_TOKEN,
];
static HIDETYPE_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "HIDETYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_hidetype"),
        required: true,
    },
    PRECAMPAIGN_TOKEN,
];
static URL_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "URL",
        grammar: TokenGrammar::PipePositional(&["text", "url", "label"]),
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("url"),
        required: true,
    },
    PRECAMPAIGN_TOKEN,
];
static ALIGNMENT_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("ALIGNMENT", "pcgen_alignment_catalog"),
    PRECAMPAIGN_TOKEN,
];
static SAVE_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("SAVE", "pcgen_save_catalog"),
    PRECAMPAIGN_TOKEN,
];
static BIOSET_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("BIOSET", "pcgen_bioset_catalog"),
    PRECAMPAIGN_TOKEN,
];
static EQUIPMOD_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("EQUIPMOD", "pcgen_equipmod_catalog"),
    PRECAMPAIGN_TOKEN,
];
static DATACONTROL_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("DATACONTROL", "pcgen_datacontrol_catalog"),
    PRECAMPAIGN_TOKEN,
];

static DYNAMIC_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("DYNAMIC", "pcgen_dynamic_catalog"),
    PRECAMPAIGN_TOKEN,
];

pub static DYNAMIC_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-dynamic-include",
    head_token: Some("DYNAMIC"),
    head_format: HeadFormat::NameOnly,
    tokens: DYNAMIC_INCLUDE_TOKENS,
    globals: &[],
};
static DATATABLE_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("DATATABLE", "pcgen_datatable_catalog"),
    PRECAMPAIGN_TOKEN,
];
static COMPANIONMOD_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("COMPANIONMOD", "pcgen_companionmod_catalog"),
    PRECAMPAIGN_TOKEN,
];
static WEAPONPROF_INCLUDE_TOKENS: &[TokenDef] = &[
    // In PCC files: WEAPONPROF:catalog.lst (plain path stored as string)
    // In PCG files: WEAPONPROF:[WEAPON:Longsword|WEAPON:Dagger|...] (bracket group)
    // BracketGroup grammar handles both: plain strings are emitted verbatim; bracket
    // values are parsed into structured [{key:"WEAPON",value:"Longsword"},…] arrays.
    TokenDef::bracket_group("WEAPONPROF", "pcgen_weaponprof_catalog"),
    PRECAMPAIGN_TOKEN,
];
static ARMORPROF_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("ARMORPROF", "pcgen_armorprof_catalog"),
    PRECAMPAIGN_TOKEN,
];
static SHIELDPROF_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("SHIELDPROF", "pcgen_shieldprof_catalog"),
    PRECAMPAIGN_TOKEN,
];
static LSTEXCLUDE_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "LSTEXCLUDE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_lstexclude"),
        required: true,
    },
    PRECAMPAIGN_TOKEN,
];
static SHOWINMENU_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::yesno("SHOWINMENU", "pcgen_showinmenu"),
    PRECAMPAIGN_TOKEN,
];

pub static LANGUAGE_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-language-include",
    head_token: Some("LANGUAGE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: LANGUAGE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static VARIABLE_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-variable-include",
    head_token: Some("VARIABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: VARIABLE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static ISMATURE_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-ismature-include",
    head_token: Some("ISMATURE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ISMATURE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static ABILITY_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-ability-include",
    head_token: Some("ABILITY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ABILITY_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static ABILITYCATEGORY_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-abilitycategory-include",
    head_token: Some("ABILITYCATEGORY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ABILITYCATEGORY_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static FEAT_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-feat-include",
    head_token: Some("FEAT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: FEAT_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static EQUIPMENT_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-equipment-include",
    head_token: Some("EQUIPMENT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: EQUIPMENT_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static SPELL_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-spell-include",
    head_token: Some("SPELL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SPELL_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static LICENSE_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-license-include",
    head_token: Some("LICENSE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: LICENSE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static INFOTEXT_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-infotext-include",
    head_token: Some("INFOTEXT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: INFOTEXT_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static FORWARDREF_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-forwardref-include",
    head_token: Some("FORWARDREF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: FORWARDREF_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static HIDETYPE_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-hidetype-include",
    head_token: Some("HIDETYPE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: HIDETYPE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static URL_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-url-include",
    head_token: Some("URL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: URL_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static ALIGNMENT_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-alignment-include",
    head_token: Some("ALIGNMENT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ALIGNMENT_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static SAVE_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-save-include",
    head_token: Some("SAVE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SAVE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static BIOSET_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-bioset-include",
    head_token: Some("BIOSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: BIOSET_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static EQUIPMOD_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-equipmod-include",
    head_token: Some("EQUIPMOD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: EQUIPMOD_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static DATACONTROL_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-datacontrol-include",
    head_token: Some("DATACONTROL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: DATACONTROL_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static DATATABLE_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-datatable-include",
    head_token: Some("DATATABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: DATATABLE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static COMPANIONMOD_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-companionmod-include",
    head_token: Some("COMPANIONMOD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: COMPANIONMOD_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static WEAPONPROF_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-weaponprof-include",
    head_token: Some("WEAPONPROF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: WEAPONPROF_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static ARMORPROF_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-armorprof-include",
    head_token: Some("ARMORPROF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ARMORPROF_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static SHIELDPROF_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-shieldprof-include",
    head_token: Some("SHIELDPROF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SHIELDPROF_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static LSTEXCLUDE_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-lstexclude-include",
    head_token: Some("LSTEXCLUDE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: LSTEXCLUDE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static SHOWINMENU_INCLUDE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:pcc-showinmenu-include",
    head_token: Some("SHOWINMENU"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SHOWINMENU_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};
