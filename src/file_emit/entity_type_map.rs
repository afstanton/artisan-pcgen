/// Describes how an entity type is mapped to a file and PCC token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityTypeEntry {
    /// File name suffix (e.g. "abilities" → `{abbrev}_abilities.lst`).
    /// `None` = skip; don't emit to a file.
    pub file_suffix: Option<&'static str>,

    /// PCC token (e.g. "ABILITY"). `None` = skip or merged, no PCC line.
    pub pcc_token: Option<&'static str>,

    /// If set, this entity type is co-emitted into another type's file.
    /// The value is the canonical file suffix to merge into.
    pub merge_into: Option<&'static str>,
}

impl EntityTypeEntry {
    const fn emit(file_suffix: &'static str, pcc_token: &'static str) -> Self {
        Self {
            file_suffix: Some(file_suffix),
            pcc_token: Some(pcc_token),
            merge_into: None,
        }
    }

    const fn merge(into_suffix: &'static str) -> Self {
        Self {
            file_suffix: None,
            pcc_token: None,
            merge_into: Some(into_suffix),
        }
    }

    const fn skip() -> Self {
        Self { file_suffix: None, pcc_token: None, merge_into: None }
    }
}

/// Look up the entity type entry for a given `pcgen_entity_type_key`.
///
/// Returns `None` for unknown type keys (treated as skip).
pub fn entry_for_type_key(key: &str) -> Option<EntityTypeEntry> {
    let entry = match key {
        "pcgen:entity:ability" => EntityTypeEntry::emit("abilities", "ABILITY"),
        "pcgen:entity:abilitycategory" => EntityTypeEntry::emit("abilitycategories", "ABILITYCATEGORY"),
        "pcgen:entity:class" => EntityTypeEntry::emit("classes", "CLASS"),
        "pcgen:entity:subclass" => EntityTypeEntry::merge("classes"),
        "pcgen:entity:classlevel" => EntityTypeEntry::merge("classes"),
        "pcgen:entity:deity" => EntityTypeEntry::emit("deities", "DEITY"),
        "pcgen:entity:equipment" => EntityTypeEntry::emit("equipment", "EQUIPMENT"),
        "pcgen:entity:feat" => EntityTypeEntry::emit("feats", "FEAT"),
        "pcgen:entity:kit" => EntityTypeEntry::emit("kits", "KIT"),
        "pcgen:entity:language" => EntityTypeEntry::emit("languages", "LANGUAGE"),
        "pcgen:entity:race" => EntityTypeEntry::emit("races", "RACE"),
        "pcgen:entity:skill" => EntityTypeEntry::emit("skills", "SKILL"),
        "pcgen:entity:spell" => EntityTypeEntry::emit("spells", "SPELL"),
        "pcgen:entity:template" => EntityTypeEntry::emit("templates", "TEMPLATE"),
        "pcgen:entity:companionmod" => EntityTypeEntry::emit("companionmods", "COMPANIONMOD"),
        "pcgen:entity:gear" => EntityTypeEntry::merge("equipment"),
        "pcgen:entity:startpack" => EntityTypeEntry::merge("kits"),
        "pcgen:entity:variable" => EntityTypeEntry::emit("variables", "VARIABLE"),
        // System-level / metadata — skip
        "pcgen:entity:modify" => EntityTypeEntry::skip(),
        _ if key.starts_with("pcgen:entity:pcc-") => EntityTypeEntry::skip(),
        _ => return None,
    };
    Some(entry)
}

/// Canonical file-suffix order used when writing PCC lines.
///
/// PCC lines are emitted in this order; only files that actually contain
/// at least one entity are included.
pub const PCC_EMIT_ORDER: &[(&str, &str)] = &[
    ("abilities", "ABILITY"),
    ("abilitycategories", "ABILITYCATEGORY"),
    ("classes", "CLASS"),
    ("deities", "DEITY"),
    ("equipment", "EQUIPMENT"),
    ("feats", "FEAT"),
    ("kits", "KIT"),
    ("languages", "LANGUAGE"),
    ("races", "RACE"),
    ("skills", "SKILL"),
    ("spells", "SPELL"),
    ("templates", "TEMPLATE"),
    ("companionmods", "COMPANIONMOD"),
    ("variables", "VARIABLE"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn known_types_have_entries() {
        let keys = [
            "pcgen:entity:ability",
            "pcgen:entity:abilitycategory",
            "pcgen:entity:class",
            "pcgen:entity:subclass",
            "pcgen:entity:classlevel",
            "pcgen:entity:deity",
            "pcgen:entity:equipment",
            "pcgen:entity:feat",
            "pcgen:entity:kit",
            "pcgen:entity:language",
            "pcgen:entity:race",
            "pcgen:entity:skill",
            "pcgen:entity:spell",
            "pcgen:entity:template",
            "pcgen:entity:companionmod",
            "pcgen:entity:gear",
            "pcgen:entity:startpack",
            "pcgen:entity:variable",
            "pcgen:entity:modify",
            "pcgen:entity:pcc-source",
        ];
        for key in keys {
            assert!(entry_for_type_key(key).is_some(), "missing entry for {key}");
        }
    }

    #[test]
    fn emit_entries_have_unique_suffixes() {
        let emit_keys = [
            "pcgen:entity:ability",
            "pcgen:entity:abilitycategory",
            "pcgen:entity:class",
            "pcgen:entity:deity",
            "pcgen:entity:equipment",
            "pcgen:entity:feat",
            "pcgen:entity:kit",
            "pcgen:entity:language",
            "pcgen:entity:race",
            "pcgen:entity:skill",
            "pcgen:entity:spell",
            "pcgen:entity:template",
            "pcgen:entity:companionmod",
            "pcgen:entity:variable",
        ];
        let mut seen = HashSet::new();
        for key in emit_keys {
            let entry = entry_for_type_key(key).unwrap();
            let suffix = entry.file_suffix.unwrap();
            assert!(seen.insert(suffix), "duplicate suffix {suffix} for {key}");
        }
    }
}
