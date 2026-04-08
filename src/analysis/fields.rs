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

    for clause in clauses {
        let ParsedClause::KeyValue { key, value } = clause else {
            continue;
        };

        match key.to_ascii_uppercase().as_str() {
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
            "FACT" => facts.push(parse_fact(value)),
            "EQMOD" => equipment_modifiers.push(parse_pipe_series(value)),
            "CLASSES" => class_lists.push(parse_pipe_series(value)),
            "SPELLS" => spell_blocks.push(parse_spells(value)),
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
