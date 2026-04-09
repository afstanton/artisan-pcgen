//! KIT entry schema.
//!
//! KIT lines use `KIT:name` heads and can reference nested kits and gear.

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static KIT_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "KIT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_kits"),
        required: false,
    },
    TokenDef {
        key: "GEAR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_gear"),
        required: false,
    },
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_abilities"),
        required: false,
    },
    TokenDef::text("OPTION", "pcgen_option"),
    TokenDef {
        key: "EQUIPBUY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_equipbuy"),
        required: false,
    },
    TokenDef::text("LOCATION", "pcgen_location"),
    TokenDef::text("QTY", "pcgen_qty"),
    TokenDef::integer("COUNT", "pcgen_count"),
    TokenDef::yesno("FREE", "pcgen_free"),
    TokenDef::text("SIZE", "pcgen_size"),
    TokenDef {
        key: "EQMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_eqmods"),
        required: false,
    },
];

pub static KIT_SCHEMA: EntitySchema = EntitySchema {
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
