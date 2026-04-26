//! EQUIPMENT entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesequipment.html`
//!
//! Equipment files define items. The head is the item name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef, TokenGrammar,
};

static QUALITY_SLOTS: &[&str] = &["name", "value"];
static PROFICIENCY_SLOTS: &[&str] = &["type", "name"];
static CHARGES_SLOTS: &[&str] = &["min", "max"];
static ARMORTYPE_SLOTS: &[&str] = &["from", "to"];

static EQUIPMENT_TOKENS: &[TokenDef] = &[
    // Weight and cost
    TokenDef::text("WT", "weight"),
    TokenDef::text("COST", "cost"),
    TokenDef::text("COSTPRE", "pcgen_costpre"),
    TokenDef::text("BASEITEM", "pcgen_baseitem"),
    TokenDef::pipe_positional("CHARGES", CHARGES_SLOTS, "pcgen_charges"),
    // Size and wield
    TokenDef::text("SIZE", "size"),
    TokenDef::text("WIELD", "pcgen_wield"),
    // Armor/shield properties
    TokenDef::integer("EDR", "pcgen_edr"),
    TokenDef::integer("SPELLFAILURE", "pcgen_spellfailure"),
    TokenDef::text("ACCHECK", "pcgen_accheck"),
    TokenDef::text("MAXDEX", "pcgen_maxdex"),
    TokenDef::text("SLOTS", "pcgen_slots"),
    // Weapon properties
    TokenDef::text("DAMAGE", "pcgen_damage"),
    TokenDef {
        key: "PART",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_part"),
        required: false,
    },
    TokenDef::text("ALTDAMAGE", "pcgen_altdamage"),
    TokenDef::text("ALTTYPE", "pcgen_alttype"),
    TokenDef::text("CRITMULT", "pcgen_critmult"),
    TokenDef::text("CRITRANGE", "pcgen_critrange"),
    TokenDef::text("FUMBLERANGE", "pcgen_fumblerange"),
    TokenDef::text("RATEOFFIRE", "pcgen_rateoffire"),
    // RANGE: distance increment in feet for ranged/thrown weapons (e.g. RANGE:20).
    TokenDef::text("RANGE", "pcgen_range"),
    TokenDef::text("REACH", "reach"),
    TokenDef::text("REACHMULT", "pcgen_reachmult"),
    TokenDef::text("ALTCRITMULT", "pcgen_altcritmult"),
    TokenDef::text("ALTCRITRANGE", "pcgen_altcritrange"),
    TokenDef::text("ALTCRITICAL", "pcgen_altcritical"),
    TokenDef::text("ALTEQMOD", "pcgen_alteqmod"),
    TokenDef::integer("PLUS", "pcgen_plus"),
    TokenDef {
        key: "ITYPE",
        grammar: TokenGrammar::DotList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_itype"),
        required: false,
    },
    TokenDef::text("NAMEOPT", "pcgen_nameopt"),
    TokenDef::text("FORMATCAT", "format_cat"),
    TokenDef::yesno("ASSIGNTOALL", "pcgen_assigntoall"),
    TokenDef::pipe_positional_repeatable("ARMORTYPE", ARMORTYPE_SLOTS, "pcgen_armortype"),
    TokenDef::pipe_positional("PROFICIENCY", PROFICIENCY_SLOTS, "proficiency"),
    TokenDef {
        key: "REPLACES",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_replaces"),
        required: false,
    },
    // Container
    TokenDef::text("CONTAINS", "contains"),
    TokenDef::text("BASEQTY", "pcgen_baseqty"),
    TokenDef::text("MODS", "pcgen_mods"),
    // Aesthetic
    TokenDef::text("ICON", "pcgen_icon"),
    TokenDef::integer("NUMPAGES", "pcgen_numpages"),
    TokenDef {
        key: "PAGEUSAGE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_pageusage"),
        required: false,
    },
    // Quality annotation (repeatable name|value pairs)
    TokenDef::pipe_positional_repeatable("QUALITY", QUALITY_SLOTS, "pcgen_qualities"),
    // Special property text
    TokenDef {
        key: "SPROP",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("sprop"),
        required: false,
    },
    TokenDef {
        key: "ABILITY",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("abilities"),
        required: false,
    },
    // Spell-like items: SPELLS:mode|TIMES=formula|CL=formula|spell,DC,...
    // Used by wands, staves, and other spell-trigger/spell-completion items.
    TokenDef {
        key: "SPELLS",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_spells"),
        required: false,
    },
    TokenDef::text("VISIBLE", "visible"),
    TokenDef::text("CATEGORY", "category"),
    // Equipment modifiers (e.g. EQMOD:MASTERWORK|SILVER). Stored as an array of
    // {raw, parts} objects by fields.rs; serialised using the raw string on emit.
    TokenDef {
        key: "EQMOD",
        grammar: TokenGrammar::PipeGroups,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_eqmods"),
        required: false,
    },
    // INFO: free-text annotation (appears in some Pathfinder/3.5e equipment files).
    TokenDef::text("INFO", "pcgen_info"),
    // KIT:name — equipment entries in kit files can have KIT cross-references.
    TokenDef {
        key: "KIT",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("kits"),
        required: false,
    },
    // -------------------------------------------------------------------------
    // Creature-property tokens
    //
    // Some PCGen equipment entries represent creatures (familiars, animal
    // companions, mounts, etc.).  These entries share tokens with race and
    // companion-modifier files.
    // -------------------------------------------------------------------------
    TokenDef::text("FACE", "face"),
    TokenDef::text("LEGS", "legs"),
    TokenDef::text("HANDS", "hands"),
    TokenDef {
        key: "DR",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_dr"),
        required: false,
    },
    TokenDef::text("SR", "pcgen_sr"),
    TokenDef::text("CR", "cr"),
    TokenDef::text("UDAM", "udam"),
    TokenDef::text("HITDICEADVANCEMENT", "pcgen_hitdiceadvancement"),
    TokenDef {
        key: "RACESUBTYPE",
        grammar: TokenGrammar::PipeList,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("race_subtype"),
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
        key: "TEMPVALUE",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_tempvalue"),
        required: false,
    },
    TokenDef {
        key: "CHANGEPROF",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("pcgen_changeprof"),
        required: false,
    },
    TokenDef {
        key: "SPELLKNOWN",
        grammar: TokenGrammar::Text,
        cardinality: Cardinality::Repeatable,
        artisan_mapping: ArtisanMapping::Field("spell_known"),
        required: false,
    },
];

static EQUIPMENT_GLOBALS: &[GlobalGroup] = &[
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
    GlobalGroup::Sab,
    GlobalGroup::ServesAs,
    GlobalGroup::Qualify,
    GlobalGroup::Template,
    GlobalGroup::CSkill,
    GlobalGroup::SourceMeta,
];

pub static EQUIPMENT_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:equipment",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: EQUIPMENT_TOKENS,
    globals: EQUIPMENT_GLOBALS,
};
