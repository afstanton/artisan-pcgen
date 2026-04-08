use indexmap::IndexMap;
use serde_json::{Map, Value, json};

use crate::ParsedClause;

pub(crate) fn project_clause_attributes(
    clauses: &[ParsedClause],
    attributes: &mut IndexMap<String, Value>,
) {
    let mut facts = Vec::new();
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
                attributes.insert("pcgen_type".to_string(), Value::String(value.clone()));
            }
            "SOURCE" => {
                attributes.insert("pcgen_source".to_string(), Value::String(value.clone()));
            }
            "SOURCELONG" => {
                attributes.insert("pcgen_source_long".to_string(), Value::String(value.clone()));
            }
            "SOURCESHORT" => {
                attributes.insert("pcgen_source_short".to_string(), Value::String(value.clone()));
            }
            "SOURCEWEB" => {
                attributes.insert("pcgen_source_web".to_string(), Value::String(value.clone()));
            }
            "SOURCEDATE" => {
                attributes.insert("pcgen_source_date".to_string(), Value::String(value.clone()));
            }
            "BASESTATSCORE" => set_i64_or_string(attributes, "pcgen_basestatscore", value),
            "STATRANGE" => set_i64_or_string(attributes, "pcgen_statrange", value),
            "ADDSPELLLEVEL" => set_i64_or_string(attributes, "pcgen_addspelllevel", value),
            "MULT" => {
                attributes.insert("pcgen_mult".to_string(), Value::String(value.clone()));
            }
            "STACK" => {
                attributes.insert("pcgen_stack".to_string(), Value::String(value.clone()));
            }
            "BENEFIT" => {
                attributes.insert("pcgen_benefit".to_string(), Value::String(value.clone()));
            }
            "ASPECT" => append_string_attr(attributes, "pcgen_aspects", value),
            "RACETYPE" => {
                attributes.insert("pcgen_racetype".to_string(), Value::String(value.clone()));
            }
            "RACESUBTYPE" => append_string_attr(attributes, "pcgen_racesubtype", value),
            "SUBRACE" => {
                attributes.insert("pcgen_subrace".to_string(), Value::String(value.clone()));
            }
            "FAVCLASS" => {
                attributes.insert("pcgen_favclass".to_string(), Value::String(value.clone()));
            }
            "FAVOREDCLASS" => {
                attributes.insert("pcgen_favoredclass".to_string(), Value::String(value.clone()));
            }
            "MONSTERCLASS" => {
                attributes.insert("pcgen_monsterclass".to_string(), Value::String(value.clone()));
            }
            "MONCSKILL" => append_string_attr(attributes, "pcgen_moncskill", value),
            "STARTFEATS" => set_i64_or_string(attributes, "pcgen_startfeats", value),
            "HITDICEADVANCEMENT" => {
                attributes.insert(
                    "pcgen_hitdiceadvancement".to_string(),
                    Value::String(value.clone()),
                );
            }
            "LEVELADJUSTMENT" => set_i64_or_string(attributes, "pcgen_leveladjustment", value),
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
            "HITDIE" => set_i64_or_string(attributes, "pcgen_hitdie", value),
            "ABB" => {
                attributes.insert("pcgen_abbreviation".to_string(), Value::String(value.clone()));
            }
            "STARTSKILLPTS" => set_i64_or_string(attributes, "pcgen_startskillpts", value),
            "LEVELSPERFEAT" => set_i64_or_string(attributes, "pcgen_levelsperfeat", value),
            "ATTACKCYCLE" => {
                attributes.insert("pcgen_attackcycle".to_string(), Value::String(value.clone()));
            }
            "SPELLTYPE" => {
                attributes.insert("pcgen_spelltype".to_string(), Value::String(value.clone()));
            }
            "BONUSSPELLSTAT" => {
                attributes.insert("pcgen_bonusspellstat".to_string(), Value::String(value.clone()));
            }
            "CAST" => append_string_attr(attributes, "pcgen_cast", value),
            "KNOWN" => append_string_attr(attributes, "pcgen_known", value),
            "KNOWNSPELLS" => {
                attributes.insert("pcgen_knownspells".to_string(), Value::String(value.clone()));
            }
            "SPECIALTYKNOWN" => {
                attributes.insert("pcgen_specialtyknown".to_string(), Value::String(value.clone()));
            }
            "SPELLLIST" => {
                attributes.insert("pcgen_spelllist".to_string(), Value::String(value.clone()));
            }
            "PROHIBITSPELL" => append_string_attr(attributes, "pcgen_prohibitspell", value),
            "KNOWNSPELLSFROMSPECIALTY" => {
                attributes.insert(
                    "pcgen_knownspellsfromspecialty".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ADDDOMAINS" => append_string_attr(attributes, "pcgen_adddomains", value),
            "DOMAIN" => {
                attributes.insert("pcgen_domains".to_string(), Value::String(value.clone()));
            }
            "ALLOWBASECLASS" => {
                attributes.insert("pcgen_allowbaseclass".to_string(), parse_yes_no_or_string(value));
            }
            "MODTOSKILLS" => {
                attributes.insert("pcgen_modtoskills".to_string(), parse_yes_no_or_string(value));
            }
            "MONSKILL" => set_i64_or_string(attributes, "pcgen_monskill", value),
            "MONNONSKILLHD" => set_i64_or_string(attributes, "pcgen_monnonskillhd", value),
            "WEAPONBONUS" => append_string_attr(attributes, "pcgen_weaponbonus", value),
            "VISIBLE" => {
                attributes.insert("pcgen_visible".to_string(), Value::String(value.clone()));
            }
            "REMOVABLE" => {
                attributes.insert("pcgen_removable".to_string(), Value::String(value.clone()));
            }
            "PROHIBITCOST" => set_i64_or_string(attributes, "pcgen_prohibitcost", value),
            "SUBCLASSLEVEL" => {
                attributes.insert("pcgen_subclasslevel".to_string(), Value::String(value.clone()));
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
            "CATEGORY" => {
                attributes.insert("pcgen_category".to_string(), Value::String(value.clone()));
            }
            "DESC" => {
                attributes.insert("pcgen_desc".to_string(), Value::String(value.clone()));
                attributes
                    .entry("description".to_string())
                    .or_insert_with(|| Value::String(value.clone()));
            }
            "KEY" => {
                attributes.insert("pcgen_key".to_string(), Value::String(value.clone()));
            }
            "RANK" => {
                if let Ok(rank) = value.trim().parse::<i64>() {
                    attributes.insert("pcgen_rank".to_string(), json!(rank));
                } else {
                    attributes.insert("pcgen_rank".to_string(), Value::String(value.clone()));
                }
            }
            "COST" => {
                attributes.insert("pcgen_cost".to_string(), Value::String(value.clone()));
            }
            "WT" => {
                attributes.insert("pcgen_weight".to_string(), Value::String(value.clone()));
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
                attributes.insert("pcgen_casttime".to_string(), Value::String(value.clone()));
            }
            "CASTTIME" => {
                attributes.insert("pcgen_casttime".to_string(), Value::String(value.clone()));
            }
            "SORTKEY" => {
                attributes.insert("pcgen_sortkey".to_string(), Value::String(value.clone()));
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
            "DESCRIPTOR" => append_string_attr(attributes, "pcgen_descriptors", value),
            "DOMAINS" => {
                attributes.insert("pcgen_domains".to_string(), Value::String(value.clone()));
            }
            "PPCOST" => set_i64_or_string(attributes, "pcgen_ppcost", value),
            "SPELLPOINTCOST" => {
                attributes.insert("pcgen_spellpointcost".to_string(), Value::String(value.clone()));
            }
            "ITEM" => append_string_attr(attributes, "pcgen_items", value),
            "GEAR" => append_string_attr(attributes, "pcgen_gear", value),
            "KIT" => append_string_attr(attributes, "pcgen_kits", value),
            "ABILITY" => append_string_attr(attributes, "pcgen_abilities", value),
            "OPTION" => {
                attributes.insert("pcgen_option".to_string(), Value::String(value.clone()));
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
            "PLURAL" => {
                attributes.insert("pcgen_plural".to_string(), Value::String(value.clone()));
            }
            "POOL" => {
                attributes.insert("pcgen_pool".to_string(), Value::String(value.clone()));
            }
            "EQUIPBUY" => append_string_attr(attributes, "pcgen_equipbuy", value),
            "LOCATION" => {
                attributes.insert("pcgen_location".to_string(), Value::String(value.clone()));
            }
            "QTY" => set_i64_or_string(attributes, "pcgen_qty", value),
            "FREE" => {
                attributes.insert("pcgen_free".to_string(), parse_yes_no_or_string(value));
            }
            "VARIANTS" => append_string_attr(attributes, "pcgen_variants", value),
            "SITUATION" => append_string_attr(attributes, "pcgen_situations", value),
            "USEUNTRAINED" => {
                attributes.insert("pcgen_useuntrained".to_string(), parse_yes_no_or_string(value));
            }
            "SIZE" => {
                attributes.insert("pcgen_size".to_string(), Value::String(value.clone()));
            }
            "FACE" => {
                attributes.insert("pcgen_face".to_string(), Value::String(value.clone()));
            }
            "VISION" => append_string_attr(attributes, "pcgen_vision", value),
            "LEGS" => {
                attributes.insert("pcgen_legs".to_string(), Value::String(value.clone()));
            }
            "HANDS" => {
                attributes.insert("pcgen_hands".to_string(), Value::String(value.clone()));
            }
            "DR" => {
                attributes.insert("pcgen_dr".to_string(), Value::String(value.clone()));
            }
            "SR" => {
                attributes.insert("pcgen_sr".to_string(), Value::String(value.clone()));
            }
            "WIELD" => {
                attributes.insert("pcgen_wield".to_string(), Value::String(value.clone()));
            }
            "EDR" => {
                attributes.insert("pcgen_edr".to_string(), Value::String(value.clone()));
            }
            "SPELLFAILURE" => {
                attributes.insert("pcgen_spellfailure".to_string(), Value::String(value.clone()));
            }
            "FUMBLERANGE" => {
                attributes.insert("pcgen_fumblerange".to_string(), Value::String(value.clone()));
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
                attributes.insert("pcgen_altcritmult".to_string(), Value::String(value.clone()));
            }
            "ALTCRITRANGE" => {
                attributes.insert("pcgen_altcritrange".to_string(), Value::String(value.clone()));
            }
            "ALTEQMOD" => {
                attributes.insert("pcgen_alteqmod".to_string(), Value::String(value.clone()));
            }
            "PROFICIENCY" => {
                attributes.insert("pcgen_proficiency".to_string(), Value::String(value.clone()));
            }
            "CONTAINS" => {
                attributes.insert("pcgen_contains".to_string(), Value::String(value.clone()));
            }
            "ICON" => {
                attributes.insert("pcgen_icon".to_string(), Value::String(value.clone()));
            }
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
            "FACT" => facts.push(parse_fact(value)),
            "EQMOD" => equipment_modifiers.push(parse_pipe_series(value)),
            "CLASSES" => class_lists.push(parse_pipe_series(value)),
            "SPELLS" => spell_blocks.push(parse_spells(value)),
            "SPELLKNOWN" => append_string_attr(attributes, "pcgen_spellknown", value),
            "MOVE" => append_string_attr(attributes, "pcgen_move", value),
            "NATURALATTACKS" => append_string_attr(attributes, "pcgen_naturalattacks", value),
            _ => {}
        }
    }

    if !facts.is_empty() {
        attributes.insert("pcgen_facts".to_string(), Value::Array(facts));
    }
    if !equipment_modifiers.is_empty() {
        attributes.insert("pcgen_eqmods".to_string(), Value::Array(equipment_modifiers));
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
        attributes.insert("pcgen_pageusage".to_string(), Value::Array(page_usage_values));
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
            assignments.insert(key.trim().to_ascii_lowercase(), Value::String(value.trim().to_string()));
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
