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
    TokenDef::text("RANK", "pcgen_rank"),
    TokenDef::text("DESC", "pcgen_desc"),
    TokenDef::text("COVER", "pcgen_cover"),
    TokenDef::text("LOGO", "pcgen_logo"),
    TokenDef::text("ALLOWDUPES", "pcgen_allowdupes"),
    TokenDef::text("DATAFORMAT", "pcgen_dataformat"),
    TokenDef::text("EXPLANATION", "pcgen_explanation"),
    TokenDef::text("REQUIRED", "pcgen_required"),
    TokenDef::text("SELECTABLE", "pcgen_selectable"),
    TokenDef::text("MAXVER", "pcgen_maxver"),
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
        artisan_mapping: ArtisanMapping::None,
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
