//! CLASS level entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesclasses.html`
//!
//! Class level lines use the level number as the head with no token prefix.

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static CLASSLEVEL_TOKENS: &[TokenDef] = &[
    // DR: damage reduction granted at this class level.
    // Can appear multiple times on one line (e.g. DR:20/Magic + DR:5/Evil), so Repeatable.
    TokenDef {
        key: "DR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_dr"),
        required: false,
    },
    // SR: spell resistance granted at this class level (e.g. SR:10+MonkSR).
    TokenDef {
        key: "SR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_sr"),
        required: false,
    },
    // MOVE: movement speed granted or modified at this class level.
    TokenDef {
        key: "MOVE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("move"),
        required: false,
    },
    TokenDef {
        key: "DONOTADD",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("do_not_add"),
        required: false,
    },
    TokenDef::text("UDAM", "udam"),
    TokenDef::integer("UMULT", "pcgen_umult"),
    // HITDIE: hit-die override at this class level.
    TokenDef::text("HITDIE", "hitdie"),
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
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
        key: "CAST",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("cast"),
        required: false,
    },
    TokenDef {
        key: "KNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("known"),
        required: false,
    },
    TokenDef {
        key: "SPECIALTYKNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_specialtyknown"),
        required: false,
    },
    // SPELLS: spell-like ability granted at this class level.
    TokenDef {
        key: "SPELLS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spells"),
        required: false,
    },
    // SPELLLEVEL: spell-level association at this class level.
    TokenDef {
        key: "SPELLLEVEL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("spell_level"),
        required: false,
    },
    // SPELLLIST: spell list override at this class level.
    TokenDef {
        key: "SPELLLIST",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spelllist"),
        required: false,
    },
    // VISION: vision type granted at this class level.
    TokenDef {
        key: "VISION",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("vision"),
        required: false,
    },
    // DOMAIN: domain granted at this class level (e.g. divine domain access).
    TokenDef {
        key: "DOMAIN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("domains"),
        required: false,
    },
    // NATURALATTACKS: natural attack grant at this class level.
    TokenDef {
        key: "NATURALATTACKS",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("natural_attacks"),
        required: false,
    },
    // WEAPONBONUS: weapon proficiency bonus granted at this class level.
    TokenDef {
        key: "WEAPONBONUS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("weapon_bonus"),
        required: false,
    },
    // ADDDOMAINS: additional domain slots granted at this class level.
    TokenDef {
        key: "ADDDOMAINS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("add_domains"),
        required: false,
    },
    // KIT: starting kit applied at this class level.
    TokenDef {
        key: "KIT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("kits"),
        required: false,
    },
    // EXCHANGELEVEL: level exchange with another class at this class level.
    TokenDef {
        key: "EXCHANGELEVEL",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_exchangelevel"),
        required: false,
    },
    // TEMPVALUE: temporary bonus value applied at this class level.
    TokenDef {
        key: "TEMPVALUE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_tempvalue"),
        required: false,
    },
    // DESC, DESCISPI, TEMPLATE are handled by their GlobalGroups (Desc, Template)
    // and must NOT be listed here — duplicate local tokens cause double-emission.
];

static CLASSLEVEL_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::LangBonus,
    GlobalGroup::CSkill,
    GlobalGroup::Sab,
    GlobalGroup::ChangeProf,
    GlobalGroup::Template,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::SourceMeta,
];

pub static CLASSLEVEL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:classlevel",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: CLASSLEVEL_TOKENS,
    globals: CLASSLEVEL_GLOBALS,
};
