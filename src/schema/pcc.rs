//! PCC (Campaign) file schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilespcc.html`
//!
//! PCC files define campaign/source metadata. Rather than entities, each
//! line is a key:value directive. This schema is registered primarily so
//! that metadata tokens are classified as `SemanticallyInterpreted` by
//! the global token policy.

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static PRECAMPAIGN_TOKEN: TokenDef = TokenDef {
    key: "PRECAMPAIGN",
    grammar: TokenGrammar::PipeList,
    cardinality: Cardinality::Repeatable,
    artisan_mapping: ArtisanMapping::Attribute("pcgen_precampaign"),
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
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef {
        key: "ABILITYCATEGORY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef {
        key: "FEAT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef {
        key: "PCC",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_pcc"),
        required: false,
    },
    TokenDef {
        key: "EQUIPMENT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef {
        key: "DATACONTROL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_datacontrol_catalog"),
        required: false,
    },
    TokenDef {
        key: "COMPANIONMOD",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_companionmod_catalog"),
        required: false,
    },
    TokenDef {
        key: "EQUIPMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_equipmod_catalog"),
        required: false,
    },
    TokenDef {
        key: "LANGUAGE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_language_catalog"),
        required: false,
    },
    TokenDef {
        key: "SPELL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef {
        key: "WEAPONPROF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_weaponprof_catalog"),
        required: false,
    },
    TokenDef {
        key: "ARMORPROF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_armorprof_catalog"),
        required: false,
    },
    TokenDef {
        key: "SHIELDPROF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_shieldprof_catalog"),
        required: false,
    },
    // Legal
    TokenDef::text("ISLICENSED", "pcgen_islicensed"),
    TokenDef {
        key: "LICENSE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef::text("STATUS", "pcgen_status"),
    TokenDef::text("GENRE", "pcgen_genre"),
    TokenDef::text("ISOGL", "pcgen_isogl"),
    TokenDef {
        key: "INFOTEXT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef::text("RANK", "pcgen_rank"),
    TokenDef::text("DESC", "pcgen_desc"),
    TokenDef {
        key: "COPYRIGHT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_copyright"),
        required: false,
    },
    TokenDef::text("COVER", "pcgen_cover"),
    TokenDef::text("LOGO", "pcgen_logo"),
    TokenDef::text("ALLOWDUPES", "pcgen_allowdupes"),
    TokenDef::text("DATAFORMAT", "pcgen_dataformat"),
    TokenDef::text("EXPLANATION", "pcgen_explanation"),
    TokenDef::text("DISPLAYNAME", "pcgen_displayname"),
    TokenDef::text("VISIBLE", "pcgen_visible"),
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
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef {
        key: "HIDETYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef {
        key: "PRECAMPAIGN",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_precampaign"),
        required: false,
    },
    TokenDef {
        key: "URL",
        grammar: TokenGrammar::PipePositional(&["text", "url", "label"]),
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::None,
        required: false,
    },
    TokenDef::text("OPTION", "pcgen_option"),
];

static PCC_GLOBALS: &[GlobalGroup] = &[GlobalGroup::SourceMeta];

pub static PCC_SCHEMA: EntitySchema = EntitySchema {
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
static COMPANIONMOD_INCLUDE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("COMPANIONMOD", "pcgen_companionmod_catalog"),
    PRECAMPAIGN_TOKEN,
];
static WEAPONPROF_INCLUDE_TOKENS: &[TokenDef] =
    &[
        TokenDef::text_required("WEAPONPROF", "pcgen_weaponprof_catalog"),
        PRECAMPAIGN_TOKEN,
    ];
static ARMORPROF_INCLUDE_TOKENS: &[TokenDef] =
    &[
        TokenDef::text_required("ARMORPROF", "pcgen_armorprof_catalog"),
        PRECAMPAIGN_TOKEN,
    ];
static SHIELDPROF_INCLUDE_TOKENS: &[TokenDef] =
    &[
        TokenDef::text_required("SHIELDPROF", "pcgen_shieldprof_catalog"),
        PRECAMPAIGN_TOKEN,
    ];

pub static LANGUAGE_INCLUDE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:pcc-language-include",
    head_token: Some("LANGUAGE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: LANGUAGE_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static BIOSET_INCLUDE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:pcc-bioset-include",
    head_token: Some("BIOSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: BIOSET_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static EQUIPMOD_INCLUDE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:pcc-equipmod-include",
    head_token: Some("EQUIPMOD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: EQUIPMOD_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static DATACONTROL_INCLUDE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:pcc-datacontrol-include",
    head_token: Some("DATACONTROL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: DATACONTROL_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static COMPANIONMOD_INCLUDE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:pcc-companionmod-include",
    head_token: Some("COMPANIONMOD"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: COMPANIONMOD_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static WEAPONPROF_INCLUDE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:pcc-weaponprof-include",
    head_token: Some("WEAPONPROF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: WEAPONPROF_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static ARMORPROF_INCLUDE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:pcc-armorprof-include",
    head_token: Some("ARMORPROF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ARMORPROF_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};

pub static SHIELDPROF_INCLUDE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:pcc-shieldprof-include",
    head_token: Some("SHIELDPROF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SHIELDPROF_INCLUDE_TOKENS,
    globals: PCC_GLOBALS,
};
