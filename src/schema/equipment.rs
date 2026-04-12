//! EQUIPMENT entity schema.
//!
//! Source: `docs/listfilepages/datafilestagpages/datafilesequipment.html`
//!
//! Equipment files define items. The head is the item name (no token prefix).

use crate::schema::{
    ArtisanMapping, Cardinality, LineGrammar, GlobalGroup, HeadFormat, TokenDef, TokenGrammar,
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
    TokenDef::text("REACH", "pcgen_reach"),
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
    TokenDef::text("FORMATCAT", "pcgen_formatcat"),
    TokenDef::yesno("ASSIGNTOALL", "pcgen_assigntoall"),
    TokenDef::pipe_positional_repeatable("ARMORTYPE", ARMORTYPE_SLOTS, "pcgen_armortype"),
    TokenDef::pipe_positional("PROFICIENCY", PROFICIENCY_SLOTS, "pcgen_proficiency"),
    TokenDef {
        key: "REPLACES",
        grammar: TokenGrammar::CommaList,
        cardinality: Cardinality::Once,
        artisan_mapping: ArtisanMapping::Field("pcgen_replaces"),
        required: false,
    },
    // Container
    TokenDef::text("CONTAINS", "pcgen_contains"),
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
        artisan_mapping: ArtisanMapping::Field("pcgen_sprop"),
        required: false,
    },
    // Magic item properties found in some third-party equipmod datasets.
    // FORTIFICATION provides a percentage chance to negate crits/sneak attacks.
    // HEALING triggers a healing spell effect when the property activates.
    // Both appear as `TOKEN:value` clauses in equipmod entity definitions.
    TokenDef::text("FORTIFICATION", "pcgen_fortification"),
    TokenDef::text("HEALING", "pcgen_healing"),
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
    GlobalGroup::SourceMeta,
];

pub static EQUIPMENT_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:entity:equipment",
    head_token: None,
    head_format: HeadFormat::NameOnly,
    tokens: EQUIPMENT_TOKENS,
    globals: EQUIPMENT_GLOBALS,
};
