use indexmap::IndexMap;
use serde_json::{Map, Value, json};

use crate::ParsedClause;
use crate::parsing::parse_modify;

// ---------------------------------------------------------------------------
// Bracket-group parsing
// ---------------------------------------------------------------------------

/// Parse a `[KEY:val|KEY:val|...]` bracket-group string into a JSON array of
/// `{"key": "K", "value": "V"}` objects.
///
/// The brackets are stripped before parsing. Inner pipes that separate entries
/// are split naively (no nested-bracket support needed for current PCGen usage).
/// If the value is not bracket-delimited it is returned as a plain string.
pub(crate) fn parse_bracket_group(value: &str) -> Value {
    let inner = value.trim();
    let inner = if inner.starts_with('[') && inner.ends_with(']') {
        &inner[1..inner.len() - 1]
    } else {
        return Value::String(value.to_string());
    };

    let items: Vec<Value> = inner
        .split('|')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            if let Some(colon) = part.find(':') {
                let key = part[..colon].trim();
                let val = &part[colon + 1..];
                if !key.is_empty() {
                    return Some(json!({"key": key, "value": val}));
                }
            }
            Some(json!({"value": part}))
        })
        .collect();
    Value::Array(items)
}

/// Append `value` onto an array attribute, creating the array if needed.
///
/// Used for tokens that can appear multiple times on a line (e.g. repeated
/// `CLASSBOUGHT` bracket groups). Each occurrence is stored as a separate
/// element so the full list is always `[occurrence1, occurrence2, ...]`.
fn append_attribute(attributes: &mut IndexMap<String, Value>, key: &str, value: Value) {
    match attributes.get_mut(key) {
        Some(Value::Array(arr)) => {
            arr.push(value);
        }
        Some(existing) => {
            // Promote scalar to array then append.
            let prev = std::mem::replace(existing, Value::Null);
            *existing = Value::Array(vec![prev, value]);
        }
        None => {
            attributes.insert(key.to_string(), Value::Array(vec![value]));
        }
    }
}

pub(crate) fn project_clause_attributes(
    head_name: &str,
    clauses: &[ParsedClause],
    attributes: &mut IndexMap<String, Value>,
) {
    let mut facts = Vec::new();
    let mut factsets = Vec::new();
    let mut equipment_modifiers = Vec::new();
    let mut class_lists = Vec::new();
    let mut spell_blocks = Vec::new();
    let mut sprop_values = Vec::new();
    let mut page_usage_values = Vec::new();

    for clause in clauses {
        let ParsedClause::KeyValue { key, value } = clause else {
            continue;
        };

        match key.to_ascii_uppercase().as_str() {
            "TYPE" => {
                attributes.insert("type".to_string(), Value::String(value.clone()));
            }
            "SOURCE" => {
                // PCG files use SOURCE:[TYPE:CLASS|NAME:Wizard] bracket groups;
                // LST/PCC files use SOURCE:plain text. Store structured when bracketed.
                attributes.insert("pcgen_source".to_string(), parse_bracket_group(value));
            }
            "SOURCELONG" => {
                attributes.insert(
                    "pcgen_source_long".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SOURCESHORT" => {
                attributes.insert(
                    "pcgen_source_short".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SOURCEWEB" => {
                attributes.insert("pcgen_source_web".to_string(), Value::String(value.clone()));
            }
            "SOURCELINK" => {
                attributes.insert(
                    "pcgen_source_link".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SOURCEDATE" => {
                attributes.insert(
                    "pcgen_source_date".to_string(),
                    Value::String(value.clone()),
                );
            }
            "GAMEMODE" => {
                attributes.insert("pcgen_gamemode".to_string(), Value::String(value.clone()));
            }
            "GENRE" => {
                attributes.insert("pcgen_genre".to_string(), Value::String(value.clone()));
            }
            "ISOGL" => {
                attributes.insert("pcgen_isogl".to_string(), parse_yes_no_or_string(value));
            }
            "PUBNAMESHORT" => {
                attributes.insert(
                    "pcgen_publisher_short".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SHOWINMENU" => {
                attributes.insert(
                    "pcgen_showinmenu".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "SETTING" => {
                attributes.insert("pcgen_setting".to_string(), Value::String(value.clone()));
            }
            "BOOKTYPE" => {
                attributes.insert("book_type".to_string(), Value::String(value.clone()));
            }
            "PCC" => append_string_attr(attributes, "pcgen_pcc", value),
            "CHOICE" => append_string_attr(attributes, "pcgen_choice", value),
            "BIOSET" => append_string_attr(attributes, "pcgen_bioset_catalog", value),
            "DATACONTROL" => append_string_attr(attributes, "pcgen_datacontrol_catalog", value),
            "DATATABLE" => append_string_attr(attributes, "pcgen_datatable_catalog", value),
            "COMPANIONMOD" => append_string_attr(attributes, "pcgen_companionmod_catalog", value),
            "ALIGNMENT" => append_string_attr(attributes, "pcgen_alignment_catalog", value),
            "SAVE" => append_string_attr(attributes, "pcgen_save_catalog", value),
            "PRECAMPAIGN" => {
                append_value_attr(attributes, "pcgen_precampaign", parse_pipe_series(value));
            }
            "BASESTATSCORE" => set_i64_or_string(attributes, "pcgen_basestatscore", value),
            "STATRANGE" => set_i64_or_string(attributes, "pcgen_statrange", value),
            "STATMOD" => {
                attributes.insert("pcgen_statmod".to_string(), Value::String(value.clone()));
            }
            "ADDSPELLLEVEL" => set_i64_or_string(attributes, "pcgen_addspelllevel", value),
            "MULT" => {
                attributes.insert("pcgen_mult".to_string(), Value::String(value.clone()));
            }
            "STACK" => {
                attributes.insert("pcgen_stack".to_string(), Value::String(value.clone()));
            }
            "BENEFIT" => {
                attributes.insert("benefit".to_string(), Value::String(value.clone()));
            }
            "TEMPDESC" => {
                attributes.insert("tempdesc".to_string(), Value::String(value.clone()));
            }
            "DESCISPI" => {
                attributes.insert("pcgen_descispi".to_string(), parse_yes_no_or_string(value));
            }
            "SPELLLEVEL" => {
                attributes.insert("pcgen_spelllevel".to_string(), Value::String(value.clone()));
            }
            "INFO" => {
                attributes.insert("pcgen_info".to_string(), Value::String(value.clone()));
            }
            "APPLY" => {
                attributes.insert("pcgen_apply".to_string(), Value::String(value.clone()));
            }
            "LOOKUP" => append_string_attr(attributes, "pcgen_lookup", value),
            "BASEAGE" => {
                attributes.insert("base_age".to_string(), Value::String(value.clone()));
            }
            "MAXAGE" => {
                attributes.insert("max_age".to_string(), Value::String(value.clone()));
            }
            "BASEAGEADD" => {
                attributes.insert("base_age_add".to_string(), Value::String(value.clone()));
            }
            "AGEDIEROLL" => {
                attributes.insert("age_die_roll".to_string(), Value::String(value.clone()));
            }
            "SEX" => append_string_attr(attributes, "sex", value),
            "HAIR" => {
                attributes.insert("hair".to_string(), Value::String(value.clone()));
            }
            "EYES" => {
                attributes.insert("eyes".to_string(), Value::String(value.clone()));
            }
            "SKINTONE" => {
                attributes.insert("skin_tone".to_string(), Value::String(value.clone()));
            }
            "ASPECT" => append_string_attr(attributes, "pcgen_aspects", value),
            "RACETYPE" => {
                attributes.insert("racetype".to_string(), Value::String(value.clone()));
            }
            "RACESUBTYPE" => append_string_attr(attributes, "race_subtype", value),
            "SUBRACE" => {
                attributes.insert("pcgen_subrace".to_string(), Value::String(value.clone()));
            }
            "SUBCLASS" => append_string_attr(attributes, "pcgen_subclass", value),
            "FAVCLASS" => {
                attributes.insert("favored_class".to_string(), Value::String(value.clone()));
            }
            "FAVOREDCLASS" => {
                attributes.insert("favored_class".to_string(), Value::String(value.clone()));
            }
            "MONSTERCLASS" => {
                attributes.insert("monsterclass".to_string(), Value::String(value.clone()));
            }
            "MONCSKILL" => append_string_attr(attributes, "pcgen_moncskill", value),
            "CCSKILL" => append_string_attr(attributes, "pcgen_ccskill", value),
            "MONCCSKILL" => append_string_attr(attributes, "pcgen_monccskill", value),
            "SKILLMULT" => append_string_attr(attributes, "pcgen_skillmult", value),
            "STARTFEATS" => set_i64_or_string(attributes, "pcgen_startfeats", value),
            "HITDICEADVANCEMENT" => {
                attributes.insert(
                    "pcgen_hitdiceadvancement".to_string(),
                    Value::String(value.clone()),
                );
            }
            "LEVELADJUSTMENT" => set_i64_or_string(attributes, "level_adjustment", value),
            "XTRASKILLPTSPERLVL" => {
                set_i64_or_string(attributes, "pcgen_xtraskillptsperlvl", value)
            }
            "GENDERLOCK" => {
                attributes.insert("pcgen_genderlock".to_string(), Value::String(value.clone()));
            }
            "BONUSSKILLPOINTS" => set_i64_or_string(attributes, "pcgen_bonusskillpoints", value),
            "ADDLEVEL" => {
                attributes.insert("pcgen_addlevel".to_string(), Value::String(value.clone()));
            }
            "REPEATLEVEL" => append_string_attr(attributes, "pcgen_repeatlevel", value),
            "MOVECLONE" => append_string_attr(attributes, "pcgen_moveclone", value),
            "HD" => set_i64_or_string(attributes, "hitdie", value),
            "HITDIE" => set_i64_or_string(attributes, "hitdie", value),
            "MAXLEVEL" => {
                attributes.insert("maxlevel".to_string(), Value::String(value.clone()));
            }
            "ABB" => {
                attributes.insert("abbreviation".to_string(), Value::String(value.clone()));
            }
            "STARTSKILLPTS" => set_i64_or_string(attributes, "pcgen_startskillpts", value),
            "LEVELSPERFEAT" => set_i64_or_string(attributes, "pcgen_levelsperfeat", value),
            "ATTACKCYCLE" => {
                attributes.insert(
                    "pcgen_attackcycle".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SPELLTYPE" => {
                attributes.insert("pcgen_spelltype".to_string(), Value::String(value.clone()));
            }
            "SPELLSTAT" => {
                attributes.insert("spellstat".to_string(), Value::String(value.clone()));
            }
            "ITEMCREATE" => {
                attributes.insert("pcgen_itemcreate".to_string(), Value::String(value.clone()));
            }
            "BONUSSPELLSTAT" => {
                attributes.insert(
                    "pcgen_bonusspellstat".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SPELLBOOK" => {
                attributes.insert("pcgen_spellbook".to_string(), parse_yes_no_or_string(value));
            }
            "MEMORIZE" => {
                attributes.insert("pcgen_memorize".to_string(), parse_yes_no_or_string(value));
            }
            "CAST" => append_string_attr(attributes, "pcgen_cast", value),
            "KNOWN" => append_string_attr(attributes, "pcgen_known", value),
            "KNOWNSPELLS" => {
                attributes.insert(
                    "pcgen_knownspells".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SPECIALTYKNOWN" => {
                attributes.insert(
                    "pcgen_specialtyknown".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SPELLLIST" => {
                attributes.insert("pcgen_spelllist".to_string(), Value::String(value.clone()));
            }
            "MAXCOST" => set_i64_or_string(attributes, "pcgen_maxcost", value),
            "PROHIBITSPELL" => append_string_attr(attributes, "pcgen_prohibitspell", value),
            "KNOWNSPELLSFROMSPECIALTY" => {
                attributes.insert(
                    "pcgen_knownspellsfromspecialty".to_string(),
                    Value::String(value.clone()),
                );
            }
            "PROHIBITED" => append_string_attr(attributes, "pcgen_prohibited", value),
            "ADDDOMAINS" => append_string_attr(attributes, "add_domains", value),
            "DOMAIN" => {
                attributes.insert("domains".to_string(), Value::String(value.clone()));
            }
            "ALIGN" => {
                attributes.insert("alignment".to_string(), Value::String(value.clone()));
            }
            "DEITYWEAP" => {
                attributes.insert("deity_weapon".to_string(), Value::String(value.clone()));
            }
            "PANTHEON" => {
                attributes.insert("pcgen_pantheon".to_string(), Value::String(value.clone()));
            }
            "GROUP" => append_string_attr(attributes, "group", value),
            "ALLOWBASECLASS" => {
                attributes.insert(
                    "pcgen_allowbaseclass".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "MODTOSKILLS" => {
                attributes.insert(
                    "pcgen_modtoskills".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "MONSKILL" => set_i64_or_string(attributes, "pcgen_monskill", value),
            "MONNONSKILLHD" => set_i64_or_string(attributes, "pcgen_monnonskillhd", value),
            "WEAPONBONUS" => append_string_attr(attributes, "weapon_bonus", value),
            "VISIBLE" => {
                attributes.insert("pcgen_visible".to_string(), Value::String(value.clone()));
            }
            "UDAM" => {
                attributes.insert("pcgen_udam".to_string(), Value::String(value.clone()));
            }
            "UMULT" => set_i64_or_string(attributes, "pcgen_umult", value),
            "DONOTADD" => append_string_attr(attributes, "pcgen_donotadd", value),
            "COMPANIONLIST" => append_string_attr(attributes, "pcgen_companionlist", value),
            "FOLLOWERS" => append_string_attr(attributes, "pcgen_followers", value),
            "REMOVABLE" => {
                attributes.insert("pcgen_removable".to_string(), Value::String(value.clone()));
            }
            "PROHIBITCOST" => set_i64_or_string(attributes, "pcgen_prohibitcost", value),
            "HASSUBCLASS" => {
                attributes.insert(
                    "pcgen_hassubclass".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "SUBCLASSLEVEL" => {
                attributes.insert(
                    "pcgen_subclasslevel".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SUBSTITUTIONCLASS" => {
                attributes.insert(
                    "pcgen_substitutionclass".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SUBSTITUTIONLEVEL" => {
                attributes.insert(
                    "pcgen_substitutionlevel".to_string(),
                    Value::String(value.clone()),
                );
            }
            "EXCLASS" => append_string_attr(attributes, "pcgen_exclass", value),
            "EXCHANGELEVEL" => append_string_attr(attributes, "pcgen_exchangelevel", value),
            "CATEGORY" => {
                attributes.insert("category".to_string(), Value::String(value.clone()));
            }
            "DESC" => {
                // Store under the canonical "description" key. The legacy pcgen_desc key
                // is retained as an alias for emit paths that read it by name.
                attributes.insert("description".to_string(), Value::String(value.clone()));
                attributes
                    .entry("pcgen_desc".to_string())
                    .or_insert_with(|| Value::String(value.clone()));
            }
            "DESC.CLEAR" => {
                attributes.insert("pcgen_desc_clear".to_string(), Value::Bool(true));
            }
            "KEY" => {
                attributes.insert("key".to_string(), Value::String(value.clone()));
            }
            "LANGBONUS" => append_string_attr(attributes, "pcgen_langbonus", value),
            "CSKILL" => append_string_attr(attributes, "cskill", value),
            "SAB" => append_string_attr(attributes, "sab", value),
            "CHANGEPROF" => append_string_attr(attributes, "pcgen_changeprof", value),
            "SERVESAS" => append_string_attr(attributes, "pcgen_servesas", value),
            "QUALIFY" => append_string_attr(attributes, "pcgen_qualify", value),
            "TEMPLATE" => append_string_attr(attributes, "pcgen_template", value),
            "OUTPUTNAME" => {
                attributes.insert("outputname".to_string(), Value::String(value.clone()));
            }
            "RANK" => {
                if let Ok(rank) = value.trim().parse::<i64>() {
                    attributes.insert("rank".to_string(), json!(rank));
                } else {
                    attributes.insert("rank".to_string(), Value::String(value.clone()));
                }
            }
            "COST" => {
                attributes.insert("cost".to_string(), Value::String(value.clone()));
            }
            "COSTPRE" => {
                attributes.insert("pcgen_costpre".to_string(), Value::String(value.clone()));
            }
            "BASEITEM" => {
                attributes.insert("pcgen_baseitem".to_string(), Value::String(value.clone()));
            }
            "WT" => {
                attributes.insert("weight".to_string(), Value::String(value.clone()));
            }
            // .pcg sub-tokens for EQUIPNAME records
            "OUTPUTORDER" => {
                set_i64_or_string(attributes, "pcgen_outputorder", value);
            }
            "LEVEL" => {
                set_i64_or_string(attributes, "pcgen_level", value);
            }
            "QUANTITY" => {
                attributes.insert("pcgen_quantity".to_string(), Value::String(value.clone()));
            }
            "NOTE" => {
                attributes.insert("note".to_string(), Value::String(value.clone()));
            }
            "CUSTOMIZATION" => {
                // EQUIPNAME CUSTOMIZATION value is a bracket group: [BASEITEM:x|DATA:y|...]
                attributes.insert(
                    "pcgen_customization".to_string(),
                    parse_bracket_group(value),
                );
            }
            // DATA was historically a separate clause when bracket content was pipe-split.
            // With bracket-aware parsing it no longer appears standalone; arm kept for safety.
            "DATA" => {
                attributes.insert("pcgen_data".to_string(), Value::String(value.clone()));
            }
            // SPECIALTIES bracket group: [SPECIALTY:Evocation|...]
            "SPECIALTIES" => {
                attributes.insert("pcgen_specialties".to_string(), parse_bracket_group(value));
            }
            // CLASS as a clause key: appears in PCG SPELLNAME, CLASSBOUGHT, etc.
            "CLASS" => {
                attributes.insert("class".to_string(), Value::String(value.clone()));
            }
            // CLASSBOUGHT is a bracket group: [CLASS:Wizard|RANKS:3.0|COST:1|CLASSSKILL:Y]
            // Multiple CLASSBOUGHT groups on a single line are accumulated as an array of arrays —
            // each group is an element: [[{key:CLASS,value:Bard},...], [{key:CLASS,value:Aristocrat},...]]
            "CLASSBOUGHT" => {
                append_attribute(attributes, "pcgen_classbought", parse_bracket_group(value));
            }
            // RANKS / CLASSSKILL historically appeared as split-out bracket sub-items;
            // kept for safety in case any corpus variant writes them standalone.
            "RANKS" => {
                attributes.insert("pcgen_ranks".to_string(), Value::String(value.clone()));
            }
            "CLASSSKILL" => {
                attributes.insert(
                    "pcgen_classskill".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            // WEAPON historically appeared as a split-out bracket sub-item of WEAPONPROF;
            // kept for safety but no longer triggered by bracket-aware parsing.
            "WEAPON" => {
                attributes.insert("pcgen_weapon_ref".to_string(), Value::String(value.clone()));
            }
            // DEITYDOMAINS bracket group: [DOMAIN:Good|DOMAIN:Sun]
            "DEITYDOMAINS" => {
                attributes.insert("pcgen_deitydomains".to_string(), parse_bracket_group(value));
            }
            "ALIGNALLOW" => {
                attributes.insert("pcgen_alignallow".to_string(), Value::String(value.clone()));
            }
            "HOLYITEM" => {
                attributes.insert("pcgen_holyitem".to_string(), Value::String(value.clone()));
            }
            // DEITYFAVWEAP bracket group: [WEAPON:Morningstar]
            "DEITYFAVWEAP" => {
                attributes.insert("pcgen_deityfavweap".to_string(), parse_bracket_group(value));
            }
            "DEITYALIGN" => {
                attributes.insert("pcgen_deityalign".to_string(), Value::String(value.clone()));
            }
            "DOMAINGRANTS" => {
                attributes.insert(
                    "pcgen_domaingrants".to_string(),
                    Value::String(value.clone()),
                );
            }
            // PCG spell record sub-tokens
            "TIMES" => {
                set_i64_or_string(attributes, "pcgen_times", value);
            }
            "BOOK" => {
                attributes.insert("book".to_string(), Value::String(value.clone()));
            }
            // FEATLIST bracket group: [FEAT:Empower Spell|FEAT:Quicken Spell]
            "FEATLIST" => {
                attributes.insert("pcgen_featlist".to_string(), parse_bracket_group(value));
            }
            // .pcg sub-tokens for EQUIPSET records
            "ID" => {
                attributes.insert(
                    "pcgen_equipset_id".to_string(),
                    Value::String(value.clone()),
                );
            }
            "USETEMPMODS" => {
                attributes.insert(
                    "pcgen_equipset_usetempmods".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            // .pcg sub-tokens for USERPOOL records
            "POOLPOINTS" => {
                attributes.insert(
                    "pcgen_userpool_poolpoints".to_string(),
                    Value::String(value.clone()),
                );
            }
            // .pcg sub-tokens for CLASS records (character-file class level info)
            "SKILLPOOL" => {
                set_i64_or_string(attributes, "pcgen_class_skillpool", value);
            }
            "SPELLBASE" => {
                attributes.insert(
                    "pcgen_class_spellbase".to_string(),
                    Value::String(value.clone()),
                );
            }
            "CANCASTPERDAY" => {
                attributes.insert(
                    "pcgen_class_cancastperday".to_string(),
                    Value::String(value.clone()),
                );
            }
            // .pcg sub-tokens for CLASSABILITIESLEVEL records
            "HITPOINTS" => {
                set_i64_or_string(attributes, "pcgen_cal_hitpoints", value);
            }
            "SKILLSGAINED" => {
                set_i64_or_string(attributes, "pcgen_cal_skillsgained", value);
            }
            "SKILLSREMAINING" => {
                set_i64_or_string(attributes, "pcgen_cal_skillsremaining", value);
            }
            // .pcg sub-token for standalone NOTE records: hierarchy parent
            "PARENTID" => {
                attributes.insert(
                    "pcgen_note_parentid".to_string(),
                    Value::String(value.clone()),
                );
            }
            // .pcg sub-token for FEAT / ABILITY records: what choice was applied
            "APPLIEDTO" => {
                attributes.insert("pcgen_appliedto".to_string(), Value::String(value.clone()));
            }
            "RANGE" => {
                attributes.insert("pcgen_range".to_string(), Value::String(value.clone()));
            }
            "SCHOOL" => {
                attributes.insert("pcgen_school".to_string(), Value::String(value.clone()));
            }
            "SUBSCHOOL" => {
                attributes.insert("pcgen_subschool".to_string(), Value::String(value.clone()));
            }
            "COMPS" => {
                attributes.insert("pcgen_comps".to_string(), Value::String(value.clone()));
            }
            "CT" => {
                set_i64_or_string(attributes, "pcgen_ct", value);
            }
            "CASTTIME" => {
                attributes.insert("pcgen_casttime".to_string(), Value::String(value.clone()));
            }
            "SORTKEY" => {
                attributes.insert("sortkey".to_string(), Value::String(value.clone()));
            }
            "VALIDFORDEITY" => {
                attributes.insert(
                    "pcgen_validfordeity".to_string(),
                    Value::String(value.clone()),
                );
            }
            "VALIDFORFOLLOWER" => {
                attributes.insert(
                    "pcgen_validforfollower".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SIZENUM" => {
                attributes.insert("pcgen_sizenum".to_string(), Value::String(value.clone()));
            }
            "ISDEFAULTSIZE" => {
                attributes.insert(
                    "pcgen_isdefaultsize".to_string(),
                    Value::String(value.clone()),
                );
            }
            "TARGETAREA" => {
                attributes.insert("pcgen_targetarea".to_string(), Value::String(value.clone()));
            }
            "DURATION" => {
                attributes.insert("pcgen_duration".to_string(), Value::String(value.clone()));
            }
            "SAVEINFO" => {
                attributes.insert("pcgen_saveinfo".to_string(), Value::String(value.clone()));
            }
            "SPELLRES" => {
                attributes.insert("pcgen_spellres".to_string(), Value::String(value.clone()));
            }
            "TEMPVALUE" => append_string_attr(attributes, "pcgen_tempvalue", value),
            "XPCOST" => {
                attributes.insert("pcgen_xpcost".to_string(), Value::String(value.clone()));
            }
            "DESCRIPTOR" => append_string_attr(attributes, "pcgen_descriptors", value),
            "DOMAINS" => {
                attributes.insert("domains".to_string(), Value::String(value.clone()));
            }
            "PPCOST" => set_i64_or_string(attributes, "pcgen_ppcost", value),
            "SPELLPOINTCOST" => {
                attributes.insert(
                    "pcgen_spellpointcost".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ITEM" => append_string_attr(attributes, "pcgen_items", value),
            "GEAR" => append_string_attr(attributes, "pcgen_gear", value),
            "KIT" => append_string_attr(attributes, "pcgen_kits", value),
            "ABILITY" => append_string_attr(attributes, "pcgen_abilities", value),
            "ABILITYCATEGORY" => append_string_attr(attributes, "pcgen_abilitycategories", value),
            "FEAT" => append_string_attr(attributes, "pcgen_feats", value),
            "EQUIPMENT" => append_string_attr(attributes, "equipment", value),
            "SPELL" => append_string_attr(attributes, "pcgen_spells", value),
            "LICENSE" => append_string_attr(attributes, "license", value),
            "INFOTEXT" => append_string_attr(attributes, "infotext", value),
            "FORWARDREF" => append_string_attr(attributes, "pcgen_forwardref", value),
            "HIDETYPE" => append_string_attr(attributes, "pcgen_hidetype", value),
            "URL" => append_string_attr(attributes, "url", value),
            "OPTION" => {
                attributes.insert("pcgen_option".to_string(), Value::String(value.clone()));
            }
            "MAXVER" => {
                attributes.insert("pcgen_maxver".to_string(), Value::String(value.clone()));
            }
            "MAXDEVVER" => {
                attributes.insert("pcgen_maxdevver".to_string(), Value::String(value.clone()));
            }
            "NEWKEY" => {
                attributes.insert("pcgen_newkey".to_string(), Value::String(value.clone()));
            }
            "NEWCATEGORY" => {
                attributes.insert("newcategory".to_string(), Value::String(value.clone()));
            }
            "VALUES" => append_string_attr(attributes, "pcgen_values", value),
            "COPYRIGHT" => append_string_attr(attributes, "copyright", value),
            "LSTEXCLUDE" => {
                append_value_attr(attributes, "pcgen_lstexclude", parse_pipe_series(value));
            }
            "FACTDEF" => {
                attributes.insert("pcgen_factdef".to_string(), Value::String(value.clone()));
            }
            "VALUE" => {
                attributes.insert("pcgen_value".to_string(), Value::String(value.clone()));
            }
            "DATAFORMAT" => {
                attributes.insert("pcgen_dataformat".to_string(), Value::String(value.clone()));
            }
            "DISPLAYNAME" => {
                attributes.insert(
                    "pcgen_displayname".to_string(),
                    Value::String(value.clone()),
                );
            }
            "DEFAULTDATASET" => {
                attributes.insert(
                    "pcgen_defaultdataset".to_string(),
                    Value::String(value.clone()),
                );
            }
            "EXPLANATION" => {
                attributes.insert("explanation".to_string(), Value::String(value.clone()));
            }
            "REQUIRED" => {
                attributes.insert("pcgen_required".to_string(), Value::String(value.clone()));
            }
            "SELECTABLE" => {
                attributes.insert("pcgen_selectable".to_string(), Value::String(value.clone()));
            }
            "NAMEISPI" => {
                attributes.insert("pcgen_nameispi".to_string(), Value::String(value.clone()));
            }
            "ABILITYLIST" => append_string_attr(attributes, "pcgen_abilitylist", value),
            "DISPLAYLOCATION" => {
                attributes.insert(
                    "pcgen_displaylocation".to_string(),
                    Value::String(value.clone()),
                );
            }
            "EDITABLE" => {
                attributes.insert("pcgen_editable".to_string(), Value::String(value.clone()));
            }
            "EDITPOOL" => {
                attributes.insert("pcgen_editpool".to_string(), Value::String(value.clone()));
            }
            "FRACTIONALPOOL" => {
                attributes.insert(
                    "pcgen_fractionalpool".to_string(),
                    Value::String(value.clone()),
                );
            }
            "GAMEMODEKEY" => {
                attributes.insert(
                    "pcgen_gamemodekey".to_string(),
                    Value::String(value.clone()),
                );
            }
            "PLURAL" => {
                attributes.insert("pcgen_plural".to_string(), Value::String(value.clone()));
            }
            "POOL" => {
                attributes.insert("pcgen_pool".to_string(), Value::String(value.clone()));
            }
            "PARM" => {
                attributes.insert("pcgen_parm".to_string(), Value::String(value.clone()));
            }
            "VAR" => {
                attributes.insert("pcgen_var".to_string(), Value::String(value.clone()));
            }
            "DEFAULT" => {
                attributes.insert("pcgen_default".to_string(), parse_yes_no_or_string(value));
            }
            "MINXP" => {
                attributes.insert("pcgen_minxp".to_string(), Value::String(value.clone()));
            }
            "CSKILLMAX" => {
                attributes.insert("pcgen_cskillmax".to_string(), Value::String(value.clone()));
            }
            "CCSKILLMAX" => {
                attributes.insert("pcgen_ccskillmax".to_string(), Value::String(value.clone()));
            }
            "EQUIPBUY" => append_string_attr(attributes, "pcgen_equipbuy", value),
            "LOCATION" => {
                attributes.insert("pcgen_location".to_string(), Value::String(value.clone()));
            }
            "QTY" => set_i64_or_string(attributes, "qty", value),
            "COUNT" => set_i64_or_string(attributes, "count", value),
            "COPYMASTERBAB" => {
                attributes.insert(
                    "pcgen_copymasterbab".to_string(),
                    Value::String(value.clone()),
                );
            }
            "COPYMASTERCHECK" => {
                attributes.insert(
                    "pcgen_copymastercheck".to_string(),
                    Value::String(value.clone()),
                );
            }
            "COPYMASTERHP" => {
                attributes.insert(
                    "pcgen_copymasterhp".to_string(),
                    Value::String(value.clone()),
                );
            }
            "USEMASTERSKILL" => {
                attributes.insert(
                    "pcgen_usemasterskill".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "GENDER" => {
                attributes.insert("gender".to_string(), Value::String(value.clone()));
            }
            "EXCLUDE" => {
                attributes.insert("pcgen_exclude".to_string(), Value::String(value.clone()));
            }
            "SUBREGION" => {
                attributes.insert("pcgen_subregion".to_string(), Value::String(value.clone()));
            }
            "POINTS" => set_i64_or_string(attributes, "pcgen_points", value),
            "MAXNONEPICLEVEL" => set_i64_or_string(attributes, "pcgen_maxnonepiclevel", value),
            "SKILLCOST_CROSSCLASS" => {
                set_i64_or_string(attributes, "pcgen_skillcost_crossclass", value)
            }
            "SKILLCOST_CLASS" => set_i64_or_string(attributes, "pcgen_skillcost_class", value),
            "SKILLCOST_EXCLUSIVE" => {
                set_i64_or_string(attributes, "pcgen_skillcost_exclusive", value)
            }
            "SPELLBASECONCENTRATION" => {
                attributes.insert(
                    "pcgen_spellbaseconcentration".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SCORE" => set_i64_or_string(attributes, "pcgen_score", value),
            "XPAWARD" => {
                attributes.insert("pcgen_xpaward".to_string(), Value::String(value.clone()));
            }
            "STATINPUT" => {
                attributes.insert("pcgen_statinput".to_string(), Value::String(value.clone()));
            }
            "PLUSCOST" => {
                attributes.insert("pcgen_pluscost".to_string(), parse_pipe_series(value));
            }
            "RESIZABLEEQUIPTYPE" => {
                attributes.insert(
                    "pcgen_resizableequiptype".to_string(),
                    parse_pipe_series(value),
                );
            }
            "METHOD" => {
                attributes.insert("pcgen_method".to_string(), Value::String(value.clone()));
            }
            "LEVELMSG" => {
                attributes.insert("pcgen_levelmsg".to_string(), Value::String(value.clone()));
            }
            "SHORTRANGE" => set_i64_or_string(attributes, "pcgen_shortrange", value),
            "RANGEPENALTY" => set_i64_or_string(attributes, "pcgen_rangepenalty", value),
            "SQUARESIZE" => set_i64_or_string(attributes, "pcgen_squaresize", value),
            "SKILLMULTIPLIER" => {
                attributes.insert(
                    "pcgen_skillmultiplier".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SPELLBASEDC" => {
                attributes.insert(
                    "pcgen_spellbasedc".to_string(),
                    Value::String(value.clone()),
                );
            }
            "WEAPONNONPROFPENALTY" => {
                set_i64_or_string(attributes, "pcgen_weaponnonprofpenalty", value)
            }
            "WEAPONREACH" => {
                attributes.insert(
                    "pcgen_weaponreach".to_string(),
                    Value::String(value.clone()),
                );
            }
            "CHARACTERTYPE" => {
                attributes.insert("pcgen_charactertype".to_string(), parse_pipe_series(value));
            }
            "CRTHRESHOLD" => {
                attributes.insert(
                    "pcgen_crthreshold".to_string(),
                    Value::String(value.clone()),
                );
            }
            "CRSTEPS" => {
                attributes.insert("pcgen_crsteps".to_string(), Value::String(value.clone()));
            }
            "MONSTERROLES" => {
                attributes.insert("pcgen_monsterroles".to_string(), parse_pipe_series(value));
            }
            "MONSTERROLEDEFAULT" => {
                attributes.insert(
                    "pcgen_monsterroledefault".to_string(),
                    Value::String(value.clone()),
                );
            }
            "XPTABLE" => {
                attributes.insert("pcgen_xptable".to_string(), Value::String(value.clone()));
            }
            "EQSIZEPENALTY" => {
                attributes.insert(
                    "pcgen_eqsizepenalty".to_string(),
                    Value::String(value.clone()),
                );
            }
            "DEFAULTUNITSET" => {
                attributes.insert(
                    "pcgen_defaultunitset".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ALLOWEDMODES" => {
                attributes.insert(
                    "pcgen_allowedmodes".to_string(),
                    Value::String(value.clone()),
                );
            }
            "BABMAXATT" => set_i64_or_string(attributes, "pcgen_babmaxatt", value),
            "BABMINVAL" => set_i64_or_string(attributes, "pcgen_babminval", value),
            "BABATTCYC" => set_i64_or_string(attributes, "pcgen_babattcyc", value),
            "ACNAME" => {
                attributes.insert("pcgen_acname".to_string(), Value::String(value.clone()));
            }
            "DOMAINFEATURE" => {
                attributes.insert(
                    "pcgen_domainfeature".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "LOADMULT" => set_i64_or_string(attributes, "pcgen_loadmult", value),
            "NUMSLOTS" => {
                attributes.insert("pcgen_numslots".to_string(), Value::String(value.clone()));
            }
            "HEAD" => set_i64_or_string(attributes, "pcgen_headslots", value),
            "DISTANCEUNIT" => {
                attributes.insert(
                    "pcgen_distanceunit".to_string(),
                    Value::String(value.clone()),
                );
            }
            "DISTANCEFACTOR" => {
                attributes.insert(
                    "pcgen_distancefactor".to_string(),
                    Value::String(value.clone()),
                );
            }
            "DISTANCEPATTERN" => {
                attributes.insert(
                    "pcgen_distancepattern".to_string(),
                    Value::String(value.clone()),
                );
            }
            "HEIGHTUNIT" => {
                attributes.insert("pcgen_heightunit".to_string(), Value::String(value.clone()));
            }
            "HEIGHTFACTOR" => {
                attributes.insert(
                    "pcgen_heightfactor".to_string(),
                    Value::String(value.clone()),
                );
            }
            "HEIGHTPATTERN" => {
                attributes.insert(
                    "pcgen_heightpattern".to_string(),
                    Value::String(value.clone()),
                );
            }
            "WEIGHTUNIT" => {
                attributes.insert("pcgen_weightunit".to_string(), Value::String(value.clone()));
            }
            "WEIGHTFACTOR" => {
                attributes.insert(
                    "pcgen_weightfactor".to_string(),
                    Value::String(value.clone()),
                );
            }
            "WEIGHTPATTERN" => {
                attributes.insert(
                    "pcgen_weightpattern".to_string(),
                    Value::String(value.clone()),
                );
            }
            "TOTALCOST" => {
                attributes.insert("pcgen_totalcost".to_string(), Value::String(value.clone()));
            }
            "EQUIPMOD" => {
                append_value_attr(
                    attributes,
                    "pcgen_equipmod_catalog",
                    parse_pipe_series(value),
                );
            }
            "LANGUAGE" => {
                append_value_attr(
                    attributes,
                    "pcgen_language_catalog",
                    parse_pipe_series(value),
                );
            }
            "WEAPONPROF" => {
                append_value_attr(
                    attributes,
                    "pcgen_weaponprof_catalog",
                    parse_pipe_series(value),
                );
            }
            "ARMORPROF" => {
                append_value_attr(
                    attributes,
                    "pcgen_armorprof_catalog",
                    parse_pipe_series(value),
                );
            }
            "SHIELDPROF" => {
                append_value_attr(
                    attributes,
                    "pcgen_shieldprof_catalog",
                    parse_pipe_series(value),
                );
            }
            "DEITY" => {
                attributes.insert("deity".to_string(), parse_pipe_series(value));
            }
            "SYMBOL" => {
                attributes.insert("pcgen_symbol".to_string(), Value::String(value.clone()));
            }
            "LANGAUTO" => {
                append_value_attr(attributes, "pcgen_langauto", parse_pipe_series(value));
            }
            "GRANT" => {
                append_value_attr(attributes, "pcgen_grant", parse_pipe_series(value));
            }
            "CRFORMULA" => {
                attributes.insert("cr_formula".to_string(), Value::String(value.clone()));
            }
            "ISMONSTER" => {
                attributes.insert("is_monster".to_string(), parse_yes_no_or_string(value));
            }
            "XPPENALTY" => {
                attributes.insert("xp_penalty".to_string(), parse_yes_no_or_string(value));
            }
            "FREE" => {
                attributes.insert("pcgen_free".to_string(), parse_yes_no_or_string(value));
            }
            "SELECTION" => append_string_attr(attributes, "pcgen_selection", value),
            "VARIANTS" => append_string_attr(attributes, "pcgen_variants", value),
            "SITUATION" => append_string_attr(attributes, "pcgen_situations", value),
            "USEUNTRAINED" => {
                attributes.insert(
                    "pcgen_useuntrained".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "EXCLUSIVE" => {
                attributes.insert("pcgen_exclusive".to_string(), parse_yes_no_or_string(value));
            }
            "KEYSTAT" => {
                attributes.insert("pcgen_keystat".to_string(), Value::String(value.clone()));
            }
            "SIZE" => {
                attributes.insert("size".to_string(), Value::String(value.clone()));
            }
            "FACE" => {
                attributes.insert("pcgen_face".to_string(), Value::String(value.clone()));
            }
            "VISION" => append_string_attr(attributes, "vision", value),
            "LEGS" => {
                set_i64_or_string(attributes, "pcgen_legs", value);
            }
            "HANDS" => {
                set_i64_or_string(attributes, "pcgen_hands", value);
            }
            "TORSO" => {
                set_i64_or_string(attributes, "pcgen_torsoslots", value);
            }
            "SHIELD" => {
                set_i64_or_string(attributes, "pcgen_shieldslots", value);
            }
            "DR" => {
                attributes.insert("pcgen_dr".to_string(), Value::String(value.clone()));
            }
            "SR" => {
                attributes.insert("pcgen_sr".to_string(), Value::String(value.clone()));
            }
            "CR" => {
                attributes.insert("cr".to_string(), Value::String(value.clone()));
            }
            "CRMOD" => {
                if value.contains('|') {
                    append_value_attr(attributes, "pcgen_crmod", parse_crmod_definition(value));
                } else {
                    set_i64_or_string(attributes, "pcgen_crmod", value);
                }
            }
            "CRMODPRIORITY" => set_i64_or_string(attributes, "pcgen_crmodpriority", value),
            "REGION" => {
                attributes.insert("region".to_string(), Value::String(value.clone()));
            }
            "ROLE" => {
                attributes.insert("pcgen_role".to_string(), Value::String(value.clone()));
            }
            "NUMBER" => {
                attributes.insert("pcgen_number".to_string(), Value::String(value.clone()));
            }
            "CONTEXT" => {
                attributes.insert("pcgen_context".to_string(), Value::String(value.clone()));
            }
            "UP" => {
                attributes.insert("pcgen_up".to_string(), Value::String(value.clone()));
            }
            "DOWN" => {
                attributes.insert("pcgen_down".to_string(), Value::String(value.clone()));
            }
            "WIELD" => {
                attributes.insert("pcgen_wield".to_string(), Value::String(value.clone()));
            }
            "EDR" => {
                attributes.insert("pcgen_edr".to_string(), Value::String(value.clone()));
            }
            "SPELLFAILURE" => {
                attributes.insert(
                    "pcgen_spellfailure".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ACCHECK" => {
                attributes.insert("pcgen_accheck".to_string(), Value::String(value.clone()));
            }
            "ACHECK" => {
                attributes.insert("pcgen_accheck".to_string(), Value::String(value.clone()));
            }
            "MAXDEX" => {
                attributes.insert("pcgen_maxdex".to_string(), Value::String(value.clone()));
            }
            "SLOTS" => {
                attributes.insert("pcgen_slots".to_string(), Value::String(value.clone()));
            }
            "PART" => append_string_attr(attributes, "pcgen_part", value),
            "DAMAGE" => {
                attributes.insert("pcgen_damage".to_string(), Value::String(value.clone()));
            }
            "ALTDAMAGE" => {
                attributes.insert("pcgen_altdamage".to_string(), Value::String(value.clone()));
            }
            "ALTTYPE" => {
                attributes.insert("pcgen_alttype".to_string(), Value::String(value.clone()));
            }
            "CRITMULT" => {
                attributes.insert("pcgen_critmult".to_string(), Value::String(value.clone()));
            }
            "CRITRANGE" => {
                attributes.insert("pcgen_critrange".to_string(), Value::String(value.clone()));
            }
            "FUMBLERANGE" => {
                attributes.insert(
                    "pcgen_fumblerange".to_string(),
                    Value::String(value.clone()),
                );
            }
            "RATEOFFIRE" => {
                attributes.insert("pcgen_rateoffire".to_string(), Value::String(value.clone()));
            }
            "REACH" => {
                attributes.insert("pcgen_reach".to_string(), Value::String(value.clone()));
            }
            "REACHMULT" => {
                attributes.insert("pcgen_reachmult".to_string(), Value::String(value.clone()));
            }
            "ALTCRITMULT" => {
                attributes.insert(
                    "pcgen_altcritmult".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ALTCRITRANGE" => {
                attributes.insert(
                    "pcgen_altcritrange".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ALTEQMOD" => {
                attributes.insert("pcgen_alteqmod".to_string(), Value::String(value.clone()));
            }
            "PLUS" => set_i64_or_string(attributes, "pcgen_plus", value),
            "ITYPE" => {
                attributes.insert("pcgen_itype".to_string(), Value::String(value.clone()));
            }
            "NAMEOPT" => {
                attributes.insert("pcgen_nameopt".to_string(), Value::String(value.clone()));
            }
            "FORMATCAT" => {
                attributes.insert("pcgen_formatcat".to_string(), Value::String(value.clone()));
            }
            "ASSIGNTOALL" => {
                attributes.insert(
                    "pcgen_assigntoall".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "SWITCH" => append_string_attr(attributes, "pcgen_switch", value),
            "SIZEDIFF" => set_i64_or_string(attributes, "pcgen_sizediff", value),
            "FINESSABLE" => {
                attributes.insert(
                    "pcgen_finessable".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "PROFICIENCY" => {
                attributes.insert("pcgen_proficiency".to_string(), parse_pipe_series(value));
            }
            "ARMORTYPE" => {
                append_value_attr(attributes, "pcgen_armortype", parse_transition_pair(value));
            }
            "REPLACES" => {
                attributes.insert("pcgen_replaces".to_string(), Value::String(value.clone()));
            }
            "UNENCUMBEREDMOVE" => {
                append_value_attr(
                    attributes,
                    "pcgen_unencumberedmove",
                    parse_pipe_series(value),
                );
            }
            "ISMATURE" => {
                attributes.insert("pcgen_ismature".to_string(), parse_yes_no_or_string(value));
            }
            "CONTAINS" => {
                attributes.insert("pcgen_contains".to_string(), Value::String(value.clone()));
            }
            "BASEQTY" => {
                attributes.insert("pcgen_baseqty".to_string(), Value::String(value.clone()));
            }
            "MODS" => {
                attributes.insert("pcgen_mods".to_string(), Value::String(value.clone()));
            }
            "ICON" => {
                attributes.insert("pcgen_icon".to_string(), Value::String(value.clone()));
            }
            "REMOVE" => append_string_attr(attributes, "pcgen_remove", value),
            "NUMPAGES" => {
                if let Ok(num) = value.trim().parse::<i64>() {
                    attributes.insert("pcgen_numpages".to_string(), json!(num));
                } else {
                    attributes.insert("pcgen_numpages".to_string(), Value::String(value.clone()));
                }
            }
            "QUALITY" => append_string_attr(attributes, "pcgen_qualities", value),
            "SPROP" => sprop_values.push(Value::String(value.clone())),
            "PAGEUSAGE" => page_usage_values.push(Value::String(value.clone())),
            "FORTIFICATION" => {
                attributes.insert(
                    "pcgen_fortification".to_string(),
                    Value::String(value.clone()),
                );
            }
            "HEALING" => {
                attributes.insert("pcgen_healing".to_string(), Value::String(value.clone()));
            }
            "CHARGES" => append_string_attr(attributes, "pcgen_charges", value),
            "FACT" => facts.push(parse_fact(value)),
            "FACTSET" => factsets.push(parse_fact(value)),
            "EQMOD" => equipment_modifiers.push(parse_pipe_series(value)),
            "CLASSES" => class_lists.push(parse_pipe_series(value)),
            "SPELLS" => spell_blocks.push(parse_spells(value)),
            "SPELLKNOWN" => append_string_attr(attributes, "pcgen_spellknown", value),
            "MOVE" => append_string_attr(attributes, "move", value),
            "NATURALATTACKS" => append_string_attr(attributes, "natural_attacks", value),
            "MODIFY" => {
                // Parse MODIFY expressions: VarName|Operation|Value
                match parse_modify(value) {
                    Ok(expr) => {
                        append_string_attr(attributes, "pcgen_modify_variable", &expr.variable);
                        append_string_attr(
                            attributes,
                            "pcgen_modify_operation",
                            &expr.operation.to_string(),
                        );
                        append_string_attr(attributes, "pcgen_modify_value", &expr.value);
                    }
                    Err(_) => {
                        // Fallback: store raw MODIFY value if parsing fails
                        append_string_attr(attributes, "pcgen_modify", value);
                    }
                }
            }
            "MODIFYOTHER" => append_string_attr(attributes, "pcgen_modifyother", value),
            // system.rs — codeControl.lst tokens
            "STATMODSAVE" => {
                attributes.insert(
                    "pcgen_statmodsave".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ALTHP" => {
                attributes.insert("pcgen_althp".to_string(), Value::String(value.clone()));
            }
            "HIDDENEQUIPTYPES" => {
                attributes.insert(
                    "pcgen_hiddenequiptypes".to_string(),
                    Value::String(value.clone()),
                );
            }
            "HIDDENFEATTYPES" => {
                attributes.insert(
                    "pcgen_hiddenfeattypes".to_string(),
                    Value::String(value.clone()),
                );
            }
            // system.rs — ROLLMETHOD sub-token
            "EXPRESSION" => {
                set_i64_or_string(attributes, "pcgen_rollmethod_expression", value);
            }
            // equipment.rs — Talislanta alternate critical hit multiplier
            "ALTCRITICAL" => {
                attributes.insert(
                    "pcgen_altcritical".to_string(),
                    Value::String(value.clone()),
                );
            }
            // class.rs — skill list assignment
            "SKILLLIST" => {
                attributes.insert("pcgen_skilllist".to_string(), Value::String(value.clone()));
            }
            // pcc.rs — campaign help file path
            "HELP" => {
                attributes.insert("pcgen_help".to_string(), Value::String(value.clone()));
            }
            // template.rs — non-party points
            "NONPP" => {
                set_i64_or_string(attributes, "pcgen_nonpp", value);
            }
            _ => {}
        }
    }

    if !facts.is_empty() {}
    if !equipment_modifiers.is_empty() {
        attributes.insert(
            "pcgen_eqmods".to_string(),
            Value::Array(equipment_modifiers),
        );
    }
    if !facts.is_empty() {
        attributes.insert("pcgen_facts".to_string(), Value::Array(facts));
    }
    if !class_lists.is_empty() {
        attributes.insert("pcgen_classes".to_string(), Value::Array(class_lists));
    }
    if !spell_blocks.is_empty() {
        attributes.insert("pcgen_spells".to_string(), Value::Array(spell_blocks));
    }
    if !sprop_values.is_empty() {
        attributes.insert("pcgen_sprop".to_string(), Value::Array(sprop_values));
    }
    if !page_usage_values.is_empty() {
        attributes.insert(
            "pcgen_pageusage".to_string(),
            Value::Array(page_usage_values),
        );
    }
    if !factsets.is_empty() {
        attributes.insert("pcgen_factsets".to_string(), Value::Array(factsets));
    }

    project_dual_name_fields(head_name, attributes);
}

pub(crate) fn project_decl_token_value(
    decl_token: &str,
    decl_value: &str,
    attributes: &mut IndexMap<String, Value>,
) {
    match decl_token.to_ascii_uppercase().as_str() {
        "ABILITY" => append_string_attr(attributes, "pcgen_abilities", decl_value),
        "ABILITYCATEGORY" => append_string_attr(attributes, "pcgen_abilitycategories", decl_value),
        "FEAT" => append_string_attr(attributes, "pcgen_feats", decl_value),
        "EQUIPMENT" => append_string_attr(attributes, "equipment", decl_value),
        "SPELL" => append_string_attr(attributes, "pcgen_spells", decl_value),
        "VARIABLE" => append_string_attr(attributes, "pcgen_variable_catalog", decl_value),
        "LICENSE" => append_string_attr(attributes, "license", decl_value),
        "INFOTEXT" => append_string_attr(attributes, "infotext", decl_value),
        "FORWARDREF" => append_string_attr(attributes, "pcgen_forwardref", decl_value),
        "HIDETYPE" => append_string_attr(attributes, "pcgen_hidetype", decl_value),
        "URL" => append_string_attr(attributes, "url", decl_value),
        "LOCAL" => {
            attributes.insert(
                "pcgen_local".to_string(),
                parse_local_definition(decl_value),
            );
        }
        "GLOBAL" => {
            attributes.insert(
                "pcgen_global".to_string(),
                parse_global_definition(decl_value),
            );
        }
        "FACTSETDEF" => {
            attributes.insert(
                "pcgen_factsetdef".to_string(),
                parse_factsetdef_definition(decl_value),
            );
        }
        "FUNCTION" => {
            attributes.insert(
                "pcgen_function".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "DYNAMICSCOPE" => {
            attributes.insert(
                "pcgen_dynamicscope".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "EQUIPMENT.PART" => {
            attributes.insert(
                "pcgen_equipment_part".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SPELLRANGE" => {
            attributes.insert(
                "pcgen_spellrange".to_string(),
                parse_spellrange_definition(decl_value),
            );
        }
        "BASEAGEADD" => {
            attributes.insert(
                "base_age_add".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "OUTPUTSHEET" => {
            attributes.insert(
                "pcgen_outputsheet".to_string(),
                parse_outputsheet_definition(decl_value),
            );
        }
        "INFOSHEET" => {
            attributes.insert(
                "pcgen_infosheet".to_string(),
                parse_infosheet_definition(decl_value),
            );
        }
        "UNITSET" => {
            attributes.insert(
                "pcgen_unitset".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "ALIGNMENTFEATURE" => {
            attributes.insert(
                "pcgen_alignmentfeature".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "CURRENCYUNITABBREV" => {
            attributes.insert(
                "pcgen_currencyunitabbrev".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "MENUENTRY" => {
            attributes.insert(
                "pcgen_menuentry".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "DISPLAYORDER" => {
            set_i64_or_string(attributes, "pcgen_displayorder", decl_value);
        }
        "DIESIZES" => {
            attributes.insert(
                "pcgen_diesizes".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "LEVELMSG" => {
            attributes.insert(
                "pcgen_levelmsg".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SHORTRANGE" => set_i64_or_string(attributes, "pcgen_shortrange", decl_value),
        "RANGEPENALTY" => set_i64_or_string(attributes, "pcgen_rangepenalty", decl_value),
        "SQUARESIZE" => set_i64_or_string(attributes, "pcgen_squaresize", decl_value),
        "SKILLMULTIPLIER" => {
            attributes.insert(
                "pcgen_skillmultiplier".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SPELLBASEDC" => {
            attributes.insert(
                "pcgen_spellbasedc".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "WEAPONNONPROFPENALTY" => {
            set_i64_or_string(attributes, "pcgen_weaponnonprofpenalty", decl_value)
        }
        "WEAPONREACH" => {
            attributes.insert(
                "pcgen_weaponreach".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CHARACTERTYPE" => {
            attributes.insert(
                "pcgen_charactertype".to_string(),
                parse_pipe_series(decl_value),
            );
        }
        "CRTHRESHOLD" => {
            attributes.insert(
                "pcgen_crthreshold".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CRSTEPS" => {
            attributes.insert(
                "pcgen_crsteps".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "MONSTERROLES" => {
            attributes.insert(
                "pcgen_monsterroles".to_string(),
                parse_pipe_series(decl_value),
            );
        }
        "MONSTERROLEDEFAULT" => {
            attributes.insert(
                "pcgen_monsterroledefault".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "XPTABLE" => {
            attributes.insert(
                "pcgen_xptable".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "PCGVERSION" => {
            attributes.insert(
                "pcgen_pcgversion".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "PURCHASEPOINTS" => {
            attributes.insert(
                "pcgen_purchasepoints".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "POOLPOINTS" => set_i64_or_string(attributes, "pcgen_poolpoints", decl_value),
        "POOLPOINTSAVAIL" => set_i64_or_string(attributes, "pcgen_poolpointsavail", decl_value),
        "TABLABEL" => set_i64_or_string(attributes, "pcgen_tablabel", decl_value),
        "AUTOSPELLS" => {
            attributes.insert(
                "pcgen_autospells".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "USEHIGHERKNOWN" => {
            attributes.insert(
                "pcgen_usehigherknown".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "USEHIGHERPREPPED" => {
            attributes.insert(
                "pcgen_usehigherprepped".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "LOADCOMPANIONS" => {
            attributes.insert(
                "pcgen_loadcompanions".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "USETEMPMODS" => {
            attributes.insert(
                "pcgen_usetempmods".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "SKILLSOUTPUTORDER" => set_i64_or_string(attributes, "pcgen_skillsoutputorder", decl_value),
        "SKILLFILTER" => set_i64_or_string(attributes, "pcgen_skillfilter", decl_value),
        "IGNORECOST" => {
            attributes.insert(
                "pcgen_ignorecost".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "ALLOWDEBT" => {
            attributes.insert(
                "pcgen_allowdebt".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "AUTORESIZEGEAR" => {
            attributes.insert(
                "pcgen_autoresizegear".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "CHARACTERNAME" => {
            attributes.insert(
                "character_name".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "PLAYERNAME" => {
            attributes.insert(
                "pcgen_playername".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "HEIGHT" => set_i64_or_string(attributes, "height", decl_value),
        "WEIGHT" => set_i64_or_string(attributes, "weight", decl_value),
        "AGE" => set_i64_or_string(attributes, "age", decl_value),
        "HANDED" => {
            attributes.insert(
                "pcgen_handed".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "EQSIZEPENALTY" => {
            attributes.insert(
                "pcgen_eqsizepenalty".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "STARTTABLE" => {
            attributes.insert(
                "pcgen_starttable".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "ENDTABLE" => {
            attributes.insert(
                "pcgen_endtable".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "MOVEMENT" => {
            attributes.insert(
                "pcgen_movement".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "DEFAULTDATASET" => {
            attributes.insert(
                "pcgen_defaultdataset".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "GAMEMODEKEY" => {
            attributes.insert(
                "pcgen_gamemodekey".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "LANGAUTO" => {
            attributes.insert("pcgen_langauto".to_string(), parse_pipe_series(decl_value));
        }
        "RESIZABLEEQUIPTYPE" => {
            attributes.insert(
                "pcgen_resizableequiptype".to_string(),
                parse_pipe_series(decl_value),
            );
        }
        "SKILLCOST_CROSSCLASS" => {
            set_i64_or_string(attributes, "pcgen_skillcost_crossclass", decl_value)
        }
        "SKILLCOST_CLASS" => set_i64_or_string(attributes, "pcgen_skillcost_class", decl_value),
        "SKILLCOST_EXCLUSIVE" => {
            set_i64_or_string(attributes, "pcgen_skillcost_exclusive", decl_value)
        }
        "SPELLBASECONCENTRATION" => {
            attributes.insert(
                "pcgen_spellbaseconcentration".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "XPAWARD" => {
            attributes.insert(
                "pcgen_xpaward".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "STATINPUT" => {
            attributes.insert(
                "pcgen_statinput".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "MAXNONEPICLEVEL" => set_i64_or_string(attributes, "pcgen_maxnonepiclevel", decl_value),
        "PLUSCOST" => {
            attributes.insert("pcgen_pluscost".to_string(), parse_pipe_series(decl_value));
        }
        "DEFAULTUNITSET" => {
            attributes.insert(
                "pcgen_defaultunitset".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "ALLOWEDMODES" => {
            attributes.insert(
                "pcgen_allowedmodes".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "BABMAXATT" => set_i64_or_string(attributes, "pcgen_babmaxatt", decl_value),
        "BABMINVAL" => set_i64_or_string(attributes, "pcgen_babminval", decl_value),
        "BABATTCYC" => set_i64_or_string(attributes, "pcgen_babattcyc", decl_value),
        "ACNAME" => {
            attributes.insert(
                "pcgen_acname".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "DOMAINFEATURE" => {
            attributes.insert(
                "pcgen_domainfeature".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "LOADMULT" => set_i64_or_string(attributes, "pcgen_loadmult", decl_value),
        "NUMSLOTS" => {
            attributes.insert(
                "pcgen_numslots".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // PCG character bio tokens
        "TABNAME" => {
            attributes.insert(
                "pcgen_tabname".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SKINCOLOR" => {
            attributes.insert(
                "skin_color".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "EYECOLOR" => {
            attributes.insert(
                "eye_color".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "HAIRCOLOR" => {
            attributes.insert(
                "hair_color".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "HAIRSTYLE" => {
            attributes.insert(
                "pcgen_hairstyle".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CITY" => {
            attributes.insert("city".to_string(), Value::String(decl_value.to_string()));
        }
        "BIRTHDAY" => {
            attributes.insert(
                "birthday".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "BIRTHPLACE" => {
            attributes.insert(
                "birthplace".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "PERSONALITYTRAIT1" => {
            attributes.insert(
                "personality_trait_1".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "PERSONALITYTRAIT2" => {
            attributes.insert(
                "personality_trait_2".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SPEECHPATTERN" => {
            attributes.insert(
                "speech_pattern".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "PHOBIAS" => {
            attributes.insert("phobias".to_string(), Value::String(decl_value.to_string()));
        }
        "INTERESTS" => {
            attributes.insert(
                "interests".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CATCHPHRASE" => {
            attributes.insert(
                "catchphrase".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "PORTRAIT" => {
            attributes.insert(
                "portrait".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // PCG character progression tokens
        "EXPERIENCE" => set_i64_or_string(attributes, "pcgen_experience", decl_value),
        "EXPERIENCETABLE" => {
            attributes.insert(
                "pcgen_experiencetable".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "MONEY" => {
            attributes.insert(
                "pcgen_money".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // PCG character description blocks
        "CHARACTERBIO" => {
            attributes.insert(
                "character_bio".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CHARACTERDESC" => {
            attributes.insert(
                "character_desc".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CHARACTERCOMP" => {
            attributes.insert(
                "character_comp".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CHARACTERASSET" => {
            attributes.insert(
                "character_asset".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CHARACTERMAGIC" => {
            attributes.insert(
                "character_magic".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CHARACTERDMNOTES" => {
            attributes.insert(
                "character_dm_notes".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // PCG misc standalone tokens
        "VERSION" => {
            attributes.insert(
                "pcgen_version".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "FEATPOOL" => {
            attributes.insert(
                "pcgen_featpool".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CALCEQUIPSET" => {
            attributes.insert(
                "pcgen_calcequipset".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SUPPRESSBIOFIELDS" => {
            attributes.insert(
                "pcgen_suppressbiofields".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // PCG complex records: store the head (name) value
        "USERPOOL" => {
            attributes.insert(
                "pcgen_userpool_name".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "EQUIPSET" => {
            attributes.insert(
                "pcgen_equipset_name".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "EQUIPNAME" => {
            attributes.insert(
                "pcgen_equipname_item".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CLASSABILITIESLEVEL" => {
            // Store the raw "ClassName=Level" value for full round-trip fidelity.
            attributes.insert(
                "pcgen_cal_classname_level".to_string(),
                Value::String(decl_value.to_string()),
            );
            // Also extract the class name and level as separate relationship/field attributes.
            // Format: CLASSABILITIESLEVEL:ClassName=LevelNumber
            // `pcgen_for_class` is the relationship back to the parent CLASS record.
            if let Some(eq_pos) = decl_value.find('=') {
                let class_name = decl_value[..eq_pos].trim();
                let level_str = decl_value[eq_pos + 1..].trim();
                if !class_name.is_empty() {
                    attributes.insert(
                        "pcgen_for_class".to_string(),
                        Value::String(class_name.to_string()),
                    );
                }
                if let Ok(level) = level_str.parse::<i64>() {
                    attributes.insert("class_level".to_string(), json!(level));
                } else if !level_str.is_empty() {
                    attributes.insert(
                        "class_level".to_string(),
                        Value::String(level_str.to_string()),
                    );
                }
            }
        }
        // Variable schemas — head token stores the variable path
        "CHANNEL" => {
            attributes.insert(
                "pcgen_channel".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // System schemas — head token is the config value
        "STATMODSAVE" => {
            attributes.insert(
                "pcgen_statmodsave".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "ALTHP" => {
            attributes.insert(
                "pcgen_althp".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "HIDDENEQUIPTYPES" => {
            attributes.insert(
                "pcgen_hiddenequiptypes".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "HIDDENFEATTYPES" => {
            attributes.insert(
                "pcgen_hiddenfeattypes".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // WEAPONPROF head value: catalog path in PCC ("file.lst") or bracket group in PCG
        // ("[WEAPON:Longsword|WEAPON:Dagger|...]"). Store structured when bracket-delimited.
        "WEAPONPROF" => {
            attributes.insert(
                "pcgen_weaponprof_catalog".to_string(),
                parse_bracket_group(decl_value),
            );
        }
        // PCG standalone note — head value is the note text
        "NOTE" => {
            attributes.insert("note".to_string(), Value::String(decl_value.to_string()));
        }
        // PCG spell record — head value is the spell name
        "SPELLNAME" => {
            attributes.insert(
                "pcgen_spellname".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // PCG deity record — head value is the deity name
        "DEITY" => {
            attributes.insert(
                "deity_name".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        // PCG domain record — head value is the domain name
        "DOMAIN" => {
            attributes.insert(
                "pcgen_domain_name".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        _ => {}
    }
}

fn project_dual_name_fields(head_name: &str, attributes: &mut IndexMap<String, Value>) {
    let Some(name_is_pi_raw) = attributes.get("pcgen_nameispi") else {
        return;
    };

    let is_pi_name = bool_like_from_value(name_is_pi_raw).unwrap_or(false);

    if is_pi_name {
        attributes.insert(
            "pcgen_name_pi".to_string(),
            Value::String(head_name.to_string()),
        );

        if let Some(open_name) = open_name_candidate(attributes, head_name) {
            attributes.insert("pcgen_name_open".to_string(), Value::String(open_name));
        }
        return;
    }

    attributes.insert(
        "pcgen_name_open".to_string(),
        Value::String(head_name.to_string()),
    );
}

fn open_name_candidate(attributes: &IndexMap<String, Value>, head_name: &str) -> Option<String> {
    ["outputname", "key"]
        .iter()
        .filter_map(|key| attributes.get(*key).and_then(Value::as_str))
        .map(str::trim)
        .find(|value| !value.is_empty() && !value.eq_ignore_ascii_case(head_name))
        .map(ToString::to_string)
}

fn bool_like_from_value(value: &Value) -> Option<bool> {
    match value {
        Value::Bool(b) => Some(*b),
        Value::String(s) => match s.trim().to_ascii_uppercase().as_str() {
            "YES" | "Y" | "TRUE" => Some(true),
            "NO" | "N" | "FALSE" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn append_string_attr(attributes: &mut IndexMap<String, Value>, key: &str, value: &str) {
    match attributes.get_mut(key) {
        Some(Value::Array(existing)) => existing.push(Value::String(value.to_string())),
        Some(Value::String(existing)) => {
            let prior = existing.clone();
            attributes.insert(
                key.to_string(),
                Value::Array(vec![Value::String(prior), Value::String(value.to_string())]),
            );
        }
        _ => {
            attributes.insert(
                key.to_string(),
                Value::Array(vec![Value::String(value.to_string())]),
            );
        }
    }
}

fn append_value_attr(attributes: &mut IndexMap<String, Value>, key: &str, value: Value) {
    match attributes.get_mut(key) {
        Some(Value::Array(existing)) => existing.push(value),
        Some(existing) => {
            let prior = existing.clone();
            attributes.insert(key.to_string(), Value::Array(vec![prior, value]));
        }
        None => {
            attributes.insert(key.to_string(), Value::Array(vec![value]));
        }
    }
}

fn set_i64_or_string(attributes: &mut IndexMap<String, Value>, key: &str, value: &str) {
    if let Ok(parsed) = value.trim().parse::<i64>() {
        attributes.insert(key.to_string(), json!(parsed));
    } else {
        attributes.insert(key.to_string(), Value::String(value.to_string()));
    }
}

fn parse_yes_no_or_string(value: &str) -> Value {
    match value.trim().to_ascii_uppercase().as_str() {
        "YES" | "Y" | "TRUE" => Value::Bool(true),
        "NO" | "N" | "FALSE" => Value::Bool(false),
        _ => Value::String(value.to_string()),
    }
}

fn parse_fact(input: &str) -> Value {
    let mut parts = input.splitn(2, '|');
    let fact_key = parts.next().unwrap_or_default().trim();
    let fact_value = parts.next().unwrap_or_default().trim();

    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));
    if !fact_key.is_empty() {
        out.insert("key".to_string(), Value::String(fact_key.to_string()));
    }
    if !fact_value.is_empty() {
        out.insert("value".to_string(), Value::String(fact_value.to_string()));
    }
    Value::Object(out)
}

fn parse_pipe_series(input: &str) -> Value {
    let parts: Vec<Value> = input
        .split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| Value::String(part.to_string()))
        .collect();

    json!({
        "raw": input,
        "parts": parts,
    })
}

fn parse_spells(input: &str) -> Value {
    let parts: Vec<&str> = input
        .split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();

    let mut assignments = Map::new();
    let mut spells = Vec::new();
    let mut mode = None;

    for (index, part) in parts.iter().enumerate() {
        if index == 0 && !part.contains('=') {
            mode = Some((*part).to_string());
            continue;
        }

        if let Some((key, value)) = part.split_once('=') {
            assignments.insert(
                key.trim().to_ascii_lowercase(),
                Value::String(value.trim().to_string()),
            );
        } else {
            spells.push(Value::String((*part).to_string()));
        }
    }

    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));
    if let Some(mode) = mode {
        out.insert("mode".to_string(), Value::String(mode));
    }
    if !assignments.is_empty() {
        out.insert("assignments".to_string(), Value::Object(assignments));
    }
    if !spells.is_empty() {
        out.insert("spells".to_string(), Value::Array(spells));
    }
    Value::Object(out)
}

fn parse_local_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let scope = parts.next().unwrap_or_default().trim();
    let binding = parts.next().unwrap_or_default().trim();

    if !scope.is_empty() {
        out.insert("scope".to_string(), Value::String(scope.to_string()));
    }
    if !binding.is_empty() {
        out.insert("binding".to_string(), parse_variable_binding(binding));
    }

    Value::Object(out)
}

fn parse_global_definition(input: &str) -> Value {
    json!({
        "raw": input,
        "binding": parse_variable_binding(input),
    })
}

fn parse_factsetdef_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let subject = parts.next().unwrap_or_default().trim();
    let field = parts.next().unwrap_or_default().trim();

    if !subject.is_empty() {
        out.insert("subject".to_string(), Value::String(subject.to_string()));
    }
    if !field.is_empty() {
        out.insert("field".to_string(), Value::String(field.to_string()));
    }

    Value::Object(out)
}

fn parse_spellrange_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let range_name = parts.next().unwrap_or_default().trim();
    let formula = parts.next().unwrap_or_default().trim();

    if !range_name.is_empty() {
        out.insert("name".to_string(), Value::String(range_name.to_string()));
    }
    if !formula.is_empty() {
        out.insert("formula".to_string(), Value::String(formula.to_string()));
    }

    Value::Object(out)
}

fn parse_transition_pair(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let from = parts.next().unwrap_or_default().trim();
    let to = parts.next().unwrap_or_default().trim();

    if !from.is_empty() {
        out.insert("from".to_string(), Value::String(from.to_string()));
    }
    if !to.is_empty() {
        out.insert("to".to_string(), Value::String(to.to_string()));
    }

    Value::Object(out)
}

fn parse_outputsheet_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let kind = parts.next().unwrap_or_default().trim();
    let path = parts.next().unwrap_or_default().trim();

    if !kind.is_empty() {
        out.insert("kind".to_string(), Value::String(kind.to_string()));
    }
    if !path.is_empty() {
        out.insert("path".to_string(), Value::String(path.to_string()));
    }

    Value::Object(out)
}

fn parse_infosheet_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let kind = parts.next().unwrap_or_default().trim();
    let path = parts.next().unwrap_or_default().trim();

    if !kind.is_empty() {
        out.insert("kind".to_string(), Value::String(kind.to_string()));
    }
    if !path.is_empty() {
        out.insert("path".to_string(), Value::String(path.to_string()));
    }

    Value::Object(out)
}

fn parse_crmod_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let scope = parts.next().unwrap_or_default().trim();
    let modifier = parts.next().unwrap_or_default().trim();

    if !scope.is_empty() {
        let class_types: Vec<Value> = scope
            .split('.')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(|part| Value::String(part.to_string()))
            .collect();
        if !class_types.is_empty() {
            out.insert("class_types".to_string(), Value::Array(class_types));
        }
    }

    if !modifier.is_empty() {
        if let Ok(num) = modifier.parse::<i64>() {
            out.insert("modifier".to_string(), json!(num));
        } else {
            out.insert("modifier".to_string(), Value::String(modifier.to_string()));
        }
    }

    Value::Object(out)
}

fn parse_variable_binding(input: &str) -> Value {
    let trimmed = input.trim();
    if let Some((kind, name)) = trimmed.split_once('=') {
        return json!({
            "raw": trimmed,
            "kind": kind.trim(),
            "name": name.trim(),
        });
    }

    json!({
        "raw": trimmed,
        "name": trimmed,
    })
}
