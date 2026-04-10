//! PCGen character-file (`.pcg`) standalone record schemas.
//!
//! These heads appear as top-level session/profile lines in character files,
//! e.g. `PCGVERSION:2.0` or `HEIGHT:51`.

use crate::schema::{EntitySchema, HeadFormat, TokenDef};

pub static PCGVERSION_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:pcgversion",
    head_token: Some("PCGVERSION"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("PCGVERSION", "pcgen_pcgversion")],
    globals: &[],
};

pub static PURCHASEPOINTS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:purchasepoints",
    head_token: Some("PURCHASEPOINTS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "PURCHASEPOINTS",
        "pcgen_purchasepoints",
    )],
    globals: &[],
};

pub static POOLPOINTS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:poolpoints",
    head_token: Some("POOLPOINTS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("POOLPOINTS", "pcgen_poolpoints")],
    globals: &[],
};

pub static POOLPOINTSAVAIL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:poolpointsavail",
    head_token: Some("POOLPOINTSAVAIL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "POOLPOINTSAVAIL",
        "pcgen_poolpointsavail",
    )],
    globals: &[],
};

pub static TABLABEL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:tablabel",
    head_token: Some("TABLABEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("TABLABEL", "pcgen_tablabel")],
    globals: &[],
};

pub static AUTOSPELLS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:autospells",
    head_token: Some("AUTOSPELLS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("AUTOSPELLS", "pcgen_autospells")],
    globals: &[],
};

pub static USEHIGHERKNOWN_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:usehigherknown",
    head_token: Some("USEHIGHERKNOWN"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("USEHIGHERKNOWN", "pcgen_usehigherknown")],
    globals: &[],
};

pub static USEHIGHERPREPPED_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:usehigherprepped",
    head_token: Some("USEHIGHERPREPPED"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno(
        "USEHIGHERPREPPED",
        "pcgen_usehigherprepped",
    )],
    globals: &[],
};

pub static LOADCOMPANIONS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:loadcompanions",
    head_token: Some("LOADCOMPANIONS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("LOADCOMPANIONS", "pcgen_loadcompanions")],
    globals: &[],
};

pub static USETEMPMODS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:usetempmods",
    head_token: Some("USETEMPMODS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("USETEMPMODS", "pcgen_usetempmods")],
    globals: &[],
};

pub static SKILLSOUTPUTORDER_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:skillsoutputorder",
    head_token: Some("SKILLSOUTPUTORDER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "SKILLSOUTPUTORDER",
        "pcgen_skillsoutputorder",
    )],
    globals: &[],
};

pub static SKILLFILTER_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:skillfilter",
    head_token: Some("SKILLFILTER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("SKILLFILTER", "pcgen_skillfilter")],
    globals: &[],
};

pub static IGNORECOST_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:ignorecost",
    head_token: Some("IGNORECOST"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("IGNORECOST", "pcgen_ignorecost")],
    globals: &[],
};

pub static ALLOWDEBT_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:allowdebt",
    head_token: Some("ALLOWDEBT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("ALLOWDEBT", "pcgen_allowdebt")],
    globals: &[],
};

pub static AUTORESIZEGEAR_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:autoresizegear",
    head_token: Some("AUTORESIZEGEAR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("AUTORESIZEGEAR", "pcgen_autoresizegear")],
    globals: &[],
};

pub static CHARACTERNAME_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:charactername",
    head_token: Some("CHARACTERNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "CHARACTERNAME",
        "pcgen_charactername",
    )],
    globals: &[],
};

pub static PLAYERNAME_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:playername",
    head_token: Some("PLAYERNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PLAYERNAME", "pcgen_playername")],
    globals: &[],
};

pub static HEIGHT_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:height",
    head_token: Some("HEIGHT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("HEIGHT", "pcgen_height")],
    globals: &[],
};

pub static WEIGHT_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:weight",
    head_token: Some("WEIGHT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("WEIGHT", "pcgen_weight_value")],
    globals: &[],
};

pub static AGE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:age",
    head_token: Some("AGE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("AGE", "pcgen_age")],
    globals: &[],
};

pub static HANDED_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:handed",
    head_token: Some("HANDED"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("HANDED", "pcgen_handed")],
    globals: &[],
};
