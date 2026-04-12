//! KIT entry schema.
//!
//! KIT lines use `KIT:name` heads and can reference nested kits and gear.

use crate::schema::{
    ArtisanMapping, Cardinality, LineGrammar, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static KIT_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "KIT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_kits"),
        required: false,
    },
    TokenDef {
        key: "GEAR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_gear"),
        required: false,
    },
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_abilities"),
        required: false,
    },
    TokenDef::text("OPTION", "pcgen_option"),
    TokenDef {
        key: "EQUIPBUY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_equipbuy"),
        required: false,
    },
    TokenDef::text("LOCATION", "pcgen_location"),
    TokenDef::text("QTY", "qty"),
    TokenDef::integer("COUNT", "count"),
    TokenDef::yesno("FREE", "pcgen_free"),
    TokenDef {
        key: "SELECTION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_selection"),
        required: false,
    },
    TokenDef::text("SIZE", "size"),
    TokenDef {
        key: "EQMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_eqmods"),
        required: false,
    },
];

pub static KIT_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:kit",
    head_token: Some("KIT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: KIT_TOKENS,
    globals: &[
        GlobalGroup::Prerequisites,
        GlobalGroup::LangBonus,
        GlobalGroup::SourceMeta,
    ],
};
