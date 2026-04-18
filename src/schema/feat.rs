//! FEAT entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesfeats.html`
//!
//! Feats are very similar to abilities but do not require a CATEGORY token.
//! The first field is the Feat Name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static ASPECT_SLOTS: &[&str] = &["name", "value", "formula"];

static FEAT_TOKENS: &[TokenDef] = &[
    // CATEGORY:FEAT is used by some PCGen data files to explicitly mark a feat
    // entity as a feat rather than an ability. Must be preserved on roundtrip so
    // that the classification signal survives re-parsing (looks_like_feat checks
    // for CATEGORY=FEAT). Feats with no CATEGORY token emit nothing here.
    TokenDef::text("CATEGORY", "category"),
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    TokenDef::integer("ADDSPELLLEVEL", "pcgen_addspelllevel"),
    TokenDef::pipe_positional_repeatable("ASPECT", ASPECT_SLOTS, "aspects"),
    TokenDef {
        key: "MODIFYFEATCHOICE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_modifyfeatchoice"),
        required: false,
    },
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
    TokenDef::pipe_list_repeatable("COMPANIONLIST", "companion_list"),
    TokenDef::pipe_positional_repeatable("FOLLOWERS", &["type", "limit"], "pcgen_followers"),
    TokenDef::pipe_list_repeatable("CCSKILL", "pcgen_ccskill"),
    TokenDef::pipe_list_repeatable("UNENCUMBEREDMOVE", "pcgen_unencumberedmove"),
    TokenDef::text("VISIBLE", "visible"),
    TokenDef {
        key: "MOVECLONE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_moveclone"),
        required: false,
    },
    // .pcg sub-token: the choice(s) this feat/ability was applied to
    TokenDef::text("APPLIEDTO", "pcgen_appliedto"),
    // BENEFIT describes the mechanical benefit of the feat (similar to ABILITY's BENEFIT).
    TokenDef::text("BENEFIT", "benefit"),
    // COST: pool-point cost (shared concept with ability entities).
    TokenDef::text("COST", "cost"),
    // INFO: free-text annotation (e.g. notes from 3rd-party publishers).
    TokenDef::text("INFO", "pcgen_info"),
    // SPELLLEVEL: associates the feat with a spell level (some feat files use this).
    TokenDef::text("SPELLLEVEL", "spell_level"),
    // MOVE, NATURALATTACKS, VISION, DR, SR — mechanical grants also found on ability entities.
    TokenDef {
        key: "MOVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("move"),
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
    TokenDef::text("DR", "pcgen_dr"),
    TokenDef::text("SR", "pcgen_sr"),
    // UDAM: unarmed damage progression (some homebrew feat files).
    TokenDef::text("UDAM", "udam"),
    // SPELLKNOWN: known-spell grant.
    TokenDef {
        key: "SPELLKNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("spell_known"),
        required: false,
    },
    // KITS: kit grants on feat entities (rare but present in some datasets).
    TokenDef {
        key: "KIT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("kits"),
        required: false,
    },
    // SPELLS: spell-like abilities granted by this feat (seen in some datasets).
    TokenDef {
        key: "SPELLS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spells"),
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
    GlobalGroup::SortKey,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::ChangeProf,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::SourceMeta,
];

pub static FEAT_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:feat",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: FEAT_TOKENS,
    globals: FEAT_GLOBALS,
};
