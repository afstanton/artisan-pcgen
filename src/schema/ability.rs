//! ABILITY entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesability.html`
//!
//! Ability files define the individual class/racial abilities that make up each
//! character. The first field is the Ability Name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static ASPECT_SLOTS: &[&str] = &["name", "value", "formula"];

static ABILITY_TOKENS: &[TokenDef] = &[
    // Required: every ability must belong to a category
    TokenDef::text_required("CATEGORY", "category"),
    // Optional entity-specific tokens (doc order)
    TokenDef::integer("ADDSPELLLEVEL", "pcgen_addspelllevel"),
    TokenDef::pipe_positional_repeatable("ASPECT", ASPECT_SLOTS, "aspects"),
    TokenDef::text("BENEFIT", "benefit"),
    TokenDef::text("COST", "cost"),
    TokenDef::text("SPELLLEVEL", "spell_level"),
    TokenDef::text("NEWCATEGORY", "newcategory"),
    TokenDef::text("INFO", "pcgen_info"),
    TokenDef::text("MULT", "pcgen_mult"),
    TokenDef::text("STACK", "pcgen_stack"),
    TokenDef {
        key: "TEMPVALUE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_tempvalue"),
        required: false,
    },
    TokenDef {
        key: "TEMPLATE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Effect,
        required: false,
    },
    TokenDef::text("VISIBLE", "visible"),
    // Spell-like abilities: SPELLS:mode|TIMES=formula|CASTERLEVEL=formula|spell,...
    TokenDef {
        key: "SPELLS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spells"),
        required: false,
    },
    TokenDef {
        key: "SPELLKNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("spell_known"),
        required: false,
    },
    TokenDef {
        key: "MOVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("move"),
        required: false,
    },
    TokenDef {
        key: "MOVECLONE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_moveclone"),
        required: false,
    },
    TokenDef {
        key: "NATURALATTACKS",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("natural_attacks"),
        required: false,
    },
    TokenDef {
        key: "VISION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("vision"),
        required: false,
    },
    TokenDef {
        key: "DR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Effect,
        required: false,
    },
    TokenDef {
        key: "SR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Effect,
        required: false,
    },
    // Equipment modification grant: EQMOD:name|key=value...
    TokenDef {
        key: "EQMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_eqmods"),
        required: false,
    },
    TokenDef::pipe_list_repeatable("COMPANIONLIST", "companion_list"),
    TokenDef::pipe_list_repeatable("CCSKILL", "pcgen_ccskill"),
    TokenDef::pipe_list_repeatable("UNENCUMBEREDMOVE", "pcgen_unencumberedmove"),
    TokenDef::pipe_positional_repeatable("FOLLOWERS", &["type", "limit"], "pcgen_followers"),
    // .pcg sub-token: the choice(s) this feat/ability was applied to
    TokenDef::text("APPLIEDTO", "pcgen_appliedto"),
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    // Unarmed damage progression (monk/brawler-style abilities).
    // Format: UDAM:1d6 (or a level-scaling die table).
    TokenDef::text("UDAM", "udam"),
    // Unarmed multiplier for critical hits.
    TokenDef::text("UMULT", "pcgen_umult"),
    // FREE:YES/NO — whether the ability costs nothing (does not use pool points).
    TokenDef::yesno("FREE", "pcgen_free"),
    // INFO:note — arbitrary informational text attached to the ability.
    TokenDef::text("INFO", "pcgen_info"),
    // PRECAMPAIGN: prerequisite that limits the ability to specific campaign settings.
    // Repeatable since multiple campaign constraints can be stacked.
    TokenDef {
        key: "PRECAMPAIGN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_precampaign"),
        required: false,
    },
    // OPTION: selection option annotation (used in .pcg ability records).
    TokenDef::text("OPTION", "option"),
    // REGION:region_name restricts ability availability to characters from a region.
    TokenDef::text("REGION", "region"),
    // REMOVE removes a previously applied ability (used in .MOD contexts).
    TokenDef::text("REMOVE", "pcgen_remove"),
    // KIT:name — kit grant on an ability entity (seen in some Pathfinder datasets).
    TokenDef {
        key: "KIT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("kits"),
        required: false,
    },
    // DONOTADD: prevent an inherited skill from being class-skill on this entity.
    TokenDef {
        key: "DONOTADD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("do_not_add"),
        required: false,
    },
    // NOTE: free-text annotation attached to the ability (seen in some datasets).
    TokenDef::text("NOTE", "note"),
];

static ABILITY_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::SortKey,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::LangBonus,
    GlobalGroup::ChangeProf,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::SourceMeta,
];

pub static ABILITY_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:ability",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: ABILITY_TOKENS,
    globals: ABILITY_GLOBALS,
};
