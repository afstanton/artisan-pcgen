//! KIT entry schema.
//!
//! KIT lines use `KIT:name` heads and can reference nested kits and gear.

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static KIT_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "KIT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("kits"),
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
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    TokenDef::text("OPTION", "option"),
    TokenDef {
        key: "EQUIPBUY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("equip_buy"),
        required: false,
    },
    TokenDef::text("LOCATION", "location"),
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
        GlobalGroup::SortKey,
        GlobalGroup::SourceMeta,
    ],
};
