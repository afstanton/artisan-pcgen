/// Convert a publisher or source title to a filesystem-safe slug.
///
/// Rules: lowercase, replace spaces/punctuation with `_`, collapse runs of `_`,
/// strip leading/trailing `_`.
///
/// Examples:
/// - "Wizards of the Coast" → "wizards_of_the_coast"
/// - "Paizo Inc."           → "paizo_inc"
/// - "Complete Divine"      → "complete_divine"
pub fn to_slug(s: &str) -> String {
    let mut slug = String::with_capacity(s.len());
    let mut prev_underscore = true; // suppress leading underscores

    for ch in s.chars() {
        if ch.is_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            prev_underscore = false;
        } else if !prev_underscore {
            slug.push('_');
            prev_underscore = true;
        }
    }

    // Strip trailing underscore
    if slug.ends_with('_') {
        slug.pop();
    }

    slug
}

/// Convert a game system string to a directory slug.
///
/// Rules: lowercase, strip all non-alphanumeric characters entirely (no `_`).
///
/// Examples:
/// - "3.5e"          → "35e"
/// - "Pathfinder 1e" → "pathfinder1e"
/// - "Pathfinder"    → "pathfinder"
/// - "5e"            → "5e"
pub fn game_system_slug(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Derive a short abbreviation (file prefix) for a source.
///
/// Preference order:
/// 1. `source_short` entity attribute (lowercased, truncated to 8 chars)
/// 2. First letter of each significant word in the title, lowercased, truncated to 8 chars.
///    "Significant" = words not in the stop list (a, an, the, of, in, to, for).
pub fn derive_abbreviation(title: &str, source_short_hint: Option<&str>) -> String {
    if let Some(hint) = source_short_hint {
        let clean: String = hint
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .take(8)
            .collect();
        if !clean.is_empty() {
            return clean;
        }
    }

    const STOP_WORDS: &[&str] = &["a", "an", "the", "of", "in", "to", "for", "and", "or"];

    let abbrev: String = title
        .split_whitespace()
        .filter(|w| {
            let lower = w.to_ascii_lowercase();
            let stripped: String = lower.chars().filter(|c| c.is_alphabetic()).collect();
            !STOP_WORDS.contains(&stripped.as_str())
        })
        .filter_map(|w| w.chars().find(|c| c.is_alphabetic()))
        .map(|c| c.to_ascii_lowercase())
        .take(8)
        .collect();

    if abbrev.is_empty() { "src".to_string() } else { abbrev }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_slug() {
        assert_eq!(to_slug("Wizards of the Coast"), "wizards_of_the_coast");
        assert_eq!(to_slug("Paizo Inc."), "paizo_inc");
        assert_eq!(to_slug("Complete Divine"), "complete_divine");
        assert_eq!(to_slug("Ultimate Combat"), "ultimate_combat");
        assert_eq!(to_slug("Paizo"), "paizo");
    }

    #[test]
    fn test_game_system_slug() {
        assert_eq!(game_system_slug("3.5e"), "35e");
        assert_eq!(game_system_slug("Pathfinder 1e"), "pathfinder1e");
        assert_eq!(game_system_slug("Pathfinder"), "pathfinder");
        assert_eq!(game_system_slug("5e"), "5e");
        assert_eq!(game_system_slug("35e"), "35e");
    }

    #[test]
    fn test_derive_abbreviation_from_hint() {
        assert_eq!(derive_abbreviation("Complete Divine", Some("CD")), "cd");
        assert_eq!(derive_abbreviation("Ultimate Combat", Some("UC")), "uc");
    }

    #[test]
    fn test_derive_abbreviation_from_title() {
        assert_eq!(derive_abbreviation("Complete Divine", None), "cd");
        assert_eq!(derive_abbreviation("Ultimate Combat", None), "uc");
        assert_eq!(derive_abbreviation("Player's Handbook", None), "ph");
        // "of" is a stop word
        assert_eq!(derive_abbreviation("Book of Exalted Deeds", None), "bed");
    }
}
