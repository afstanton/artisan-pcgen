use artisan_core::domain::rules::{Effect, Prerequisite};

use crate::{ParsedClause, parsing::line_codec::find_key_value};

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

pub(crate) fn infer_entity_type_key(clauses: &[ParsedClause]) -> String {
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
