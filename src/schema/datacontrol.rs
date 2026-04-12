//! Data-control schemas.
//!
//! Sources:
//! - corpus `*_datacontrols.lst` files
//! - PCC `DATACONTROL:` include docs in `datafilespcc.html`

use crate::schema::{LineGrammar, HeadFormat, TokenDef};

static FACTSETDEF_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("FACTSETDEF", "pcgen_factsetdef"),
    TokenDef::text("DATAFORMAT", "pcgen_dataformat"),
    TokenDef::text("VISIBLE", "pcgen_visible"),
    TokenDef::text("SELECTABLE", "pcgen_selectable"),
    TokenDef::text("REQUIRED", "pcgen_required"),
    TokenDef::text("DISPLAYNAME", "pcgen_displayname"),
    TokenDef::text("EXPLANATION", "explanation"),
];

pub static FACTSETDEF_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:datacontrol-factsetdef",
    head_token: Some("FACTSETDEF"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: FACTSETDEF_TOKENS,
    globals: &[],
};

static FUNCTION_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("VALUE", "pcgen_value"),
    TokenDef::text("EXPLANATION", "explanation"),
];

pub static FUNCTION_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:datacontrol-function",
    head_token: Some("FUNCTION"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: FUNCTION_TOKENS,
    globals: &[],
};

pub static DYNAMICSCOPE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:datacontrol-dynamicscope",
    head_token: Some("DYNAMICSCOPE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[],
    globals: &[],
};
