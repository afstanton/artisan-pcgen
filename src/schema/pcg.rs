//! PCGen character-file (`.pcg`) standalone record schemas.
//!
//! These heads appear as top-level session/profile lines in character files,
//! e.g. `PCGVERSION:2.0` or `HEIGHT:51`.

use crate::schema::{LineGrammar, HeadFormat, TokenDef};

pub static PCGVERSION_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:pcgversion",
    head_token: Some("PCGVERSION"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("PCGVERSION", "pcgen_pcgversion")],
    globals: &[],
};

pub static PURCHASEPOINTS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:purchasepoints",
    head_token: Some("PURCHASEPOINTS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "PURCHASEPOINTS",
        "pcgen_purchasepoints",
    )],
    globals: &[],
};

pub static POOLPOINTS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:poolpoints",
    head_token: Some("POOLPOINTS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("POOLPOINTS", "pcgen_poolpoints")],
    globals: &[],
};

pub static POOLPOINTSAVAIL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:poolpointsavail",
    head_token: Some("POOLPOINTSAVAIL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "POOLPOINTSAVAIL",
        "pcgen_poolpointsavail",
    )],
    globals: &[],
};

pub static TABLABEL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:tablabel",
    head_token: Some("TABLABEL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("TABLABEL", "pcgen_tablabel")],
    globals: &[],
};

pub static AUTOSPELLS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:autospells",
    head_token: Some("AUTOSPELLS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("AUTOSPELLS", "pcgen_autospells")],
    globals: &[],
};

pub static USEHIGHERKNOWN_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:usehigherknown",
    head_token: Some("USEHIGHERKNOWN"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("USEHIGHERKNOWN", "pcgen_usehigherknown")],
    globals: &[],
};

pub static USEHIGHERPREPPED_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:usehigherprepped",
    head_token: Some("USEHIGHERPREPPED"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno(
        "USEHIGHERPREPPED",
        "pcgen_usehigherprepped",
    )],
    globals: &[],
};

pub static LOADCOMPANIONS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:loadcompanions",
    head_token: Some("LOADCOMPANIONS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("LOADCOMPANIONS", "pcgen_loadcompanions")],
    globals: &[],
};

pub static USETEMPMODS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:usetempmods",
    head_token: Some("USETEMPMODS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("USETEMPMODS", "pcgen_usetempmods")],
    globals: &[],
};

pub static SKILLSOUTPUTORDER_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:skillsoutputorder",
    head_token: Some("SKILLSOUTPUTORDER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer(
        "SKILLSOUTPUTORDER",
        "pcgen_skillsoutputorder",
    )],
    globals: &[],
};

pub static SKILLFILTER_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:skillfilter",
    head_token: Some("SKILLFILTER"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("SKILLFILTER", "pcgen_skillfilter")],
    globals: &[],
};

pub static IGNORECOST_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:ignorecost",
    head_token: Some("IGNORECOST"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("IGNORECOST", "pcgen_ignorecost")],
    globals: &[],
};

pub static ALLOWDEBT_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:allowdebt",
    head_token: Some("ALLOWDEBT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("ALLOWDEBT", "pcgen_allowdebt")],
    globals: &[],
};

pub static AUTORESIZEGEAR_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:autoresizegear",
    head_token: Some("AUTORESIZEGEAR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::yesno("AUTORESIZEGEAR", "pcgen_autoresizegear")],
    globals: &[],
};

pub static CHARACTERNAME_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:charactername",
    head_token: Some("CHARACTERNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required(
        "CHARACTERNAME",
        "character_name",
    )],
    globals: &[],
};

pub static PLAYERNAME_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:playername",
    head_token: Some("PLAYERNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PLAYERNAME", "pcgen_playername")],
    globals: &[],
};

pub static HEIGHT_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:height",
    head_token: Some("HEIGHT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("HEIGHT", "height")],
    globals: &[],
};

pub static WEIGHT_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:weight",
    head_token: Some("WEIGHT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("WEIGHT", "weight")],
    globals: &[],
};

pub static AGE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:age",
    head_token: Some("AGE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("AGE", "age")],
    globals: &[],
};

pub static HANDED_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:handed",
    head_token: Some("HANDED"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text_required("HANDED", "pcgen_handed")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character bio tokens (standalone)
// ---------------------------------------------------------------------------

pub static TABNAME_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:tabname",
    head_token: Some("TABNAME"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("TABNAME", "pcgen_tabname")],
    globals: &[],
};

pub static SKINCOLOR_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:skincolor",
    head_token: Some("SKINCOLOR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("SKINCOLOR", "skin_color")],
    globals: &[],
};

pub static EYECOLOR_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:eyecolor",
    head_token: Some("EYECOLOR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("EYECOLOR", "eye_color")],
    globals: &[],
};

pub static HAIRCOLOR_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:haircolor",
    head_token: Some("HAIRCOLOR"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("HAIRCOLOR", "hair_color")],
    globals: &[],
};

pub static HAIRSTYLE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:hairstyle",
    head_token: Some("HAIRSTYLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("HAIRSTYLE", "pcgen_hairstyle")],
    globals: &[],
};

pub static CITY_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:city",
    head_token: Some("CITY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CITY", "city")],
    globals: &[],
};

pub static BIRTHDAY_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:birthday",
    head_token: Some("BIRTHDAY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("BIRTHDAY", "birthday")],
    globals: &[],
};

pub static BIRTHPLACE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:birthplace",
    head_token: Some("BIRTHPLACE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("BIRTHPLACE", "birthplace")],
    globals: &[],
};

pub static PERSONALITYTRAIT1_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:personalitytrait1",
    head_token: Some("PERSONALITYTRAIT1"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PERSONALITYTRAIT1", "personality_trait_1")],
    globals: &[],
};

pub static PERSONALITYTRAIT2_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:personalitytrait2",
    head_token: Some("PERSONALITYTRAIT2"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PERSONALITYTRAIT2", "personality_trait_2")],
    globals: &[],
};

pub static SPEECHPATTERN_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:speechpattern",
    head_token: Some("SPEECHPATTERN"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("SPEECHPATTERN", "speech_pattern")],
    globals: &[],
};

pub static PHOBIAS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:phobias",
    head_token: Some("PHOBIAS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PHOBIAS", "phobias")],
    globals: &[],
};

pub static INTERESTS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:interests",
    head_token: Some("INTERESTS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("INTERESTS", "interests")],
    globals: &[],
};

pub static CATCHPHRASE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:catchphrase",
    head_token: Some("CATCHPHRASE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CATCHPHRASE", "catchphrase")],
    globals: &[],
};

pub static PORTRAIT_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:portrait",
    head_token: Some("PORTRAIT"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("PORTRAIT", "portrait")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character progression tokens (standalone)
// ---------------------------------------------------------------------------

pub static EXPERIENCE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:experience",
    head_token: Some("EXPERIENCE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::integer("EXPERIENCE", "pcgen_experience")],
    globals: &[],
};

pub static EXPERIENCETABLE_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:experiencetable",
    head_token: Some("EXPERIENCETABLE"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("EXPERIENCETABLE", "pcgen_experiencetable")],
    globals: &[],
};

/// Currency total in base units. Stored as text to accommodate decimal values.
pub static MONEY_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:money",
    head_token: Some("MONEY"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("MONEY", "pcgen_money")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character description blocks (standalone)
// ---------------------------------------------------------------------------

pub static CHARACTERBIO_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:characterbio",
    head_token: Some("CHARACTERBIO"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERBIO", "character_bio")],
    globals: &[],
};

pub static CHARACTERDESC_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:characterdesc",
    head_token: Some("CHARACTERDESC"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERDESC", "character_desc")],
    globals: &[],
};

pub static CHARACTERCOMP_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:charactercomp",
    head_token: Some("CHARACTERCOMP"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERCOMP", "character_comp")],
    globals: &[],
};

pub static CHARACTERASSET_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:characterasset",
    head_token: Some("CHARACTERASSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERASSET", "character_asset")],
    globals: &[],
};

pub static CHARACTERMAGIC_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:charactermagic",
    head_token: Some("CHARACTERMAGIC"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERMAGIC", "character_magic")],
    globals: &[],
};

pub static CHARACTERDMNOTES_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:characterdmnotes",
    head_token: Some("CHARACTERDMNOTES"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CHARACTERDMNOTES", "character_dm_notes")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Misc standalone tokens
// ---------------------------------------------------------------------------

/// PCGen application version used to save the character file.
pub static VERSION_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:version",
    head_token: Some("VERSION"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("VERSION", "pcgen_version")],
    globals: &[],
};

/// Ability pool point total for the character. Stored as text (may be decimal).
pub static FEATPOOL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:featpool",
    head_token: Some("FEATPOOL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("FEATPOOL", "pcgen_featpool")],
    globals: &[],
};

/// ID of the active equipment set.
pub static CALCEQUIPSET_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:calcequipset",
    head_token: Some("CALCEQUIPSET"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("CALCEQUIPSET", "pcgen_calcequipset")],
    globals: &[],
};

/// Comma-delimited list of suppressed biography field names.
pub static SUPPRESSBIOFIELDS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:suppressbiofields",
    head_token: Some("SUPPRESSBIOFIELDS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: &[TokenDef::text("SUPPRESSBIOFIELDS", "pcgen_suppressbiofields")],
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character class record (CLASS:name|LEVEL:n|SKILLPOOL:n|...)
// ---------------------------------------------------------------------------

static PCG_CLASS_TOKENS: &[TokenDef] = &[
    TokenDef::integer("LEVEL", "pcgen_level"),
    TokenDef::integer("SKILLPOOL", "pcgen_class_skillpool"),
    TokenDef::text("SPELLBASE", "pcgen_class_spellbase"),
    TokenDef::text("CANCASTPERDAY", "pcgen_class_cancastperday"),
    TokenDef::text("SUBCLASS", "pcgen_subclass"),
    TokenDef::pipe_list_repeatable("PROHIBITED", "pcgen_prohibited"),
];

pub static PCG_CLASS_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:class",
    head_token: Some("CLASS"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: PCG_CLASS_TOKENS,
    globals: &[],
};

// ---------------------------------------------------------------------------
// Character skill record (SKILL:name|OUTPUTORDER:n|CLASSBOUGHT:[...])
// ---------------------------------------------------------------------------

static PCG_SKILL_TOKENS: &[TokenDef] = &[
    TokenDef::integer("OUTPUTORDER", "pcgen_outputorder"),
    // CLASSBOUGHT is a bracket group: [CLASS:Wizard|RANKS:3.0|COST:1|CLASSSKILL:Y]
    // Repeatable: a single SKILL line may have multiple adjacent groups (no pipe between them).
    TokenDef::bracket_group_repeatable("CLASSBOUGHT", "pcgen_classbought"),
];

pub static PCG_SKILL_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:skill",
    head_token: Some("SKILL"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: PCG_SKILL_TOKENS,
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

pub static USERPOOL_SCHEMA: LineGrammar = LineGrammar {
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
    TokenDef::text("NOTE", "note"),
];

pub static EQUIPSET_SCHEMA: LineGrammar = LineGrammar {
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
    TokenDef::text("COST", "cost"),
    TokenDef::text("WT", "weight"),
    TokenDef::text("QUANTITY", "pcgen_quantity"),
    // CUSTOMIZATION bracket group: [BASEITEM:Longsword|DATA:EQMOD=STEEL|...]
    // DATA sub-items live inside this bracket group; no separate DATA token needed.
    TokenDef::bracket_group("CUSTOMIZATION", "pcgen_customization"),
    TokenDef::text("NOTE", "note"),
];

pub static EQUIPNAME_SCHEMA: LineGrammar = LineGrammar {
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
    // SPECIALTIES bracket group: [SPECIALTY:Evocation|SPECIALTY:...]
    TokenDef::bracket_group("SPECIALTIES", "pcgen_specialties"),
];

pub static CLASSABILITIESLEVEL_SCHEMA: LineGrammar = LineGrammar {
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
    TokenDef::text_required("NOTE", "note"),
    // ID and PARENTID reuse the equipset id field (same projector slot)
    TokenDef::text("ID", "pcgen_equipset_id"),
    TokenDef::text("PARENTID", "pcgen_note_parentid"),
    TokenDef::text("VALUE", "pcgen_value"),
];

pub static NOTE_SCHEMA: LineGrammar = LineGrammar {
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
    // SOURCE in PCG context is a bracket group: [TYPE:CLASS|NAME:Wizard]
    TokenDef::bracket_group("SOURCE", "pcgen_source"),
    // FEATLIST bracket group: [FEAT:Empower Spell|FEAT:...]
    TokenDef::bracket_group("FEATLIST", "pcgen_featlist"),
];

pub static SPELLNAME_SCHEMA: LineGrammar = LineGrammar {
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
    // DEITYDOMAINS bracket group: [DOMAIN:Good|DOMAIN:Sun|...]
    TokenDef::bracket_group("DEITYDOMAINS", "pcgen_deitydomains"),
    TokenDef::text("ALIGNALLOW", "pcgen_alignallow"),
    TokenDef::text("HOLYITEM", "pcgen_holyitem"),
    // DEITYFAVWEAP bracket group: [WEAPON:Morningstar|...]
    TokenDef::bracket_group("DEITYFAVWEAP", "pcgen_deityfavweap"),
    TokenDef::text("DEITYALIGN", "pcgen_deityalign"),
    TokenDef::text("DOMAINGRANTS", "pcgen_domaingrants"),
];

pub static PCG_DEITY_SCHEMA: LineGrammar = LineGrammar {
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
    // SOURCE in PCG context is a bracket group: [TYPE:DEITY|NAME:Pelor]
    TokenDef::bracket_group("SOURCE", "pcgen_source"),
];

pub static PCG_DOMAIN_SCHEMA: LineGrammar = LineGrammar {
    entity_type_key: "pcgen:pcg:domain",
    head_token: Some("DOMAIN"),
    head_format: HeadFormat::TokenPrefixed,
    tokens: PCG_DOMAIN_TOKENS,
    globals: &[],
};
