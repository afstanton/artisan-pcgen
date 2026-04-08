//! PCGen entity-type schemas.
//!
//! Each schema captures the complete token grammar for one PCGen entity type,
//! derived from the official PCGen LST documentation. The same schema drives
//! both parsing (token classification, value interpretation) and emission
//! (serializing artisan `Entity` values back to `.lst` text).

pub mod ability;
pub mod abilitycategory;
pub mod class;
pub mod equipment;
pub mod feat;
pub mod pcc;
pub mod race;
pub mod skill;
pub mod spell;
pub mod template;

pub use ability::ABILITY_SCHEMA;
pub use abilitycategory::ABILITYCATEGORY_SCHEMA;
pub use class::CLASS_SCHEMA;
pub use equipment::EQUIPMENT_SCHEMA;
pub use feat::FEAT_SCHEMA;
pub use pcc::PCC_SCHEMA;
pub use race::RACE_SCHEMA;
pub use skill::SKILL_SCHEMA;
pub use spell::SPELL_SCHEMA;
pub use template::TEMPLATE_SCHEMA;

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

/// How the record head for this entity type is formatted in a `.lst` file.
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
    /// All `PRExxx:` and `!PRExxx:` prerequisite tokens (repeatable)
    Prerequisites,
    /// `SOURCEPAGE:x` — source page citation
    SourcePage,
    /// `OUTPUTNAME:x` — display name override
    OutputName,
    /// `TEMPLATE:x`, `TEMPLATE:ADDCHOICE:x`, `TEMPLATE:CHOOSE:x` (repeatable)
    Template,
    /// `CSKILL:x|x` — cross-class skill (repeatable)
    CSkill,
    /// `SAB:x` — special ability text (repeatable)
    Sab,
    /// `QUALIFY:type|...` — qualification grant (repeatable)
    Qualify,
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
            GlobalGroup::Desc => &["DESC"],
            GlobalGroup::Fact => &["FACT"],
            GlobalGroup::Bonus => &["BONUS"],
            GlobalGroup::Add => &["ADD"],
            GlobalGroup::Choose => &["CHOOSE"],
            GlobalGroup::Auto => &["AUTO"],
            GlobalGroup::Define => &["DEFINE", "DEFINESTAT"],
            GlobalGroup::Prerequisites => &["PRE", "!PRE"],
            GlobalGroup::SourcePage => &["SOURCEPAGE"],
            GlobalGroup::OutputName => &["OUTPUTNAME"],
            GlobalGroup::Template => &["TEMPLATE"],
            GlobalGroup::CSkill => &["CSKILL"],
            GlobalGroup::Sab => &["SAB"],
            GlobalGroup::Qualify => &["QUALIFY"],
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
            ],
        }
    }

    /// Returns true if the given (already-uppercased) key belongs to this group.
    pub fn matches(self, upper_key: &str) -> bool {
        match self {
            // Prerequisites use prefix matching
            GlobalGroup::Prerequisites => {
                upper_key.starts_with("PRE") || upper_key.starts_with("!PRE")
            }
            // ADD uses prefix matching too (ADD:ABILITY, ADD:FEAT, etc.)
            GlobalGroup::Add => upper_key == "ADD" || upper_key.starts_with("ADD:"),
            // CHOOSE uses prefix matching (CHOOSE:ABILITY, CHOOSE:FEAT, etc.)
            GlobalGroup::Choose => upper_key == "CHOOSE" || upper_key.starts_with("CHOOSE"),
            // BONUS uses prefix matching (BONUS:COMBAT, BONUS:SKILL, etc.)
            GlobalGroup::Bonus => upper_key == "BONUS" || upper_key.starts_with("BONUS"),
            // TEMPLATE uses prefix matching
            GlobalGroup::Template => upper_key == "TEMPLATE" || upper_key.starts_with("TEMPLATE"),
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
    /// How the head record is formatted in `.lst` output.
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
    &class::CLASS_SCHEMA,
    &equipment::EQUIPMENT_SCHEMA,
    &feat::FEAT_SCHEMA,
    &pcc::PCC_SCHEMA,
    &race::RACE_SCHEMA,
    &skill::SKILL_SCHEMA,
    &spell::SPELL_SCHEMA,
    &template::TEMPLATE_SCHEMA,
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
    ALL_SCHEMAS.iter().any(|s| s.knows_token_key(key))
}
