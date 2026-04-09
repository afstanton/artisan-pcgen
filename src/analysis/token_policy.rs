use crate::ParsedClause;

pub(crate) fn classify_clause_token(clause: &ParsedClause) -> ClauseSupportLevel {
    match clause {
        ParsedClause::KeyValue { key, .. } => classify_token_key(key, false),
        ParsedClause::Bare(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                ClauseSupportLevel::PolicySupported
            } else if is_known_bare_directive(trimmed) {
                ClauseSupportLevel::PolicySupported
            } else {
                ClauseSupportLevel::Artifact
            }
        }
    }
}

pub(crate) fn classify_token_key(input: &str, is_bare: bool) -> ClauseSupportLevel {
    let raw = input.trim();
    if raw.is_empty() {
        return ClauseSupportLevel::Artifact;
    }

    // Real PCGen token keys are uppercase. Lowercase or mixed-case segments are
    // typically prose or URL schemes that were split out of a value.
    if raw.chars().any(|c| c.is_ascii_lowercase()) {
        return ClauseSupportLevel::Artifact;
    }

    let token = raw.to_ascii_uppercase();

    if !is_plausible_token_name(&token) {
        return ClauseSupportLevel::Artifact;
    }

    // "EFFECTS:" appears in corpus prose (e.g., "DIMINISHED EFFECTS:" inside DESC)
    // and should not be treated as a standalone token key.
    if token == "EFFECTS" {
        return ClauseSupportLevel::Artifact;
    }

    // Selector-style token used in variable-path contexts; keep distinct from PART
    // while treating it as intentionally supported syntax.
    if token == "EQUIPMENT.PART" {
        return ClauseSupportLevel::PolicySupported;
    }

    // Schema-driven lookup: any registered schema that knows this token
    // classifies it as semantically interpreted.
    if crate::schema::any_schema_knows_token(&token) {
        return ClauseSupportLevel::SemanticallyInterpreted;
    }

    // Bare PRE* tokens not caught by the schema (edge-case aliases)
    if is_bare && (token.starts_with("PRE") || token.starts_with("!PRE")) {
        return ClauseSupportLevel::PolicySupported;
    }

    ClauseSupportLevel::Unhandled(token)
}

pub(crate) enum ClauseSupportLevel {
    SemanticallyInterpreted,
    PolicySupported,
    Unhandled(String),
    Artifact,
}

fn is_known_bare_directive(input: &str) -> bool {
    let upper = input.to_ascii_uppercase();

    // PRE* bare directives
    if upper.starts_with("PRE") || upper.starts_with("!PRE") {
        return true;
    }

    // Bare tokens that appear without a colon in PCGen data
    if crate::schema::any_schema_knows_token(&upper) {
        return true;
    }

    matches!(upper.as_str(), "AUTOMATIC" | "VIRTUAL" | "PRERULE" | "!PRERULE" | "SET")
}

fn is_plausible_token_name(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    // Titles like "Part I:" and "Part II:" in prose can be split into fake
    // key/value clauses. Treat standalone Roman numerals as artifacts.
    if is_standalone_roman_numeral(token) {
        return false;
    }

    if !token
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '!' | '-' | '.'))
    {
        return false;
    }

    if token.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    token.chars().any(|c| c.is_ascii_alphabetic())
}

fn is_standalone_roman_numeral(token: &str) -> bool {
    matches!(
        token,
        "I"
            | "II"
            | "III"
            | "IV"
            | "V"
            | "VI"
            | "VII"
            | "VIII"
            | "IX"
            | "X"
            | "XI"
            | "XII"
    )
}

#[cfg(test)]
mod tests {
    use super::{ClauseSupportLevel, classify_clause_token, classify_token_key};
    use crate::ParsedClause;

    #[test]
    fn classify_token_key_rejects_lowercase_url_scheme() {
        assert!(matches!(
            classify_token_key("http", false),
            ClauseSupportLevel::Artifact
        ));
    }

    #[test]
    fn classify_token_key_rejects_mixed_case_text() {
        assert!(matches!(
            classify_token_key("SourceWeb", false),
            ClauseSupportLevel::Artifact
        ));
    }

    #[test]
    fn classify_clause_token_treats_url_fragment_as_artifact() {
        let clause = ParsedClause::KeyValue {
            key: "http".to_string(),
            value: "//example.com".to_string(),
        };

        assert!(matches!(
            classify_clause_token(&clause),
            ClauseSupportLevel::Artifact
        ));
    }

    #[test]
    fn classify_token_key_treats_effects_as_artifact() {
        assert!(matches!(
            classify_token_key("EFFECTS", false),
            ClauseSupportLevel::Artifact
        ));
    }

    #[test]
    fn classify_token_key_treats_equipment_part_as_policy_supported() {
        assert!(matches!(
            classify_token_key("EQUIPMENT.PART", false),
            ClauseSupportLevel::PolicySupported
        ));
    }

    #[test]
    fn classify_token_key_distinguishes_part_from_equipment_part() {
        assert!(matches!(
            classify_token_key("PART", false),
            ClauseSupportLevel::SemanticallyInterpreted
        ));
        assert!(matches!(
            classify_token_key("EQUIPMENT.PART", false),
            ClauseSupportLevel::PolicySupported
        ));
    }

    #[test]
    fn classify_token_key_rejects_standalone_roman_numerals() {
        assert!(matches!(
            classify_token_key("I", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("II", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("IV", false),
            ClauseSupportLevel::Artifact
        ));
    }
}
