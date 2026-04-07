use std::{fs, io, path::Path};

use artisan_core::{
    CanonicalId, Entity, EntityType,
    domain::{
        CitationLocator, CitationRecord, PublisherRecord, SourceRecord, SubjectRef,
        VerificationState,
        entity::CompletenessState,
        rules::{Effect, Prerequisite},
    },
    id::{ExternalId, FormatId},
    reconcile::{ImportCandidate, SourceHint},
};
use indexmap::IndexMap;
use lalrpop_util::lalrpop_mod;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use uuid::Uuid;

mod parser_tokens;
lalrpop_mod!(line_grammar);

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
            if let Some((key, value)) = split_first_key_value(&parsed) {
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

    let mut metadata = PcgenMetadata::default();
    let mut entities = Vec::new();
    let mut citations = Vec::new();
    for (line_number, raw_line) in text.lines().enumerate() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let parsed_line = parse_line(trimmed);
        collect_metadata(&parsed_line, trimmed, &mut metadata);
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
        attributes.insert("clauses".to_string(), json!(clauses_to_json(&parsed_line.clauses)));
        attributes.insert("line_number".to_string(), json!(line_number + 1));
        attributes.insert("pcgen_line_number".to_string(), json!(line_number + 1));
        attributes.insert("source_format".to_string(), Value::String(ext.to_string()));

        let mut entity_citations = Vec::new();
        if let Some(source_page) = find_key_value(&parsed_line.clauses, "SOURCEPAGE") {
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

        let inferred_type_key = infer_entity_type_key(&parsed_line.clauses);
        attributes.insert(
            "pcgen_entity_type_key".to_string(),
            Value::String(inferred_type_key.clone()),
        );

        let external_id = ExternalId {
            format: FormatId::Pcgen,
            namespace: Some(ext.to_string()),
            value: format!("{}:{}", source_name, line_number + 1),
        };

        let mut effects = Vec::new();
        let mut prerequisites = Vec::new();
        project_semantics(&parsed_line.clauses, &mut effects, &mut prerequisites);

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
            .and_then(clauses_from_json)
            .unwrap_or_default();
        lines.push(unparse_line(&head, &clauses));
    }
    lines.join("\n")
}

pub fn parse_line(line: &str) -> ParsedLine {
    let segments = parse_segments_with_generated_parser(line);
    let mut iter = segments.into_iter();
    let head = iter.next().unwrap_or_default();
    let clauses = iter
        .map(|segment| parse_clause(&segment))
        .collect();
    ParsedLine { head, clauses }
}

pub fn unparse_line(head: &str, clauses: &[ParsedClause]) -> String {
    let mut parts = vec![escape_head_segment(head)];
    for clause in clauses {
        match clause {
            ParsedClause::Bare(value) => parts.push(escape_segment(value)),
            ParsedClause::KeyValue { key, value } => {
                parts.push(format!("{}:{}", escape_segment(key), escape_segment(value)));
            }
        }
    }
    parts.join("|")
}

fn escape_head_segment(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        match ch {
            '\\' | '|' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

fn deterministic_id(namespace: Uuid, key: &str) -> CanonicalId {
    CanonicalId(Uuid::new_v5(&namespace, key.as_bytes()))
}

fn parse_segments_with_generated_parser(line: &str) -> Vec<String> {
    let parser = line_grammar::SegmentsParser::new();
    match parser.parse(parser_tokens::line_tokens(line)) {
        Ok(segments) => segments
            .into_iter()
            .map(|segment| segment.trim().to_string())
            .collect(),
        Err(_) => vec![line.trim().to_string()],
    }
}

fn parse_clause(segment: &str) -> ParsedClause {
    let tokens = parser_tokens::clause_tokens(segment);
    let mut key = String::new();
    let mut value = String::new();
    let mut seen_colon = false;

    for (_, token, _) in tokens {
        match token {
            parser_tokens::ClauseToken::Colon => {
                if seen_colon {
                    value.push(':');
                } else {
                    seen_colon = true;
                }
            }
            parser_tokens::ClauseToken::Piece(part) => {
                if seen_colon {
                    value.push_str(&part);
                } else {
                    key.push_str(&part);
                }
            }
        }
    }

    if seen_colon {
        let key_trimmed = key.trim().to_string();
        if !key_trimmed.is_empty() {
            return ParsedClause::KeyValue {
                key: key_trimmed,
                value: value.trim().to_string(),
            };
        }
        return ParsedClause::Bare(format!("{}:{}", key, value).trim().to_string());
    }

    ParsedClause::Bare(key.trim().to_string())
}

fn project_semantics(clauses: &[ParsedClause], effects: &mut Vec<Effect>, prerequisites: &mut Vec<Prerequisite>) {
    for clause in clauses {
        if let ParsedClause::KeyValue { key, value } = clause {
            if key.starts_with("PRE") {
                prerequisites.push(Prerequisite {
                    kind: key.clone(),
                    expression: if value.is_empty() { None } else { Some(value.clone()) },
                });
                continue;
            }

            if key == "BONUS" || key == "AUTO" || key == "DEFINE" || key == "CHOOSE" {
                effects.push(Effect {
                    kind: key.clone(),
                    target: value.clone(),
                    value: None,
                });
            }
        }
    }
}

fn split_first_key_value(parsed: &ParsedLine) -> Option<(String, String)> {
    if let Some((k, v)) = parse_head_key_value(&parsed.head) {
        return Some((k, v));
    }
    for clause in &parsed.clauses {
        if let ParsedClause::KeyValue { key, value } = clause {
            return Some((key.clone(), value.clone()));
        }
    }
    None
}

fn parse_head_key_value(head: &str) -> Option<(String, String)> {
    let idx = head.find(':')?;
    let key = head[..idx].trim();
    let value = head[idx + 1..].trim();
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key.to_string(), value.to_string()))
}

fn find_key_value(clauses: &[ParsedClause], key: &str) -> Option<String> {
    for clause in clauses {
        if let ParsedClause::KeyValue { key: k, value } = clause {
            if k.eq_ignore_ascii_case(key) {
                return Some(value.clone());
            }
        }
    }
    None
}

fn infer_entity_type_key(clauses: &[ParsedClause]) -> String {
    if let Some(value) = find_key_value(clauses, "TYPE") {
        let normalized = value
            .split('.')
            .next()
            .unwrap_or(value.as_str())
            .trim()
            .to_ascii_lowercase()
            .replace(' ', "-");
        if !normalized.is_empty() {
            return format!("pcgen:type:{normalized}");
        }
    }
    "pcgen:type:unresolved".to_string()
}

fn clauses_to_json(clauses: &[ParsedClause]) -> Vec<Value> {
    clauses
        .iter()
        .map(|clause| match clause {
            ParsedClause::Bare(value) => json!({"kind": "bare", "value": value}),
            ParsedClause::KeyValue { key, value } => {
                json!({"kind": "key_value", "key": key, "value": value})
            }
        })
        .collect()
}

fn clauses_from_json(value: &Value) -> Option<Vec<ParsedClause>> {
    let array = value.as_array()?;
    let mut out = Vec::new();
    for item in array {
        let kind = item.get("kind")?.as_str()?;
        match kind {
            "bare" => {
                out.push(ParsedClause::Bare(item.get("value")?.as_str()?.to_string()));
            }
            "key_value" => {
                out.push(ParsedClause::KeyValue {
                    key: item.get("key")?.as_str()?.to_string(),
                    value: item.get("value")?.as_str()?.to_string(),
                });
            }
            _ => return None,
        }
    }
    Some(out)
}

#[derive(Default)]
struct PcgenMetadata {
    campaign: Option<String>,
    source_title: Option<String>,
    source_short: Option<String>,
    source_web: Option<String>,
    source_date: Option<String>,
    publisher_long: Option<String>,
    publisher_short: Option<String>,
    game_mode: Option<String>,
    setting: Option<String>,
    book_type: Option<String>,
}

fn collect_metadata(parsed_line: &ParsedLine, raw_line: &str, metadata: &mut PcgenMetadata) {
    // Many PCGen metadata lines are whitespace-separated key:value blocks, not pipe clauses.
    for (key, value) in extract_metadata_pairs(raw_line) {
        match key.as_str() {
            "CAMPAIGN" if metadata.campaign.is_none() => metadata.campaign = Some(value),
            "SOURCELONG" | "SOURCE" if metadata.source_title.is_none() => {
                metadata.source_title = Some(value)
            }
            "SOURCESHORT" if metadata.source_short.is_none() => metadata.source_short = Some(value),
            "SOURCEWEB" if metadata.source_web.is_none() => metadata.source_web = Some(value),
            "SOURCEDATE" if metadata.source_date.is_none() => metadata.source_date = Some(value),
            "PUBNAMELONG" | "PUBLISHER" | "PUBLISHERNAME" if metadata.publisher_long.is_none() => {
                metadata.publisher_long = Some(value)
            }
            "PUBNAMESHORT" if metadata.publisher_short.is_none() => {
                metadata.publisher_short = Some(value)
            }
            "GAMEMODE" if metadata.game_mode.is_none() => metadata.game_mode = Some(value),
            "SETTING" if metadata.setting.is_none() => metadata.setting = Some(value),
            "BOOKTYPE" if metadata.book_type.is_none() => metadata.book_type = Some(value),
            _ => {}
        }
    }

    if metadata.campaign.is_none()
        && let Some((key, value)) = parse_head_key_value(&parsed_line.head)
        && key.eq_ignore_ascii_case("CAMPAIGN")
    {
        metadata.campaign = Some(value);
    }

    for clause in &parsed_line.clauses {
        if let ParsedClause::KeyValue { key, value } = clause {
            let key_upper = key.to_ascii_uppercase();
            if metadata.source_title.is_none() && (key_upper == "SOURCELONG" || key_upper == "SOURCE") {
                metadata.source_title = Some(value.clone());
            }
            if metadata.source_short.is_none() && key_upper == "SOURCESHORT" {
                metadata.source_short = Some(value.clone());
            }
            if metadata.source_web.is_none() && key_upper == "SOURCEWEB" {
                metadata.source_web = Some(value.clone());
            }
            if metadata.source_date.is_none() && key_upper == "SOURCEDATE" {
                metadata.source_date = Some(value.clone());
            }
            if metadata.publisher_long.is_none()
                && (key_upper == "PUBNAMELONG" || key_upper == "PUBLISHER" || key_upper == "PUBLISHERNAME")
            {
                metadata.publisher_long = Some(value.clone());
            }
            if metadata.publisher_short.is_none() && key_upper == "PUBNAMESHORT" {
                metadata.publisher_short = Some(value.clone());
            }
            if metadata.game_mode.is_none() && key_upper == "GAMEMODE" {
                metadata.game_mode = Some(value.clone());
            }
            if metadata.setting.is_none() && key_upper == "SETTING" {
                metadata.setting = Some(value.clone());
            }
            if metadata.book_type.is_none() && key_upper == "BOOKTYPE" {
                metadata.book_type = Some(value.clone());
            }
        }
    }
}

fn extract_metadata_pairs(line: &str) -> Vec<(String, String)> {
    let keys = [
        "CAMPAIGN",
        "SOURCELONG",
        "SOURCE",
        "SOURCESHORT",
        "SOURCEWEB",
        "SOURCEDATE",
        "PUBNAMELONG",
        "PUBNAMESHORT",
        "PUBLISHER",
        "PUBLISHERNAME",
        "GAMEMODE",
        "BOOKTYPE",
        "SETTING",
    ];

    let mut marks: Vec<(usize, &'static str)> = Vec::new();
    for key in keys {
        let needle = format!("{key}:");
        let mut cursor = 0usize;
        while let Some(pos) = line[cursor..].find(&needle) {
            let start = cursor + pos;
            marks.push((start, key));
            cursor = start + needle.len();
            if cursor >= line.len() {
                break;
            }
        }
    }

    marks.sort_by_key(|(start, _)| *start);
    marks.dedup_by_key(|(start, _)| *start);

    let mut out = Vec::new();
    for (idx, (start, key)) in marks.iter().enumerate() {
        let value_start = start + key.len() + 1;
        let value_end = marks
            .get(idx + 1)
            .map(|(next, _)| *next)
            .unwrap_or(line.len());
        if value_start > line.len() || value_start > value_end {
            continue;
        }
        let value = line[value_start..value_end].trim();
        if !value.is_empty() {
            out.push(((*key).to_string(), value.to_string()));
        }
    }

    out
}

fn escape_segment(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        match ch {
            '\\' | '|' | ':' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
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
}
