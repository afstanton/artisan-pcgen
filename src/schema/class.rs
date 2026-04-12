//! CLASS entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesclasses.html`
//!
//! Class files define character classes. The head is token-prefixed: `CLASS:name`.

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static SPELLLIST_SLOTS: &[&str] = &["count", "type", "class"];

static CLASS_TOKENS: &[TokenDef] = &[
    // Core class statistics
    TokenDef::integer("HITDIE", "hitdie"),
    TokenDef::integer("HD", "hitdie"),
    TokenDef::text("MAXLEVEL", "maxlevel"),
    TokenDef::text("ABB", "abbreviation"),
    TokenDef::integer("STARTSKILLPTS", "pcgen_startskillpts"),
    TokenDef::integer("LEVELSPERFEAT", "pcgen_levelsperfeat"),
    TokenDef::text("ATTACKCYCLE", "pcgen_attackcycle"),
    TokenDef {
        key: "LANGAUTO",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_langauto"),
        required: false,
    },
    // Spell-related
    TokenDef::text("SPELLTYPE", "pcgen_spelltype"),
    TokenDef::text("SPELLSTAT", "spellstat"),
    TokenDef::text("BONUSSPELLSTAT", "pcgen_bonusspellstat"),
    TokenDef::text("ITEMCREATE", "pcgen_itemcreate"),
    TokenDef::yesno("SPELLBOOK", "pcgen_spellbook"),
    TokenDef::yesno("MEMORIZE", "pcgen_memorize"),
    TokenDef {
        key: "CAST",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_cast"),
        required: false,
    },
    TokenDef {
        key: "KNOWN",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_known"),
        required: false,
    },
    TokenDef {
        key: "KNOWNSPELLS",
        grammar: TokenGrammar::PipeGroups,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_knownspells"),
        required: false,
    },
    TokenDef {
        key: "SPECIALTYKNOWN",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_specialtyknown"),
        required: false,
    },
    TokenDef::pipe_positional("SPELLLIST", SPELLLIST_SLOTS, "pcgen_spelllist"),
    TokenDef {
        key: "PROHIBITSPELL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_prohibitspell"),
        required: false,
    },
    TokenDef::text("KNOWNSPELLSFROMSPECIALTY", "pcgen_knownspellsfromspecialty"),
    // Prohibited spell schools (specialist wizards). Repeatable; each occurrence
    // is one school name. Also appears as a sub-token in .pcg CLASS lines.
    TokenDef {
        key: "PROHIBITED",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_prohibited"),
        required: false,
    },
    // Domain
    TokenDef {
        key: "ADDDOMAINS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("add_domains"),
        required: false,
    },
    TokenDef {
        key: "DOMAIN",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("domains"),
        required: false,
    },
    // Class mechanics
    TokenDef::yesno("ALLOWBASECLASS", "pcgen_allowbaseclass"),
    TokenDef::text("COST", "cost"),
    TokenDef::yesno("MODTOSKILLS", "pcgen_modtoskills"),
    TokenDef::integer("MONSKILL", "pcgen_monskill"),
    TokenDef::integer("MONNONSKILLHD", "pcgen_monnonskillhd"),
    TokenDef {
        key: "WEAPONBONUS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("weapon_bonus"),
        required: false,
    },
    TokenDef::text("VISIBLE", "pcgen_visible"),
    TokenDef::pipe_list_repeatable("COMPANIONLIST", "pcgen_companionlist"),
    TokenDef {
        key: "DEITY",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("deity"),
        required: false,
    },
    TokenDef::pipe_positional_repeatable("FOLLOWERS", &["type", "limit"], "pcgen_followers"),
    TokenDef::text("UDAM", "pcgen_udam"),
    TokenDef::integer("UMULT", "pcgen_umult"),
    // Sub-class / substitution
    TokenDef::integer("PROHIBITCOST", "pcgen_prohibitcost"),
    TokenDef::yesno("HASSUBCLASS", "pcgen_hassubclass"),
    TokenDef::text("SUBCLASSLEVEL", "pcgen_subclasslevel"),
    TokenDef::text("SUBCLASS", "pcgen_subclass"),
    TokenDef::text("SUBSTITUTIONCLASS", "pcgen_substitutionclass"),
    TokenDef::text("SUBSTITUTIONLEVEL", "pcgen_substitutionlevel"),
    TokenDef {
        key: "EXCLASS",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_exclass"),
        required: false,
    },
    TokenDef {
        key: "EXCHANGELEVEL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_exchangelevel"),
        required: false,
    },
    TokenDef::pipe_list_repeatable("CCSKILL", "pcgen_ccskill"),
    TokenDef::pipe_list_repeatable("UNENCUMBEREDMOVE", "pcgen_unencumberedmove"),
    TokenDef::text("ROLE", "pcgen_role"),
    // SKILLLIST:count|ClassName — assigns skill list from another class (3e Scarred Lands)
    TokenDef {
        key: "SKILLLIST",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_skilllist"),
        required: false,
    },
    // Additional structured tokens for roundtrip emission
    TokenDef {
        key: "FUNCTION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_function"),
        required: false,
    },
    TokenDef {
        key: "EQUIPMOD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_equipmod_catalog"),
        required: false,
    },
    TokenDef {
        key: "WEAPONPROF",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_weaponprof_catalog"),
        required: false,
    },
    TokenDef {
        key: "EXPLANATION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("explanation"),
        required: false,
    },
    TokenDef {
        key: "DYNAMICSCOPE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_dynamicscope"),
        required: false,
    },
    TokenDef {
        key: "SKILLCOST_CLASS",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_skillcost_class"),
        required: false,
    },
    TokenDef {
        key: "SKILLCOST_EXCLUSIVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_skillcost_exclusive"),
        required: false,
    },
    TokenDef {
        key: "SPELLBASECONCENTRATION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spellbaseconcentration"),
        required: false,
    },
    TokenDef {
        key: "XPAWARD",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_xpaward"),
        required: false,
    },
    TokenDef::yesno("HASSUBCLASS", "pcgen_hassubclass"),
    TokenDef {
        key: "COSTPRE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_costpre"),
        required: false,
    },
    TokenDef {
        key: "BASEAGEADD",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("base_age_add"),
        required: false,
    },
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_abilities"),
        required: false,
    },
    // .pcg character-file sub-tokens (appear in CLASS lines inside .pcg files)
    TokenDef::integer("SKILLPOOL", "pcgen_class_skillpool"),
    TokenDef::text("SPELLBASE", "pcgen_class_spellbase"),
    TokenDef {
        key: "CANCASTPERDAY",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_class_cancastperday"),
        required: false,
    },
];

static CLASS_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::LangBonus,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::ChangeProf,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::Template,
    GlobalGroup::SourceMeta,
];

pub static CLASS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:class",
    head_token: Some("CLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: CLASS_TOKENS,
    globals: CLASS_GLOBALS,
};
