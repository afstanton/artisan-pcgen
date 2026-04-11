//! STARTPACK entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesstartingkits.html`
//!
//! Starting kit files use `STARTPACK:name` heads and mostly consist of global
//! effect and prerequisite tags.

use crate::schema::{
    ArtisanMapping, Cardinality, LineGrammar, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static STARTPACK_TOKENS: &[TokenDef] = &[
    TokenDef::text("APPLY", "pcgen_apply"),
    TokenDef::text("EQUIPBUY", "pcgen_equipbuy"),
    TokenDef::text("TOTALCOST", "pcgen_totalcost"),
    TokenDef {
        key: "LOOKUP",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_lookup"),
        required: false,
    },
];

static FUNDS_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("FUNDS", "pcgen_funds"),
    TokenDef::text("QTY", "pcgen_qty"),
];

static GENDER_TOKENS: &[TokenDef] = &[TokenDef::text_required("GENDER", "pcgen_gender")];

static LANGAUTO_TOKENS: &[TokenDef] = &[TokenDef {
    key: "LANGAUTO",
    grammar: TokenGrammar::PipeList,
    cardinality: Cardinality::Once,
    artisan_mapping: ArtisanMapping::Field("pcgen_langauto"),
    required: true,
}];

static TOTALCOST_TOKENS: &[TokenDef] = &[TokenDef::text_required("TOTALCOST", "pcgen_totalcost")];

pub static STARTPACK_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:startpack",
    head_token: Some("STARTPACK"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: STARTPACK_TOKENS,
    globals: &[
        GlobalGroup::Prerequisites,
        GlobalGroup::Bonus,
        GlobalGroup::Add,
        GlobalGroup::Choose,
        GlobalGroup::Auto,
        GlobalGroup::Define,
        GlobalGroup::Modify,
        GlobalGroup::LangBonus,
        GlobalGroup::SourceMeta,
    ],
};

pub static FUNDS_STARTPACK_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:startpack-funds",
    head_token: Some("FUNDS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: FUNDS_TOKENS,
    globals: &[GlobalGroup::Prerequisites, GlobalGroup::SourceMeta],
};

pub static GENDER_STARTPACK_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:startpack-gender",
    head_token: Some("GENDER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: GENDER_TOKENS,
    globals: &[GlobalGroup::Prerequisites, GlobalGroup::SourceMeta],
};

pub static LANGAUTO_STARTPACK_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:startpack-langauto",
    head_token: Some("LANGAUTO"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: LANGAUTO_TOKENS,
    globals: &[GlobalGroup::Prerequisites, GlobalGroup::SourceMeta],
};

pub static TOTALCOST_STARTPACK_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:startpack-totalcost",
    head_token: Some("TOTALCOST"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: TOTALCOST_TOKENS,
    globals: &[GlobalGroup::Prerequisites, GlobalGroup::SourceMeta],
};
