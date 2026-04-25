use std::{fs, io, path::Path};

use artisan_core::{
    CanonicalId, Entity, EntityType,
    domain::{
        CitationLocator, CitationRecord, PublisherRecord, SourceRecord, SubjectRef,
        VerificationState, entity::CompletenessState,
    },
    id::{ExternalId, FormatId},
    reconcile::{ImportCandidate, SourceHint},
};
use indexmap::IndexMap;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use uuid::Uuid;

mod analysis;
mod emit;
mod parsing;
pub mod schema;

use analysis::{fields, metadata, semantics, signals};
pub use emit::fallback_keys_for_entity;
pub use emit::{emit_entity, emit_entity_auto, emittable_keys_for_entity};
pub use artisan_core::ParsedCatalog;
use parsing::line_codec;

const ENTITY_TYPE_NAMESPACE: Uuid = Uuid::from_u128(0x6c8fdbf43f4f4a4ba4d846e2bf8b9c10);
const ENTITY_NAMESPACE: Uuid = Uuid::from_u128(0x5ea8a1062b0842beaf2fcb5966e30f3a);
const PUBLISHER_NAMESPACE: Uuid = Uuid::from_u128(0x4a3decc22d7745618872f8361653dc61);
const SOURCE_NAMESPACE: Uuid = Uuid::from_u128(0x17a9126be16f4ddcbfce46c80dd60f2f);
const CITATION_NAMESPACE: Uuid = Uuid::from_u128(0x0ab04cb4229a4ba39f5947655e0f4d28);

#[derive(Debug, Clone)]
pub struct ParsedEntityCandidate {
    pub entity: Entity,
    pub entity_type_key: String,
    pub source_hints: Vec<SourceHint>,
}

impl ParsedEntityCandidate {
    pub fn into_import_candidate(self) -> ImportCandidate<Entity> {
        ImportCandidate {
            external_ids: self.entity.external_ids.clone(),
            display_name: Some(self.entity.name.clone()),
            source_hints: self.source_hints,
            provenance: self.entity.provenance.clone(),
            payload: self.entity,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParsedCampaign {
    pub metadata: BTreeMap<String, Vec<String>>,
}

pub struct PcgenLoader;

impl PcgenLoader {
    pub fn parse_entities(input: &str) -> Result<Vec<Entity>, String> {
        Ok(parse_text_to_catalog(input, "inline", "lst").entities)
    }

    pub fn parse_pcg(input: &str) -> Result<Vec<Entity>, String> {
        Ok(parse_text_to_catalog(input, "inline", "pcg").entities)
    }

    pub fn parse_entity_candidates(input: &str) -> Result<Vec<ParsedEntityCandidate>, String> {
        Self::parse_entity_candidates_with_context(input, None, None)
    }

    pub fn parse_entity_candidates_with_context(
        input: &str,
        game_system_hint: Option<&str>,
        source_title_hint: Option<&str>,
    ) -> Result<Vec<ParsedEntityCandidate>, String> {
        let catalog = parse_text_to_catalog(input, "inline", "lst");
        let mut out = Vec::with_capacity(catalog.entities.len());

        for entity in catalog.entities {
            let entity_type_key = entity
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .unwrap_or_else(|| "pcgen:type:unresolved".to_string());

            let mut source_hints = Vec::new();
            if source_title_hint.is_some() || game_system_hint.is_some() {
                source_hints.push(SourceHint {
                    title: source_title_hint.map(ToString::to_string),
                    publisher: None,
                    game_system: game_system_hint.map(ToString::to_string),
                });
            }

            out.push(ParsedEntityCandidate {
                entity,
                entity_type_key,
                source_hints,
            });
        }

        Ok(out)
    }

    pub fn parse_pcc(input: &str) -> Result<ParsedCampaign, String> {
        let normalized;
        let input = if input.contains('\r') {
            normalized = input.replace("\r\n", "\n").replace('\r', "\n");
            normalized.as_str()
        } else {
            input
        };
        let mut campaign = ParsedCampaign::default();
        for raw_line in input.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parsed = parse_line(line);
            if let Some((key, value)) = line_codec::split_first_key_value(&parsed) {
                campaign
                    .metadata
                    .entry(key.to_ascii_uppercase())
                    .or_default()
                    .push(value);
            }
        }
        Ok(campaign)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedLine {
    pub head: String,
    pub clauses: Vec<ParsedClause>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedClause {
    Bare(String),
    KeyValue { key: String, value: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSupportLevel {
    SemanticallyInterpreted,
    PolicySupported,
    Unhandled(String),
    Artifact,
}

pub fn classify_clause_handling(clause: &ParsedClause) -> TokenSupportLevel {
    match analysis::token_policy::classify_clause_token(clause) {
        analysis::token_policy::ClauseSupportLevel::SemanticallyInterpreted => {
            TokenSupportLevel::SemanticallyInterpreted
        }
        analysis::token_policy::ClauseSupportLevel::PolicySupported => {
            TokenSupportLevel::PolicySupported
        }
        analysis::token_policy::ClauseSupportLevel::Unhandled(token_key) => {
            TokenSupportLevel::Unhandled(token_key)
        }
        analysis::token_policy::ClauseSupportLevel::Artifact => TokenSupportLevel::Artifact,
    }
}

pub fn classify_token_key_support(token_key: &str, is_bare: bool) -> TokenSupportLevel {
    match analysis::token_policy::classify_token_key(token_key, is_bare) {
        analysis::token_policy::ClauseSupportLevel::SemanticallyInterpreted => {
            TokenSupportLevel::SemanticallyInterpreted
        }
        analysis::token_policy::ClauseSupportLevel::PolicySupported => {
            TokenSupportLevel::PolicySupported
        }
        analysis::token_policy::ClauseSupportLevel::Unhandled(token_key) => {
            TokenSupportLevel::Unhandled(token_key)
        }
        analysis::token_policy::ClauseSupportLevel::Artifact => TokenSupportLevel::Artifact,
    }
}

pub fn parse_file(path: &Path) -> io::Result<ParsedCatalog> {
    let bytes = fs::read(path)?;
    let text = String::from_utf8_lossy(&bytes).to_string();
    let source_name = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("fixture");
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_else(|| "unknown".to_string());
    Ok(parse_text_to_catalog(&text, source_name, &ext))
}

/// Find an existing EntityType whose PCGen schema key ExternalId matches
/// `schema_key`, or create one with a deterministic id and insert it.
/// Returns the `CanonicalId` of the found-or-created type.
///
/// `entity_types` is keyed by PCGen schema key (e.g. `"pcgen:entity:race"`)
/// and acts as the accumulator during a single `parse_text_to_catalog` call.
/// `game_system` is `None` during the parse loop and is back-filled after
/// the PCC metadata has been read.
fn find_or_create_entity_type(
    entity_types: &mut BTreeMap<String, EntityType>,
    schema_key: &str,
) -> CanonicalId {
    if let Some(et) = entity_types.get(schema_key) {
        return et.id;
    }
    let id = deterministic_id(ENTITY_TYPE_NAMESPACE, schema_key);
    let display_name = schema_key
        .rsplit(':')
        .next()
        .unwrap_or(schema_key)
        .replace('-', " ")
        .split_whitespace()
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    entity_types.insert(
        schema_key.to_string(),
        EntityType {
            id,
            key: schema_key.to_string(),
            name: display_name,
            game_system: None,
            parent: None,
            fields: Vec::new(),
            relationships: Vec::new(),
            external_ids: vec![ExternalId {
                format: FormatId::Pcgen,
                namespace: Some("entity_type_key".to_string()),
                value: schema_key.to_string(),
            }],
            provenance: None,
        },
    );
    id
}

pub fn parse_text_to_catalog(text: &str, source_name: &str, ext: &str) -> ParsedCatalog {
    // Normalize line endings: Rust's str::lines() splits only on \n and \r\n,
    // not on lone \r (classic Mac CR-only files). Convert \r\n → \n first to
    // avoid double-splitting, then \r → \n.
    let normalized;
    let text = if text.contains('\r') {
        normalized = text.replace("\r\n", "\n").replace('\r', "\n");
        normalized.as_str()
    } else {
        text
    };

    // Accumulates per-schema EntityTypes as entities are parsed.
    // Keyed by PCGen schema key, e.g. "pcgen:entity:race".
    let mut entity_types: BTreeMap<String, EntityType> = BTreeMap::new();

    let mut metadata = metadata::PcgenMetadata::default();
    let mut entities = Vec::new();
    let mut citations = Vec::new();
    let mut skipped_unknown_clause_counts: BTreeMap<String, usize> = BTreeMap::new();
    // LST multi-line entity index: (decl_token_upper, decl_value) → index into `entities`.
    // When a second `CLASS:Faceman` line appears we merge into the first rather than creating
    // a duplicate. Only applies to `lst` format token-entry records.
    let mut lst_entity_index: BTreeMap<(String, String), usize> = BTreeMap::new();
    for (line_number, raw_line) in text.lines().enumerate() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let parsed_line = parse_line(trimmed);
        metadata::collect_metadata(&parsed_line, trimmed, &mut metadata);
        let mut supported_clauses = Vec::new();
        for clause in &parsed_line.clauses {
            match analysis::token_policy::classify_clause_token(clause) {
                analysis::token_policy::ClauseSupportLevel::SemanticallyInterpreted
                | analysis::token_policy::ClauseSupportLevel::PolicySupported => {
                    supported_clauses.push(clause.clone());
                }
                analysis::token_policy::ClauseSupportLevel::Unhandled(token_key) => {
                    *skipped_unknown_clause_counts.entry(token_key).or_insert(0) += 1;
                }
                analysis::token_policy::ClauseSupportLevel::Artifact => {}
            }
        }

        let name = semantics::derive_entity_name(&parsed_line.head, &supported_clauses)
            .unwrap_or_else(|| parsed_line.head.trim().to_string());
        let name = if name.is_empty() {
            format!("line_{}", line_number + 1)
        } else {
            name
        };
        let head_name = semantics::declared_entity(&parsed_line.head)
            .map(|(_, value)| value)
            .unwrap_or_else(|| parsed_line.head.trim().to_string());
        let entity_id = deterministic_id(
            ENTITY_NAMESPACE,
            &format!("{source_name}:{ext}:{}:{trimmed}", line_number + 1),
        );

        let mut attributes = IndexMap::new();
        attributes.insert("head".to_string(), Value::String(parsed_line.head.clone()));
        attributes.insert(
            "clauses".to_string(),
            json!(line_codec::clauses_to_json(&supported_clauses)),
        );
        if let Some((decl_key, decl_value)) = semantics::declared_entity(&parsed_line.head) {
            attributes.insert("pcgen_decl_token".to_string(), Value::String(decl_key));
            attributes.insert("pcgen_decl_value".to_string(), Value::String(decl_value));
        }
        attributes.insert("line_number".to_string(), json!(line_number + 1));
        attributes.insert("pcgen_line_number".to_string(), json!(line_number + 1));
        attributes.insert("source_format".to_string(), Value::String(ext.to_string()));
        fields::project_clause_attributes(&head_name, &supported_clauses, &mut attributes);
        if let Some((decl_key, decl_value)) = semantics::declared_entity(&parsed_line.head) {
            fields::project_decl_token_value(&decl_key, &decl_value, &mut attributes);
        }

        let mut entity_citations = Vec::new();
        if let Some(source_page) = line_codec::find_key_value(&supported_clauses, "SOURCEPAGE") {
            attributes.insert(
                "pcgen_source_page".to_string(),
                Value::String(source_page.clone()),
            );

            // Build all available locators from SOURCEPAGE + optional SOURCELONG/SOURCELINK
            let mut locators = vec![CitationLocator {
                kind: "page".to_string(),
                value: source_page.clone(),
                canonical: true,
            }];
            if let Some(source_long) = line_codec::find_key_value(&supported_clauses, "SOURCELONG")
            {
                locators.push(CitationLocator {
                    kind: "long".to_string(),
                    value: source_long,
                    canonical: false,
                });
            }
            if let Some(source_link) = line_codec::find_key_value(&supported_clauses, "SOURCELINK")
            {
                locators.push(CitationLocator {
                    kind: "link".to_string(),
                    value: source_link,
                    canonical: false,
                });
            }

            let citation_id = deterministic_id(
                CITATION_NAMESPACE,
                &format!("{source_name}:{ext}:{}:{source_page}", line_number + 1),
            );
            citations.push(CitationRecord {
                id: citation_id,
                subject: SubjectRef::Entity(entity_id),
                source: CanonicalId(Uuid::nil()),
                locators,
                verification: VerificationState::Unverified,
                external_ids: vec![ExternalId {
                    format: FormatId::Pcgen,
                    namespace: Some("citation".to_string()),
                    value: format!("{}:{}:sourcepage", source_name, line_number + 1),
                }],
            });
            entity_citations.push(citation_id);
        }

        let inferred_type_key =
            semantics::infer_entity_type_key_for_format(&parsed_line.head, &supported_clauses, ext);
        attributes.insert(
            "pcgen_record_family".to_string(),
            Value::String(infer_record_family(ext, &parsed_line, &inferred_type_key).to_string()),
        );
        attributes.insert(
            "pcgen_record_style".to_string(),
            Value::String(
                infer_record_style(trimmed, ext, &parsed_line, &inferred_type_key).to_string(),
            ),
        );
        attributes.insert(
            "pcgen_entity_type_key".to_string(),
            Value::String(inferred_type_key.clone()),
        );
        let schema_entity_type_id = find_or_create_entity_type(&mut entity_types, &inferred_type_key);

        // Deduplicate identical (key, value) clauses from the same line (e.g. STARTFEATS:1\tSTARTFEATS:1).
        // Keep-first for Once-cardinality tokens only.  Skip deduplication when:
        //   (a) the token has Cardinality::Repeatable in the entity's specific token list, or
        //   (b) the key matches any of the entity's GlobalGroups (BONUS, ABILITY, etc.) —
        //       GlobalGroups are always repeatable and may appear many times on one emitted line
        //       when continuation lines from the source are merged into a single entity.
        {
            let entity_schema = schema::schema_for_entity_type_key(&inferred_type_key);
            let mut seen_kvs: std::collections::HashSet<(String, String)> =
                std::collections::HashSet::new();
            supported_clauses.retain(|clause| {
                if let ParsedClause::KeyValue { key, value } = clause {
                    let upper = key.to_ascii_uppercase();
                    let is_repeatable = entity_schema.is_some_and(|s| {
                        s.tokens
                            .iter()
                            .any(|t| t.key.eq_ignore_ascii_case(key) && t.cardinality == schema::Cardinality::Repeatable)
                        || s.globals.iter().any(|g| g.matches(&upper))
                    });
                    if is_repeatable {
                        return true;
                    }
                    seen_kvs.insert((key.clone(), value.clone()))
                } else {
                    true
                }
            });
        }

        let mechanical_signals = signals::extract_mechanical_signals(&supported_clauses);
        if !mechanical_signals.is_empty() {
            attributes.insert(
                "pcgen_mechanical_signals".to_string(),
                json!(mechanical_signals),
            );
        }

        let external_id = ExternalId {
            format: FormatId::Pcgen,
            namespace: Some(ext.to_string()),
            value: format!("{}:{}", source_name, line_number + 1),
        };

        let mut effects = Vec::new();
        let mut prerequisites = Vec::new();
        semantics::project_semantics(&supported_clauses, &mut effects, &mut prerequisites);

        // For LST token-entry records, check if an entity with the same
        // (decl_token, decl_value) already exists on a prior line.  In PCGen
        // LST files a single logical entity (e.g. CLASS:Faceman) is often
        // split across two or more lines to stay within line-length limits.
        // We merge subsequent lines into the first rather than producing
        // duplicate entities.
        //
        // Only applies to entity types that PCGen documents as supporting
        // multi-line continuation.  Some LST files (e.g. biosettings) use
        // repeated heads for genuinely distinct records (different age
        // maturity brackets), so we must not merge those.
        let merge_key: Option<(String, String)> = if ext.eq_ignore_ascii_case("lst")
            && is_multiline_lst_entity_type(&inferred_type_key)
        {
            if let (Some(Value::String(tok)), Some(Value::String(val))) = (
                attributes.get("pcgen_decl_token"),
                attributes.get("pcgen_decl_value"),
            ) {
                Some((tok.clone(), val.clone()))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(ref key) = merge_key {
            if let Some(&existing_idx) = lst_entity_index.get(key) {
                // Merge: fold the new line's projected attributes into the
                // existing entity, skipping identity/structural fields that
                // belong to the first line only.
                let existing: &mut Entity = &mut entities[existing_idx];
                // CLASS entities use multi-line continuation to spread data
                // across many CLASS:Name lines.  Each continuation line may add
                // new Repeatable-token values (WEAPONBONUS, ABILITY, …) that
                // must accumulate into the existing arrays.  For all other
                // entity types the first-wins (or_insert) rule is correct:
                // e.g. ABILITY:Feat kit lines are the same entity redeclared in
                // different kit contexts — their decl-value projection should
                // not be duplicated.
                //
                // Only extend arrays for Cardinality::Repeatable fields: those
                // emit each element as a separate token, so accumulating them
                // across continuation lines produces a stable roundtrip.
                // Cardinality::Once fields (even if stored as arrays via
                // append_string_attr) are emitted as a single pipe-joined token,
                // so extending them would produce a multi-element array that
                // re-parses back as a single element — a mismatch.
                let class_schema = (inferred_type_key == "pcgen:entity:class")
                    .then(|| schema::schema_for_entity_type_key("pcgen:entity:class"))
                    .flatten();
                for (attr_key, attr_val) in attributes {
                    match attr_key.as_str() {
                        // Keep the first line's identity fields unchanged.
                        "head"
                        | "clauses"
                        | "line_number"
                        | "pcgen_line_number"
                        | "pcgen_record_family"
                        | "pcgen_record_style"
                        | "pcgen_entity_type_key"
                        | "source_format" => {}
                        _ => {
                            // For CLASS continuation lines, extend arrays only
                            // when the backing schema token is Repeatable.
                            let should_extend = class_schema
                                .and_then(|s| {
                                    s.tokens.iter().find(|t| {
                                        matches!(
                                            &t.artisan_mapping,
                                            schema::ArtisanMapping::Field(f) if *f == attr_key
                                        ) && t.cardinality == schema::Cardinality::Repeatable
                                    })
                                })
                                .is_some();

                            if should_extend {
                                match (existing.attributes.get_mut(&attr_key), &attr_val) {
                                    (Some(Value::Array(existing_arr)), Value::Array(new_arr)) => {
                                        existing_arr.extend(new_arr.iter().cloned());
                                    }
                                    _ => {
                                        existing.attributes.entry(attr_key).or_insert(attr_val);
                                    }
                                }
                            } else {
                                existing.attributes.entry(attr_key).or_insert(attr_val);
                            }
                        }
                    }
                }
                // Carry forward effects, prerequisites, and citation ids.
                existing.effects.extend(effects);
                existing.prerequisites.extend(prerequisites);
                // entity_citations contains CanonicalIds of CitationRecords that were
                // already pushed to the top-level `citations` vec above.
                // Re-point those citations to the FIRST entity's id so that the
                // semantic snapshot resolves them to the correct entity name.
                // (Citation subjects were created with the continuation line's entity_id,
                // which is never inserted into catalog.entities.)
                let first_entity_id = existing.id;
                for cit_id in &entity_citations {
                    if let Some(cit) = citations.iter_mut().find(|c| c.id == *cit_id) {
                        cit.subject = SubjectRef::Entity(first_entity_id);
                    }
                }
                existing.citations.extend(entity_citations);
                continue;
            }
        }

        let new_idx = entities.len();
        entities.push(Entity {
            id: entity_id,
            entity_type: schema_entity_type_id,
            name,
            attributes,
            effects,
            prerequisites,
            rule_hooks: Vec::new(),
            citations: entity_citations,
            external_ids: vec![external_id],
            completeness: CompletenessState::Descriptive,
            provenance: None,
        });
        if let Some(key) = merge_key {
            lst_entity_index.insert(key, new_idx);
        }
    }

    if !skipped_unknown_clause_counts.is_empty() {
        let total_skipped: usize = skipped_unknown_clause_counts.values().sum();
        eprintln!(
            "PCGen parse skipped {total_skipped} unknown clauses across {} token kinds while parsing {source_name}",
            skipped_unknown_clause_counts.len()
        );
        for (token_key, count) in skipped_unknown_clause_counts {
            eprintln!("  UNKNOWN {count:>6} | {token_key}");
        }
    }

    let publisher_name = metadata
        .publisher_long
        .clone()
        .or(metadata.publisher_short.clone())
        .filter(|s| !s.trim().is_empty() && !s.eq_ignore_ascii_case("na"));
    let source_title = metadata
        .source_title
        .clone()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| metadata.campaign.clone().filter(|s| !s.trim().is_empty()))
        .or_else(|| {
            metadata
                .source_short
                .clone()
                .filter(|s| !s.trim().is_empty())
        })
        .unwrap_or_else(|| source_name.to_string());

    let source_id = deterministic_id(
        SOURCE_NAMESPACE,
        &format!("pcgen:{ext}:{source_name}:{source_title}"),
    );

    let mut publishers = Vec::new();
    let mut publisher_ids = Vec::new();
    if let Some(name) = publisher_name {
        let publisher_id =
            deterministic_id(PUBLISHER_NAMESPACE, &format!("pcgen:publisher:{name}"));
        publisher_ids.push(publisher_id);
        publishers.push(PublisherRecord {
            id: publisher_id,
            name: name.clone(),
            external_ids: vec![ExternalId {
                format: FormatId::Pcgen,
                namespace: Some("publisher".to_string()),
                value: name,
            }],
        });
    }

    let mut game_systems = Vec::new();
    if let Some(game_mode) = metadata.game_mode.filter(|s| !s.trim().is_empty()) {
        game_systems.push(game_mode);
    }
    if let Some(setting) = metadata.setting.filter(|s| !s.trim().is_empty()) {
        for part in setting.split('|') {
            let trimmed = part.trim();
            if !trimmed.is_empty() && !game_systems.iter().any(|g| g.eq_ignore_ascii_case(trimmed))
            {
                game_systems.push(trimmed.to_string());
            }
        }
    }

    let mut source_external_ids = vec![ExternalId {
        format: FormatId::Pcgen,
        namespace: Some("source".to_string()),
        value: format!("{ext}:{source_name}"),
    }];
    if let Some(short) = metadata
        .source_short
        .clone()
        .filter(|s| !s.trim().is_empty())
    {
        source_external_ids.push(ExternalId {
            format: FormatId::Pcgen,
            namespace: Some("source_short".to_string()),
            value: short,
        });
    }
    if let Some(web) = metadata.source_web.clone().filter(|s| !s.trim().is_empty()) {
        source_external_ids.push(ExternalId {
            format: FormatId::Pcgen,
            namespace: Some("source_web".to_string()),
            value: web,
        });
    }

    let source = SourceRecord {
        id: source_id,
        title: source_title,
        publisher: publishers.first().map(|p| p.name.clone()),
        publisher_ids,
        edition: metadata.source_date.filter(|s| !s.trim().is_empty()),
        license: None,
        game_systems,
        external_ids: source_external_ids,
    };

    for citation in &mut citations {
        citation.source = source_id;
    }

    // Back-fill game_system onto every EntityType created during parsing.
    // The primary game system is the first GAMEMODE value (if any).
    let primary_game_system = source.game_systems.first().cloned();
    if let Some(ref gs) = primary_game_system {
        for et in entity_types.values_mut() {
            et.game_system = Some(gs.clone());
        }
    }

    ParsedCatalog {
        publishers,
        sources: vec![source],
        citations,
        entity_types: entity_types.into_values().collect(),
        entities,
        ..ParsedCatalog::default()
    }
}

pub fn unparse_catalog_to_text(catalog: &ParsedCatalog) -> String {
    let mut lines = Vec::new();
    for entity in ordered_entities_for_unparse(catalog) {
        // Prefer schema-driven emission; fall back to raw clause reconstruction.
        if let Some(line) = emit_entity_auto(entity) {
            lines.push(line);
        } else {
            let head = entity
                .attributes
                .get("head")
                .and_then(Value::as_str)
                .unwrap_or(&entity.name)
                .to_string();
            let clauses = entity
                .attributes
                .get("clauses")
                .and_then(line_codec::clauses_from_json)
                .unwrap_or_default();
            let separator = match entity
                .attributes
                .get("pcgen_record_style")
                .and_then(Value::as_str)
            {
                Some("pipe") => "|",
                _ => "\t",
            };
            lines.push(line_codec::unparse_line_internal_with_separator(
                &head, &clauses, separator,
            ));
        }
    }
    lines.join("\n")
}

pub fn parse_line(line: &str) -> ParsedLine {
    line_codec::parse_line_internal(line)
}

pub fn unparse_line(head: &str, clauses: &[ParsedClause]) -> String {
    line_codec::unparse_line_internal(head, clauses)
}

fn infer_record_style<'a>(
    raw_line: &'a str,
    ext: &str,
    parsed_line: &ParsedLine,
    inferred_type_key: &str,
) -> &'a str {
    if raw_line.contains('\t') {
        return "tab";
    }

    if ext.eq_ignore_ascii_case("pcg")
        && line_codec::parse_head_key_value(&parsed_line.head).is_some()
    {
        return "pipe";
    }

    if ext.eq_ignore_ascii_case("pcc")
        && inferred_type_key == "pcgen:entity:pcc"
        && line_codec::parse_head_key_value(&parsed_line.head).is_some()
        && !parsed_line.clauses.is_empty()
    {
        return "space";
    }

    // LST lines that use pipe as the top-level separator (no tabs) must be
    // emitted back with `|` so the fallback emitter produces lines that the
    // second-pass parser can split correctly.  Without this, the fallback uses
    // `\t`, which gets absorbed into the head segment on re-parse (because
    // bare-clause segments like `GROUP=…` and `SET` have no colon and do not
    // trigger whitespace-token-start splitting).  Only set "pipe" when the raw
    // line actually contains a `|`; single-token lines without a pipe should
    // remain "tab" (the separator is irrelevant for them but "tab" is the
    // conventional default).
    if ext.eq_ignore_ascii_case("lst") && raw_line.contains('|') {
        return "pipe";
    }

    "tab"
}

fn infer_record_family(
    ext: &str,
    parsed_line: &ParsedLine,
    inferred_type_key: &str,
) -> &'static str {
    match ext.to_ascii_lowercase().as_str() {
        "pcg" => {
            if line_codec::parse_head_key_value(&parsed_line.head).is_some() {
                "pcg:token-record"
            } else {
                "pcg:name-record"
            }
        }
        "pcc" => {
            if inferred_type_key == "pcgen:entity:pcc" {
                "pcc:directive"
            } else {
                "pcc:include"
            }
        }
        _ => {
            if line_codec::parse_head_key_value(&parsed_line.head).is_some() {
                "lst:token-entry"
            } else {
                "lst:name-entry"
            }
        }
    }
}

/// Returns `true` if entities of the given type key support multi-line
/// continuation in PCGen LST files (i.e. two lines with the same head token
/// and name should be merged into one logical entity).
///
/// PCGen's documentation explicitly describes multi-line continuation for
/// game-rule entity types like CLASS, RACE, FEAT, ABILITY, SPELL, EQUIPMENT,
/// SKILL, DEITY, and TEMPLATE.  System-configuration LST types (biosettings,
/// sizeatttributes, etc.) use repeated heads for *distinct* records (e.g.
/// different age maturity brackets) and must NOT be merged.
fn is_multiline_lst_entity_type(type_key: &str) -> bool {
    matches!(
        type_key,
        "pcgen:entity:class"
            | "pcgen:entity:subclass"
            | "pcgen:entity:race"
            | "pcgen:entity:feat"
            | "pcgen:entity:ability"
            | "pcgen:entity:spell"
            | "pcgen:entity:equipment"
            | "pcgen:entity:equipmod"
            | "pcgen:entity:skill"
            | "pcgen:entity:deity"
            | "pcgen:entity:template"
            | "pcgen:entity:language"
            | "pcgen:entity:companionmod"
            | "pcgen:entity:kit"
    )
}

fn ordered_entities_for_unparse(catalog: &ParsedCatalog) -> Vec<&Entity> {
    let mut indexed: Vec<(usize, &Entity)> = catalog.entities.iter().enumerate().collect();
    indexed.sort_by(|(left_idx, left), (right_idx, right)| {
        compare_entities_for_unparse(left, right).then_with(|| left_idx.cmp(right_idx))
    });
    indexed.into_iter().map(|(_, entity)| entity).collect()
}

fn compare_entities_for_unparse(left: &Entity, right: &Entity) -> std::cmp::Ordering {
    let left_format = entity_attr_str(left, "source_format");
    let right_format = entity_attr_str(right, "source_format");

    left_format
        .cmp(&right_format)
        .then_with(
            || match (entity_line_number(left), entity_line_number(right)) {
                (Some(a), Some(b)) => a.cmp(&b),
                _ => std::cmp::Ordering::Equal,
            },
        )
        .then_with(|| {
            record_family_priority(left_format, entity_attr_str(left, "pcgen_record_family")).cmp(
                &record_family_priority(
                    right_format,
                    entity_attr_str(right, "pcgen_record_family"),
                ),
            )
        })
        .then_with(|| {
            entity_attr_str(left, "pcgen_decl_token")
                .cmp(&entity_attr_str(right, "pcgen_decl_token"))
        })
        .then_with(|| left.name.cmp(&right.name))
}

fn entity_line_number(entity: &Entity) -> Option<u64> {
    entity
        .attributes
        .get("pcgen_line_number")
        .and_then(Value::as_u64)
        .or_else(|| entity.attributes.get("line_number").and_then(Value::as_u64))
}

fn entity_attr_str<'a>(entity: &'a Entity, key: &str) -> &'a str {
    entity
        .attributes
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
}

fn record_family_priority(source_format: &str, family: &str) -> u8 {
    match (source_format, family) {
        ("pcc", "pcc:directive") => 0,
        ("pcc", "pcc:include") => 1,
        ("pcg", "pcg:token-record") => 0,
        ("pcg", "pcg:name-record") => 1,
        ("lst", "lst:name-entry") => 0,
        ("lst", "lst:token-entry") => 1,
        _ => 9,
    }
}

fn deterministic_id(namespace: Uuid, key: &str) -> CanonicalId {
    CanonicalId(Uuid::new_v5(&namespace, key.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema;

    #[test]
    fn parse_line_splits_clauses_and_key_values() {
        let parsed = parse_line("Feat Name|TYPE:Combat|PREVARGTEQ:STR,13|VISIBLE");
        assert_eq!(parsed.head, "Feat Name");
        assert_eq!(parsed.clauses.len(), 3);
        assert!(matches!(
            &parsed.clauses[0],
            ParsedClause::KeyValue { key, value } if key == "TYPE" && value == "Combat"
        ));
        assert!(matches!(
            &parsed.clauses[2],
            ParsedClause::Bare(value) if value == "VISIBLE"
        ));
    }

    #[test]
    fn parse_line_splits_whitespace_delimited_top_level_tokens() {
        let parsed = parse_line(
            "CLASS:Psion           HD:4 TYPE:Base.Psionic.PC BONUS:COMBAT|BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")/2|TYPE=Base.REPLACE",
        );

        assert_eq!(parsed.head, "CLASS:Psion");
        assert!(matches!(
            &parsed.clauses[0],
            ParsedClause::KeyValue { key, value } if key == "HD" && value == "4"
        ));
        assert!(matches!(
            &parsed.clauses[1],
            ParsedClause::KeyValue { key, value } if key == "TYPE" && value == "Base.Psionic.PC"
        ));
        assert!(matches!(
            &parsed.clauses[2],
            ParsedClause::KeyValue { key, value }
                if key == "BONUS" && value == "COMBAT|BASEAB|classlevel(\"APPLIEDAS=NONEPIC\")/2|TYPE=Base.REPLACE"
        ));
    }

    #[test]
    fn parse_line_does_not_split_on_mixed_case_text_that_looks_like_token() {
        let parsed =
            parse_line("Feat Name DESC:This references Type:Combat in prose TYPE:General.Combat");

        assert_eq!(parsed.head, "Feat Name");
        assert_eq!(parsed.clauses.len(), 2);
        assert!(matches!(
            &parsed.clauses[0],
            ParsedClause::KeyValue { key, value }
                if key == "DESC" && value == "This references Type:Combat in prose"
        ));
        assert!(matches!(
            &parsed.clauses[1],
            ParsedClause::KeyValue { key, value }
                if key == "TYPE" && value == "General.Combat"
        ));
    }

    #[test]
    fn parse_and_unparse_line_preserves_escaped_separators() {
        let original = r"Name\|WithPipe|DESC:Use \: carefully|TAG:ONE\|TWO";
        let parsed = parse_line(original);
        let reparsed = unparse_line(&parsed.head, &parsed.clauses);
        assert_eq!(reparsed, original);
    }

    #[test]
    fn parse_line_keeps_bracketed_pipe_groups_together() {
        let parsed = parse_line("WEAPONPROF:[WEAPON:Longsword|WEAPON:Dagger|WEAPON:Quarterstaff]");
        assert_eq!(
            parsed.head,
            "WEAPONPROF:[WEAPON:Longsword|WEAPON:Dagger|WEAPON:Quarterstaff]"
        );
        assert!(parsed.clauses.is_empty());

        let deity = parse_line(
            "DEITY:Pelor|DEITYDOMAINS:[DOMAIN:Good|DOMAIN:Sun]|DEITYFAVWEAP:[WEAPON:Morningstar]",
        );
        assert_eq!(deity.head, "DEITY:Pelor");
        assert_eq!(
            deity.clauses,
            vec![
                ParsedClause::KeyValue {
                    key: "DEITYDOMAINS".to_string(),
                    value: "[DOMAIN:Good|DOMAIN:Sun]".to_string(),
                },
                ParsedClause::KeyValue {
                    key: "DEITYFAVWEAP".to_string(),
                    value: "[WEAPON:Morningstar]".to_string(),
                },
            ]
        );
    }

    #[test]
    fn parse_text_projects_pre_and_effect_semantics() {
        let catalog = parse_text_to_catalog(
            "Power Attack|PREVARGTEQ:STR,13|BONUS:COMBAT|TOHIT|-1",
            "sample.lst",
            "lst",
        );
        assert_eq!(catalog.entities.len(), 1);
        let entity = &catalog.entities[0];
        assert!(
            entity
                .prerequisites
                .iter()
                .any(|p| p.kind == "PREVARGTEQ" && p.expression.as_deref() == Some("STR,13"))
        );
        assert!(
            entity
                .effects
                .iter()
                .any(|e| e.kind == "BONUS" && e.target == "COMBAT")
        );

        let catalog = parse_text_to_catalog(
            "No Swim Speed VISIBLE:NO !PREMOVE:1,Swim=1 MOVECLONE:Walk,Swim,*1",
            "templates.lst",
            "lst",
        );
        let entity = &catalog.entities[0];
        assert!(
            entity
                .prerequisites
                .iter()
                .any(|p| p.kind == "!PREMOVE" && p.expression.as_deref() == Some("1,Swim=1"))
        );
    }

    #[test]
    fn parse_text_uses_declared_entity_head_for_name_and_type() {
        let catalog = parse_text_to_catalog(
            "SKILL:Bluff   RANK:9\nCLASS:Psion   HD:4 TYPE:Base.Psionic.PC",
            "sample.lst",
            "lst",
        );

        assert_eq!(catalog.entities[0].name, "Bluff");
        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("pcgen_decl_token")
                .and_then(Value::as_str),
            Some("SKILL")
        );
        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:skill")
        );

        assert_eq!(catalog.entities[1].name, "Psion");
        assert_eq!(
            catalog.entities[1]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:class")
        );
    }

    #[test]
    fn parse_text_infers_pcg_specific_class_and_skill_record_types() {
        let catalog = parse_text_to_catalog(
            "CLASS:Wizard|SUBCLASS:Evoker|LEVEL:8|SKILLPOOL:23|PROHIBITED:Conjuration\nSKILL:Spellcraft|OUTPUTORDER:24|CLASSBOUGHT:[CLASS:Wizard|RANKS:11.0|COST:1|CLASSSKILL:Y]",
            "sample.pcg",
            "pcg",
        );

        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:pcg:class")
        );
        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("level")
                .and_then(Value::as_i64),
            Some(8)
        );

        assert_eq!(
            catalog.entities[1]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:pcg:skill")
        );
        assert_eq!(
            catalog.entities[1]
                .attributes
                .get("output_order")
                .and_then(Value::as_i64),
            Some(24)
        );
        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("pcgen_record_style")
                .and_then(Value::as_str),
            Some("pipe")
        );
        assert_eq!(
            catalog.entities[1]
                .attributes
                .get("pcgen_record_style")
                .and_then(Value::as_str),
            Some("pipe")
        );
    }

    #[test]
    fn parse_text_tracks_tab_style_for_name_only_pcg_lines() {
        let catalog = parse_text_to_catalog(
            "Feral Hound\tSTARTFEATS:1\tSIZE:M\tTYPE:Animal",
            "sample.pcg",
            "pcg",
        );

        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("pcgen_record_style")
                .and_then(Value::as_str),
            Some("tab")
        );
        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("pcgen_record_family")
                .and_then(Value::as_str),
            Some("pcg:name-record")
        );
    }

    #[test]
    fn parse_text_tracks_pcc_directive_family_and_space_style() {
        let catalog = parse_text_to_catalog(
            "GAMEMODE:Starwars_SE BOOKTYPE:Core Rulebook SETTING:Space Opera",
            "sample.pcc",
            "pcc",
        );

        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("pcgen_record_style")
                .and_then(Value::as_str),
            Some("space")
        );
        assert_eq!(
            catalog.entities[0]
                .attributes
                .get("pcgen_record_family")
                .and_then(Value::as_str),
            Some("pcc:directive")
        );
    }

    #[test]
    fn unparse_orders_entities_by_family_when_line_numbers_are_missing() {
        let mut catalog = parse_text_to_catalog(
            "VARIABLE:vars.lst\nCAMPAIGN:Test Campaign",
            "sample.pcc",
            "pcc",
        );

        catalog.entities.reverse();
        for entity in &mut catalog.entities {
            entity.attributes.swap_remove("line_number");
            entity.attributes.swap_remove("pcgen_line_number");
        }

        let text = unparse_catalog_to_text(&catalog);
        let lines: Vec<_> = text.lines().collect();

        assert_eq!(lines[0], "CAMPAIGN:Test Campaign");
        assert_eq!(lines[1], "VARIABLE:vars.lst");
    }

    #[test]
    fn parse_text_projects_structured_field_attributes() {
        let catalog = parse_text_to_catalog(
            "Ability Name KEY:Nightblade ~ Spells CATEGORY:Special Ability DESC:Spellcasting text FACT:ClassType|PC SPELLS:Hellknight|TIMES=3+CHA|CASTERLEVEL=TL|Discern Lies,14+CHA CLASSES:Bard,Wizard=2|Cleric=3 EQMOD:Special Ability ~ Uses per Day / 1|CHARGES[1] COST:22000 RANK:4",
            "fields.lst",
            "lst",
        );

        let entity = &catalog.entities[0];
        assert_eq!(
            entity.attributes.get("key").and_then(Value::as_str),
            Some("Nightblade ~ Spells")
        );
        assert_eq!(
            entity.attributes.get("category").and_then(Value::as_str),
            Some("Special Ability")
        );
        assert_eq!(
            entity.attributes.get("description").and_then(Value::as_str),
            Some("Spellcasting text")
        );
        assert_eq!(
            entity.attributes.get("cost").and_then(Value::as_str),
            Some("22000")
        );
        assert_eq!(
            entity.attributes.get("rank").and_then(Value::as_i64),
            Some(4)
        );

        let facts = entity
            .attributes
            .get("pcgen_facts")
            .and_then(Value::as_array)
            .expect("facts should be projected");
        assert_eq!(facts.len(), 1);
        assert_eq!(
            facts[0].get("key").and_then(Value::as_str),
            Some("ClassType")
        );
        assert_eq!(facts[0].get("value").and_then(Value::as_str), Some("PC"));

        let spells = entity
            .attributes
            .get("pcgen_spells")
            .and_then(Value::as_array)
            .expect("spells should be projected");
        assert_eq!(
            spells[0].get("mode").and_then(Value::as_str),
            Some("Hellknight")
        );
        assert_eq!(
            spells[0]
                .get("assignments")
                .and_then(Value::as_object)
                .and_then(|obj| obj.get("times"))
                .and_then(Value::as_str),
            Some("3+CHA")
        );

        let classes = entity
            .attributes
            .get("classes")
            .and_then(Value::as_array)
            .expect("classes should be projected");
        assert_eq!(
            classes[0]
                .get("parts")
                .and_then(Value::as_array)
                .map(|parts| parts.len()),
            Some(2)
        );

        let eqmods = entity
            .attributes
            .get("pcgen_eqmods")
            .and_then(Value::as_array)
            .expect("eqmods should be projected");
        assert_eq!(
            eqmods[0]
                .get("parts")
                .and_then(Value::as_array)
                .map(|parts| parts.len()),
            Some(2)
        );
    }

    #[test]
    fn parse_text_projects_spell_and_equipment_descriptor_fields() {
        let catalog = parse_text_to_catalog(
            "Magic Missile SCHOOL:Evocation SUBSCHOOL:Force COMPS:V,S CASTTIME:1 standard action CT:41 RANGE:Medium TARGETAREA:Up to five targets DURATION:Instantaneous SAVEINFO:None SPELLRES:Yes WT:4 SIZE:M WIELD:OneHanded EDR:0 SPELLFAILURE:0 FUMBLERANGE:1 RATEOFFIRE:1 REACH:5 REACHMULT:1 ALTCRITMULT:x2 ALTCRITRANGE:19-20 ALTEQMOD:Masterwork|COST=300 PROFICIENCY:WEAPON|Longsword CONTAINS:0|QTY=0 ICON:weapon_longsword NUMPAGES:1 PAGEUSAGE:1 QUALITY:Material|Steel SPROP:Martial melee weapon TYPE:Spell.Arcane",
            "descriptors.lst",
            "lst",
        );

        let entity = &catalog.entities[0];
        assert_eq!(
            entity
                .attributes
                .get("school")
                .and_then(Value::as_str),
            Some("Evocation")
        );
        assert_eq!(
            entity
                .attributes
                .get("subschool")
                .and_then(Value::as_str),
            Some("Force")
        );
        assert_eq!(
            entity.attributes.get("pcgen_comps").and_then(Value::as_str),
            Some("V,S")
        );
        assert_eq!(
            entity
                .attributes
                .get("pcgen_casttime")
                .and_then(Value::as_str),
            Some("1 standard action")
        );
        assert_eq!(
            entity.attributes.get("pcgen_ct").and_then(Value::as_i64),
            Some(41)
        );
        assert_eq!(
            entity.attributes.get("pcgen_range").and_then(Value::as_str),
            Some("Medium")
        );
        assert_eq!(
            entity
                .attributes
                .get("pcgen_targetarea")
                .and_then(Value::as_str),
            Some("Up to five targets")
        );
        assert_eq!(
            entity
                .attributes
                .get("pcgen_duration")
                .and_then(Value::as_str),
            Some("Instantaneous")
        );
        assert_eq!(
            entity
                .attributes
                .get("pcgen_saveinfo")
                .and_then(Value::as_str),
            Some("None")
        );
        assert_eq!(
            entity
                .attributes
                .get("pcgen_spellres")
                .and_then(Value::as_str),
            Some("Yes")
        );
        assert_eq!(
            entity.attributes.get("weight").and_then(Value::as_str),
            Some("4")
        );
        assert_eq!(
            entity.attributes.get("type").and_then(Value::as_str),
            Some("Spell.Arcane")
        );
        assert_eq!(
            entity
                .attributes
                .get("pcgen_numpages")
                .and_then(Value::as_i64),
            Some(1)
        );

        let sprop = entity
            .attributes
            .get("sprop")
            .and_then(Value::as_array)
            .expect("sprop should be projected");
        assert_eq!(
            sprop.first().and_then(Value::as_str),
            Some("Martial melee weapon")
        );
    }

    #[test]
    fn parse_text_projects_pi_and_open_names_when_marked_pi() {
        let catalog = parse_text_to_catalog(
            "Legacy Name NAMEISPI:YES OUTPUTNAME:Open Name KEY:Legacy Key CATEGORY:Special Ability",
            "namepi.lst",
            "lst",
        );

        let entity = &catalog.entities[0];
        assert_eq!(entity.name, "Legacy Key");
        assert_eq!(
            entity
                .attributes
                .get("pcgen_name_pi")
                .and_then(Value::as_str),
            Some("Legacy Name")
        );
        assert_eq!(
            entity
                .attributes
                .get("pcgen_name_open")
                .and_then(Value::as_str),
            Some("Open Name")
        );
    }

    #[test]
    fn parse_text_projects_open_name_from_head_when_not_pi() {
        let catalog = parse_text_to_catalog(
            "Open Name NAMEISPI:NO CATEGORY:Special Ability",
            "nameopen.lst",
            "lst",
        );

        let entity = &catalog.entities[0];
        assert_eq!(entity.name, "Open Name");
        assert_eq!(
            entity
                .attributes
                .get("pcgen_name_open")
                .and_then(Value::as_str),
            Some("Open Name")
        );
        assert!(entity.attributes.get("pcgen_name_pi").is_none());
    }

    #[test]
    fn infer_name_only_entities_to_schema_backed_entity_keys() {
        let ability = parse_text_to_catalog(
            "Nightblade Spellcraft CATEGORY:Special Ability ADDSPELLLEVEL:1 TYPE:Special.Magic",
            "ability.lst",
            "lst",
        );
        assert_eq!(
            ability.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:ability")
        );

        let spell = parse_text_to_catalog(
            "Magic Missile SCHOOL:Evocation COMPS:V,S TYPE:Spell.Arcane",
            "spell.lst",
            "lst",
        );
        assert_eq!(
            spell.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:spell")
        );

        let equipment = parse_text_to_catalog(
            "Longsword WT:4 WIELD:OneHanded SPROP:Martial melee weapon TYPE:Weapon.Martial",
            "equipment.lst",
            "lst",
        );
        assert_eq!(
            equipment.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:equipment")
        );

        let class_like = parse_text_to_catalog(
            "Psion CAST:MEMORIZE,INT KNOWN:0,1,2 TYPE:Base.Psionic.PC",
            "class_like.lst",
            "lst",
        );
        assert_eq!(
            class_like.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:class")
        );

        let skill_like = parse_text_to_catalog(
            "Bluff USEUNTRAINED:YES SITUATION:Feint|2 TYPE:Social",
            "skill_like.lst",
            "lst",
        );
        assert_eq!(
            skill_like.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:skill")
        );

        let pcc_like = parse_text_to_catalog(
            "CAMPAIGN:Sample Campaign\nGAMEMODE:Pathfinder BOOKTYPE:Core",
            "sample.pcc",
            "pcc",
        );
        assert_eq!(
            pcc_like.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:pcc")
        );

        let feat = parse_text_to_catalog(
            "Power Attack CATEGORY:FEAT PRESTAT:1,STR=13 TYPE:General.Fighter",
            "feat.lst",
            "lst",
        );
        assert_eq!(
            feat.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:feat")
        );

        let template = parse_text_to_catalog(
            "Male Clone VISIBLE:NO GENDERLOCK:Male",
            "templates.lst",
            "lst",
        );
        assert_eq!(
            template.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:template")
        );

        let startpack = parse_text_to_catalog(
            "STARTPACK:AquaticElfLang2 !PRELANG:1,Aquan",
            "kits.lst",
            "lst",
        );
        assert_eq!(
            startpack.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:startpack")
        );

        let startpack_langauto =
            parse_text_to_catalog("LANGAUTO:Common|Dwarven|Uluik", "kits.lst", "lst");
        assert_eq!(
            startpack_langauto.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:startpack-langauto")
        );

        let ancestry_like = parse_text_to_catalog(
            "Dwarf FACT:BaseSize|M SIZE:M MOVE:Walk,0 RACETYPE:Humanoid TYPE:Humanoid GRANT:MOVEMENT|Walk",
            "ancestry.lst",
            "lst",
        );
        assert_eq!(
            ancestry_like.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:race")
        );

        let bonus_spell_level = parse_text_to_catalog(
            "BONUSSPELLLEVEL:1 BASESTATSCORE:12 STATRANGE:8",
            "statsandchecks.lst",
            "lst",
        );
        assert_eq!(
            bonus_spell_level.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:system:bonusspelllevel")
        );

        let preview_sheet =
            parse_text_to_catalog("PREVIEWSHEET:Standard.htm.ftl", "miscinfo.lst", "lst");
        assert_eq!(
            preview_sheet.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:system:previewsheet")
        );

        let category_head = parse_text_to_catalog(
            "CATEGORY=Special Ability|Divine Speed.MOD ABILITY:Internal|AUTOMATIC|Fine Speed|PRELEGSLTEQ:3|PREMOVE:1,Walk=1|PRESIZEEQ:F",
            "abilities.lst",
            "lst",
        );
        assert_eq!(
            category_head.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:ability")
        );

        let template_like = parse_text_to_catalog(
            "No Swim Speed VISIBLE:NO !PREMOVE:1,Swim=1 MOVECLONE:Walk,Swim,*1",
            "templates.lst",
            "lst",
        );
        assert_eq!(
            template_like.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:template")
        );

        let age_template = parse_text_to_catalog(
            "Timeless Body ~ Adult VISIBLE:NO !PREAGESET:1,Middle-Aged BONUS:STAT|STR,CON,DEX|1",
            "templates.lst",
            "lst",
        );
        assert_eq!(
            age_template.entities[0]
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:template")
        );
    }

    #[test]
    fn ancestry_like_race_lines_emit_grant_structurally() {
        let catalog = parse_text_to_catalog(
            "Dwarf\t\tSORTKEY:A_PC_RACE\tFACT:BaseSize|M\tSIZE:M\tMOVE:Walk,0\tFACT:Speed|20\tBONUS:HP|CURRENTMAX|10|TYPE=Ancestry\tBONUS:VAR|Walk_Race|20\tABILITY:Ancestry|AUTOMATIC|Dwarf\tGROUP:RaceType_Humanoid\tRACETYPE:Humanoid\tTYPE:Humanoid\tGRANT:MOVEMENT|Walk\tMODIFY:RaceType_Humanoid|SET|True\tMODIFYOTHER:PC.MOVEMENT|Walk|Speed|SET|20",
            "ancestry.lst",
            "lst",
        );

        let entity = &catalog.entities[0];
        assert_eq!(
            entity
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str),
            Some("pcgen:entity:race")
        );

        let race_schema = schema::schema_for_entity_type_key("pcgen:entity:race")
            .expect("race schema should exist");
        let emittable = emittable_keys_for_entity(entity, race_schema);
        assert!(emittable.iter().any(|key| key == "GRANT"));

        let emitted = emit_entity(entity, race_schema);
        assert!(emitted.contains("GRANT:MOVEMENT|Walk"));
    }

    #[test]
    fn parse_text_extracts_publisher_and_source_metadata_from_whitespace_tokens() {
        let text = concat!(
            "CAMPAIGN:Star Wars Saga Edition Core Rulebook\n",
            "GAMEMODE:Starwars_SE BOOKTYPE:Core Rulebook SETTING:Space Opera\n",
            "PUBNAMELONG:Wizards of the Coast PUBNAMESHORT:WotC\n",
            "SOURCELONG:Star Wars Saga Edition Core Rulebook SOURCESHORT:SWSECR SOURCEWEB:www.wizards.com SOURCEDATE:2007-01\n"
        );

        let catalog =
            parse_text_to_catalog(text, "a_star_wars_saga_edition_core_rulebook.pcc", "pcc");

        assert_eq!(catalog.publishers.len(), 1);
        assert_eq!(catalog.publishers[0].name, "Wizards of the Coast");
        assert_eq!(catalog.sources.len(), 1);
        assert_eq!(
            catalog.sources[0].title,
            "Star Wars Saga Edition Core Rulebook"
        );
        assert!(
            catalog.sources[0]
                .game_systems
                .iter()
                .any(|g| g == "Starwars_SE")
        );
        assert!(
            catalog.sources[0]
                .game_systems
                .iter()
                .any(|g| g == "Space Opera")
        );
    }

    #[test]
    fn parse_text_extracts_mechanical_signals_for_reconciliation_hints() {
        let text = "Power Attack|TYPE:Combat|PRESTAT:1,STR=13|BONUS:COMBAT\\|TOHIT\\|1|CHOOSE:STAT";
        let catalog = parse_text_to_catalog(text, "signals.lst", "lst");
        let entity = &catalog.entities[0];

        let signals = entity
            .attributes
            .get("pcgen_mechanical_signals")
            .and_then(Value::as_array)
            .expect("mechanical signals should be present");

        let strings: Vec<&str> = signals.iter().filter_map(Value::as_str).collect();
        assert!(strings.contains(&"type_token:combat"));
        assert!(strings.contains(&"pre_key:prestat"));
        assert!(strings.contains(&"prestat:str"));
        assert!(strings.contains(&"bonus_category:combat"));
        assert!(strings.contains(&"bonus_target:tohit"));
        assert!(strings.contains(&"effect_key:choose"));
        assert!(strings.contains(&"effect_target:stat"));
    }

    // -----------------------------------------------------------------------
    // Schema + Emitter tests
    // -----------------------------------------------------------------------

    #[test]
    fn schema_registry_looks_up_by_entity_type_key() {
        let s = schema::schema_for_entity_type_key("pcgen:entity:ability")
            .expect("ABILITY schema should be registered");
        assert_eq!(s.entity_type_key, "pcgen:entity:ability");
        assert!(s.token_def("CATEGORY").is_some());
        assert!(s.token_def("COST").is_some());
        assert!(s.knows_token_key("BONUS"));
        assert!(s.knows_token_key("PREFEAT"));
        assert!(s.knows_token_key("!PREMULT"));
        assert!(!s.knows_token_key("XYZZY"));
    }

    #[test]
    fn schema_for_head_token_looks_up_class_and_skill() {
        let class_schema = schema::schema_for_head_token("CLASS")
            .expect("CLASS schema should be registered by head token");
        assert_eq!(class_schema.head_token, Some("CLASS"));

        let skill_schema = schema::schema_for_head_token("SKILL")
            .expect("SKILL schema should be registered by head token");
        assert_eq!(skill_schema.head_token, Some("SKILL"));

        let ability_include_schema = schema::schema_for_head_token("ABILITY")
            .expect("ABILITY PCC include schema should be registered by head token");
        assert_eq!(ability_include_schema.head_token, Some("ABILITY"));
    }

    #[test]
    fn emit_entity_produces_ability_line_from_parsed_entity() {
        // Parse an ability line, then re-emit it using the schema-driven emitter.
        let catalog = parse_text_to_catalog(
            "Toughness\tCATEGORY:General\tTYPE:General\tDESC:You gain extra hit points.",
            "ability.lst",
            "lst",
        );
        let entity = &catalog.entities[0];
        // Entity type is inferred from CATEGORY presence → pcgen:type:general (not ability),
        // so emit_entity_auto won't find a schema; use ABILITY schema directly.
        let ability_schema = schema::schema_for_entity_type_key("pcgen:entity:ability")
            .expect("ability schema must exist");
        let line = emit_entity(entity, ability_schema);

        assert!(
            line.contains("CATEGORY:General"),
            "line should contain CATEGORY: {line}"
        );
        assert!(line.contains("DESC:"), "line should contain DESC: {line}");
    }

    #[test]
    fn emit_entity_auto_produces_class_line_from_declared_entity() {
        let catalog = parse_text_to_catalog(
            "CLASS:Psion\tHITDIE:4\tTYPE:Base.Psionic.PC",
            "class.lst",
            "lst",
        );
        let entity = &catalog.entities[0];

        // CLASS entity type is recognized via declared entity head
        let line = emit_entity_auto(entity).expect("CLASS entity should have a schema");

        assert!(
            line.starts_with("CLASS:Psion"),
            "CLASS line should be prefixed: {line}"
        );
        assert!(
            line.contains("HITDIE:4"),
            "HD should round-trip as HITDIE: {line}"
        );
    }

    #[test]
    fn any_schema_knows_token_covers_previously_hardcoded_tokens() {
        use schema::any_schema_knows_token;
        assert!(any_schema_knows_token("TYPE"));
        assert!(any_schema_knows_token("BONUS"));
        assert!(any_schema_knows_token("CATEGORY"));
        assert!(any_schema_knows_token("SOURCELONG"));
        assert!(any_schema_knows_token("GAMEMODE"));
        assert!(any_schema_knows_token("PREFEAT"));
        assert!(any_schema_knows_token("PREVARGTEQ"));
        assert!(any_schema_knows_token("WT"));
        assert!(any_schema_knows_token("SCHOOL"));
        assert!(any_schema_knows_token("RANGE"));
        assert!(any_schema_knows_token("SPROP"));
        assert!(any_schema_knows_token("RACETYPE"));
        assert!(any_schema_knows_token("HITDIE"));
        assert!(any_schema_knows_token("CLASS"));
        assert!(any_schema_knows_token("SKILL"));
        assert!(any_schema_knows_token("STARTPACK"));
        assert!(any_schema_knows_token("CASTTIME"));
        assert!(any_schema_knows_token("SORTKEY"));
        assert!(any_schema_knows_token("SPELLKNOWN"));
        assert!(any_schema_knows_token("MOVE"));
        assert!(any_schema_knows_token("NATURALATTACKS"));
        assert!(any_schema_knows_token("KIT"));
        assert!(any_schema_knows_token("GEAR"));
        assert!(any_schema_knows_token("OPTION"));
        assert!(any_schema_knows_token("EQUIPBUY"));
        assert!(any_schema_knows_token("ABILITYLIST"));
        assert!(any_schema_knows_token("DISPLAYLOCATION"));
        assert!(any_schema_knows_token("DISPLAYNAME"));
        assert!(any_schema_knows_token("EDITABLE"));
        assert!(any_schema_knows_token("EDITPOOL"));
        assert!(any_schema_knows_token("FRACTIONALPOOL"));
        assert!(any_schema_knows_token("PLURAL"));
        assert!(any_schema_knows_token("POOL"));
        assert!(any_schema_knows_token("DAMAGE"));
        assert!(any_schema_knows_token("CRITMULT"));
        assert!(any_schema_knows_token("CRITRANGE"));
        assert!(any_schema_knows_token("ACCHECK"));
        assert!(any_schema_knows_token("ACHECK"));
        assert!(any_schema_knows_token("KEYSTAT"));
        assert!(any_schema_knows_token("MAXDEX"));
        assert!(any_schema_knows_token("SLOTS"));
        assert!(any_schema_knows_token("ALTDAMAGE"));
        assert!(any_schema_knows_token("ALTTYPE"));
        assert!(any_schema_knows_token("BASEQTY"));
        assert!(any_schema_knows_token("MODS"));
        assert!(any_schema_knows_token("HD"));
        assert!(any_schema_knows_token("MAXLEVEL"));
        assert!(any_schema_knows_token("SPELLSTAT"));
        assert!(any_schema_knows_token("SPELLLEVEL"));
        assert!(any_schema_knows_token("INFO"));
        assert!(any_schema_knows_token("MAXVER"));
        assert!(any_schema_knows_token("NEWKEY"));
        assert!(any_schema_knows_token("DATAFORMAT"));
        assert!(any_schema_knows_token("EXPLANATION"));
        assert!(any_schema_knows_token("REQUIRED"));
        assert!(any_schema_knows_token("SELECTABLE"));
        assert!(any_schema_knows_token("FACTSETDEF"));
        assert!(any_schema_knows_token("LOCAL"));
        assert!(any_schema_knows_token("GLOBAL"));
        assert!(any_schema_knows_token("BIOSET"));
        assert!(any_schema_knows_token("EXCLUDE"));
        assert!(any_schema_knows_token("SPELLRANGE"));
        assert!(any_schema_knows_token("SUBREGION"));
        assert!(any_schema_knows_token("ARMORTYPE"));
        assert!(any_schema_knows_token("COUNT"));
        assert!(any_schema_knows_token("OUTPUTSHEET"));
        assert!(any_schema_knows_token("INFOSHEET"));
        assert!(any_schema_knows_token("UNITSET"));
        assert!(any_schema_knows_token("DISTANCEUNIT"));
        assert!(any_schema_knows_token("DISTANCEFACTOR"));
        assert!(any_schema_knows_token("DISTANCEPATTERN"));
        assert!(any_schema_knows_token("HEIGHTUNIT"));
        assert!(any_schema_knows_token("HEIGHTFACTOR"));
        assert!(any_schema_knows_token("HEIGHTPATTERN"));
        assert!(any_schema_knows_token("WEIGHTUNIT"));
        assert!(any_schema_knows_token("WEIGHTFACTOR"));
        assert!(any_schema_knows_token("WEIGHTPATTERN"));
        assert!(any_schema_knows_token("CCSKILL"));
        assert!(any_schema_knows_token("UNENCUMBEREDMOVE"));
        assert!(any_schema_knows_token("LSTEXCLUDE"));
        assert!(any_schema_knows_token("CRMOD"));
        assert!(any_schema_knows_token("CRMODPRIORITY"));
        assert!(any_schema_knows_token("SAVE"));
        assert!(any_schema_knows_token("ALIGNMENT"));
        assert!(any_schema_knows_token("MAXCOST"));
        assert!(any_schema_knows_token("NAMEISPI"));
        assert!(any_schema_knows_token("DESCISPI"));
        assert!(any_schema_knows_token("COPYRIGHT"));
        assert!(any_schema_knows_token("FACTDEF"));
        assert!(any_schema_knows_token("FACTSET"));
        assert!(any_schema_knows_token("PANTHEON"));
        assert!(any_schema_knows_token("COPYMASTERBAB"));
        assert!(any_schema_knows_token("COPYMASTERCHECK"));
        assert!(any_schema_knows_token("COPYMASTERHP"));
        assert!(any_schema_knows_token("USEMASTERSKILL"));
        assert!(any_schema_knows_token("ALIGNMENTFEATURE"));
        assert!(any_schema_knows_token("CURRENCYUNITABBREV"));
        assert!(any_schema_knows_token("MENUENTRY"));
        assert!(any_schema_knows_token("DISPLAYORDER"));
        assert!(any_schema_knows_token("DIESIZES"));
        assert!(any_schema_knows_token("DEFAULTUNITSET"));
        assert!(any_schema_knows_token("ALLOWEDMODES"));
        assert!(any_schema_knows_token("BABMAXATT"));
        assert!(any_schema_knows_token("BABMINVAL"));
        assert!(any_schema_knows_token("BABATTCYC"));
        assert!(any_schema_knows_token("ACNAME"));
        assert!(any_schema_knows_token("DOMAINFEATURE"));
        assert!(any_schema_knows_token("LOADMULT"));
        assert!(any_schema_knows_token("NUMSLOTS"));
        assert!(any_schema_knows_token("HEAD"));
        assert!(any_schema_knows_token("TORSO"));
        assert!(any_schema_knows_token("SHIELD"));
        assert!(any_schema_knows_token("LEVELMSG"));
        assert!(any_schema_knows_token("SHORTRANGE"));
        assert!(any_schema_knows_token("RANGEPENALTY"));
        assert!(any_schema_knows_token("SQUARESIZE"));
        assert!(any_schema_knows_token("SKILLMULTIPLIER"));
        assert!(any_schema_knows_token("SPELLBASEDC"));
        assert!(any_schema_knows_token("WEAPONNONPROFPENALTY"));
        assert!(any_schema_knows_token("WEAPONREACH"));
        assert!(any_schema_knows_token("CHARACTERTYPE"));
        assert!(any_schema_knows_token("SYMBOL"));
        assert!(any_schema_knows_token("CRTHRESHOLD"));
        assert!(any_schema_knows_token("CRSTEPS"));
        assert!(any_schema_knows_token("MONSTERROLES"));
        assert!(any_schema_knows_token("MONSTERROLEDEFAULT"));
        assert!(any_schema_knows_token("XPTABLE"));
        assert!(any_schema_knows_token("EQSIZEPENALTY"));
        assert!(any_schema_knows_token("RESIZABLEEQUIPTYPE"));
        assert!(any_schema_knows_token("SKILLCOST_CROSSCLASS"));
        assert!(any_schema_knows_token("MAXNONEPICLEVEL"));
        assert!(any_schema_knows_token("PLUSCOST"));
        assert!(any_schema_knows_token("SHOWINMENU"));
        assert!(any_schema_knows_token("LANGAUTO"));
        assert!(any_schema_knows_token("SELECTION"));
        assert!(any_schema_knows_token("GRANT"));
        assert!(any_schema_knows_token("ITEMCREATE"));
        assert!(any_schema_knows_token("STARTTABLE"));
        assert!(any_schema_knows_token("MOVEMENT"));
        assert!(any_schema_knows_token("DATATABLE"));
        assert!(any_schema_knows_token("DEFAULTDATASET"));
        assert!(any_schema_knows_token("GAMEMODEKEY"));
        assert!(any_schema_knows_token("ENDTABLE"));
        assert!(any_schema_knows_token("ALIGN"));
        assert!(any_schema_knows_token("STAT"));
        assert!(any_schema_knows_token("RACE"));
        assert!(any_schema_knows_token("NAME"));
        assert!(any_schema_knows_token("ITYPE"));
        assert!(any_schema_knows_token("NAMEOPT"));
        assert!(any_schema_knows_token("TEMPDESC"));
        assert!(any_schema_knows_token("MINXP"));
        assert!(any_schema_knows_token("CSKILLMAX"));
        assert!(any_schema_knows_token("CCSKILLMAX"));
        assert!(any_schema_knows_token("TEMPBONUS"));
        assert!(any_schema_knows_token("SELECT"));
        assert!(any_schema_knows_token("SOURCELINK"));
        assert!(any_schema_knows_token("UDAM"));
        assert!(any_schema_knows_token("UMULT"));
        assert!(any_schema_knows_token("DEITYWEAP"));
        assert!(any_schema_knows_token("GROUP"));
        assert!(any_schema_knows_token("DONOTADD"));
        assert!(any_schema_knows_token("MEMORIZE"));
        assert!(any_schema_knows_token("COMPANIONLIST"));
        assert!(any_schema_knows_token("FOLLOWERS"));
        assert!(any_schema_knows_token("LANGBONUS"));
        assert!(any_schema_knows_token("CHANGEPROF"));
        assert!(any_schema_knows_token("SERVESAS"));
        assert!(any_schema_knows_token("VALIDFORDEITY"));
        assert!(any_schema_knows_token("VALIDFORFOLLOWER"));
        assert!(any_schema_knows_token("STATMOD"));
        assert!(any_schema_knows_token("SIZENAME"));
        assert!(any_schema_knows_token("SIZENUM"));
        assert!(any_schema_knows_token("ISDEFAULTSIZE"));
        assert!(any_schema_knows_token("APPLY"));
        assert!(any_schema_knows_token("LOOKUP"));
        assert!(any_schema_knows_token("BASEITEM"));
        assert!(any_schema_knows_token("FOLLOWER"));
        assert!(any_schema_knows_token("MASTERBONUSRACE"));
        assert!(any_schema_knows_token("RACENAME"));
        assert!(any_schema_knows_token("BASEAGE"));
        assert!(any_schema_knows_token("MAXAGE"));
        assert!(any_schema_knows_token("AGEDIEROLL"));
        assert!(any_schema_knows_token("SEX"));
        assert!(any_schema_knows_token("HAIR"));
        assert!(any_schema_knows_token("EYES"));
        assert!(any_schema_knows_token("SKINTONE"));
        assert!(any_schema_knows_token("LEGS"));
        assert!(any_schema_knows_token("HANDS"));
        assert!(any_schema_knows_token("FACE"));
        assert!(any_schema_knows_token("VISION"));
        assert!(any_schema_knows_token("DR"));
        assert!(any_schema_knows_token("SR"));
        assert!(any_schema_knows_token("CR"));
        assert!(any_schema_knows_token("ROLE"));
        assert!(any_schema_knows_token("EXCLUSIVE"));
        assert!(any_schema_knows_token("REGION"));
        assert!(any_schema_knows_token("PARM"));
        assert!(any_schema_knows_token("VAR"));
        assert!(any_schema_knows_token("DEFAULT"));
        assert!(any_schema_knows_token("ACTYPE"));
        assert!(any_schema_knows_token("REMOVE"));
        assert!(any_schema_knows_token("BASEDICE"));
        assert!(any_schema_knows_token("UP"));
        assert!(any_schema_knows_token("DOWN"));
        assert!(any_schema_knows_token("WIELDCATEGORY"));
        assert!(any_schema_knows_token("SWITCH"));
        assert!(any_schema_knows_token("EQSLOT"));
        assert!(any_schema_knows_token("NUMBER"));
        assert!(any_schema_knows_token("TAB"));
        assert!(any_schema_knows_token("CONTEXT"));
        assert!(any_schema_knows_token("AGESET"));
        assert!(any_schema_knows_token("REPLACES"));
        assert!(any_schema_knows_token("SUBRACE"));
        assert!(any_schema_knows_token("REMOVABLE"));
        assert!(any_schema_knows_token("SUBCLASS"));
        assert!(any_schema_knows_token("MODIFY"));
        assert!(any_schema_knows_token("MODIFYOTHER"));
        assert!(any_schema_knows_token("PART"));
        assert!(any_schema_knows_token("FUNDS"));
        assert!(any_schema_knows_token("LANGUAGE"));
        assert!(any_schema_knows_token("EQUIPMOD"));
        assert!(any_schema_knows_token("DATACONTROL"));
        assert!(any_schema_knows_token("COMPANIONMOD"));
        assert!(any_schema_knows_token("WEAPONPROF"));
        assert!(any_schema_knows_token("ARMORPROF"));
        assert!(any_schema_knows_token("SHIELDPROF"));
        assert!(any_schema_knows_token("GENDER"));
        assert!(any_schema_knows_token("TOTALCOST"));
        assert!(any_schema_knows_token("METHOD"));
        assert!(any_schema_knows_token("SIZEMULT"));
        assert!(any_schema_knows_token("ENCUMBRANCE"));
        assert!(any_schema_knows_token("DEFAULTVARIABLEVALUE"));
        assert!(any_schema_knows_token("POINTS"));
        assert!(any_schema_knows_token("WEAPONCATEGORY"));
        assert!(any_schema_knows_token("ROLLMETHOD"));
        assert!(any_schema_knows_token("CLASSTYPE"));
        assert!(any_schema_knows_token("CRFORMULA"));
        assert!(any_schema_knows_token("ISMONSTER"));
        assert!(any_schema_knows_token("XPPENALTY"));
        assert!(any_schema_knows_token("SPELL"));
        assert!(any_schema_knows_token("WEAPONTYPE"));
        assert!(any_schema_knows_token("TABLE"));
        assert!(any_schema_knows_token("VALUES"));
        assert!(any_schema_knows_token("NEWCATEGORY"));
        assert!(!any_schema_knows_token("XYZZY"));
        assert!(!any_schema_knows_token("NOTAREALTOKEN"));
    }

    #[test]
    fn token_alias_registry_exposes_explicit_rules() {
        use schema::all_token_aliases;

        let aliases = all_token_aliases();
        assert!(
            aliases
                .iter()
                .any(|a| a.alias == "HD" && a.canonical == "HITDIE")
        );
    }

    #[test]
    fn token_alias_lookup_respects_scope() {
        use schema::token_aliases::canonical_lookup_key;

        assert_eq!(canonical_lookup_key("CASTTIME", None), "CASTTIME");
        assert_eq!(canonical_lookup_key("PART", None), "PART");
        assert_eq!(
            canonical_lookup_key("EQUIPMENT.PART", None),
            "EQUIPMENT.PART"
        );
        assert_eq!(
            canonical_lookup_key("HD", Some("pcgen:entity:class")),
            "HITDIE"
        );
        assert_eq!(
            canonical_lookup_key("HD", Some("pcgen:entity:template")),
            "HD"
        );
    }
}
