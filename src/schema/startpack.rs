//! STARTPACK entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesstartingkits.html`
//!
//! Starting kit files use `STARTPACK:name` heads and mostly consist of global
//! effect and prerequisite tags.

use crate::schema::{EntitySchema, GlobalGroup, HeadFormat};

pub static STARTPACK_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:startpack",
    head_token: Some("STARTPACK"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[
        GlobalGroup::Prerequisites,
        GlobalGroup::Bonus,
        GlobalGroup::Add,
        GlobalGroup::Choose,
        GlobalGroup::Auto,
        GlobalGroup::Define,
        GlobalGroup::SourceMeta,
    ],
};
