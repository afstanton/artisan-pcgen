use artisan_core::domain::rules::{Effect, Prerequisite};

use crate::{ParsedClause, parsing::line_codec::{find_key_value, parse_head_key_value}};

pub(crate) fn project_semantics(
    clauses: &[ParsedClause],
    effects: &mut Vec<Effect>,
    prerequisites: &mut Vec<Prerequisite>,
) {
    for clause in clauses {
        if let ParsedClause::KeyValue { key, value } = clause {
            if key.starts_with("PRE") {
                prerequisites.push(Prerequisite {
                    kind: key.clone(),
                    expression: if value.is_empty() {
                        None
                    } else {
                        Some(value.clone())
                    },
                });
                continue;
            }

            if key == "BONUS" || key == "AUTO" || key == "DEFINE" || key == "CHOOSE" {
                effects.push(Effect {
                    kind: key.clone(),
                    target: value.clone(),
                    value: None,
                });
            }
        }
    }
}

pub(crate) fn infer_entity_type_key(head: &str, clauses: &[ParsedClause]) -> String {
    if let Some((decl_key, _)) = declared_entity(head) {
        return format!("pcgen:entity:{}", decl_key.to_ascii_lowercase());
    }

    if let Some(value) = find_key_value(clauses, "TYPE") {
        let normalized = value
            .split('.')
            .next()
            .unwrap_or(value.as_str())
            .trim()
            .to_ascii_lowercase()
            .replace(' ', "-");
        if !normalized.is_empty() {
            return format!("pcgen:type:{normalized}");
        }
    }
    "pcgen:type:unresolved".to_string()
}

pub(crate) fn derive_entity_name(head: &str, clauses: &[ParsedClause]) -> Option<String> {
    let (decl_key, decl_value) = declared_entity(head)?;

    match decl_key.as_str() {
        "ABILITY" => {
            if let Some(key_value) = find_key_value(clauses, "KEY") {
                return Some(key_value);
            }
            if let Some(ParsedClause::Bare(value)) = clauses.first() {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
            Some(decl_value)
        }
        _ => Some(decl_value),
    }
}

pub(crate) fn declared_entity(head: &str) -> Option<(String, String)> {
    let (key, value) = parse_head_key_value(head)?;
    let key_upper = key.to_ascii_uppercase();

    match key_upper.as_str() {
        "ABILITY" | "SKILL" | "GEAR" | "CLASS" | "STARTPACK" | "ABILITYCATEGORY" => {
            Some((key_upper, value))
        }
        _ => None,
    }
}
