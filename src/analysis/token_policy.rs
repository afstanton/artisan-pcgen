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

    // These pseudo-keys commonly appear inside free-text fields (SPROP/DESC)
    // due to sentence punctuation and should not be treated as standalone tokens.
    if matches!(
        token.as_str(),
        "EFFECTS"
            | "DC"
            | "SCREAM"
            | "TARGET"
            | "THROW"
            | "AC"
            | "ACTIVATION"
            | "GRANTED"
            | "USAGE"
    ) {
        return ClauseSupportLevel::Artifact;
    }

    // Paper layout tokens — appear in paperInfo.lst files, not game-data files.
    // WIDTH and HEIGHT are paper dimensions; LEFTMARGIN etc. are margins.
    // (Note: HEIGHT is also a valid .pcg bio token — but paperInfo HEIGHT is
    // harmlessly classified as SemanticallyInterpreted via the pcg HEIGHT schema,
    // so only WIDTH needs explicit Artifact treatment here.)
    if matches!(
        token.as_str(),
        "LEFTMARGIN" | "RIGHTMARGIN" | "TOPMARGIN" | "BOTTOMMARGIN" | "WIDTH"
    ) {
        return ClauseSupportLevel::Artifact;
    }

    // Dice notation fragments (e.g. D0, D1, D1.5) parsed as token heads when
    // dice expressions appear at the start of a clause value.
    if matches!(token.as_str(), "D0" | "D1" | "D1.5") {
        return ClauseSupportLevel::Artifact;
    }

    // Parser noise: single/two-letter fragments, typos, proper names, and
    // abbreviations that appear as token keys due to prose formatting or
    // malformed corpus entries.
    if matches!(
        token.as_str(),
        // Single/two-letter noise
        "R" | "F" | "IE"
        // Typos and misspellings
        | "VIIBLE" | "SERVAAS" | "SERVEAS"
        // Prerequisite expression fragment split by the parser
        | "IV.PRECLASS"
        // Proper names and abbreviations found in body text or deity names
        | "SELUNE" | "WWII" | "STR"
        // Equipment slot names that appear as token keys in free text
        | "ARMS" | "RINGS" | "VEHICLE" | "SPECIAL"
        // Currency abbreviations appearing in prose or SPROP text
        | "CP" | "PP"
        // "HP" appears in SPROP free text (e.g. "TEMP HP: 1") split at the colon
        | "HP"
        // DIVINITY appears when ability names like "CHANNEL DIVINITY: ..." are split at the colon
        | "DIVINITY"
        // Prose/format markers that appear as token keys in body text
        | "FOLIO"
        // Comment line fragment (#COMMENT or similar) parsed as a token
        | "COMMENT"
        // Placeholder text appearing as token key in restriction clauses
        | "RESTRICTION"
        // Commented-out line artifact in companion/race data
        | "CREATUREHANDS"
        // Old-style product-identity flag (one occurrence in 3.5e dataset);
        // modern corpus uses NAMEISPI / DESCISPI instead
        | "ISPI"
    ) {
        return ClauseSupportLevel::Artifact;
    }

    // Schema-driven lookup: any registered schema that knows this token
    // classifies it as semantically interpreted.
    if crate::schema::any_schema_knows_token(&token) {
        return ClauseSupportLevel::SemanticallyInterpreted;
    }

    // Selector-style token used in variable-path contexts; if it is not
    // schema-backed in the current context, keep treating it as intentionally
    // supported syntax rather than unhandled noise.
    if token == "EQUIPMENT.PART" {
        return ClauseSupportLevel::PolicySupported;
    }

    // TOKEN.CLEAR is a standard PCGen modifier that removes accumulated values
    // for repeatable tokens (e.g. DESC.CLEAR, TYPE.CLEAR). If a specific clear
    // token is not schema-backed yet, keep treating it as intentionally
    // supported syntax instead of unhandled.
    if token.ends_with(".CLEAR") {
        return ClauseSupportLevel::PolicySupported;
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

    matches!(
        upper.as_str(),
        "AUTOMATIC" | "VIRTUAL" | "PRERULE" | "!PRERULE" | "SET"
    )
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
        "I" | "II" | "III" | "IV" | "V" | "VI" | "VII" | "VIII" | "IX" | "X" | "XI" | "XII"
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
        assert!(matches!(
            classify_token_key("DC", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("SCREAM", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("TARGET", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("THROW", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("AC", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("ACTIVATION", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("GRANTED", false),
            ClauseSupportLevel::Artifact
        ));
        assert!(matches!(
            classify_token_key("USAGE", false),
            ClauseSupportLevel::Artifact
        ));
    }

    #[test]
    fn classify_token_key_treats_equipment_part_as_semantically_interpreted() {
        assert!(matches!(
            classify_token_key("EQUIPMENT.PART", false),
            ClauseSupportLevel::SemanticallyInterpreted
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
            ClauseSupportLevel::SemanticallyInterpreted
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

    #[test]
    fn classify_token_key_rejects_output_sheet_layout_tokens() {
        for token in &["LEFTMARGIN", "RIGHTMARGIN", "TOPMARGIN", "BOTTOMMARGIN"] {
            assert!(
                matches!(
                    classify_token_key(token, false),
                    ClauseSupportLevel::Artifact
                ),
                "{token} should be Artifact"
            );
        }
    }

    #[test]
    fn classify_token_key_rejects_dice_notation_fragments() {
        for token in &["D0", "D1", "D1.5"] {
            assert!(
                matches!(
                    classify_token_key(token, false),
                    ClauseSupportLevel::Artifact
                ),
                "{token} should be Artifact"
            );
        }
    }

    #[test]
    fn classify_token_key_rejects_known_parser_noise() {
        for token in &[
            "R",
            "F",
            "IE",
            "VIIBLE",
            "SERVAAS",
            "SERVEAS",
            "IV.PRECLASS",
            "SELUNE",
            "WWII",
            "STR",
        ] {
            assert!(
                matches!(
                    classify_token_key(token, false),
                    ClauseSupportLevel::Artifact
                ),
                "{token} should be Artifact"
            );
        }
    }
}
