//! Explicit token alias/deprecation registry.
//!
//! This module centralizes token synonym/deprecation decisions so parser and
//! schema lookups don't rely on scattered hardcoded special-cases.

/// How an alias relationship should be interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AliasStatus {
    /// Legacy/deprecated spelling that should map to canonical spelling.
    DeprecatedAlias,
    /// Compatibility alias accepted for migration/interop.
    CompatibilityAlias,
}

/// Optional scope for an alias mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AliasScope {
    /// Alias applies globally.
    Global,
    /// Alias applies only to a specific schema entity type.
    EntityType(&'static str),
}

/// A single alias mapping rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenAlias {
    /// Non-canonical token spelling.
    pub alias: &'static str,
    /// Canonical token spelling.
    pub canonical: &'static str,
    /// Scope restriction.
    pub scope: AliasScope,
    /// Relationship status.
    pub status: AliasStatus,
    /// Human-readable rationale.
    pub note: &'static str,
}

/// Known aliases/synonyms retained for parser compatibility.
///
/// IMPORTANT: only place entries here when we intentionally treat two token
/// keys as equivalent for classification/lookup. Similar-looking names that are
/// semantically distinct (e.g. ACHECK vs ACCHECK in different docs) should not
/// be added.
static TOKEN_ALIASES: &[TokenAlias] = &[TokenAlias {
    alias: "HD",
    canonical: "HITDIE",
    scope: AliasScope::EntityType("pcgen:entity:class"),
    status: AliasStatus::CompatibilityAlias,
    note: "Class-line HD shorthand normalized to HITDIE for canonical emission.",
}];

/// Exposes all configured token alias rules.
pub fn all_token_aliases() -> &'static [TokenAlias] {
    TOKEN_ALIASES
}

/// Returns the canonical lookup key for `token` under an optional entity scope.
pub fn canonical_lookup_key(token: &str, entity_type_key: Option<&str>) -> String {
    let upper = token.trim().to_ascii_uppercase();
    for alias in TOKEN_ALIASES {
        if alias.alias != upper {
            continue;
        }

        match alias.scope {
            AliasScope::Global => return alias.canonical.to_string(),
            AliasScope::EntityType(scope_key) => {
                if entity_type_key == Some(scope_key) {
                    return alias.canonical.to_string();
                }
            }
        }
    }

    upper
}
