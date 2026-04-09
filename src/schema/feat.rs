//! FEAT entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesfeats.html`
//!
//! Feats are very similar to abilities but do not require a CATEGORY token.
//! The first field is the Feat Name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static ASPECT_SLOTS: &[&str] = &["name", "value", "formula"];

static FEAT_TOKENS: &[TokenDef] = &[
    TokenDef::integer("ADDSPELLLEVEL", "pcgen_addspelllevel"),
    TokenDef::pipe_positional_repeatable("ASPECT", ASPECT_SLOTS, "pcgen_aspects"),
    TokenDef {
        key: "MODIFYFEATCHOICE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_modifyfeatchoice"),
        required: false,
    },
    TokenDef::text("MULT", "pcgen_mult"),
    TokenDef::text("STACK", "pcgen_stack"),
    TokenDef {
        key: "TEMPVALUE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Attribute("pcgen_tempvalue"),
        required: false,
    },
    TokenDef {
        key: "TEMPLATE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Effect,
        required: false,
    },
    TokenDef::pipe_list_repeatable("COMPANIONLIST", "pcgen_companionlist"),
    TokenDef::pipe_positional_repeatable(
        "FOLLOWERS",
        &["type", "limit"],
        "pcgen_followers",
    ),
    TokenDef::text("VISIBLE", "pcgen_visible"),
       TokenDef {
           key: "MOVECLONE",
           grammar: TokenGrammar::Text,
           cardinality: Cardinality::Repeatable,
           artisan_mapping: ArtisanMapping::Attribute("pcgen_moveclone"),
           required: false,
       },
    ];

static FEAT_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::ChangeProf,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::SourceMeta,
];

pub static FEAT_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:entity:feat",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: FEAT_TOKENS,
    globals: FEAT_GLOBALS,
};
