//! CLASS level entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesclasses.html`
//!
//! Class level lines use the level number as the head with no token prefix.

use crate::schema::{ArtisanMapping, Cardinality, HeadFormat, LineGrammar, TokenDef, TokenGrammar};

static CLASSLEVEL_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "DONOTADD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_donotadd"),
        required: false,
    },
    TokenDef::text("UDAM", "pcgen_udam"),
    TokenDef::integer("UMULT", "pcgen_umult"),
];

pub static CLASSLEVEL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:classlevel",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: CLASSLEVEL_TOKENS,
    globals: &[],
};
