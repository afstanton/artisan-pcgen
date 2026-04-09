//! SUBCLASS entity schema.
//!
//! SUBCLASS lines define class specialization options. The head is
//! token-prefixed: `SUBCLASS:name`.

use crate::schema::{
    EntitySchema, GlobalGroup, HeadFormat, TokenDef,
};

static SUBCLASS_TOKENS: &[TokenDef] = &[
    TokenDef::text("COST", "pcgen_cost"),
    TokenDef::pipe_list_repeatable("CHOICE", "pcgen_choice"),
    TokenDef::pipe_positional_repeatable("SPELLLIST", &["level", "list"], "pcgen_spelllist"),
    TokenDef::text("KNOWNSPELLSFROMSPECIALTY", "pcgen_knownspellsfromspecialty"),
    TokenDef::integer("PROHIBITCOST", "pcgen_prohibitcost"),
];

static SUBCLASS_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::SourceLink,
    GlobalGroup::OutputName,
    GlobalGroup::LangBonus,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::ChangeProf,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::Template,
    GlobalGroup::SourceMeta,
];

pub static SUBCLASS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:subclass",
    head_token: Some("SUBCLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SUBCLASS_TOKENS,
    globals: SUBCLASS_GLOBALS,
};
