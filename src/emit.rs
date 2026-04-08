//! Schema-driven PCGen entity emitter.
//!
//! Produces valid PCGen `.lst` text from artisan `Entity` values using the
//! registered `EntitySchema` for each entity type. The schema defines which
//! tokens to emit, in what order, and how to serialize each artisan field.
//!
//! # Output format
//! Top-level tokens are separated by `\t`. Within a token's value, the
//! separator is `|` (pipe), matching standard PCGen `.lst` format.

use artisan_core::Entity;
use serde_json::Value;

use crate::{
    ParsedClause, ParsedLine,
    schema::{
        self, ArtisanMapping, Cardinality, EntitySchema, GlobalGroup, HeadFormat, TokenDef,
        TokenGrammar,
    },
};
use std::collections::BTreeSet;

/// Emit a PCGen `.lst` line for `entity` using the provided `schema`.
///
/// Returns the full line text with tab-separated top-level tokens.
pub fn emit_entity(entity: &Entity, schema: &EntitySchema) -> String {
    let mut parts: Vec<String> = Vec::new();

    // --- Head ---
    let name = entity
        .attributes
        .get("pcgen_key")
        .and_then(Value::as_str)
        .unwrap_or(entity.name.as_str());

    let head = match schema.head_format {
        HeadFormat::NameOnly => name.to_string(),
        HeadFormat::TokenPrefixed => {
            let token = schema.head_token.unwrap_or("");
            format!("{token}:{name}")
        }
    };
    parts.push(head);

    // --- Entity-specific tokens ---
    for token_def in schema.tokens {
        emit_token_def(token_def, entity, &mut parts);
    }

    // --- Global token groups ---
    for group in schema.globals {
        emit_global_group(*group, entity, &mut parts);
    }

    parts.join("\t")
}

/// Attempt to emit `entity` by looking up its schema from
/// `entity.attributes["pcgen_entity_type_key"]`.
///
/// Returns `None` if no schema is registered for the entity type.
pub fn emit_entity_auto(entity: &Entity) -> Option<String> {
    let type_key = entity
        .attributes
        .get("pcgen_entity_type_key")
        .and_then(Value::as_str)?;
    let schema = schema::schema_for_entity_type_key(type_key)?;
    Some(emit_entity(entity, schema))
}

/// Returns the list of token keys that would use raw-clause fallback for this
/// entity and schema.
///
/// This is a migration aid for eliminating fallback entirely.
pub fn fallback_keys_for_entity(entity: &Entity, schema: &EntitySchema) -> Vec<String> {
    let mut keys = BTreeSet::new();

    for token_def in schema.tokens {
        if uses_fallback_for_token(entity, token_def) {
            keys.insert(token_def.key.to_string());
        }
    }

    // Global TYPE fallback
    if schema.globals.contains(&GlobalGroup::Type)
        && entity.attributes.get("pcgen_type").and_then(Value::as_str).is_none()
        && !raw_clause_values_for_key(entity, "TYPE").is_empty()
    {
        keys.insert("TYPE".to_string());
    }

    keys.into_iter().collect()
}

/// Convert an artisan entity + schema into a `ParsedLine` (structured form).
///
/// Useful when you need the intermediate representation rather than raw text.
pub fn entity_to_parsed_line(entity: &Entity, schema: &EntitySchema) -> ParsedLine {
    let line_text = emit_entity(entity, schema);
    crate::parse_line(&line_text)
}

// ---------------------------------------------------------------------------
// Per-token emission
// ---------------------------------------------------------------------------

fn emit_token_def(token_def: &TokenDef, entity: &Entity, parts: &mut Vec<String>) {
    match token_def.artisan_mapping {
        ArtisanMapping::Attribute(field) => {
            if let Some(value) = entity.attributes.get(field) {
                let serialized =
                    serialize_value(value, token_def.grammar, token_def.cardinality);
                for s in serialized {
                    parts.push(format!("{}:{}", token_def.key, s));
                }
            } else {
                // Fallback: if parser captured the raw token in `clauses` but no
                // structured projection exists yet, preserve that data on emit.
                for raw in raw_clause_values_for_key(entity, token_def.key) {
                    parts.push(format!("{}:{}", token_def.key, raw));
                }
            }
        }
        ArtisanMapping::Effect => {
            for effect in &entity.effects {
                if effect.kind.eq_ignore_ascii_case(token_def.key) {
                    parts.push(format!("{}:{}", token_def.key, effect.target));
                }
            }
        }
        ArtisanMapping::Prerequisite | ArtisanMapping::EntityName | ArtisanMapping::None => {}
    }
}

fn uses_fallback_for_token(entity: &Entity, token_def: &TokenDef) -> bool {
    match token_def.artisan_mapping {
        ArtisanMapping::Attribute(field) => {
            entity.attributes.get(field).is_none()
                && !raw_clause_values_for_key(entity, token_def.key).is_empty()
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Global group emission
// ---------------------------------------------------------------------------

fn emit_global_group(group: GlobalGroup, entity: &Entity, parts: &mut Vec<String>) {
    match group {
        GlobalGroup::Bonus => {
            for effect in &entity.effects {
                if effect.kind.eq_ignore_ascii_case("BONUS") {
                    parts.push(format!("BONUS:{}", effect.target));
                }
            }
        }
        GlobalGroup::Add => {
            for effect in &entity.effects {
                if effect.kind.eq_ignore_ascii_case("ADD") {
                    parts.push(format!("ADD:{}", effect.target));
                }
            }
        }
        GlobalGroup::Choose => {
            for effect in &entity.effects {
                if effect.kind.eq_ignore_ascii_case("CHOOSE") {
                    parts.push(format!("CHOOSE:{}", effect.target));
                }
            }
        }
        GlobalGroup::Auto => {
            for effect in &entity.effects {
                if effect.kind.eq_ignore_ascii_case("AUTO") {
                    parts.push(format!("AUTO:{}", effect.target));
                }
            }
        }
        GlobalGroup::Define => {
            for effect in &entity.effects {
                if effect.kind.eq_ignore_ascii_case("DEFINE") {
                    parts.push(format!("DEFINE:{}", effect.target));
                }
            }
        }
        GlobalGroup::Prerequisites => {
            for prereq in &entity.prerequisites {
                match &prereq.expression {
                    Some(expr) => parts.push(format!("{}:{}", prereq.kind, expr)),
                    None => parts.push(prereq.kind.clone()),
                }
            }
        }
        GlobalGroup::Type => {
            if let Some(type_val) =
                entity.attributes.get("pcgen_type").and_then(Value::as_str)
            {
                parts.push(format!("TYPE:{type_val}"));
            } else {
                for raw in raw_clause_values_for_key(entity, "TYPE") {
                    parts.push(format!("TYPE:{raw}"));
                }
            }
        }
        GlobalGroup::Key => {
            if let Some(key_val) =
                entity.attributes.get("pcgen_key").and_then(Value::as_str)
            {
                parts.push(format!("KEY:{key_val}"));
            }
        }
        GlobalGroup::Desc => {
            // Prefer the parsed PCGen desc; fall back to the canonical description
            let desc = entity
                .attributes
                .get("pcgen_desc")
                .and_then(Value::as_str)
                .or_else(|| entity.attributes.get("description").and_then(Value::as_str));
            if let Some(desc) = desc {
                parts.push(format!("DESC:{desc}"));
            }
        }
        GlobalGroup::Fact => {
            if let Some(facts) = entity
                .attributes
                .get("pcgen_facts")
                .and_then(Value::as_array)
            {
                for fact in facts {
                    if let (Some(k), Some(v)) = (
                        fact.get("key").and_then(Value::as_str),
                        fact.get("value").and_then(Value::as_str),
                    ) {
                        parts.push(format!("FACT:{k}|{v}"));
                    }
                }
            }
        }
        GlobalGroup::SourcePage => {
            if let Some(sp) = entity
                .attributes
                .get("pcgen_source_page")
                .and_then(Value::as_str)
            {
                parts.push(format!("SOURCEPAGE:{sp}"));
            }
        }
        GlobalGroup::OutputName => {
            if let Some(on) = entity
                .attributes
                .get("pcgen_outputname")
                .and_then(Value::as_str)
            {
                parts.push(format!("OUTPUTNAME:{on}"));
            }
        }
        GlobalGroup::SortKey => {
            if let Some(sk) = entity
                .attributes
                .get("pcgen_sortkey")
                .and_then(Value::as_str)
            {
                parts.push(format!("SORTKEY:{sk}"));
            }
        }
        // Template, CSkill, Sab, Qualify, SourceMeta: not yet mapped from artisan model
        GlobalGroup::Template
        | GlobalGroup::CSkill
        | GlobalGroup::Sab
        | GlobalGroup::Qualify
        | GlobalGroup::SourceMeta => {}
    }
}

// ---------------------------------------------------------------------------
// Value serialization
// ---------------------------------------------------------------------------

/// Serialize a JSON attribute value according to its token grammar.
///
/// Returns one string per emitted instance. For `Cardinality::Repeatable`
/// array values with per-item grammars, returns multiple strings (one per
/// token occurrence).
fn serialize_value(value: &Value, grammar: TokenGrammar, cardinality: Cardinality) -> Vec<String> {
    match value {
        Value::Null => vec![],
        Value::String(s) => vec![s.clone()],
        Value::Number(n) => vec![n.to_string()],
        Value::Bool(b) => match grammar {
            TokenGrammar::YesNo => vec![if *b { "YES".to_string() } else { "NO".to_string() }],
            _ => vec![b.to_string()],
        },
        Value::Array(arr) => serialize_array(arr, grammar, cardinality),
        Value::Object(obj) => {
            // Objects come from FACT-style or ASPECT-style parsed items.
            // Re-emit via the "raw" field if available, else reconstruct.
            if let Some(raw) = obj.get("raw").and_then(Value::as_str) {
                vec![raw.to_string()]
            } else {
                vec![]
            }
        }
    }
}

fn serialize_array(arr: &[Value], grammar: TokenGrammar, cardinality: Cardinality) -> Vec<String> {
    if arr.is_empty() {
        return vec![];
    }

    match (grammar, cardinality) {
        // Repeatable tokens: each array element becomes its own token occurrence
        (_, Cardinality::Repeatable) => arr
            .iter()
            .flat_map(|item| serialize_value(item, grammar, Cardinality::Once))
            .collect(),

        // Once/DotList: join with "."
        (TokenGrammar::DotList, Cardinality::Once) => {
            let joined = arr
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(".");
            vec![joined]
        }

        // Once/CommaList: join with ","
        (TokenGrammar::CommaList, Cardinality::Once) => {
            let joined = arr
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(",");
            vec![joined]
        }

        // Once/PipeList: join with "|"
        (TokenGrammar::PipeList, Cardinality::Once) => {
            let joined = arr
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join("|");
            vec![joined]
        }

        // PipeGroups: serialize each group, join with "|"
        (TokenGrammar::PipeGroups, _) => {
            let groups: Vec<String> = arr
                .iter()
                .filter_map(|item| {
                    item.as_str()
                        .map(str::to_string)
                        .or_else(|| item.get("raw").and_then(Value::as_str).map(str::to_string))
                })
                .collect();
            if groups.is_empty() {
                vec![]
            } else {
                vec![groups.join("|")]
            }
        }

        // PipePositional with Repeatable: each array element is one full token occurrence
        (TokenGrammar::PipePositional(_), _) => arr
            .iter()
            .flat_map(|item| match item {
                Value::String(s) => vec![s.clone()],
                Value::Object(obj) => {
                    if let Some(raw) = obj.get("raw").and_then(Value::as_str) {
                        vec![raw.to_string()]
                    } else {
                        vec![]
                    }
                }
                Value::Array(inner) => {
                    let joined = inner
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join("|");
                    if joined.is_empty() { vec![] } else { vec![joined] }
                }
                _ => vec![],
            })
            .collect(),

        // Fallback: join with "|"
        _ => {
            let joined = arr
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join("|");
            vec![joined]
        }
    }
}

fn raw_clause_values_for_key(entity: &Entity, key: &str) -> Vec<String> {
    let Some(raw) = entity.attributes.get("clauses") else {
        return Vec::new();
    };
    let Some(clauses) = crate::parsing::line_codec::clauses_from_json(raw) else {
        return Vec::new();
    };

    clauses
        .into_iter()
        .filter_map(|clause| match clause {
            ParsedClause::KeyValue { key: k, value } if k.eq_ignore_ascii_case(key) => Some(value),
            _ => None,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Convenience: build ParsedClause list from entity + schema
// ---------------------------------------------------------------------------

/// Reconstruct the `ParsedClause` list that `emit_entity` would produce.
///
/// The head token is excluded — only the clause tokens are returned.
pub fn entity_to_clauses(entity: &Entity, schema: &EntitySchema) -> Vec<ParsedClause> {
    let line_text = emit_entity(entity, schema);
    let parsed = crate::parse_line(&line_text);
    parsed.clauses
}
