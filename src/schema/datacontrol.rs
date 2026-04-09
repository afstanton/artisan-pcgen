//! Data-control schemas.
//!
//! Sources:
//! - corpus `*_datacontrols.lst` files
//! - PCC `DATACONTROL:` include docs in `datafilespcc.html`

use crate::schema::{EntitySchema, HeadFormat, TokenDef};

static FACTSETDEF_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("FACTSETDEF", "pcgen_factsetdef"),
    TokenDef::text("DATAFORMAT", "pcgen_dataformat"),
    TokenDef::text("VISIBLE", "pcgen_visible"),
    TokenDef::text("SELECTABLE", "pcgen_selectable"),
    TokenDef::text("REQUIRED", "pcgen_required"),
    TokenDef::text("DISPLAYNAME", "pcgen_displayname"),
    TokenDef::text("EXPLANATION", "pcgen_explanation"),
];

pub static FACTSETDEF_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:datacontrol-factsetdef",
    head_token: Some("FACTSETDEF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: FACTSETDEF_TOKENS,
    globals: &[],
};
