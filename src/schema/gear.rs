//! GEAR entry schema for starting kit files.
//!
//! GEAR lines use `GEAR:name` heads and optional placement/modifier clauses.

use crate::schema::{
    ArtisanMapping, Cardinality, LineGrammar, HeadFormat, TokenDef, TokenGrammar,
};

static GEAR_TOKENS: &[TokenDef] = &[
    TokenDef::text("LOCATION", "pcgen_location"),
    TokenDef::text("QTY", "qty"),
    TokenDef::text("OPTION", "pcgen_option"),
    TokenDef::text("LOOKUP", "pcgen_lookup"),
    TokenDef::text("SIZE", "size"),
    TokenDef::text("MAXCOST", "pcgen_maxcost"),
    TokenDef {
        key: "EQMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_eqmods"),
        required: false,
    },
];

pub static GEAR_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:gear",
    head_token: Some("GEAR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: GEAR_TOKENS,
    globals: &[],
};
