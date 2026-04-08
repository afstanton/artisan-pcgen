use std::{fs, io, path::Path};

use artisan_core::{
    CanonicalId, Entity, EntityType,
    domain::{
        CitationLocator, CitationRecord, PublisherRecord, SourceRecord, SubjectRef,
        VerificationState,
        entity::CompletenessState,
    },
    id::{ExternalId, FormatId},
    reconcile::{ImportCandidate, SourceHint},
};
use indexmap::IndexMap;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use uuid::Uuid;

mod analysis;
mod parsing;

use analysis::{metadata, semantics, signals};
use parsing::line_codec;

const ENTITY_TYPE_NAMESPACE: Uuid = Uuid::from_u128(0x6c8fdbf43f4f4a4ba4d846e2bf8b9c10);
const ENTITY_NAMESPACE: Uuid = Uuid::from_u128(0x5ea8a1062b0842beaf2fcb5966e30f3a);
const PUBLISHER_NAMESPACE: Uuid = Uuid::from_u128(0x4a3decc22d7745618872f8361653dc61);
const SOURCE_NAMESPACE: Uuid = Uuid::from_u128(0x17a9126be16f4ddcbfce46c80dd60f2f);
const CITATION_NAMESPACE: Uuid = Uuid::from_u128(0x0ab04cb4229a4ba39f5947655e0f4d28);

#[derive(Debug, Clone)]
pub struct ParsedCatalog {
    pub publishers: Vec<PublisherRecord>,
    pub sources: Vec<SourceRecord>,
    pub citations: Vec<CitationRecord>,
    pub entity_types: Vec<EntityType>,
    pub entities: Vec<Entity>,
}

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

pub fn parse_file(path: &Path) -> io::Result<ParsedCatalog> {
    let bytes = fs::read(path)?;
    let text = String::from_utf8_lossy(&bytes).to_string();
    let source_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("fixture");
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_else(|| "unknown".to_string());
    Ok(parse_text_to_catalog(&text, source_name, &ext))
}

pub fn parse_text_to_catalog(text: &str, source_name: &str, ext: &str) -> ParsedCatalog {
    let type_key = format!("pcgen.{ext}");
    let entity_type_id = deterministic_id(ENTITY_TYPE_NAMESPACE, &type_key);

    let entity_type = EntityType {
        id: entity_type_id,
        key: type_key,
        name: format!("PCGen {}", ext.to_ascii_uppercase()),
        parent: None,
        descriptive_fields: IndexMap::new(),
        mechanical_fields: IndexMap::new(),
        external_ids: vec![ExternalId {
            format: FormatId::Pcgen,
            namespace: Some("entity_type".to_string()),
            value: format!("pcgen:{ext}"),
        }],
        provenance: None,
    };

    let mut metadata = metadata::PcgenMetadata::default();
    let mut entities = Vec::new();
    let mut citations = Vec::new();
    for (line_number, raw_line) in text.lines().enumerate() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let parsed_line = parse_line(trimmed);
        metadata::collect_metadata(&parsed_line, trimmed, &mut metadata);
        let name = parsed_line
            .head
            .trim()
            .to_string();
        let name = if name.is_empty() {
            format!("line_{}", line_number + 1)
        } else {
            name
        };
        let entity_id = deterministic_id(
            ENTITY_NAMESPACE,
            &format!("{source_name}:{ext}:{}:{trimmed}", line_number + 1),
        );

        let mut attributes = IndexMap::new();
        attributes.insert("head".to_string(), Value::String(parsed_line.head.clone()));
        attributes.insert(
            "clauses".to_string(),
            json!(line_codec::clauses_to_json(&parsed_line.clauses)),
        );
        attributes.insert("line_number".to_string(), json!(line_number + 1));
        attributes.insert("pcgen_line_number".to_string(), json!(line_number + 1));
        attributes.insert("source_format".to_string(), Value::String(ext.to_string()));

        let mut entity_citations = Vec::new();
        if let Some(source_page) = line_codec::find_key_value(&parsed_line.clauses, "SOURCEPAGE") {
            attributes.insert(
                "pcgen_source_page".to_string(),
                Value::String(source_page.clone()),
            );

            let citation_id = deterministic_id(
                CITATION_NAMESPACE,
                &format!("{source_name}:{ext}:{}:{source_page}", line_number + 1),
            );
            citations.push(CitationRecord {
                id: citation_id,
                subject: SubjectRef::Entity(entity_id),
                source: CanonicalId(Uuid::nil()),
                locators: vec![CitationLocator {
                    kind: "page".to_string(),
                    value: source_page,
                    canonical: true,
                }],
                verification: VerificationState::Unverified,
                external_ids: vec![ExternalId {
                    format: FormatId::Pcgen,
                    namespace: Some("citation".to_string()),
                    value: format!("{}:{}:sourcepage", source_name, line_number + 1),
                }],
            });
            entity_citations.push(citation_id);
        }

        let inferred_type_key = semantics::infer_entity_type_key(&parsed_line.clauses);
        attributes.insert(
            "pcgen_entity_type_key".to_string(),
            Value::String(inferred_type_key.clone()),
        );
        let mechanical_signals = signals::extract_mechanical_signals(&parsed_line.clauses);
        if !mechanical_signals.is_empty() {
            attributes.insert("pcgen_mechanical_signals".to_string(), json!(mechanical_signals));
        }

        let external_id = ExternalId {
            format: FormatId::Pcgen,
            namespace: Some(ext.to_string()),
            value: format!("{}:{}", source_name, line_number + 1),
        };

        let mut effects = Vec::new();
        let mut prerequisites = Vec::new();
        semantics::project_semantics(&parsed_line.clauses, &mut effects, &mut prerequisites);

        entities.push(Entity {
            id: entity_id,
            entity_type: entity_type_id,
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
        .or_else(|| metadata.source_short.clone().filter(|s| !s.trim().is_empty()))
        .unwrap_or_else(|| source_name.to_string());

    let source_id = deterministic_id(
        SOURCE_NAMESPACE,
        &format!("pcgen:{ext}:{source_name}:{source_title}"),
    );

    let mut publishers = Vec::new();
    let mut publisher_ids = Vec::new();
    if let Some(name) = publisher_name {
        let publisher_id = deterministic_id(PUBLISHER_NAMESPACE, &format!("pcgen:publisher:{name}"));
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
            if !trimmed.is_empty() && !game_systems.iter().any(|g| g.eq_ignore_ascii_case(trimmed)) {
                game_systems.push(trimmed.to_string());
            }
        }
    }

    let mut source_external_ids = vec![ExternalId {
        format: FormatId::Pcgen,
        namespace: Some("source".to_string()),
        value: format!("{ext}:{source_name}"),
    }];
    if let Some(short) = metadata.source_short.clone().filter(|s| !s.trim().is_empty()) {
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

    ParsedCatalog {
        publishers,
        sources: vec![source],
        citations,
        entity_types: vec![entity_type],
        entities,
    }
}

pub fn unparse_catalog_to_text(catalog: &ParsedCatalog) -> String {
    let mut lines = Vec::new();
    for entity in &catalog.entities {
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
        lines.push(unparse_line(&head, &clauses));
    }
    lines.join("\n")
}

pub fn parse_line(line: &str) -> ParsedLine {
    line_codec::parse_line_internal(line)
}

pub fn unparse_line(head: &str, clauses: &[ParsedClause]) -> String {
    line_codec::unparse_line_internal(head, clauses)
}

fn deterministic_id(namespace: Uuid, key: &str) -> CanonicalId {
    CanonicalId(Uuid::new_v5(&namespace, key.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn parse_and_unparse_line_preserves_escaped_separators() {
        let original = r"Name\|WithPipe|DESC:Use \: carefully|TAG:ONE\|TWO";
        let parsed = parse_line(original);
        let reparsed = unparse_line(&parsed.head, &parsed.clauses);
        assert_eq!(reparsed, original);
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
        assert!(entity
            .prerequisites
            .iter()
            .any(|p| p.kind == "PREVARGTEQ" && p.expression.as_deref() == Some("STR,13")));
        assert!(entity
            .effects
            .iter()
            .any(|e| e.kind == "BONUS" && e.target == "COMBAT"));
    }

    #[test]
    fn parse_text_extracts_publisher_and_source_metadata_from_whitespace_tokens() {
        let text = concat!(
            "CAMPAIGN:Star Wars Saga Edition Core Rulebook\n",
            "GAMEMODE:Starwars_SE BOOKTYPE:Core Rulebook SETTING:Space Opera\n",
            "PUBNAMELONG:Wizards of the Coast PUBNAMESHORT:WotC\n",
            "SOURCELONG:Star Wars Saga Edition Core Rulebook SOURCESHORT:SWSECR SOURCEWEB:www.wizards.com SOURCEDATE:2007-01\n"
        );

        let catalog = parse_text_to_catalog(text, "a_star_wars_saga_edition_core_rulebook.pcc", "pcc");

        assert_eq!(catalog.publishers.len(), 1);
        assert_eq!(catalog.publishers[0].name, "Wizards of the Coast");
        assert_eq!(catalog.sources.len(), 1);
        assert_eq!(
            catalog.sources[0].title,
            "Star Wars Saga Edition Core Rulebook"
        );
        assert!(catalog.sources[0]
            .game_systems
            .iter()
            .any(|g| g == "Starwars_SE"));
        assert!(catalog.sources[0]
            .game_systems
            .iter()
            .any(|g| g == "Space Opera"));
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
}
