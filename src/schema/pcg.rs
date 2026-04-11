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

// ---------------------------------------------------------------------------
// Character bio tokens (standalone)
// ---------------------------------------------------------------------------

pub static TABNAME_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:tabname",
    head_token: Some("TABNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("TABNAME", "pcgen_tabname")],
    globals: &[],
};

pub static SKINCOLOR_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:skincolor",
    head_token: Some("SKINCOLOR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("SKINCOLOR", "pcgen_skincolor")],
    globals: &[],
};

pub static EYECOLOR_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:eyecolor",
    head_token: Some("EYECOLOR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("EYECOLOR", "pcgen_eyecolor")],
    globals: &[],
};

pub static HAIRCOLOR_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:haircolor",
    head_token: Some("HAIRCOLOR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("HAIRCOLOR", "pcgen_haircolor")],
    globals: &[],
};

pub static HAIRSTYLE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:hairstyle",
    head_token: Some("HAIRSTYLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("HAIRSTYLE", "pcgen_hairstyle")],
    globals: &[],
};

pub static CITY_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:city",
    head_token: Some("CITY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CITY", "pcgen_city")],
    globals: &[],
};

pub static BIRTHDAY_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:birthday",
    head_token: Some("BIRTHDAY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("BIRTHDAY", "pcgen_birthday")],
    globals: &[],
};

pub static BIRTHPLACE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:birthplace",
    head_token: Some("BIRTHPLACE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("BIRTHPLACE", "pcgen_birthplace")],
    globals: &[],
};

pub static PERSONALITYTRAIT1_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:personalitytrait1",
    head_token: Some("PERSONALITYTRAIT1"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PERSONALITYTRAIT1", "pcgen_personalitytrait1")],
    globals: &[],
};

pub static PERSONALITYTRAIT2_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:personalitytrait2",
    head_token: Some("PERSONALITYTRAIT2"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PERSONALITYTRAIT2", "pcgen_personalitytrait2")],
    globals: &[],
};

pub static SPEECHPATTERN_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:speechpattern",
    head_token: Some("SPEECHPATTERN"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("SPEECHPATTERN", "pcgen_speechpattern")],
    globals: &[],
};

pub static PHOBIAS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:phobias",
    head_token: Some("PHOBIAS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PHOBIAS", "pcgen_phobias")],
    globals: &[],
};

pub static INTERESTS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:interests",
    head_token: Some("INTERESTS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("INTERESTS", "pcgen_interests")],
    globals: &[],
};

pub static CATCHPHRASE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:catchphrase",
    head_token: Some("CATCHPHRASE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CATCHPHRASE", "pcgen_catchphrase")],
    globals: &[],
};

pub static PORTRAIT_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:portrait",
    head_token: Some("PORTRAIT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PORTRAIT", "pcgen_portrait")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character progression tokens (standalone)
// ---------------------------------------------------------------------------

pub static EXPERIENCE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:experience",
    head_token: Some("EXPERIENCE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("EXPERIENCE", "pcgen_experience")],
    globals: &[],
};

pub static EXPERIENCETABLE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:experiencetable",
    head_token: Some("EXPERIENCETABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("EXPERIENCETABLE", "pcgen_experiencetable")],
    globals: &[],
};

/// Currency total in base units. Stored as text to accommodate decimal values.
pub static MONEY_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:money",
    head_token: Some("MONEY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("MONEY", "pcgen_money")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character description blocks (standalone)
// ---------------------------------------------------------------------------

pub static CHARACTERBIO_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:characterbio",
    head_token: Some("CHARACTERBIO"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERBIO", "pcgen_characterbio")],
    globals: &[],
};

pub static CHARACTERDESC_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:characterdesc",
    head_token: Some("CHARACTERDESC"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERDESC", "pcgen_characterdesc")],
    globals: &[],
};

pub static CHARACTERCOMP_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:charactercomp",
    head_token: Some("CHARACTERCOMP"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERCOMP", "pcgen_charactercomp")],
    globals: &[],
};

pub static CHARACTERASSET_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:characterasset",
    head_token: Some("CHARACTERASSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERASSET", "pcgen_characterasset")],
    globals: &[],
};

pub static CHARACTERMAGIC_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:charactermagic",
    head_token: Some("CHARACTERMAGIC"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERMAGIC", "pcgen_charactermagic")],
    globals: &[],
};

pub static CHARACTERDMNOTES_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:characterdmnotes",
    head_token: Some("CHARACTERDMNOTES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERDMNOTES", "pcgen_characterdmnotes")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Misc standalone tokens
// ---------------------------------------------------------------------------

/// PCGen application version used to save the character file.
pub static VERSION_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:version",
    head_token: Some("VERSION"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("VERSION", "pcgen_version")],
    globals: &[],
};

/// Ability pool point total for the character. Stored as text (may be decimal).
pub static FEATPOOL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:featpool",
    head_token: Some("FEATPOOL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("FEATPOOL", "pcgen_featpool")],
    globals: &[],
};

/// ID of the active equipment set.
pub static CALCEQUIPSET_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:calcequipset",
    head_token: Some("CALCEQUIPSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CALCEQUIPSET", "pcgen_calcequipset")],
    globals: &[],
};

/// Comma-delimited list of suppressed biography field names.
pub static SUPPRESSBIOFIELDS_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:suppressbiofields",
    head_token: Some("SUPPRESSBIOFIELDS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("SUPPRESSBIOFIELDS", "pcgen_suppressbiofields")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Ability pool usage (USERPOOL:name|POOLPOINTS:n)
// ---------------------------------------------------------------------------

static USERPOOL_TOKENS: &[TokenDef] = &[
    // POOLPOINTS value is decimal in pcg context (e.g. 0.0); use a distinct
    // attribute key from the standalone POOLPOINTS schema (pcgen_poolpoints)
    TokenDef::text("POOLPOINTS", "pcgen_userpool_poolpoints"),
];

pub static USERPOOL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:userpool",
    head_token: Some("USERPOOL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: USERPOOL_TOKENS,
    globals: &[],
};

// ---------------------------------------------------------------------------
// Equipment set (EQUIPSET:name|ID:n|VALUE:item|QUANTITY:n|USETEMPMODS:Y)
// ---------------------------------------------------------------------------

static EQUIPSET_TOKENS: &[TokenDef] = &[
    // pcgen_equipset_id: distinct from any other "ID" usage
    TokenDef::text("ID", "pcgen_equipset_id"),
    // VALUE uses the canonical projector attribute name (same as FUNCTION schema)
    TokenDef::text("VALUE", "pcgen_value"),
    TokenDef::text("QUANTITY", "pcgen_quantity"),
    // pcgen_equipset_usetempmods: distinct from standalone pcgen_usetempmods
    TokenDef::yesno("USETEMPMODS", "pcgen_equipset_usetempmods"),
    // Optional free-text annotation on an equipment set entry
    TokenDef::text("NOTE", "pcgen_note"),
];

pub static EQUIPSET_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:equipset",
    head_token: Some("EQUIPSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: EQUIPSET_TOKENS,
    globals: &[],
};

// ---------------------------------------------------------------------------
// Equipment name record (EQUIPNAME:name|OUTPUTORDER:n|COST:n|WT:n|QUANTITY:n|...)
// ---------------------------------------------------------------------------

static EQUIPNAME_TOKENS: &[TokenDef] = &[
    // Reuse canonical attribute names that the projector already maps these to
    TokenDef::integer("OUTPUTORDER", "pcgen_outputorder"),
    TokenDef::text("COST", "pcgen_cost"),
    TokenDef::text("WT", "pcgen_weight"),
    TokenDef::text("QUANTITY", "pcgen_quantity"),
    TokenDef::text("CUSTOMIZATION", "pcgen_customization"),
    TokenDef::text("NOTE", "pcgen_note"),
    // DATA appears as a separate clause when bracket content in CUSTOMIZATION is pipe-split
    TokenDef::text("DATA", "pcgen_data"),
];

pub static EQUIPNAME_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:equipname",
    head_token: Some("EQUIPNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: EQUIPNAME_TOKENS,
    globals: &[],
};

// ---------------------------------------------------------------------------
// Class abilities by level (CLASSABILITIESLEVEL:Class=N|HITPOINTS:N|...)
// ---------------------------------------------------------------------------

static CLASSABILITIESLEVEL_TOKENS: &[TokenDef] = &[
    TokenDef::integer("HITPOINTS", "pcgen_cal_hitpoints"),
    TokenDef::integer("SKILLSGAINED", "pcgen_cal_skillsgained"),
    TokenDef::integer("SKILLSREMAINING", "pcgen_cal_skillsremaining"),
    // SPECIALTIES records chosen specialist school etc. for class levels
    TokenDef::text("SPECIALTIES", "pcgen_specialties"),
];

pub static CLASSABILITIESLEVEL_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:classabilitieslevel",
    head_token: Some("CLASSABILITIESLEVEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: CLASSABILITIESLEVEL_TOKENS,
    globals: &[
        // PRESTAT appears as a sub-token in some CLASSABILITIESLEVEL lines
        crate::schema::GlobalGroup::Prerequisites,
    ],
};

// ---------------------------------------------------------------------------
// Standalone character note (NOTE:text|ID:n|PARENTID:n|VALUE:v)
// ---------------------------------------------------------------------------

static NOTE_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("NOTE", "pcgen_note"),
    // ID and PARENTID reuse the equipset id field (same projector slot)
    TokenDef::text("ID", "pcgen_equipset_id"),
    TokenDef::text("PARENTID", "pcgen_note_parentid"),
    TokenDef::text("VALUE", "pcgen_value"),
];

pub static NOTE_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:note",
    head_token: Some("NOTE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: NOTE_TOKENS,
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character spell record (SPELLNAME:name|TIMES:n|CLASS:c|BOOK:b|SPELLLEVEL:n)
// ---------------------------------------------------------------------------

static SPELLNAME_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("SPELLNAME", "pcgen_spellname"),
    TokenDef::integer("TIMES", "pcgen_times"),
    TokenDef::text("BOOK", "pcgen_book"),
    TokenDef::text("SPELLLEVEL", "pcgen_spelllevel"),
    TokenDef::text("CLASS", "pcgen_class"),
    TokenDef::text("SOURCE", "pcgen_source"),
    TokenDef::text("FEATLIST", "pcgen_featlist"),
];

pub static SPELLNAME_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:spellname",
    head_token: Some("SPELLNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: SPELLNAME_TOKENS,
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character deity record (DEITY:name|DEITYDOMAINS:[…]|ALIGNALLOW:|…)
// ---------------------------------------------------------------------------

static PCG_DEITY_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("DEITY", "pcgen_deity_name"),
    TokenDef::text("DEITYDOMAINS", "pcgen_deitydomains"),
    TokenDef::text("ALIGNALLOW", "pcgen_alignallow"),
    TokenDef::text("HOLYITEM", "pcgen_holyitem"),
    TokenDef::text("DEITYFAVWEAP", "pcgen_deityfavweap"),
    TokenDef::text("DEITYALIGN", "pcgen_deityalign"),
    TokenDef::text("DOMAINGRANTS", "pcgen_domaingrants"),
];

pub static PCG_DEITY_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:deity",
    head_token: Some("DEITY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: PCG_DEITY_TOKENS,
    globals: &[crate::schema::GlobalGroup::Desc],
};

// ---------------------------------------------------------------------------
// Character domain record (DOMAIN:name|DOMAINGRANTS:text|SOURCE:[…])
// ---------------------------------------------------------------------------

static PCG_DOMAIN_TOKENS: &[TokenDef] = &[
    TokenDef::text_required("DOMAIN", "pcgen_domain_name"),
    TokenDef::text("DOMAINGRANTS", "pcgen_domaingrants"),
    TokenDef::text("SOURCE", "pcgen_source"),
];

pub static PCG_DOMAIN_SCHEMA: EntitySchema = EntitySchema {
    entity_type_key: "pcgen:pcg:domain",
    head_token: Some("DOMAIN"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: PCG_DOMAIN_TOKENS,
    globals: &[],
};
