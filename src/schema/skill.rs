//! SKILL entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesskills.html`
//!
//! Skill files define individual skills. The head is token-prefixed: `SKILL:name`.

use crate::schema::{
    ArtisanMapping, Cardinality, LineGrammar, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
};

static SITUATION_SLOTS: &[&str] = &["name", "modifier"];

static SKILL_TOKENS: &[TokenDef] = &[
    // CLASSES is the skill's class list: which classes treat this as a class skill
    TokenDef {
        key: "CLASSES",
        grammar: TokenGrammar::PipeGroups,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_classes"),
        required: false,
    },
    // SITUATION adds a conditional modifier to the skill
    TokenDef::pipe_positional_repeatable("SITUATION", SITUATION_SLOTS, "pcgen_situations"),
    TokenDef::text("RANK", "pcgen_rank"),
    TokenDef::integer("COUNT", "pcgen_count"),
    TokenDef::yesno("FREE", "pcgen_free"),
    TokenDef {
        key: "SELECTION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_selection"),
        required: false,
    },
    TokenDef::yesno("USEUNTRAINED", "pcgen_useuntrained"),
    TokenDef::yesno("EXCLUSIVE", "pcgen_exclusive"),
    TokenDef::text("KEYSTAT", "pcgen_keystat"),
    TokenDef::text("ACHECK", "pcgen_accheck"),
    TokenDef::text("VISIBLE", "pcgen_visible"),
    // PCG character file: CLASSBOUGHT bracket group [CLASS:Wizard|RANKS:3.0|COST:1|CLASSSKILL:Y]
    // A skill may have multiple CLASSBOUGHT entries (one per class), written as adjacent groups
    // with no pipe separator: CLASSBOUGHT:[...]CLASSBOUGHT:[...]
    TokenDef::bracket_group_repeatable("CLASSBOUGHT", "pcgen_classbought"),
];

static SKILL_GLOBALS: &[GlobalGroup] = &[
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

pub static SKILL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:skill",
    head_token: Some("SKILL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SKILL_TOKENS,
    globals: SKILL_GLOBALS,
};
