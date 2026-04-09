//! ABILITYCATEGORY entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesabilitycategory.html`
//!
//! Ability category records are represented by head token `ABILITYCATEGORY:<name>`.

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static ABILITYCATEGORY_TOKENS: &[TokenDef] = &[
    TokenDef {
        key: "ABILITYLIST",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_abilitylist"),
        required: false,
    },
    TokenDef::text("DISPLAYLOCATION", "pcgen_displaylocation"),
    TokenDef::text("DISPLAYNAME", "pcgen_displayname"),
    TokenDef::text("EDITABLE", "pcgen_editable"),
    TokenDef::text("EDITPOOL", "pcgen_editpool"),
    TokenDef::text("FRACTIONALPOOL", "pcgen_fractionalpool"),
    TokenDef::text("PLURAL", "pcgen_plural"),
    TokenDef::text("POOL", "pcgen_pool"),
];

static ABILITYCATEGORY_GLOBALS: &[GlobalGroup] = &[
    GlobalGroup::Type,
    GlobalGroup::Key,
    GlobalGroup::Desc,
    GlobalGroup::Fact,
    GlobalGroup::Bonus,
    GlobalGroup::Add,
    GlobalGroup::Choose,
    GlobalGroup::Auto,
    GlobalGroup::Define,
    GlobalGroup::Modify,
    GlobalGroup::Prerequisites,
    GlobalGroup::SourcePage,
    GlobalGroup::SourceLink,
    GlobalGroup::OutputName,
    GlobalGroup::SourceMeta,
];

pub static ABILITYCATEGORY_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:abilitycategory",
    head_token: Some("ABILITYCATEGORY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: ABILITYCATEGORY_TOKENS,
    globals: ABILITYCATEGORY_GLOBALS,
};
