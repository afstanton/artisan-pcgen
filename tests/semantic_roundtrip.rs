use std::{collections::BTreeMap, fs, io, path::{Path, PathBuf}};

use artisan_pcgen::{parse_file, parse_text_to_catalog, unparse_catalog_to_text};
use serde_json::{Value, json};

fn fixture_root() -> PathBuf {
    if let Ok(custom) = std::env::var("ARTISAN_PCGEN_FIXTURES_DIR") {
        return PathBuf::from(custom);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pcgen")
}

#[test]
fn semantic_roundtrip_single_fixture_file() {
    let file = fixture_root().join("sample.lst");
    assert!(file.exists(), "expected fixture file at {}", file.display());
    assert_semantic_roundtrip_file(&file).expect("single fixture roundtrip should succeed");
}

#[test]
fn semantic_roundtrip_all_fixture_files() {
    let root = fixture_root();
    assert!(root.exists(), "fixture root does not exist: {}", root.display());

    let files = collect_all_fixture_files(&root).expect("collect fixture files");
    assert!(
        !files.is_empty(),
        "no fixture files found under {}",
        root.display()
    );

    let exercised = assert_semantic_roundtrip_for_all_fixtures(Path::new(&root))
        .expect("roundtrip all fixtures");
    assert_eq!(
        exercised,
        files.len(),
        "every fixture file should be exercised exactly once"
    );
}

fn assert_semantic_roundtrip_file(path: &Path) -> io::Result<()> {
    let first = parse_file(path)?;
    let generated = unparse_catalog_to_text(&first);
    let source_name = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("generated");
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_else(|| "unknown".to_string());
    let second = parse_text_to_catalog(&generated, source_name, &ext);

    let before = semantic_snapshot(&first);
    let after = semantic_snapshot(&second);
    assert_eq!(before, after, "semantic roundtrip mismatch: {}", path.display());
    Ok(())
}

fn assert_semantic_roundtrip_for_all_fixtures(root: &Path) -> io::Result<usize> {
    let files = collect_all_fixture_files(root)?;
    let mut exercised = 0usize;
    for file in files {
        assert_semantic_roundtrip_file(&file)?;
        exercised += 1;
    }
    Ok(exercised)
}

fn collect_all_fixture_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    collect_all_files_recursive(root, &mut out)?;
    out.sort();
    Ok(out)
}

fn collect_all_files_recursive(path: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        out.push(path.to_path_buf());
        return Ok(());
    }

    if !path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child = entry.path();
        collect_all_files_recursive(&child, out)?;
    }

    Ok(())
}

fn semantic_snapshot(catalog: &artisan_pcgen::ParsedCatalog) -> Value {
    let mut type_key_by_id: BTreeMap<String, String> = BTreeMap::new();
    for entity_type in &catalog.entity_types {
        type_key_by_id.insert(entity_type.id.0.to_string(), entity_type.key.clone());
    }

    let mut source_title_by_id: BTreeMap<String, String> = BTreeMap::new();
    for source in &catalog.sources {
        source_title_by_id.insert(source.id.0.to_string(), source.title.clone());
    }

    let mut entity_name_by_id: BTreeMap<String, String> = BTreeMap::new();
    for entity in &catalog.entities {
        entity_name_by_id.insert(entity.id.0.to_string(), entity.name.clone());
    }

    let mut types = Vec::new();
    for entity_type in &catalog.entity_types {
        types.push(json!({
            "key": entity_type.key,
            "name": entity_type.name,
            "parent": entity_type.parent.map(|p| p.0.to_string()),
            "descriptive_fields": entity_type.descriptive_fields,
            "mechanical_fields": entity_type.mechanical_fields,
        }));
    }

    let mut entities = Vec::new();
    for entity in &catalog.entities {
        let type_id = entity.entity_type.0.to_string();
        let type_key = type_key_by_id
            .get(&type_id)
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        entities.push(json!({
            "entity_type": type_key,
            "name": entity.name,
            "attributes": entity.attributes,
            "effects": entity.effects,
            "prerequisites": entity.prerequisites,
            "rule_hooks": entity.rule_hooks,
            "completeness": entity.completeness,
        }));
    }

    let mut publishers = Vec::new();
    for publisher in &catalog.publishers {
        publishers.push(json!({
            "name": publisher.name,
        }));
    }

    let mut sources = Vec::new();
    for source in &catalog.sources {
        let mut game_systems = source.game_systems.clone();
        game_systems.sort();
        sources.push(json!({
            "title": source.title,
            "publisher": source.publisher,
            "edition": source.edition,
            "game_systems": game_systems,
        }));
    }

    let mut citations = Vec::new();
    for citation in &catalog.citations {
        let subject = match &citation.subject {
            artisan_core::domain::SubjectRef::Entity(id) => entity_name_by_id
                .get(&id.0.to_string())
                .map(|name| format!("entity:{name}"))
                .unwrap_or_else(|| "entity:unknown".to_string()),
            artisan_core::domain::SubjectRef::EntityType(id) => type_key_by_id
                .get(&id.0.to_string())
                .map(|key| format!("entity_type:{key}"))
                .unwrap_or_else(|| "entity_type:unknown".to_string()),
        };
        let source = source_title_by_id
            .get(&citation.source.0.to_string())
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        citations.push(json!({
            "subject": subject,
            "source": source,
            "locators": citation.locators,
            "verification": citation.verification,
        }));
    }

    types.sort_by(|a, b| a["key"].as_str().cmp(&b["key"].as_str()));
    publishers.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    sources.sort_by(|a, b| a["title"].as_str().cmp(&b["title"].as_str()));
    citations.sort_by(|a, b| {
        a["subject"]
            .as_str()
            .cmp(&b["subject"].as_str())
            .then_with(|| a["source"].as_str().cmp(&b["source"].as_str()))
    });
    entities.sort_by(|a, b| {
        let a_line = a["attributes"]["line_number"].as_u64().unwrap_or(0);
        let b_line = b["attributes"]["line_number"].as_u64().unwrap_or(0);
        a_line
            .cmp(&b_line)
            .then_with(|| a["name"].as_str().cmp(&b["name"].as_str()))
    });

    json!({
        "publishers": publishers,
        "sources": sources,
        "citations": citations,
        "entity_types": types,
        "entities": entities,
    })
}

#[test]
fn unparse_emits_pcgen_metadata_lines_for_pcc() {
    let file = fixture_root().join("metadata_whitespace.pcc");
    let parsed = parse_file(&file).expect("parse metadata fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains("PUBNAMELONG:Wizards of the Coast"));
    assert!(generated.contains("SOURCELONG:Star Wars Saga Edition Core Rulebook"));
    assert!(generated.contains("SOURCESHORT:SWSECR"));
    assert!(generated.contains("GAMEMODE:Starwars_SE"));
}
