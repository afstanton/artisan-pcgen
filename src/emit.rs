//! Schema-driven PCGen entity emitter.
//!
//! Produces valid PCGen line-oriented text (`.lst`, `.pcc`, `.pcg`) from artisan `Entity` values using the
//! registered `LineGrammar` for each entity type. The schema defines which
//! tokens to emit, in what order, and how to serialize each artisan field.
//!
//! # Output format
//! Top-level tokens are separated by `\t`. Within a token's value, the
//! separator is `|` (pipe), matching standard PCGen line format.

use artisan_core::Entity;
use serde_json::Value;

use crate::{
    ParsedClause, ParsedLine,
    schema::{
        self, ArtisanMapping, Cardinality, GlobalGroup, HeadFormat, LineGrammar, TokenDef,
        TokenGrammar,
    },
};
use std::collections::{BTreeSet, HashSet};

/// Emit a PCGen line for `entity` using the provided `schema`.
///
/// Returns the full line text with tab-separated top-level tokens.
pub fn emit_entity(entity: &Entity, schema: &LineGrammar) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut emitted_attribute_fields: HashSet<&'static str> = HashSet::new();

    // --- Head ---
    let name = head_name_for_entity(entity);

    let head = match schema.head_format {
        HeadFormat::NameOnly => {
            let decl_token = entity
                .attributes
                .get("pcgen_decl_token")
                .and_then(Value::as_str)
                .map(|s| s.to_ascii_uppercase());
            let schema_head = schema.head_token.map(|s| s.to_ascii_uppercase());

            if let (Some(decl), Some(expected)) = (&decl_token, &schema_head)
                && decl == expected
            {
                format!("{decl}:{name}")
            } else if let Some(decl) = &decl_token
                && schema_head.is_none()
            {
                // Entity was parsed from a TOKEN:value head (e.g. ABILITY:Category|TYPE|Name
                // on a tab-indented continuation line), but the schema has no fixed head
                // token. Preserve the original TOKEN:name head so the second parse sees
                // the same token-prefixed head and assigns the same entity type.
                //
                // Mark the token's backing field as already emitted so the token loop
                // below doesn't also emit an ABILITY:name clause — that would cause the
                // re-parser to double-count the value in the abilities array.
                if let Some(field) = schema.tokens.iter()
                    .find(|t| t.key.eq_ignore_ascii_case(decl))
                    .and_then(|t| {
                        if let ArtisanMapping::Field(f) = t.artisan_mapping { Some(f) } else { None }
                    })
                {
                    emitted_attribute_fields.insert(field);
                }
                format!("{decl}:{name}")
            } else {
                name.to_string()
            }
        }
        HeadFormat::TokenPrefixed => {
            let token = schema.head_token.unwrap_or("");
            let value =
                token_prefixed_head_value(entity, schema).unwrap_or_else(|| name.to_string());
            // Preserve the original head-token presence/absence.
            //
            // Entities that were parsed from lines WITHOUT a token prefix
            // (e.g. `Acrobatics.MOD\tSITUATION:When Jumping` in a skills
            // file) have no `pcgen_decl_token` attribute.  Emitting them
            // WITH the prefix (e.g. `SKILL:Acrobatics.MOD`) causes the
            // second parse to assign `pcgen_decl_token`, making the entity
            // gain a merge key it did not originally have.  Subsequent
            // same-name lines then collapse into a single entity, reducing
            // the entity count and dropping data.
            //
            // If the entity DID originate from a prefixed head (CLASS:Wizard,
            // ABILITYCATEGORY:Special Ability, …) `pcgen_decl_token` is
            // present and we emit the prefix unchanged.
            let had_prefix = entity
                .attributes
                .get("pcgen_decl_token")
                .and_then(Value::as_str)
                .is_some();
            if had_prefix {
                format!("{token}:{value}")
            } else {
                value
            }
        }
    };
    parts.push(head);

    // --- Entity-specific tokens ---
    // Collect schema token keys first so the Prerequisites global-group emitter
    // can skip tokens that were already emitted by the token-def loop (e.g.
    // PRECAMPAIGN, which starts with "PRE" but is a schema token, not a
    // free-standing prerequisite).
    let schema_token_keys: HashSet<String> = schema
        .tokens
        .iter()
        .map(|t| t.key.to_ascii_uppercase())
        .collect();

    for token_def in schema.tokens {
        if should_skip_duplicate_head_token(entity, schema, token_def) {
            continue;
        }
        emit_token_def(token_def, entity, &mut parts, &mut emitted_attribute_fields);
    }

    // --- Global token groups ---
    for group in schema.globals {
        emit_global_group(*group, entity, &mut parts, &schema_token_keys);
    }

    // Always emit prerequisites regardless of whether the schema declares
    // GlobalGroup::Prerequisites.  PRE* tokens are valid on any PCGen entity
    // type; the parser always captures them into entity.prerequisites, but
    // lightweight schemas (e.g. many system-mode schemas) omit the global
    // group.  Emitting here ensures they survive the roundtrip.  Skip if the
    // schema already handled them via GlobalGroup::Prerequisites to avoid
    // duplication.  Also skip any prerequisite whose kind matches a schema
    // token def (e.g. PRECAMPAIGN in PCC schemas) — those were already emitted
    // by the token-def loop above.
    if !schema.globals.contains(&GlobalGroup::Prerequisites) && !entity.prerequisites.is_empty() {
        // Reuse `schema_token_keys` computed above — no need to rebuild it.
        for prereq in &entity.prerequisites {
            let kind_upper = prereq.kind.to_ascii_uppercase();
            if schema_token_keys.contains(&kind_upper) {
                // Already emitted by the schema token-def loop — skip.
                continue;
            }
            match &prereq.expression {
                Some(expr) => parts.push(format!("{}:{}", prereq.kind, expr)),
                // PCGen PRE* tokens always use KEY:value format; preserve the
                // colon even when the value is empty so the re-parser can
                // recognise `PRETEXT:` (and similar) as a token-start boundary
                // and split correctly rather than absorbing it into the
                // preceding clause value.
                None => parts.push(format!("{}:", prereq.kind)),
            }
        }
    }

    parts.join(top_level_separator(entity, schema))
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
pub fn fallback_keys_for_entity(entity: &Entity, schema: &LineGrammar) -> Vec<String> {
    let mut keys = BTreeSet::new();

    for token_def in schema.tokens {
        if uses_fallback_for_token(entity, token_def) {
            keys.insert(token_def.key.to_string());
        }
    }

    // Global TYPE fallback
    if schema.globals.contains(&GlobalGroup::Type)
        && entity
            .attributes
            .get("type")
            .and_then(Value::as_str)
            .is_none()
        && !raw_clause_values_for_key(entity, "TYPE").is_empty()
    {
        keys.insert("TYPE".to_string());
    }

    keys.into_iter().collect()
}

/// Returns the list of token keys that this entity would actually emit with the
/// current schema-driven emitter.
pub fn emittable_keys_for_entity(entity: &Entity, schema: &LineGrammar) -> Vec<String> {
    let mut keys = BTreeSet::new();

    if let Some(head_key) = emitted_head_key(entity, schema) {
        keys.insert(head_key);
    }

    for token_def in schema.tokens {
        if should_skip_duplicate_head_token(entity, schema, token_def) {
            continue;
        }
        match token_def.artisan_mapping {
            ArtisanMapping::Field(field) => {
                if let Some(value) = entity.attributes.get(field)
                    && !serialize_value(value, token_def.grammar, token_def.cardinality).is_empty()
                {
                    keys.insert(token_def.key.to_string());
                }
            }
            ArtisanMapping::Effect => {
                if entity
                    .effects
                    .iter()
                    .any(|effect| effect.kind.eq_ignore_ascii_case(token_def.key))
                {
                    keys.insert(token_def.key.to_string());
                }
            }
            ArtisanMapping::Prerequisite | ArtisanMapping::EntityName | ArtisanMapping::None => {}
        }
    }

    for group in schema.globals {
        collect_emittable_global_keys(*group, entity, &mut keys);
    }

    keys.into_iter().collect()
}

/// Convert an artisan entity + schema into a `ParsedLine` (structured form).
///
/// Useful when you need the intermediate representation rather than raw text.
pub fn entity_to_parsed_line(entity: &Entity, schema: &LineGrammar) -> ParsedLine {
    let line_text = emit_entity(entity, schema);
    crate::parse_line(&line_text)
}

// ---------------------------------------------------------------------------
// Per-token emission
// ---------------------------------------------------------------------------

fn emit_token_def(
    token_def: &TokenDef,
    entity: &Entity,
    parts: &mut Vec<String>,
    emitted_attribute_fields: &mut HashSet<&'static str>,
) {
    match token_def.artisan_mapping {
        ArtisanMapping::Field(field) => {
            if emitted_attribute_fields.contains(field) {
                return;
            }

            if let Some(value) = entity.attributes.get(field) {
                let serialized = serialize_value(value, token_def.grammar, token_def.cardinality);
                for s in serialized {
                    parts.push(format!("{}:{}", token_def.key, s));
                }
                emitted_attribute_fields.insert(field);
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
        ArtisanMapping::Field(field) => {
            entity.attributes.get(field).is_none()
                && !raw_clause_values_for_key(entity, token_def.key).is_empty()
        }
        _ => false,
    }
}

fn emitted_head_key(entity: &Entity, schema: &LineGrammar) -> Option<String> {
    match schema.head_format {
        HeadFormat::TokenPrefixed => schema.head_token.map(str::to_string),
        HeadFormat::NameOnly => {
            if let Some(decl) = entity
                .attributes
                .get("pcgen_decl_token")
                .and_then(Value::as_str)
                .map(|s| s.to_ascii_uppercase())
            {
                if schema
                    .head_token
                    .is_some_and(|head| decl.eq_ignore_ascii_case(head))
                {
                    return Some(decl);
                }
            }

            if schema.head_token.is_none() {
                let name = entity
                    .attributes
                    .get("key")
                    .and_then(Value::as_str)
                    .unwrap_or(entity.name.as_str());
                return name
                    .split_once(':')
                    .map(|(key, _)| key.trim().to_ascii_uppercase());
            }

            None
        }
    }
}

fn should_skip_duplicate_head_token(
    entity: &Entity,
    schema: &LineGrammar,
    token_def: &TokenDef,
) -> bool {
    if !matches!(schema.head_format, HeadFormat::TokenPrefixed) {
        return false;
    }

    if !schema
        .head_token
        .is_some_and(|head| head.eq_ignore_ascii_case(token_def.key))
    {
        return false;
    }

    let ArtisanMapping::Field(field) = token_def.artisan_mapping else {
        return false;
    };

    let Some(value) = entity.attributes.get(field) else {
        return false;
    };

    let serialized = serialize_value(value, token_def.grammar, token_def.cardinality);
    if serialized.len() != 1 {
        return false;
    }

    let Some(head_value) = token_prefixed_head_value(entity, schema) else {
        return false;
    };

    yesno_equal(token_def.grammar, &serialized[0], &head_value)
}

/// Compare a serialized token value against the raw head value, normalising YesNo variants.
/// PCG files store "Y"/"N" but `serialize_value` produces "YES"/"NO"; without this,
/// `should_skip_duplicate_head_token` would fail to recognise the pair as equal.
fn yesno_equal(grammar: TokenGrammar, serialized: &str, head: &str) -> bool {
    if matches!(grammar, TokenGrammar::YesNo) {
        let is_truthy = |s: &str| matches!(s.to_ascii_uppercase().as_str(), "Y" | "YES" | "TRUE" | "1");
        is_truthy(serialized) == is_truthy(head)
    } else {
        serialized == head
    }
}

fn head_name_for_entity(entity: &Entity) -> &str {
    entity
        .attributes
        .get("pcgen_key")
        .and_then(Value::as_str)
        .unwrap_or(entity.name.as_str())
}

fn top_level_separator(entity: &Entity, schema: &LineGrammar) -> &'static str {
    if let Some(style) = entity
        .attributes
        .get("pcgen_record_style")
        .and_then(Value::as_str)
    {
        return match style {
            "pipe" => "|",
            "space" => " ",
            _ => "\t",
        };
    }

    if emits_pcg_style(entity, schema) {
        "|"
    } else {
        "\t"
    }
}

fn emits_pcg_style(entity: &Entity, schema: &LineGrammar) -> bool {
    if entity
        .attributes
        .get("source_format")
        .and_then(Value::as_str)
        .is_some_and(|format| format.eq_ignore_ascii_case("pcg"))
    {
        return matches!(schema.head_format, HeadFormat::TokenPrefixed);
    }

    schema.entity_type_key.starts_with("pcgen:pcg:")
}

fn token_prefixed_head_value(entity: &Entity, schema: &LineGrammar) -> Option<String> {
    if !emits_pcg_style(entity, schema) {
        return Some(head_name_for_entity(entity).to_string());
    }

    // For PCG-style entities, prefer the raw `pcgen_decl_value` over the
    // serialized attribute value.  Serialisation normalises booleans (e.g.
    // Y→YES for yesno tokens), which changes the entity name on the second
    // parse and causes spurious attribute drops.
    //
    // Fall back to the serialized attribute value when `pcgen_decl_value` is
    // absent (e.g. artisan-constructed entities that were never parsed from
    // PCGen text).
    if let Some(raw) = entity
        .attributes
        .get("pcgen_decl_value")
        .and_then(Value::as_str)
    {
        return Some(raw.to_string());
    }

    let head_token = schema.head_token?;
    let token_def = schema.token_def(head_token)?;
    let ArtisanMapping::Field(field) = token_def.artisan_mapping else {
        return Some(head_name_for_entity(entity).to_string());
    };
    let value = entity.attributes.get(field)?;
    let serialized = serialize_value(value, token_def.grammar, token_def.cardinality);
    if serialized.len() == 1 {
        Some(serialized[0].clone())
    } else {
        Some(head_name_for_entity(entity).to_string())
    }
}

fn collect_emittable_global_keys(group: GlobalGroup, entity: &Entity, keys: &mut BTreeSet<String>) {
    match group {
        GlobalGroup::Bonus => {
            if entity
                .effects
                .iter()
                .any(|effect| effect.kind.eq_ignore_ascii_case("BONUS"))
            {
                keys.insert("BONUS".to_string());
            }
            if entity
                .effects
                .iter()
                .any(|effect| effect.kind.eq_ignore_ascii_case("TEMPBONUS"))
            {
                keys.insert("TEMPBONUS".to_string());
            }
        }
        GlobalGroup::Add => {
            if entity
                .effects
                .iter()
                .any(|effect| effect.kind.eq_ignore_ascii_case("ADD"))
            {
                keys.insert("ADD".to_string());
            }
        }
        GlobalGroup::Choose => {
            if entity
                .effects
                .iter()
                .any(|effect| effect.kind.eq_ignore_ascii_case("CHOOSE"))
            {
                keys.insert("CHOOSE".to_string());
            }
            if entity
                .effects
                .iter()
                .any(|effect| effect.kind.eq_ignore_ascii_case("SELECT"))
            {
                keys.insert("SELECT".to_string());
            }
        }
        GlobalGroup::Auto => {
            if entity
                .effects
                .iter()
                .any(|effect| effect.kind.eq_ignore_ascii_case("AUTO"))
            {
                keys.insert("AUTO".to_string());
            }
        }
        GlobalGroup::Define => {
            if entity
                .effects
                .iter()
                .any(|effect| effect.kind.eq_ignore_ascii_case("DEFINE"))
            {
                keys.insert("DEFINE".to_string());
            }
            if entity
                .effects
                .iter()
                .any(|effect| effect.kind.eq_ignore_ascii_case("DEFINESTAT"))
            {
                keys.insert("DEFINESTAT".to_string());
            }
        }
        GlobalGroup::Modify => {
            let has_projected_modify = entity.attributes.contains_key("pcgen_modify_variable")
                && entity.attributes.contains_key("pcgen_modify_operation")
                && entity.attributes.contains_key("pcgen_modify_value");
            let has_raw_modify = entity.attributes.contains_key("pcgen_modify");
            if has_projected_modify || has_raw_modify {
                keys.insert("MODIFY".to_string());
            }
        }
        GlobalGroup::Prerequisites => {
            for prereq in &entity.prerequisites {
                keys.insert(prereq.kind.to_ascii_uppercase());
            }
        }
        GlobalGroup::Type => {
            if entity
                .attributes
                .get("type")
                .and_then(Value::as_str)
                .is_some()
            {
                keys.insert("TYPE".to_string());
            }
        }
        GlobalGroup::Key => {
            if entity
                .attributes
                .get("key")
                .and_then(Value::as_str)
                .is_some()
            {
                keys.insert("KEY".to_string());
            }
        }
        GlobalGroup::Desc => {
            if entity
                .attributes
                .get("description")
                .and_then(Value::as_str)
                .is_some()
                || entity
                    .attributes
                    .get("pcgen_desc")
                    .and_then(Value::as_str)
                    .is_some()
            {
                keys.insert("DESC".to_string());
            }
            if entity
                .attributes
                .get("pcgen_desc_clear")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                keys.insert("DESC.CLEAR".to_string());
            }
            if entity
                .attributes
                .get("tempdesc")
                .and_then(Value::as_str)
                .is_some()
            {
                keys.insert("TEMPDESC".to_string());
            }
            if bool_like_attribute(entity, "pcgen_descispi").is_some() {
                keys.insert("DESCISPI".to_string());
            }
            if entity
                .attributes
                .get("pcgen_nameispi")
                .and_then(Value::as_str)
                .is_some()
            {
                keys.insert("NAMEISPI".to_string());
            }
        }
        GlobalGroup::Fact => {
            if entity
                .attributes
                .get("pcgen_facts")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("FACT".to_string());
            }
            if entity
                .attributes
                .get("fact_sets")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("FACTSET".to_string());
            }
        }
        GlobalGroup::SourcePage => {
            if entity
                .attributes
                .get("pcgen_source_page")
                .and_then(Value::as_str)
                .is_some()
            {
                keys.insert("SOURCEPAGE".to_string());
            }
        }
        GlobalGroup::SourceLink => {
            if entity
                .attributes
                .get("pcgen_source_link")
                .and_then(Value::as_str)
                .is_some()
            {
                keys.insert("SOURCELINK".to_string());
            }
        }
        GlobalGroup::OutputName => {
            if entity
                .attributes
                .get("outputname")
                .and_then(Value::as_str)
                .is_some()
            {
                keys.insert("OUTPUTNAME".to_string());
            }
        }
        GlobalGroup::SortKey => {
            if entity
                .attributes
                .get("sortkey")
                .and_then(Value::as_str)
                .is_some()
            {
                keys.insert("SORTKEY".to_string());
            }
        }
        GlobalGroup::LangBonus => {
            if entity
                .attributes
                .get("lang_bonus")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("LANGBONUS".to_string());
            }
        }
        GlobalGroup::CSkill => {
            if entity
                .attributes
                .get("cskill")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("CSKILL".to_string());
            }
        }
        GlobalGroup::Sab => {
            if entity
                .attributes
                .get("sab")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("SAB".to_string());
            }
        }
        GlobalGroup::ChangeProf => {
            if entity
                .attributes
                .get("pcgen_changeprof")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("CHANGEPROF".to_string());
            }
        }
        GlobalGroup::ServesAs => {
            if entity
                .attributes
                .get("serves_as")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("SERVESAS".to_string());
            }
        }
        GlobalGroup::Qualify => {
            if entity
                .attributes
                .get("pcgen_qualify")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("QUALIFY".to_string());
            }
        }
        GlobalGroup::Template => {
            if entity
                .attributes
                .get("pcgen_template")
                .and_then(Value::as_array)
                .is_some_and(|values| !values.is_empty())
            {
                keys.insert("TEMPLATE".to_string());
            }
        }
        GlobalGroup::SourceMeta => {}
    }
}

// ---------------------------------------------------------------------------
// Global group emission
// ---------------------------------------------------------------------------

fn emit_global_group(
    group: GlobalGroup,
    entity: &Entity,
    parts: &mut Vec<String>,
    schema_token_keys: &HashSet<String>,
) {
    match group {
        GlobalGroup::Bonus => {
            for effect in &entity.effects {
                if effect.kind.eq_ignore_ascii_case("BONUS") {
                    let s = match &effect.value {
                        Some(v) => format!("BONUS:{}|{}", effect.target, v),
                        None => format!("BONUS:{}", effect.target),
                    };
                    parts.push(s);
                } else if effect.kind.eq_ignore_ascii_case("TEMPBONUS") {
                    let s = match &effect.value {
                        Some(v) => format!("TEMPBONUS:{}|{}", effect.target, v),
                        None => format!("TEMPBONUS:{}", effect.target),
                    };
                    parts.push(s);
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
                } else if effect.kind.eq_ignore_ascii_case("SELECT") {
                    parts.push(format!("SELECT:{}", effect.target));
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
                    let s = match &effect.value {
                        Some(v) => format!("DEFINE:{}|{}", effect.target, v),
                        None => format!("DEFINE:{}", effect.target),
                    };
                    parts.push(s);
                } else if effect.kind.eq_ignore_ascii_case("DEFINESTAT") {
                    let s = match &effect.value {
                        Some(v) => format!("DEFINESTAT:{}|{}", effect.target, v),
                        None => format!("DEFINESTAT:{}", effect.target),
                    };
                    parts.push(s);
                }
            }
        }
        GlobalGroup::Modify => {
            if let (Some(variables), Some(operations), Some(values)) = (
                entity
                    .attributes
                    .get("pcgen_modify_variable")
                    .and_then(Value::as_array),
                entity
                    .attributes
                    .get("pcgen_modify_operation")
                    .and_then(Value::as_array),
                entity
                    .attributes
                    .get("pcgen_modify_value")
                    .and_then(Value::as_array),
            ) {
                let count = variables.len().min(operations.len()).min(values.len());
                for index in 0..count {
                    if let (Some(variable), Some(operation), Some(value)) = (
                        variables[index].as_str(),
                        operations[index].as_str(),
                        values[index].as_str(),
                    ) {
                        parts.push(format!("MODIFY:{variable}|{operation}|{value}"));
                    }
                }
            } else if let Some(value) = entity
                .attributes
                .get("pcgen_modify")
                .and_then(Value::as_str)
            {
                parts.push(format!("MODIFY:{value}"));
            } else if let Some(values) = entity
                .attributes
                .get("pcgen_modify")
                .and_then(Value::as_array)
            {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("MODIFY:{value}"));
                }
            }

            if let Some(value) = entity
                .attributes
                .get("pcgen_modifyother")
                .and_then(Value::as_str)
            {
                parts.push(format!("MODIFYOTHER:{value}"));
            } else if let Some(values) = entity
                .attributes
                .get("pcgen_modifyother")
                .and_then(Value::as_array)
            {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("MODIFYOTHER:{value}"));
                }
            }
        }
        GlobalGroup::Prerequisites => {
            for prereq in &entity.prerequisites {
                // Skip prerequisites whose kind matches a schema token def
                // (e.g. PRECAMPAIGN in the ability schema).  Those tokens are
                // emitted by the token-def loop and must not be double-emitted
                // here.
                let kind_upper = prereq.kind.to_ascii_uppercase();
                if schema_token_keys.contains(&kind_upper) {
                    continue;
                }
                match &prereq.expression {
                    Some(expr) => parts.push(format!("{}:{}", prereq.kind, expr)),
                    // Always emit with colon so the re-parser sees a valid
                    // token-start boundary (e.g. `PRETEXT:` with empty value).
                    None => parts.push(format!("{}:", prereq.kind)),
                }
            }
        }
        GlobalGroup::Type => {
            if let Some(type_val) = entity.attributes.get("type").and_then(Value::as_str) {
                parts.push(format!("TYPE:{type_val}"));
            }
        }
        GlobalGroup::Key => {
            if let Some(key_val) = entity.attributes.get("key").and_then(Value::as_str) {
                parts.push(format!("KEY:{key_val}"));
            }
        }
        GlobalGroup::Desc => {
            // Prefer the canonical description; pcgen_desc kept as alias for legacy data
            let desc = entity
                .attributes
                .get("description")
                .and_then(Value::as_str)
                .or_else(|| entity.attributes.get("pcgen_desc").and_then(Value::as_str));
            if let Some(desc) = desc {
                parts.push(format!("DESC:{desc}"));
            }
            if entity
                .attributes
                .get("pcgen_desc_clear")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                parts.push("DESC.CLEAR:".to_string());
            }
            if let Some(tempdesc) = entity.attributes.get("tempdesc").and_then(Value::as_str) {
                parts.push(format!("TEMPDESC:{tempdesc}"));
            }
            if let Some(desc_is_pi) = bool_like_attribute(entity, "pcgen_descispi") {
                parts.push(format!(
                    "DESCISPI:{}",
                    if desc_is_pi { "YES" } else { "NO" }
                ));
            }
            if let Some(name_is_pi) = entity.attributes.get("pcgen_nameispi").and_then(Value::as_str) {
                parts.push(format!("NAMEISPI:{name_is_pi}"));
            }
        }
        GlobalGroup::Fact => {
            if let Some(facts) = entity
                .attributes
                .get("pcgen_facts")
                .and_then(Value::as_array)
            {
                for fact in facts {
                    // Prefer the stored raw value so that whitespace within the
                    // fact (e.g. "CompMaterial| see text") is preserved exactly.
                    // Fall back to reconstructed form for facts without a raw
                    // field (e.g. facts created programmatically).
                    if let Some(raw) = fact.get("raw").and_then(Value::as_str) {
                        parts.push(format!("FACT:{raw}"));
                    } else if let Some(k) = fact.get("key").and_then(Value::as_str) {
                        if let Some(v) = fact.get("value").and_then(Value::as_str) {
                            parts.push(format!("FACT:{k}|{v}"));
                        } else {
                            // Fact with no value: emit bare FACT:key
                            parts.push(format!("FACT:{k}"));
                        }
                    }
                }
            }
            if let Some(factsets) = entity
                .attributes
                .get("fact_sets")
                .and_then(Value::as_array)
            {
                for factset in factsets {
                    // Prefer raw for FACTSET too.
                    if let Some(raw) = factset.get("raw").and_then(Value::as_str) {
                        parts.push(format!("FACTSET:{raw}"));
                    } else if let (Some(k), Some(v)) = (
                        factset.get("key").and_then(Value::as_str),
                        factset.get("value").and_then(Value::as_str),
                    ) {
                        parts.push(format!("FACTSET:{k}|{v}"));
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
        GlobalGroup::SourceLink => {
            if let Some(sl) = entity
                .attributes
                .get("pcgen_source_link")
                .and_then(Value::as_str)
            {
                parts.push(format!("SOURCELINK:{sl}"));
            }
        }
        GlobalGroup::OutputName => {
            if let Some(on) = entity.attributes.get("outputname").and_then(Value::as_str) {
                parts.push(format!("OUTPUTNAME:{on}"));
            }
        }
        GlobalGroup::SortKey => {
            if let Some(sk) = entity.attributes.get("sortkey").and_then(Value::as_str) {
                parts.push(format!("SORTKEY:{sk}"));
            }
        }
        GlobalGroup::LangBonus => {
            if let Some(values) = entity
                .attributes
                .get("lang_bonus")
                .and_then(Value::as_array)
            {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("LANGBONUS:{value}"));
                }
            }
        }
        GlobalGroup::CSkill => {
            if let Some(values) = entity.attributes.get("cskill").and_then(Value::as_array) {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("CSKILL:{value}"));
                }
            }
        }
        GlobalGroup::Sab => {
            if let Some(values) = entity.attributes.get("sab").and_then(Value::as_array) {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("SAB:{value}"));
                }
            }
        }
        GlobalGroup::ChangeProf => {
            if let Some(values) = entity
                .attributes
                .get("pcgen_changeprof")
                .and_then(Value::as_array)
            {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("CHANGEPROF:{value}"));
                }
            }
        }
        GlobalGroup::ServesAs => {
            if let Some(values) = entity
                .attributes
                .get("serves_as")
                .and_then(Value::as_array)
            {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("SERVESAS:{value}"));
                }
            }
        }
        GlobalGroup::Qualify => {
            if let Some(values) = entity
                .attributes
                .get("pcgen_qualify")
                .and_then(Value::as_array)
            {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("QUALIFY:{value}"));
                }
            }
        }
        GlobalGroup::Template => {
            if let Some(values) = entity
                .attributes
                .get("pcgen_template")
                .and_then(Value::as_array)
            {
                for value in values.iter().filter_map(Value::as_str) {
                    parts.push(format!("TEMPLATE:{value}"));
                }
            }
        }
        GlobalGroup::SourceMeta => {
            // Emit inline source meta tokens that appear on individual entity
            // lines (as opposed to the PCC-level source block).
            if let Some(v) = entity.attributes.get("pcgen_source_long").and_then(Value::as_str) {
                parts.push(format!("SOURCELONG:{v}"));
            }
            if let Some(v) = entity.attributes.get("source_short").and_then(Value::as_str) {
                parts.push(format!("SOURCESHORT:{v}"));
            }
            if let Some(v) = entity.attributes.get("source_web").and_then(Value::as_str) {
                parts.push(format!("SOURCEWEB:{v}"));
            }
            if let Some(v) = entity.attributes.get("source_date").and_then(Value::as_str) {
                parts.push(format!("SOURCEDATE:{v}"));
            }
        }
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
            TokenGrammar::YesNo => vec![if *b {
                "YES".to_string()
            } else {
                "NO".to_string()
            }],
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
                    if joined.is_empty() {
                        vec![]
                    } else {
                        vec![joined]
                    }
                }
                _ => vec![],
            })
            .collect(),

        // BracketGroup: reconstruct [KEY:val|KEY:val|...] from array of {key, value} objects
        (TokenGrammar::BracketGroup, _) => {
            let s = serialize_bracket_group(arr);
            if s.is_empty() { vec![] } else { vec![s] }
        }

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

/// Serialize a bracket group array into `[KEY:val|KEY:val|...]` notation.
///
/// Each item in `arr` should be an object with `"key"` and `"value"` fields.
/// Items that lack a key are emitted as bare values. An empty array produces an
/// empty string (caller should suppress the token entirely).
fn serialize_bracket_group(arr: &[Value]) -> String {
    if arr.is_empty() {
        return String::new();
    }
    let parts: Vec<String> = arr
        .iter()
        .filter_map(|item| {
            let val = item.get("value").and_then(Value::as_str).unwrap_or("");
            if let Some(k) = item.get("key").and_then(Value::as_str) {
                Some(format!("{k}:{val}"))
            } else if !val.is_empty() {
                Some(val.to_string())
            } else {
                None
            }
        })
        .collect();
    if parts.is_empty() {
        return String::new();
    }
    format!("[{}]", parts.join("|"))
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

fn bool_like_attribute(entity: &Entity, key: &str) -> Option<bool> {
    let value = entity.attributes.get(key)?;
    match value {
        Value::Bool(b) => Some(*b),
        Value::String(s) => match s.trim().to_ascii_uppercase().as_str() {
            "YES" | "Y" | "TRUE" => Some(true),
            "NO" | "N" | "FALSE" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Convenience: build ParsedClause list from entity + schema
// ---------------------------------------------------------------------------

/// Reconstruct the `ParsedClause` list that `emit_entity` would produce.
///
/// The head token is excluded — only the clause tokens are returned.
pub fn entity_to_clauses(entity: &Entity, schema: &LineGrammar) -> Vec<ParsedClause> {
    let line_text = emit_entity(entity, schema);
    let parsed = crate::parse_line(&line_text);
    parsed.clauses
}
