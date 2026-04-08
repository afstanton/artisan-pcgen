//! System-file schemas for game mode and registry records.
//!
//! Sources:
//! - `docs/listfilepages/systemfilestagpages/gamemodestatsandcheckslist.html`
//! - `docs/listfilepages/systemfilestagpages/gamemodemiscinfolist.html`
//! - `docs/listfilepages/systemfilestagpages/equipiconslst.html`

use crate::schema::{EntitySchema, HeadFormat, TokenDef};

static BONUSSPELLLEVEL_TOKENS: &[TokenDef] = &[
    TokenDef::integer("BASESTATSCORE", "pcgen_basestatscore"),
    TokenDef::integer("STATRANGE", "pcgen_statrange"),
];

pub static BONUSSPELLLEVEL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:bonusspelllevel",
    head_token: Some("BONUSSPELLLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: BONUSSPELLLEVEL_TOKENS,
    globals: &[],
};

pub static BONUSSTACKS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:bonusstacks",
    head_token: Some("BONUSSTACKS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static BONUSFEATLEVELSTARTINTERVAL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:bonusfeatlevelstartinterval",
    head_token: Some("BONUSFEATLEVELSTARTINTERVAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static BONUSSTATLEVELSTARTINTERVAL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:bonusstatlevelstartinterval",
    head_token: Some("BONUSSTATLEVELSTARTINTERVAL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static PREVIEWDIR_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:previewdir",
    head_token: Some("PREVIEWDIR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static PREVIEWSHEET_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:previewsheet",
    head_token: Some("PREVIEWSHEET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static ICON_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:icon",
    head_token: Some("ICON"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static ALIGN_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:align",
    head_token: Some("ALIGN"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static STAT_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:stat",
    head_token: Some("STAT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static RACE_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:race",
    head_token: Some("RACE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};

pub static NAME_SYSTEM_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:system:name",
    head_token: Some("NAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};
