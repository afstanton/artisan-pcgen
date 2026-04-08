//! ABILITYCATEGORY entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesabilitycategory.html`
//!
//! Ability category records are represented by head token `ABILITYCATEGORY:<name>`.

use crate::schema::{EntitySchema, GlobalGroup, HeadFormat, TokenDef};

static ABILITYCATEGORY_TOKENS: &[TokenDef] = &[];

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
    GlobalGroup::Prerequisites,
    GlobalGroup::SourcePage,
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
