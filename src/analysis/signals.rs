use std::collections::BTreeSet;

use crate::ParsedClause;

pub(crate) fn extract_mechanical_signals(clauses: &[ParsedClause]) -> Vec<String> {
    let mut signals = BTreeSet::new();

    for clause in clauses {
        let ParsedClause::KeyValue { key, value } = clause else {
            continue;
        };

        let key_upper = key.to_ascii_uppercase();
        let key_lower = key_upper.to_ascii_lowercase();

        if key_upper.starts_with("PRE") {
            signals.insert(format!("pre_key:{key_lower}"));
        }

        match key_upper.as_str() {
            "PRESTAT" => {
                for part in value.split([',', '|']) {
                    let lhs = part.split('=').next().unwrap_or_default();
                    if let Some(token) = normalize_signal_token(lhs) {
                        signals.insert(format!("prestat:{token}"));
                    }
                }
            }
            "BONUS" => {
                let mut parts = value.split('|').map(str::trim);
                if let Some(category) = parts.next().and_then(normalize_signal_token) {
                    signals.insert(format!("bonus_category:{category}"));
                }
                if let Some(target) = parts.next().and_then(normalize_signal_token) {
                    signals.insert(format!("bonus_target:{target}"));
                }
            }
            "AUTO" | "DEFINE" | "CHOOSE" => {
                signals.insert(format!("effect_key:{key_lower}"));
                if let Some(token) = value
                    .split(['|', ',', '='])
                    .next()
                    .and_then(normalize_signal_token)
                {
                    signals.insert(format!("effect_target:{token}"));
                }
            }
            "TYPE" => {
                if let Some(token) = value
                    .split(['.', '|', ',', ' '])
                    .next()
                    .and_then(normalize_signal_token)
                {
                    signals.insert(format!("type_token:{token}"));
                }
            }
            _ => {}
        }
    }

    signals.into_iter().collect()
}

fn normalize_signal_token(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let normalized: String = trimmed
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
        .collect::<String>()
        .to_ascii_lowercase();

    if normalized.is_empty() {
        return None;
    }
    if normalized.len() > 48 {
        return None;
    }
    Some(normalized)
}
