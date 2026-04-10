//! PCGen entity-type schemas.
//!
//! Each schema captures the complete token grammar for one PCGen entity type,
//! derived from the official PCGen LST documentation. The same schema drives
//! both parsing (token classification, value interpretation) and emission
//! (serializing artisan `Entity` values back to line-oriented PCGen text,
//! including `.lst`, `.pcc`, and `.pcg`).

pub mod ability;
pub mod abilitycategory;
pub mod classlevel;
pub mod companionmod;
pub mod startpack;
pub mod system;
pub mod biosettings;
pub mod class;
pub mod deity;
pub mod datacontrol;
pub mod equipment;
pub mod feat;
pub mod gear;
pub mod kit;
pub mod modify;
pub mod pcc;
pub mod race;
pub mod skill;
pub mod spell;
pub mod subclass;
pub mod template;
pub mod token_aliases;
pub mod variables;

pub use ability::ABILITY_SCHEMA;
pub use abilitycategory::ABILITYCATEGORY_SCHEMA;
pub use classlevel::CLASSLEVEL_SCHEMA;
pub use companionmod::{FOLLOWER_COMPANIONMOD_SCHEMA, MASTERBONUSRACE_COMPANIONMOD_SCHEMA};
pub use startpack::{FUNDS_STARTPACK_SCHEMA, GENDER_STARTPACK_SCHEMA, LANGAUTO_STARTPACK_SCHEMA, STARTPACK_SCHEMA, TOTALCOST_STARTPACK_SCHEMA};
pub use biosettings::BIOSETTINGS_SCHEMA;
pub use system::{
    ACTYPE_SYSTEM_SCHEMA, AGESET_SYSTEM_SCHEMA, ALIGN_SYSTEM_SCHEMA, BASEDICE_SYSTEM_SCHEMA,
    ACNAME_SYSTEM_SCHEMA, ALLOWEDMODES_SYSTEM_SCHEMA,
    ALIGNMENTFEATURE_SYSTEM_SCHEMA,
    BABATTCYC_SYSTEM_SCHEMA, BABMAXATT_SYSTEM_SCHEMA, BABMINVAL_SYSTEM_SCHEMA,
    BONUSFEATLEVELSTARTINTERVAL_SCHEMA, BONUSSPELLLEVEL_SCHEMA, BONUSSTACKS_SCHEMA,
    BONUSSTATLEVELSTARTINTERVAL_SCHEMA, CLASSTYPE_SYSTEM_SCHEMA, CRSTEPS_SYSTEM_SCHEMA,
    CRTHRESHOLD_SYSTEM_SCHEMA,
    CHARACTERTYPE_SYSTEM_SCHEMA,
    DEFAULTDATASET_SYSTEM_SCHEMA,
    DEFAULTUNITSET_SYSTEM_SCHEMA, DEFAULTVARIABLEVALUE_SYSTEM_SCHEMA,
    DOMAINFEATURE_SYSTEM_SCHEMA, ENCUMBRANCE_SYSTEM_SCHEMA, EQSLOT_SYSTEM_SCHEMA,
    EQSIZEPENALTY_SYSTEM_SCHEMA,
    CURRENCYUNITABBREV_SYSTEM_SCHEMA, DIESIZES_SYSTEM_SCHEMA, DISPLAYORDER_SYSTEM_SCHEMA,
    ENDTABLE_SYSTEM_SCHEMA,
    GAMEMODEKEY_SYSTEM_SCHEMA,
    ICON_SYSTEM_SCHEMA, INFOSHEET_SYSTEM_SCHEMA, LEVEL_SYSTEM_SCHEMA, LOAD_SYSTEM_SCHEMA,
    LEVELMSG_SYSTEM_SCHEMA,
    LOADMULT_SYSTEM_SCHEMA,
    MAXNONEPICLEVEL_SYSTEM_SCHEMA,
    MENUENTRY_SYSTEM_SCHEMA,
    METHOD_SYSTEM_SCHEMA,
    MOVEMENT_SYSTEM_SCHEMA,
    MONSTERROLEDEFAULT_SYSTEM_SCHEMA, MONSTERROLES_SYSTEM_SCHEMA,
    NAME_SYSTEM_SCHEMA, NUMSLOTS_SYSTEM_SCHEMA, OUTPUTSHEET_SYSTEM_SCHEMA, PREVIEWDIR_SCHEMA,
    PLUSCOST_SYSTEM_SCHEMA,
    PREVIEWSHEET_SCHEMA, RACE_SYSTEM_SCHEMA,
    RESIZABLEEQUIPTYPE_SYSTEM_SCHEMA,
    RANGEPENALTY_SYSTEM_SCHEMA, ROLLMETHOD_SYSTEM_SCHEMA, SHORTRANGE_SYSTEM_SCHEMA,
    SIZEADJUSTMENT_SYSTEM_SCHEMA, SIZEMULT_SYSTEM_SCHEMA, SKILLMULTIPLIER_SYSTEM_SCHEMA,
    STARTTABLE_SYSTEM_SCHEMA,
    SKILLCOST_CLASS_SYSTEM_SCHEMA, SKILLCOST_CROSSCLASS_SYSTEM_SCHEMA, SKILLCOST_EXCLUSIVE_SYSTEM_SCHEMA,
    SPELLBASECONCENTRATION_SYSTEM_SCHEMA, STATINPUT_SYSTEM_SCHEMA,
    SPELLBASEDC_SYSTEM_SCHEMA, SPELLRANGE_SYSTEM_SCHEMA, SQUARESIZE_SYSTEM_SCHEMA,
    STAT_SYSTEM_SCHEMA, SUBCLASSLEVEL_SYSTEM_SCHEMA,
    SUBSTITUTIONCLASS_SYSTEM_SCHEMA, SUBSTITUTIONLEVEL_SYSTEM_SCHEMA, TAB_SYSTEM_SCHEMA, TABLE_SYSTEM_SCHEMA,
    UNITSET_SYSTEM_SCHEMA, WEAPONCATEGORY_SYSTEM_SCHEMA, WEAPONTYPE_SYSTEM_SCHEMA,
    WEAPONNONPROFPENALTY_SYSTEM_SCHEMA, WEAPONREACH_SYSTEM_SCHEMA,
    WIELDCATEGORY_SYSTEM_SCHEMA, XPAWARD_SYSTEM_SCHEMA, XPTABLE_SYSTEM_SCHEMA,
};
pub use class::CLASS_SCHEMA;
pub use deity::DEITY_SCHEMA;
pub use datacontrol::{DYNAMICSCOPE_SCHEMA, FACTSETDEF_SCHEMA, FUNCTION_SCHEMA};
pub use equipment::EQUIPMENT_SCHEMA;
pub use feat::FEAT_SCHEMA;
pub use gear::GEAR_SCHEMA;
pub use kit::KIT_SCHEMA;
pub use modify::MODIFY_SCHEMA;
pub use pcc::{
    ABILITYCATEGORY_INCLUDE_SCHEMA, ABILITY_INCLUDE_SCHEMA,
    ALIGNMENT_INCLUDE_SCHEMA, ARMORPROF_INCLUDE_SCHEMA, BIOSET_INCLUDE_SCHEMA, COMPANIONMOD_INCLUDE_SCHEMA,
    DATACONTROL_INCLUDE_SCHEMA, DATATABLE_INCLUDE_SCHEMA, EQUIPMOD_INCLUDE_SCHEMA, LANGUAGE_INCLUDE_SCHEMA,
    EQUIPMENT_INCLUDE_SCHEMA, FEAT_INCLUDE_SCHEMA, FORWARDREF_INCLUDE_SCHEMA, HIDETYPE_INCLUDE_SCHEMA,
    INFOTEXT_INCLUDE_SCHEMA, LICENSE_INCLUDE_SCHEMA,
    LSTEXCLUDE_INCLUDE_SCHEMA, PCC_SCHEMA, SAVE_INCLUDE_SCHEMA, SHIELDPROF_INCLUDE_SCHEMA,
    SHOWINMENU_INCLUDE_SCHEMA, SPELL_INCLUDE_SCHEMA, URL_INCLUDE_SCHEMA,
    WEAPONPROF_INCLUDE_SCHEMA, DYNAMIC_INCLUDE_SCHEMA,
};
pub use race::RACE_SCHEMA;
pub use skill::SKILL_SCHEMA;
pub use spell::SPELL_SCHEMA;
pub use subclass::SUBCLASS_SCHEMA;
pub use template::TEMPLATE_SCHEMA;
pub use token_aliases::{AliasScope, AliasStatus, TokenAlias, all_token_aliases};
pub use variables::{GLOBAL_VARIABLE_SCHEMA, LOCAL_VARIABLE_SCHEMA};

// ---------------------------------------------------------------------------
// Core grammar types
// ---------------------------------------------------------------------------

/// Value-level grammar for a PCGen token.
///
/// Describes how the token's value string should be interpreted and serialized.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenGrammar {
    /// Single verbatim text value: `KEY:value`
    Text,
    /// Integer value: `KEY:4`
    Integer,
    /// YES/NO value: `KEY:YES`
    YesNo,
    /// Dot-delimited type list: `TYPE:Combat.Melee.Sword`
    DotList,
    /// Comma-delimited item list: `COMPS:V,S,M`
    CommaList,
    /// Pipe-delimited positional arguments: `BONUS:COMBAT|TOHIT|-2`
    ///
    /// The slot name strings are documentation only — they are not parsed.
    PipePositional(&'static [&'static str]),
    /// Pipe-delimited list of items with no positional semantics: `AUTO:LANG|Common|Elven`
    PipeList,
    /// Pipe-separated groups where each group uses comma-separated `name=value` pairs:
    /// `CLASSES:Wizard=2,Bard=2|Cleric=3`
    PipeGroups,
    /// Formula / expression value (stored verbatim): `DEFINE:myVar|0`
    Formula,
}

/// How many times a token may appear per entity line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    /// At most once; last definition wins on `.MOD`.
    Once,
    /// May appear multiple times; all instances are collected.
    Repeatable,
}

/// How a token's parsed value maps into the artisan `Entity` data model.
#[derive(Debug, Clone, Copy)]
pub enum ArtisanMapping {
    /// Maps to `entity.attributes[field_key]`.
    Attribute(&'static str),
    /// Contributes to `entity.effects` (kind = token key, target = value).
    Effect,
    /// Contributes to `entity.prerequisites`.
    Prerequisite,
    /// Derived from `entity.name`.
    EntityName,
    /// Not yet mapped to the artisan model.
    None,
}

/// Definition of a single PCGen token within an entity type's grammar.
#[derive(Debug, Clone, Copy)]
pub struct TokenDef {
    /// PCGen token key, e.g. `"CATEGORY"`, `"BONUS"`, `"HITDIE"`.
    pub key: &'static str,
    /// Value-level grammar for this token.
    pub grammar: TokenGrammar,
    /// How many times this token may appear per entity line.
    pub cardinality: Cardinality,
    /// How this token's value maps into the artisan entity.
    pub artisan_mapping: ArtisanMapping,
    /// Whether this token is required for valid output.
    pub required: bool,
}

impl TokenDef {
    /// Single-occurrence text token mapping to an attribute.
    pub const fn text(key: &'static str, field: &'static str) -> Self {
        Self {
            key,
            grammar: TokenGrammar::Text,
            cardinality: Cardinality::Once,
            artisan_mapping: ArtisanMapping::Attribute(field),
            required: false,
        }
    }

    /// Required single-occurrence text token mapping to an attribute.
    pub const fn text_required(key: &'static str, field: &'static str) -> Self {
        Self {
            key,
            grammar: TokenGrammar::Text,
            cardinality: Cardinality::Once,
            artisan_mapping: ArtisanMapping::Attribute(field),
            required: true,
        }
    }

    /// Single-occurrence integer token mapping to an attribute.
    pub const fn integer(key: &'static str, field: &'static str) -> Self {
        Self {
            key,
            grammar: TokenGrammar::Integer,
            cardinality: Cardinality::Once,
            artisan_mapping: ArtisanMapping::Attribute(field),
            required: false,
        }
    }

    /// Single-occurrence YES/NO token mapping to an attribute.
    pub const fn yesno(key: &'static str, field: &'static str) -> Self {
        Self {
            key,
            grammar: TokenGrammar::YesNo,
            cardinality: Cardinality::Once,
            artisan_mapping: ArtisanMapping::Attribute(field),
            required: false,
        }
    }

    /// Repeatable pipe-list token mapping to an attribute.
    pub const fn pipe_list_repeatable(key: &'static str, field: &'static str) -> Self {
        Self {
            key,
            grammar: TokenGrammar::PipeList,
            cardinality: Cardinality::Repeatable,
            artisan_mapping: ArtisanMapping::Attribute(field),
            required: false,
        }
    }

    /// Single-occurrence pipe-positional token mapping to an attribute.
    pub const fn pipe_positional(
        key: &'static str,
        slots: &'static [&'static str],
        field: &'static str,
    ) -> Self {
        Self {
            key,
            grammar: TokenGrammar::PipePositional(slots),
            cardinality: Cardinality::Once,
            artisan_mapping: ArtisanMapping::Attribute(field),
            required: false,
        }
    }

    /// Repeatable pipe-positional token mapping to an attribute.
    pub const fn pipe_positional_repeatable(
        key: &'static str,
        slots: &'static [&'static str],
        field: &'static str,
    ) -> Self {
        Self {
            key,
            grammar: TokenGrammar::PipePositional(slots),
            cardinality: Cardinality::Repeatable,
            artisan_mapping: ArtisanMapping::Attribute(field),
            required: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Head format
// ---------------------------------------------------------------------------

/// How the record head for this entity type is formatted in a line-oriented
/// PCGen file (`.lst`, `.pcc`, or `.pcg`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadFormat {
    /// Entity name only: `Toughness` (abilities, feats, spells, races, …).
    NameOnly,
    /// Token-prefixed: `CLASS:Psion`, `SKILL:Bluff`.
    TokenPrefixed,
}

// ---------------------------------------------------------------------------
// Global token groups
// ---------------------------------------------------------------------------

/// Cross-cutting token groups that are valid for multiple entity types.
///
/// Defined in `docs/listfilepages/globalfilestagpages/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalGroup {
    /// `TYPE:a.b.c` — dot-delimited type list
    Type,
    /// `KEY:x` — alternate lookup key
    Key,
    /// `DESC:text` — description
    Desc,
    /// `FACT:name|value` — key/value annotation (repeatable)
    Fact,
    /// `BONUS:type|targets|formula[|conditions]` — mechanical bonus (repeatable)
    Bonus,
    /// `ADD:type|...` — add to character (repeatable)
    Add,
    /// `CHOOSE:...` — player choice (repeatable)
    Choose,
    /// `AUTO:type|items` — automatic grant (repeatable)
    Auto,
    /// `DEFINE:var|formula` — variable definition (repeatable)
    Define,
    /// `MODIFY:var|op|value` and `MODIFYOTHER:...` — variable modification (repeatable)
    Modify,
    /// All `PRExxx:` and `!PRExxx:` prerequisite tokens (repeatable)
    Prerequisites,
    /// `SOURCEPAGE:x` — source page citation
    SourcePage,
    /// `SOURCELINK:x` — source reference URL
    SourceLink,
    /// `OUTPUTNAME:x` — display name override
    OutputName,
    /// `SORTKEY:x` — sort/display ordering hint
    SortKey,
    /// `TEMPLATE:x`, `TEMPLATE:ADDCHOICE:x`, `TEMPLATE:CHOOSE:x` (repeatable)
    Template,
    /// `CSKILL:x|x`, `CCSKILL:x|x`, and `UNENCUMBEREDMOVE:x|x` (repeatable)
    CSkill,
    /// `SAB:x` — special ability text (repeatable)
    Sab,
    /// `QUALIFY:type|...` — qualification grant (repeatable)
    Qualify,
    /// `LANGBONUS:x` — bonus language choices (repeatable)
    LangBonus,
    /// `CHANGEPROF:x` — remap proficiency category (repeatable)
    ChangeProf,
    /// `SERVESAS:x` — entity stands in for one or more others (repeatable)
    ServesAs,
    /// Source/publisher metadata tokens (PCC-style, valid at top of any file)
    SourceMeta,
}

impl GlobalGroup {
    /// Token key prefixes associated with this group.
    ///
    /// A token key belongs to this group if it exactly matches one of these
    /// prefixes, or (for `Prerequisites`) starts with one.
    pub fn token_key_prefixes(self) -> &'static [&'static str] {
        match self {
            GlobalGroup::Type => &["TYPE"],
            GlobalGroup::Key => &["KEY"],
            GlobalGroup::Desc => &["DESC", "TEMPDESC", "DESCISPI"],
            GlobalGroup::Fact => &["FACT", "FACTSET"],
            GlobalGroup::Bonus => &["BONUS", "TEMPBONUS"],
            GlobalGroup::Add => &["ADD"],
            GlobalGroup::Choose => &["CHOOSE", "SELECT"],
            GlobalGroup::Auto => &["AUTO"],
            GlobalGroup::Define => &["DEFINE", "DEFINESTAT"],
            GlobalGroup::Modify => &["MODIFY", "MODIFYOTHER"],
            GlobalGroup::Prerequisites => &["PRE", "!PRE"],
            GlobalGroup::SourcePage => &["SOURCEPAGE"],
            GlobalGroup::SourceLink => &["SOURCELINK"],
            GlobalGroup::OutputName => &["OUTPUTNAME"],
            GlobalGroup::SortKey => &["SORTKEY"],
            GlobalGroup::Template => &["TEMPLATE"],
            GlobalGroup::CSkill => &["CSKILL", "CCSKILL", "UNENCUMBEREDMOVE"],
            GlobalGroup::Sab => &["SAB"],
            GlobalGroup::Qualify => &["QUALIFY"],
            GlobalGroup::LangBonus => &["LANGBONUS"],
            GlobalGroup::ChangeProf => &["CHANGEPROF"],
            GlobalGroup::ServesAs => &["SERVESAS"],
            GlobalGroup::SourceMeta => &[
                "CAMPAIGN",
                "SOURCELONG",
                "SOURCE",
                "SOURCESHORT",
                "SOURCEWEB",
                "SOURCEDATE",
                "PUBNAMELONG",
                "PUBNAMESHORT",
                "PUBNAMEWEB",
                "PUBLISHER",
                "PUBLISHERNAME",
                "GAMEMODE",
                "SETTING",
                "BOOKTYPE",
                "FACTDEF",
            ],
        }
    }

    /// Returns true if the given (already-uppercased) key belongs to this group.
    pub fn matches(self, upper_key: &str) -> bool {
        match self {
            // Prerequisites use prefix matching
            GlobalGroup::Prerequisites => {
                (upper_key.starts_with("PRE") || upper_key.starts_with("!PRE"))
                    && !matches!(upper_key, "PREREQUISITE" | "PREREQUISITES")
            }
            // ADD uses prefix matching too (ADD:ABILITY, ADD:FEAT, etc.)
            GlobalGroup::Add => upper_key == "ADD",
            // CHOOSE uses prefix matching (CHOOSE:ABILITY, CHOOSE:FEAT, etc.)
            GlobalGroup::Choose => upper_key == "CHOOSE" || upper_key == "SELECT",
            // BONUS uses prefix matching (BONUS:COMBAT, BONUS:SKILL, etc.)
            GlobalGroup::Bonus => upper_key == "BONUS" || upper_key == "TEMPBONUS",
            // TEMPLATE uses prefix matching
            GlobalGroup::Template => upper_key == "TEMPLATE",
            GlobalGroup::SourceLink => upper_key == "SOURCELINK",
            // SourceMeta: exact match against any of the known keys
            GlobalGroup::SourceMeta => self.token_key_prefixes().contains(&upper_key),
            // All others: exact match
            _ => self.token_key_prefixes().iter().any(|p| *p == upper_key),
        }
    }
}

// ---------------------------------------------------------------------------
// EntitySchema
// ---------------------------------------------------------------------------

/// Complete token grammar for one PCGen entity type.
pub struct EntitySchema {
    /// Artisan entity type key, e.g. `"pcgen:entity:ability"`.
    pub entity_type_key: &'static str,
    /// PCGen head token for token-prefixed entities, e.g. `"CLASS"`, `"SKILL"`.
    /// `None` for name-only entities (abilities, feats, spells, races, …).
    pub head_token: Option<&'static str>,
    /// How the head record is formatted in line-oriented PCGen output.
    pub head_format: HeadFormat,
    /// Entity-type-specific token definitions, in preferred emission order.
    pub tokens: &'static [TokenDef],
    /// Cross-cutting global token groups applicable to this entity type.
    pub globals: &'static [GlobalGroup],
}

impl EntitySchema {
    /// Returns true if this schema recognizes `key` as a known token
    /// (either entity-specific or via a global group).
    pub fn knows_token_key(&self, key: &str) -> bool {
        let upper = key.to_ascii_uppercase();

        // Check entity-specific tokens
        if self.tokens.iter().any(|t| t.key.eq_ignore_ascii_case(&upper)) {
            return true;
        }

        // Check global groups
        self.globals.iter().any(|g| g.matches(&upper))
    }

    /// Returns the `TokenDef` for `key` if it is an entity-specific token.
    pub fn token_def(&self, key: &str) -> Option<&TokenDef> {
        self.tokens.iter().find(|t| t.key.eq_ignore_ascii_case(key))
    }
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

static ALL_SCHEMAS: &[&EntitySchema] = &[
    &ability::ABILITY_SCHEMA,
    &abilitycategory::ABILITYCATEGORY_SCHEMA,
    &classlevel::CLASSLEVEL_SCHEMA,
    &companionmod::FOLLOWER_COMPANIONMOD_SCHEMA,
    &companionmod::MASTERBONUSRACE_COMPANIONMOD_SCHEMA,
    &datacontrol::FACTSETDEF_SCHEMA,
    &datacontrol::FUNCTION_SCHEMA,
    &datacontrol::DYNAMICSCOPE_SCHEMA,
    &startpack::STARTPACK_SCHEMA,
    &startpack::FUNDS_STARTPACK_SCHEMA,
    &startpack::GENDER_STARTPACK_SCHEMA,
    &startpack::LANGAUTO_STARTPACK_SCHEMA,
    &startpack::TOTALCOST_STARTPACK_SCHEMA,
    &biosettings::BIOSETTINGS_SCHEMA,
    &system::BONUSSPELLLEVEL_SCHEMA,
    &system::BONUSSTACKS_SCHEMA,
    &system::BONUSFEATLEVELSTARTINTERVAL_SCHEMA,
    &system::BONUSSTATLEVELSTARTINTERVAL_SCHEMA,
    &system::ALIGNMENTFEATURE_SYSTEM_SCHEMA,
    &system::CURRENCYUNITABBREV_SYSTEM_SCHEMA,
    &system::MENUENTRY_SYSTEM_SCHEMA,
    &system::DISPLAYORDER_SYSTEM_SCHEMA,
    &system::DIESIZES_SYSTEM_SCHEMA,
    &system::DEFAULTDATASET_SYSTEM_SCHEMA,
    &system::DEFAULTUNITSET_SYSTEM_SCHEMA,
    &system::ALLOWEDMODES_SYSTEM_SCHEMA,
    &system::GAMEMODEKEY_SYSTEM_SCHEMA,
    &system::BABMAXATT_SYSTEM_SCHEMA,
    &system::BABMINVAL_SYSTEM_SCHEMA,
    &system::BABATTCYC_SYSTEM_SCHEMA,
    &system::ACNAME_SYSTEM_SCHEMA,
    &system::DOMAINFEATURE_SYSTEM_SCHEMA,
    &system::LEVELMSG_SYSTEM_SCHEMA,
    &system::SHORTRANGE_SYSTEM_SCHEMA,
    &system::RANGEPENALTY_SYSTEM_SCHEMA,
    &system::SQUARESIZE_SYSTEM_SCHEMA,
    &system::SKILLMULTIPLIER_SYSTEM_SCHEMA,
    &system::SPELLBASEDC_SYSTEM_SCHEMA,
    &system::WEAPONNONPROFPENALTY_SYSTEM_SCHEMA,
    &system::WEAPONREACH_SYSTEM_SCHEMA,
    &system::CHARACTERTYPE_SYSTEM_SCHEMA,
    &system::CRTHRESHOLD_SYSTEM_SCHEMA,
    &system::CRSTEPS_SYSTEM_SCHEMA,
    &system::MONSTERROLES_SYSTEM_SCHEMA,
    &system::MONSTERROLEDEFAULT_SYSTEM_SCHEMA,
    &system::XPTABLE_SYSTEM_SCHEMA,
    &system::EQSIZEPENALTY_SYSTEM_SCHEMA,
    &system::RESIZABLEEQUIPTYPE_SYSTEM_SCHEMA,
    &system::SKILLCOST_CROSSCLASS_SYSTEM_SCHEMA,
    &system::SKILLCOST_CLASS_SYSTEM_SCHEMA,
    &system::SKILLCOST_EXCLUSIVE_SYSTEM_SCHEMA,
    &system::SPELLBASECONCENTRATION_SYSTEM_SCHEMA,
    &system::XPAWARD_SYSTEM_SCHEMA,
    &system::STATINPUT_SYSTEM_SCHEMA,
    &system::MAXNONEPICLEVEL_SYSTEM_SCHEMA,
    &system::PLUSCOST_SYSTEM_SCHEMA,
    &system::PREVIEWDIR_SCHEMA,
    &system::PREVIEWSHEET_SCHEMA,
    &system::LOAD_SYSTEM_SCHEMA,
    &system::LOADMULT_SYSTEM_SCHEMA,
    &system::NUMSLOTS_SYSTEM_SCHEMA,
    &system::METHOD_SYSTEM_SCHEMA,
    &system::SIZEMULT_SYSTEM_SCHEMA,
    &system::ENCUMBRANCE_SYSTEM_SCHEMA,
    &system::DEFAULTVARIABLEVALUE_SYSTEM_SCHEMA,
    &system::SPELLRANGE_SYSTEM_SCHEMA,
    &system::OUTPUTSHEET_SYSTEM_SCHEMA,
    &system::INFOSHEET_SYSTEM_SCHEMA,
    &system::UNITSET_SYSTEM_SCHEMA,
    &system::WEAPONCATEGORY_SYSTEM_SCHEMA,
    &system::ROLLMETHOD_SYSTEM_SCHEMA,
    &system::CLASSTYPE_SYSTEM_SCHEMA,
    &system::ACTYPE_SYSTEM_SCHEMA,
    &system::BASEDICE_SYSTEM_SCHEMA,
    &system::WIELDCATEGORY_SYSTEM_SCHEMA,
    &system::TAB_SYSTEM_SCHEMA,
    &system::EQSLOT_SYSTEM_SCHEMA,
    &system::AGESET_SYSTEM_SCHEMA,
    &system::LEVEL_SYSTEM_SCHEMA,
    &system::ICON_SYSTEM_SCHEMA,
    &system::ALIGN_SYSTEM_SCHEMA,
    &system::STAT_SYSTEM_SCHEMA,
    &system::SIZEADJUSTMENT_SYSTEM_SCHEMA,
    &system::RACE_SYSTEM_SCHEMA,
    &system::NAME_SYSTEM_SCHEMA,
    &system::WEAPONTYPE_SYSTEM_SCHEMA,
    &system::TABLE_SYSTEM_SCHEMA,
    &system::STARTTABLE_SYSTEM_SCHEMA,
    &system::ENDTABLE_SYSTEM_SCHEMA,
    &system::MOVEMENT_SYSTEM_SCHEMA,
    &system::SUBCLASSLEVEL_SYSTEM_SCHEMA,
    &system::SUBSTITUTIONCLASS_SYSTEM_SCHEMA,
    &system::SUBSTITUTIONLEVEL_SYSTEM_SCHEMA,
    &class::CLASS_SCHEMA,
    &subclass::SUBCLASS_SCHEMA,
    &deity::DEITY_SCHEMA,
    &equipment::EQUIPMENT_SCHEMA,
    &feat::FEAT_SCHEMA,
    &gear::GEAR_SCHEMA,
    &kit::KIT_SCHEMA,
    &modify::MODIFY_SCHEMA,
    &pcc::PCC_SCHEMA,
    &pcc::LANGUAGE_INCLUDE_SCHEMA,
    &pcc::ABILITY_INCLUDE_SCHEMA,
    &pcc::ABILITYCATEGORY_INCLUDE_SCHEMA,
    &pcc::FEAT_INCLUDE_SCHEMA,
    &pcc::EQUIPMENT_INCLUDE_SCHEMA,
    &pcc::SPELL_INCLUDE_SCHEMA,
    &pcc::LICENSE_INCLUDE_SCHEMA,
    &pcc::INFOTEXT_INCLUDE_SCHEMA,
    &pcc::FORWARDREF_INCLUDE_SCHEMA,
    &pcc::HIDETYPE_INCLUDE_SCHEMA,
    &pcc::URL_INCLUDE_SCHEMA,
    &pcc::ALIGNMENT_INCLUDE_SCHEMA,
    &pcc::SAVE_INCLUDE_SCHEMA,
    &pcc::BIOSET_INCLUDE_SCHEMA,
    &pcc::EQUIPMOD_INCLUDE_SCHEMA,
    &pcc::DATACONTROL_INCLUDE_SCHEMA,
    &pcc::DYNAMIC_INCLUDE_SCHEMA,
    &pcc::DATATABLE_INCLUDE_SCHEMA,
    &pcc::COMPANIONMOD_INCLUDE_SCHEMA,
    &pcc::WEAPONPROF_INCLUDE_SCHEMA,
    &pcc::ARMORPROF_INCLUDE_SCHEMA,
    &pcc::SHIELDPROF_INCLUDE_SCHEMA,
    &pcc::LSTEXCLUDE_INCLUDE_SCHEMA,
    &pcc::SHOWINMENU_INCLUDE_SCHEMA,
    &race::RACE_SCHEMA,
    &skill::SKILL_SCHEMA,
    &spell::SPELL_SCHEMA,
    &template::TEMPLATE_SCHEMA,
    &variables::LOCAL_VARIABLE_SCHEMA,
    &variables::GLOBAL_VARIABLE_SCHEMA,
];

/// Look up a schema by artisan entity type key.
pub fn schema_for_entity_type_key(key: &str) -> Option<&'static EntitySchema> {
    ALL_SCHEMAS.iter().copied().find(|s| s.entity_type_key == key)
}

/// Look up a schema by PCGen head token (e.g. `"CLASS"`, `"SKILL"`).
///
/// Returns `None` for entity types with name-only heads.
pub fn schema_for_head_token(token: &str) -> Option<&'static EntitySchema> {
    ALL_SCHEMAS
        .iter()
        .copied()
        .find(|s| s.head_token.map_or(false, |ht| ht.eq_ignore_ascii_case(token)))
}

/// Returns true if any registered schema recognizes `key` as a known token.
///
/// Used by `token_policy` for global (context-free) token classification.
pub fn any_schema_knows_token(key: &str) -> bool {
    let normalized = token_aliases::canonical_lookup_key(key, None);

    ALL_SCHEMAS.iter().any(|s| {
        s.knows_token_key(&normalized)
            || s
                .head_token
                .is_some_and(|head| head.eq_ignore_ascii_case(&normalized))
    })
}
