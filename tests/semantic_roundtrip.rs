use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
};

use artisan_pcgen::{
    ParsedClause, fallback_keys_for_entity, parse_file, parse_line, parse_text_to_catalog,
    unparse_catalog_to_text,
};
use serde_json::{Value, json};

fn fixture_root() -> PathBuf {
    if let Ok(custom) = std::env::var("ARTISAN_PCGEN_FIXTURES_DIR") {
        return PathBuf::from(custom);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pcgen")
}

/// Fixtures excluded from the **semantic** roundtrip test.
///
/// Semantic roundtrip: parse → emit → re-parse, then compare projected
/// `pcgen_*` attributes (format-independent meaning), ignoring wire-format
/// provenance (`clauses`, `head`, line numbers, record family/style).
fn should_semantic_roundtrip_fixture(path: &Path) -> bool {
    !matches!(
        path.file_name().and_then(|f| f.to_str()),
        // Policy tokens have no emit path, so re-parsing yields fewer entities.
        Some("roundtrip_policy_tokens.lst")
    )
}

/// Fixtures excluded from the **text-fidelity** roundtrip test.
///
/// Text fidelity: parse → emit, then assert every token in the original source
/// appears in the emitted output (token-order-normalised, line-order-normalised).
fn should_text_fidelity_roundtrip_fixture(path: &Path) -> bool {
    !matches!(
        path.file_name().and_then(|f| f.to_str()),
        // Policy tokens have no emit path.
        Some("roundtrip_policy_tokens.lst")
        // Multi-line LST entities merge on parse and emit as a single line.
        | Some("roundtrip_multiline_class.lst")
        // FACTDEF uses `|` inside its value (FACTDEF:RACE|BaseSize) making
        // intra-line token-order normalisation ambiguous.  Semantic fidelity
        // is tested by the semantic roundtrip instead.
        | Some("metadata_whitespace.pcc")
        // ABILITY with KEY≠head: PCGen's KEY token overrides the entity name,
        // so the emitted head differs from the source head.  This is
        // intentional and tested in the semantic roundtrip (which compares
        // pcgen_key, not the raw head).
        | Some("roundtrip_ability.lst")
    )
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
    assert!(
        root.exists(),
        "fixture root does not exist: {}",
        root.display()
    );

    let files = collect_all_fixture_files(&root).expect("collect fixture files");
    let roundtrip_files: Vec<_> = files
        .into_iter()
        .filter(|path| should_semantic_roundtrip_fixture(path))
        .collect();
    assert!(
        !roundtrip_files.is_empty(),
        "no fixture files found under {}",
        root.display()
    );

    let exercised = assert_semantic_roundtrip_for_all_fixtures(Path::new(&root))
        .expect("roundtrip all fixtures");
    assert_eq!(
        exercised,
        roundtrip_files.len(),
        "every fixture file should be exercised exactly once"
    );
}

/// Text-fidelity roundtrip: parse → emit, then verify that every non-comment
/// line from the original source appears (possibly reordered) in the emitted
/// output.  This catches token-level regressions where the *meaning* survives
/// but a specific token stops being emitted.
///
/// This is intentionally weaker than byte-for-byte equality: PCGen files use
/// varying whitespace and column order within a line, and the emitter
/// normalises both.  The fidelity guarantee is at the **token** level, not the
/// character level.
#[test]
fn text_fidelity_all_fixture_files() {
    let root = fixture_root();
    let files = collect_all_fixture_files(&root).expect("collect fixture files");
    let mut exercised = 0usize;

    for file in &files {
        if !should_text_fidelity_roundtrip_fixture(file) {
            continue;
        }

        let source_name = file
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("fixture");
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_else(|| "unknown".to_string());

        let original_text = fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("read fixture {}: {e}", file.display()));
        let parsed = parse_text_to_catalog(&original_text, source_name, &ext);
        let emitted = unparse_catalog_to_text(&parsed);

        let original_lines = text_fidelity_snapshot(&original_text);
        let emitted_lines = text_fidelity_snapshot(&emitted);

        assert_eq!(
            original_lines,
            emitted_lines,
            "text-fidelity mismatch for {}:\n  original lines: {original_lines:?}\n  emitted lines:  {emitted_lines:?}",
            file.display(),
        );

        exercised += 1;
    }

    assert!(exercised > 0, "no text-fidelity fixture files exercised");
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
    assert_eq!(
        before,
        after,
        "semantic roundtrip mismatch: {}",
        path.display()
    );
    Ok(())
}

/// Verify that bracket-group token values are parsed into structured JSON arrays,
/// not stored as raw opaque strings.
#[test]
fn bracket_group_values_are_structured() {
    // CLASSBOUGHT — single group → array containing one bracket-group array
    // pcgen_classbought is always [[{key,value},...]] so each group is an element.
    let cat = parse_text_to_catalog(
        "SKILL:Spellcraft|CLASSBOUGHT:[CLASS:Wizard|RANKS:3.0|COST:1|CLASSSKILL:Y]",
        "test.pcg",
        "pcg",
    );
    let entity = &cat.entities[0];
    let classbought = entity
        .attributes
        .get("pcgen_classbought")
        .expect("pcgen_classbought should be set");
    let groups = classbought.as_array().expect("pcgen_classbought should be an array");
    assert_eq!(groups.len(), 1, "single CLASSBOUGHT group should produce a 1-element outer array");
    let arr = groups[0].as_array().expect("each CLASSBOUGHT group should be an inner array");
    assert_eq!(arr.len(), 4, "CLASSBOUGHT group should have 4 sub-entries");
    assert_eq!(arr[0]["key"], "CLASS");
    assert_eq!(arr[0]["value"], "Wizard");
    assert_eq!(arr[1]["key"], "RANKS");
    assert_eq!(arr[1]["value"], "3.0");
    assert_eq!(arr[2]["key"], "COST");
    assert_eq!(arr[3]["key"], "CLASSSKILL");
    assert_eq!(arr[3]["value"], "Y");

    // DEITYDOMAINS — multiple same-key entries
    let cat2 = parse_text_to_catalog(
        "DEITY:Pelor|DEITYDOMAINS:[DOMAIN:Good|DOMAIN:Sun]|DEITYALIGN:NG",
        "test.pcg",
        "pcg",
    );
    let e2 = &cat2.entities[0];
    let domains = e2
        .attributes
        .get("pcgen_deitydomains")
        .and_then(|v| v.as_array())
        .expect("pcgen_deitydomains should be an array");
    assert_eq!(domains.len(), 2);
    assert_eq!(domains[0]["key"], "DOMAIN");
    assert_eq!(domains[0]["value"], "Good");
    assert_eq!(domains[1]["key"], "DOMAIN");
    assert_eq!(domains[1]["value"], "Sun");

    // WEAPONPROF head value — stored as bracket group on the entity
    let cat3 = parse_text_to_catalog(
        "WEAPONPROF:[WEAPON:Longsword|WEAPON:Dagger|WEAPON:Quarterstaff]",
        "test.pcg",
        "pcg",
    );
    let e3 = &cat3.entities[0];
    let weapons = e3
        .attributes
        .get("pcgen_weaponprof_catalog")
        .and_then(|v| v.as_array())
        .expect("pcgen_weaponprof_catalog should be an array for PCG bracket form");
    assert_eq!(weapons.len(), 3);
    assert_eq!(weapons[0]["key"], "WEAPON");
    assert_eq!(weapons[0]["value"], "Longsword");
    assert_eq!(weapons[2]["value"], "Quarterstaff");
}

/// Verify that bracket-group structured values survive a full parse → emit → reparse cycle
/// with all sub-entries intact and no data loss.
#[test]
fn bracket_group_round_trips_preserve_sub_entries() {
    let lines = "\
SKILL:Spellcraft|CLASSBOUGHT:[CLASS:Wizard|RANKS:3.0|COST:1|CLASSSKILL:Y]\n\
DEITY:Pelor|DEITYDOMAINS:[DOMAIN:Good|DOMAIN:Sun]|DEITYFAVWEAP:[WEAPON:Morningstar]|DEITYALIGN:NG\n\
WEAPONPROF:[WEAPON:Longsword|WEAPON:Dagger|WEAPON:Quarterstaff]\n\
SPELLNAME:Fireball|TIMES:2|CLASS:Wizard|BOOK:Combat Book|SPELLLEVEL:3|FEATLIST:[FEAT:Heighten Spell|FEAT:Empower Spell]";

    let first = parse_text_to_catalog(lines, "test.pcg", "pcg");
    let emitted = unparse_catalog_to_text(&first);
    let second = parse_text_to_catalog(&emitted, "test.pcg", "pcg");

    // CLASSBOUGHT round-trip — single group wraps as [[{sub-entries}]]
    let skill_after = second
        .entities
        .iter()
        .find(|e| e.name == "Spellcraft")
        .expect("Spellcraft entity should survive round-trip");
    let cb_groups = skill_after
        .attributes
        .get("pcgen_classbought")
        .and_then(|v| v.as_array())
        .expect("pcgen_classbought should be an array after round-trip");
    assert_eq!(cb_groups.len(), 1);
    let cb = cb_groups[0].as_array().expect("inner group should be an array");
    assert_eq!(cb.len(), 4);
    assert_eq!(cb[0]["key"], "CLASS");
    assert_eq!(cb[0]["value"], "Wizard");

    // DEITYFAVWEAP round-trip
    let deity_after = second
        .entities
        .iter()
        .find(|e| e.name == "Pelor")
        .expect("Pelor entity should survive round-trip");
    let favweap = deity_after
        .attributes
        .get("pcgen_deityfavweap")
        .and_then(|v| v.as_array())
        .expect("pcgen_deityfavweap should be an array after round-trip");
    assert_eq!(favweap.len(), 1);
    assert_eq!(favweap[0]["key"], "WEAPON");
    assert_eq!(favweap[0]["value"], "Morningstar");

    // WEAPONPROF round-trip — all three weapons present
    let weaponprof_after = second
        .entities
        .iter()
        .find(|e| {
            e.attributes
                .get("pcgen_entity_type_key")
                .and_then(|v| v.as_str())
                .is_some_and(|k| k.contains("weaponprof"))
        })
        .expect("WEAPONPROF entity should survive round-trip");
    let wps = weaponprof_after
        .attributes
        .get("pcgen_weaponprof_catalog")
        .and_then(|v| v.as_array())
        .expect("pcgen_weaponprof_catalog should be an array after round-trip");
    assert_eq!(wps.len(), 3);
    assert!(wps.iter().any(|w| w["value"] == "Longsword"));
    assert!(wps.iter().any(|w| w["value"] == "Dagger"));
    assert!(wps.iter().any(|w| w["value"] == "Quarterstaff"));

    // FEATLIST round-trip — two feats present
    let spell_after = second
        .entities
        .iter()
        .find(|e| e.name == "Fireball")
        .expect("Fireball entity should survive round-trip");
    let feats = spell_after
        .attributes
        .get("pcgen_featlist")
        .and_then(|v| v.as_array())
        .expect("pcgen_featlist should be an array after round-trip");
    assert_eq!(feats.len(), 2);
    assert!(feats.iter().any(|f| f["value"] == "Heighten Spell"));
    assert!(feats.iter().any(|f| f["value"] == "Empower Spell"));
}

fn assert_semantic_roundtrip_for_all_fixtures(root: &Path) -> io::Result<usize> {
    let files = collect_all_fixture_files(root)?;
    let mut exercised = 0usize;
    for file in files {
        if !should_semantic_roundtrip_fixture(&file) {
            continue;
        }
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
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_ascii_lowercase();
            if ext == "lst" || ext == "pcc" || ext == "pcg" {
                out.push(path.to_path_buf());
            }
        }
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

/// Parse-provenance attributes that are artifacts of the PCGen wire format and
/// parse position.  These are excluded from the semantic snapshot so that the
/// roundtrip comparison reflects *meaning*, not representation.
///
/// - `head` / `clauses`: raw PCGen line structure before projection
/// - `line_number` / `pcgen_line_number`: position in the source file
/// - `pcgen_record_family` / `pcgen_record_style`: structural classification
///   of the line (e.g. "lst:token-entry") — doesn't survive emit/re-parse
///   unchanged for merged entities
/// - `source_format`: redundant with the file extension used to parse
///
/// Everything that starts with `pcgen_` but is NOT in this set is a projected
/// semantic field and is included in the comparison.
const PROVENANCE_ATTRS: &[&str] = &[
    "head",
    "clauses",
    "line_number",
    "pcgen_line_number",
    "pcgen_record_family",
    "pcgen_record_style",
    "source_format",
];

/// Build a canonical, provenance-free snapshot of a parsed catalog for
/// semantic comparison.
///
/// This intentionally does NOT include wire-format artifacts.  If you need
/// to verify exact line reconstruction use `text_fidelity_snapshot` instead.
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
        }));
    }

    let mut entities = Vec::new();
    for entity in &catalog.entities {
        let type_id = entity.entity_type.0.to_string();
        let type_key = type_key_by_id
            .get(&type_id)
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());

        // Retain only projected semantic attributes; drop provenance.
        let mut semantic_attrs: BTreeMap<String, Value> = entity
            .attributes
            .iter()
            .filter(|(k, _)| !PROVENANCE_ATTRS.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Normalise the pcgen entity type key to the canonical type key so
        // that the comparison is independent of how the type was inferred.
        let entity_type_key = semantic_attrs
            .get("pcgen_entity_type_key")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        if !entity_type_key.is_empty() {
            semantic_attrs.insert(
                "pcgen_entity_type_key".to_string(),
                Value::String(entity_type_key),
            );
        }

        // Sort effects and prerequisites so comparison is order-independent.
        // The emitter may reorder effects relative to the source (e.g. token-def
        // fields before global-group effects), so we must not treat insertion
        // order as semantically significant.
        let mut effects: Vec<Value> = entity
            .effects
            .iter()
            .map(|e| json!({ "kind": e.kind, "target": e.target, "value": e.value }))
            .collect();
        effects.sort_by(|a, b| {
            a["kind"]
                .as_str()
                .cmp(&b["kind"].as_str())
                .then_with(|| a["target"].as_str().cmp(&b["target"].as_str()))
        });

        let mut prereqs: Vec<Value> = entity
            .prerequisites
            .iter()
            .map(|p| json!({ "kind": p.kind, "expression": p.expression }))
            .collect();
        prereqs.sort_by(|a, b| {
            a["kind"]
                .as_str()
                .cmp(&b["kind"].as_str())
                .then_with(|| a["expression"].as_str().cmp(&b["expression"].as_str()))
        });

        entities.push(json!({
            "entity_type": type_key,
            "name": entity.name,
            "attributes": semantic_attrs,
            "effects": effects,
            "prerequisites": prereqs,
            "completeness": entity.completeness,
        }));
    }

    let mut publishers = Vec::new();
    for publisher in &catalog.publishers {
        publishers.push(json!({ "name": publisher.name }));
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
    // Sort by (entity_type_key, name) — stable regardless of parse order or
    // whether multi-line merging happened.
    entities.sort_by(|a, b| {
        let a_type = a["attributes"]["pcgen_entity_type_key"]
            .as_str()
            .unwrap_or("");
        let b_type = b["attributes"]["pcgen_entity_type_key"]
            .as_str()
            .unwrap_or("");
        a_type
            .cmp(b_type)
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

/// Build a snapshot of ONLY the artisan-core canonical fields — entity name, entity type,
/// effects, and prerequisites — with no `pcgen_*` attributes whatsoever.
///
/// This is the basis for the canonical roundtrip test.  Where this snapshot is thin
/// (e.g. `effects: []`) despite a content-rich entity, that gap is the signal for future
/// canonical-model work: those tokens currently live only in the `pcgen_*` projection and
/// have not yet been lifted into the artisan-core canonical model.
fn core_snapshot(catalog: &artisan_pcgen::ParsedCatalog) -> Value {
    let mut type_key_by_id: BTreeMap<String, String> = BTreeMap::new();
    for et in &catalog.entity_types {
        type_key_by_id.insert(et.id.0.to_string(), et.key.clone());
    }

    let mut entities: Vec<Value> = catalog
        .entities
        .iter()
        .map(|entity| {
            let type_key = type_key_by_id
                .get(&entity.entity_type.0.to_string())
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());

            // Sort so comparison is order-independent.
            let mut effects: Vec<Value> = entity
                .effects
                .iter()
                .map(|e| json!({ "kind": e.kind, "target": e.target, "value": e.value }))
                .collect();
            effects.sort_by(|a, b| {
                a["kind"]
                    .as_str()
                    .cmp(&b["kind"].as_str())
                    .then_with(|| a["target"].as_str().cmp(&b["target"].as_str()))
            });

            let mut prereqs: Vec<Value> = entity
                .prerequisites
                .iter()
                .map(|p| json!({ "kind": p.kind, "expression": p.expression }))
                .collect();
            prereqs.sort_by(|a, b| {
                a["kind"]
                    .as_str()
                    .cmp(&b["kind"].as_str())
                    .then_with(|| a["expression"].as_str().cmp(&b["expression"].as_str()))
            });

            json!({
                "name": entity.name,
                "entity_type": type_key,
                "effects": effects,
                "prerequisites": prereqs,
            })
        })
        .collect();

    // Stable sort by (entity_type, name) regardless of parse order.
    entities.sort_by(|a, b| {
        a["entity_type"]
            .as_str()
            .cmp(&b["entity_type"].as_str())
            .then_with(|| a["name"].as_str().cmp(&b["name"].as_str()))
    });

    json!({ "entities": entities })
}

/// Produce a human-readable diff summary between two `core_snapshot` values,
/// naming the specific entity and field that diverged.  Used in assertion
/// failure messages.
fn diff_core_snapshots(before: &Value, after: &Value) -> String {
    let empty = vec![];
    let before_entities = before["entities"].as_array().unwrap_or(&empty);
    let after_entities = after["entities"].as_array().unwrap_or(&empty);

    if before_entities.len() != after_entities.len() {
        return format!(
            "entity count changed: {} → {}",
            before_entities.len(),
            after_entities.len()
        );
    }

    for (b, a) in before_entities.iter().zip(after_entities.iter()) {
        let name = b["name"].as_str().unwrap_or("?");
        let etype = b["entity_type"].as_str().unwrap_or("?");
        if b["name"] != a["name"] || b["entity_type"] != a["entity_type"] {
            return format!(
                "entity identity changed: ({etype}, {name}) → ({}, {})",
                a["entity_type"].as_str().unwrap_or("?"),
                a["name"].as_str().unwrap_or("?"),
            );
        }
        if b["effects"] != a["effects"] {
            return format!(
                "effects mismatch for entity ({etype}, {name}):\n  before: {}\n  after:  {}",
                b["effects"], a["effects"]
            );
        }
        if b["prerequisites"] != a["prerequisites"] {
            return format!(
                "prerequisites mismatch for entity ({etype}, {name}):\n  before: {}\n  after:  {}",
                b["prerequisites"], a["prerequisites"]
            );
        }
    }

    // Snapshots are equal — no diff to report.
    String::new()
}

/// Fixtures included in the **canonical** roundtrip test.
///
/// More permissive than the text-fidelity filter.  The canonical test only checks
/// artisan-core fields (name, entity_type, effects, prerequisites), so it can include
/// fixtures where the text form changes across the roundtrip.
fn should_canonical_roundtrip_fixture(path: &Path) -> bool {
    !matches!(
        path.file_name().and_then(|f| f.to_str()),
        // Policy tokens have no emit path; re-parsing yields fewer entities.
        Some("roundtrip_policy_tokens.lst")
    )
}

/// Print the canonical artisan-core content across all fixture files.
///
/// Provides a readable map of what the canonical model currently captures vs. what
/// remains only in `pcgen_*` projected attributes.  Run with `--nocapture` to see output.
/// Also prints a frequency table of `pcgen_*` attributes found on sparse entities
/// (those with neither effects nor prerequisites), showing which tokens are the
/// best candidates for lifting into the canonical model next.
#[test]
fn canonical_coverage_report() {
    use std::collections::HashMap;

    let root = fixture_root();
    let files = collect_all_fixture_files(&root).expect("collect fixture files");

    // Infrastructure-only attribute keys that don't count as canonical content.
    // These are parser bookkeeping and provenance, not game data.
    const INFRASTRUCTURE_ATTRS: &[&str] = &[
        "pcgen_decl_token",
        "pcgen_decl_value",
        "pcgen_mechanical_signals",
        "pcgen_entity_type_key",
        "decl_token",
        "decl_value",
        "mechanical_signals",
        "entity_type_key",
    ];

    let mut total_entities = 0usize;
    let mut covered_effects_only = 0usize;
    let mut covered_prereqs_only = 0usize;
    let mut covered_both = 0usize;
    let mut covered_canonical_attr_only = 0usize; // has clean attr or citation but no effects/prereqs
    let mut truly_sparse = 0usize;

    // For sparse entities: tally pcgen_* keys still remaining (Phase N work)
    let mut sparse_pcgen_attr_counts: HashMap<String, usize> = HashMap::new();
    // For sparse entities: tally by entity type to identify frontiers
    let mut sparse_type_counts: HashMap<String, usize> = HashMap::new();
    // For covered entities with clean canonical attrs: tally those keys
    let mut canonical_attr_counts: HashMap<String, usize> = HashMap::new();
    // For ALL entities: effect and prereq kinds
    let mut effect_kind_counts: HashMap<String, usize> = HashMap::new();
    let mut prereq_kind_counts: HashMap<String, usize> = HashMap::new();

    for file in &files {
        if !should_canonical_roundtrip_fixture(file) {
            continue;
        }
        let source_name = file.file_name().and_then(|f| f.to_str()).unwrap_or("?");
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_default();
        let text = fs::read_to_string(file).expect("read");
        let cat = parse_text_to_catalog(&text, source_name, &ext);

        for entity in &cat.entities {
            total_entities += 1;
            let has_effects = !entity.effects.is_empty();
            let has_prereqs = !entity.prerequisites.is_empty();
            // An entity has canonical attributes if any attribute key is neither a
            // pcgen_* key nor an infrastructure-only key.
            let has_canonical_attrs = entity.attributes.keys().any(|k| {
                !k.starts_with("pcgen_")
                    && !PROVENANCE_ATTRS.contains(&k.as_str())
                    && !INFRASTRUCTURE_ATTRS.contains(&k.as_str())
            });
            let has_citations = !entity.citations.is_empty();
            let is_covered = has_effects || has_prereqs || has_canonical_attrs || has_citations;

            match (has_effects, has_prereqs) {
                (true, true) => covered_both += 1,
                (true, false) => covered_effects_only += 1,
                (false, true) => covered_prereqs_only += 1,
                (false, false) if is_covered => covered_canonical_attr_only += 1,
                _ => {
                    truly_sparse += 1;
                    // Track sparse entity type distribution
                    let type_key = entity
                        .attributes
                        .get("pcgen_entity_type_key")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    *sparse_type_counts.entry(type_key).or_insert(0) += 1;
                    // Tally pcgen_* keys still on sparse entities (Phase N candidates)
                    for key in entity.attributes.keys() {
                        if key.starts_with("pcgen_")
                            && !PROVENANCE_ATTRS.contains(&key.as_str())
                            && !INFRASTRUCTURE_ATTRS.contains(&key.as_str())
                        {
                            *sparse_pcgen_attr_counts.entry(key.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }

            // Tally clean canonical attribute keys across ALL covered entities
            if is_covered {
                for key in entity.attributes.keys() {
                    if !key.starts_with("pcgen_")
                        && !PROVENANCE_ATTRS.contains(&key.as_str())
                        && !INFRASTRUCTURE_ATTRS.contains(&key.as_str())
                    {
                        *canonical_attr_counts.entry(key.clone()).or_insert(0) += 1;
                    }
                }
            }

            for e in &entity.effects {
                *effect_kind_counts.entry(e.kind.clone()).or_insert(0) += 1;
            }
            for p in &entity.prerequisites {
                *prereq_kind_counts.entry(p.kind.clone()).or_insert(0) += 1;
            }
        }
    }

    let covered = total_entities - truly_sparse;
    let pct = if total_entities > 0 { 100 * covered / total_entities } else { 0 };

    println!("\n=== Canonical Model Coverage Report ===");
    println!("Total entities:                    {total_entities}");
    println!("  effects + prerequisites:         {covered_both}");
    println!("  effects only:                    {covered_effects_only}");
    println!("  prerequisites only:              {covered_prereqs_only}");
    println!("  canonical attrs / citations:     {covered_canonical_attr_only}");
    println!("  truly sparse (pcgen_* only):     {truly_sparse}");
    println!("  canonical coverage:              {pct}% have ≥1 canonical field");

    // Effect kinds in use
    let mut effect_kinds: Vec<_> = effect_kind_counts.iter().collect();
    effect_kinds.sort_by(|a, b| b.1.cmp(a.1));
    println!("\n--- Effect kinds (BONUS/ADD/etc.) across all entities ---");
    for (kind, count) in &effect_kinds {
        println!("  {:6} {kind}", count);
    }

    // Prerequisite kinds in use (top 20)
    let mut prereq_kinds: Vec<_> = prereq_kind_counts.iter().collect();
    prereq_kinds.sort_by(|a, b| b.1.cmp(a.1));
    println!("\n--- Prerequisite kinds (PRE*/!PRE*) across all entities (top 20) ---");
    for (kind, count) in prereq_kinds.iter().take(20) {
        println!("  {:6} {kind}", count);
    }

    // Canonical attribute keys on covered entities
    let mut canonical_attrs: Vec<_> = canonical_attr_counts.iter().collect();
    canonical_attrs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    println!("\n--- Canonical attribute keys on covered entities (top 20) ---");
    for (attr, count) in canonical_attrs.iter().take(20) {
        println!("  {:6} {attr}", count);
    }

    // Remaining pcgen_* attributes on truly sparse entities — Phase N candidates
    let mut sparse_attrs: Vec<_> = sparse_pcgen_attr_counts.iter().collect();
    sparse_attrs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    println!("\n--- pcgen_* attributes on truly sparse entities (top 30) ---");
    println!("  (These tokens still need canonical mapping)");
    for (attr, count) in sparse_attrs.iter().take(30) {
        let display = attr.strip_prefix("pcgen_").unwrap_or(attr);
        println!("  {:6} {display}", count);
    }
    // Sparse entity type breakdown
    let mut sparse_types: Vec<_> = sparse_type_counts.iter().collect();
    sparse_types.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    println!("\n--- Sparse entities by type (top 20) ---");
    for (entity_type, count) in sparse_types.iter().take(20) {
        println!("  {:6} {entity_type}", count);
    }

    println!(
        "\nNote: 'truly sparse' entities have only pcgen_* attributes (no canonical fields).\n"
    );
}

/// Canonical artisan-core roundtrip: parse → emit → re-parse, then compare ONLY the
/// artisan-core canonical fields (name, entity_type, effects, prerequisites).
///
/// All `pcgen_*` projected attributes are intentionally excluded from this comparison.
/// Sparseness in the snapshot — an entity with `effects: []` despite having complex
/// game mechanics — is expected and deliberate: it reveals exactly what work remains
/// to lift more semantic content into the artisan-core canonical model.
#[test]
fn canonical_roundtrip_all_fixture_files() {
    let root = fixture_root();
    let files = collect_all_fixture_files(&root).expect("collect fixture files");
    let mut exercised = 0usize;

    for file in &files {
        if !should_canonical_roundtrip_fixture(file) {
            continue;
        }

        let source_name = file
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("fixture");
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_else(|| "unknown".to_string());

        let original_text = fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("read fixture {}: {e}", file.display()));

        let first = parse_text_to_catalog(&original_text, source_name, &ext);
        let generated = unparse_catalog_to_text(&first);
        let second = parse_text_to_catalog(&generated, source_name, &ext);

        let before = core_snapshot(&first);
        let after = core_snapshot(&second);

        if before != after {
            let diff = diff_core_snapshots(&before, &after);
            panic!(
                "canonical roundtrip mismatch for {}:\n{diff}",
                file.display()
            );
        }

        exercised += 1;
    }

    assert!(exercised > 0, "no canonical roundtrip fixture files exercised");
}

/// Build a snapshot focused on text-level fidelity.
///
/// Each line is normalised by sorting its pipe/tab-delimited tokens
/// alphabetically (after the head), so that token-order differences in the
/// emitter don't cause spurious failures.  Lines are then sorted as a whole
/// so that entity-order differences don't matter either.
///
/// Used by the text-fidelity roundtrip test, which verifies that every token
/// from the original source is reconstructed by the emitter.
fn text_fidelity_snapshot(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = text
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(normalise_token_order)
        .collect();
    lines.sort();
    lines
}

/// Normalise token order within a single PCGen line so that intra-line token
/// ordering differences between the original source and the emitter don't
/// cause false test failures.
///
/// PCGen lines use one of three separators: tab (`\t`), pipe (`|`), or
/// whitespace (for PCC-style `KEY:VALUE KEY:VALUE` directives).  In all cases
/// the first segment is the "head" and is kept in place; remaining tokens are
/// sorted alphabetically.
fn normalise_token_order(line: String) -> String {
    // Tab-separated (LST / PCG token-entry lines).
    if line.contains('\t') {
        let parts: Vec<&str> = line.split('\t').collect();
        let head = parts[0];
        let mut rest: Vec<&str> = parts[1..].to_vec();
        rest.sort();
        return format!("{head}\t{}", rest.join("\t"));
    }

    // Pipe-separated (PCG inline records, PCC include lines).
    // Only normalise if every non-head segment looks like KEY:VALUE or a
    // bare word — don't reorder prose that happens to contain a pipe.
    if line.contains('|') {
        let parts: Vec<&str> = line.split('|').collect();
        let head = parts[0];
        let mut rest: Vec<&str> = parts[1..].to_vec();
        rest.sort();
        return format!("{head}|{}", rest.join("|"));
    }

    // Whitespace-separated (PCC metadata directives like
    // `GAMEMODE:x BOOKTYPE:y SETTING:z`).  Split on whitespace boundaries
    // that precede a TOKEN: start, sort, rejoin with spaces.
    let tokens = artisan_pcgen::parse_line(&line);
    if tokens.clauses.len() > 1 {
        // Re-emit as sorted KEY:VALUE pairs.
        let head_str = tokens.head.clone();
        let mut clause_strs: Vec<String> = tokens
            .clauses
            .iter()
            .map(|c| match c {
                artisan_pcgen::ParsedClause::Bare(v) => v.clone(),
                artisan_pcgen::ParsedClause::KeyValue { key, value } => {
                    format!("{key}:{value}")
                }
            })
            .collect();
        clause_strs.sort();
        return format!("{head_str} {}", clause_strs.join(" "));
    }

    line
}

/// Verify that multiline LST entities (CLASS:Name spanning 3 lines) have their
/// continuation-line tokens (MODTOSKILLS, MONSKILL, MONNONSKILLHD) correctly
/// reported as fully-structured, using entity-key lookup rather than line number.
#[test]
fn multiline_entity_continuation_tokens_count_as_fully_structured() {
    // Three-line CLASS:Aberration as it appears in srd_classes_creature_types.lst.
    // Line 1: core stats.  Line 2: race prerequisite.  Line 3: skill config.
    let content = "\
CLASS:Aberration\tHD:8\tTYPE:Monster\tMAXLEVEL:NOLIMIT\n\
CLASS:Aberration\tPRERAGETYPE:Aberration\n\
CLASS:Aberration\tSTARTSKILLPTS:2\tMODTOSKILLS:NO\tMONSKILL:2*INTSCORE\tMONNONSKILLHD:1\n";

    // Should parse as a single merged entity
    let cat = parse_text_to_catalog(content, "test.lst", "lst");
    assert_eq!(cat.entities.len(), 1, "multiline CLASS should merge to one entity");

    let entity = &cat.entities[0];
    let type_key = entity.attributes.get("pcgen_entity_type_key")
        .and_then(|v| v.as_str()).unwrap_or("MISSING");
    let line_num = entity.attributes.get("pcgen_line_number")
        .and_then(|v| v.as_u64()).unwrap_or(0);
    // Merged entity lives at line 1 (the first CLASS:Aberration line)
    assert_eq!(line_num, 1, "merged entity should have first-line number");

    let schema = artisan_pcgen::schema::schema_for_entity_type_key(type_key).unwrap();
    let emittable: std::collections::HashSet<String> =
        artisan_pcgen::emittable_keys_for_entity(entity, schema)
            .into_iter()
            .collect();

    // All three tokens from line 3 must be emittable (proving the schema handles them)
    assert!(emittable.contains("MODTOSKILLS"), "MODTOSKILLS not emittable: {emittable:?}");
    assert!(emittable.contains("MONSKILL"),    "MONSKILL not emittable: {emittable:?}");
    assert!(emittable.contains("MONNONSKILLHD"), "MONNONSKILLHD not emittable: {emittable:?}");

    // Verify the entity's decl_token/decl_value are set (needed for entity-key lookup)
    let decl_token = entity.attributes.get("pcgen_decl_token")
        .and_then(|v| v.as_str()).unwrap_or("MISSING");
    let decl_value = entity.attributes.get("pcgen_decl_value")
        .and_then(|v| v.as_str()).unwrap_or("MISSING");
    assert_eq!(decl_token, "CLASS", "decl_token should be CLASS");
    assert_eq!(decl_value, "Aberration", "decl_value should be Aberration");
}

#[test]
fn unparse_emits_pcgen_metadata_lines_for_pcc() {
    let file = fixture_root().join("metadata_whitespace.pcc");
    let parsed = parse_file(&file).expect("parse metadata fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains("PUBNAMELONG:Wizards of the Coast"));
    assert!(generated.contains("SOURCELONG:Star Wars Saga Edition Core Rulebook"));
    assert!(generated.contains("SOURCESHORT:SWSECR"));
    assert!(generated.contains("GAMEMODE:Starwars_SE SETTING:Space Opera BOOKTYPE:Core Rulebook"));
    assert!(generated.contains("GAMEMODE:Starwars_SE"));
    assert!(generated.contains("OPTION:pcgen.options.optionSourcesAllowMultiLine=true"));
    assert!(generated.contains("MAXVER:6.10.0"));
    assert!(generated.contains("NEWKEY:SOURCEPAGE"));
    assert!(generated.contains("DATAFORMAT:LST"));
    assert!(generated.contains("DISPLAYNAME:Source Metadata Label"));
    assert!(generated.contains("EXPLANATION:Core rules metadata"));
    assert!(generated.contains("REQUIRED:YES"));
    assert!(generated.contains("SELECTABLE:YES"));
    assert!(generated.contains("NAMEISPI:YES"));
    assert!(generated.contains("COPYRIGHT:Open Game License v1.0a, Section 15 excerpt"));
    assert!(generated.contains("FACTDEF:RACE|BaseSize"));
}

#[test]
fn unparse_emits_structured_pcc_backlog_tokens() {
    let file = fixture_root().join("roundtrip_pcc_backlog_tokens.pcc");
    let parsed = parse_file(&file).expect("parse pcc backlog fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains("LICENSE:Community Use"));
    assert!(generated.contains("INFOTEXT:Supports 6.10.0"));
    assert!(generated.contains("FORWARDREF:RACE|Orc"));
    assert!(generated.contains("HIDETYPE:FEAT|AttackOption|ModifyAC"));
    assert!(generated.contains("URL:WEBSITE|http://example.com/|Example"));
    assert!(generated.contains("ISMATURE:NO"));
    assert!(generated.contains("VARIABLE:backlog_variables.lst"));
}

#[test]
fn unparse_emits_structured_system_misc_tokens() {
    let file = fixture_root().join("roundtrip_system_misc_tokens.lst");
    let parsed = parse_file(&file).expect("parse system misc fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains("SKILLCOST_CLASS:1"));
    assert!(generated.contains("SKILLCOST_EXCLUSIVE:4"));
    assert!(generated.contains("SPELLBASECONCENTRATION:CASTERLEVEL+BASESPELLSTAT"));
    assert!(generated.contains("XPAWARD:1/8=50|1/6=65|1=400"));
    assert!(generated.contains("STATINPUT:STATSCORE"));
}

#[test]
fn unparse_emits_structured_datacontrol_tokens() {
    let file = fixture_root().join("roundtrip_datacontrol_tokens.lst");
    let parsed = parse_file(&file).expect("parse datacontrol fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains("FUNCTION:d20Mod"));
    assert!(generated.contains("VALUE:floor((arg(0)-10)/2)"));
    assert!(generated.contains("EXPLANATION:For Ability Score Calculation of Bonus"));
    assert!(generated.contains("DYNAMICSCOPE:MOVEMENT"));
    assert!(generated.contains("DYNAMICSCOPE:VISION"));
}

#[test]
fn unparse_emits_structured_entity_gap_tokens() {
    let file = fixture_root().join("roundtrip_entity_gap_tokens.lst");
    let parsed = parse_file(&file).expect("parse entity gap fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(
        generated.contains("HASSUBCLASS:YES"),
        "HASSUBCLASS should be emitted: {generated}"
    );
    assert!(
        generated.contains("COSTPRE:9000"),
        "COSTPRE should be emitted: {generated}"
    );
    assert!(
        generated.contains("BASEAGEADD:3"),
        "BASEAGEADD should be emitted: {generated}"
    );
    assert!(
        generated.contains("PROHIBITED:Necromancy|Enchantment"),
        "PROHIBITED should be emitted: {generated}"
    );
    assert!(
        generated.contains("FORTIFICATION:25"),
        "FORTIFICATION should be emitted: {generated}"
    );
    assert!(
        generated.contains("HEALING:5"),
        "HEALING should be emitted: {generated}"
    );
}

#[test]
fn unparse_emits_structured_pcg_standalone_tokens() {
    let file = fixture_root().join("roundtrip_pcg_standalone_tokens.pcg");
    let parsed = parse_file(&file).expect("parse pcg standalone fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains("PCGVERSION:2.0"));
    assert!(generated.contains("PURCHASEPOINTS:N"));
    assert!(generated.contains("POOLPOINTS:0"));
    assert!(generated.contains("POOLPOINTSAVAIL:-1"));
    assert!(generated.contains("TABLABEL:0"));
    assert!(generated.contains("AUTOSPELLS:YES"));
    assert!(generated.contains("USEHIGHERKNOWN:NO"));
    assert!(generated.contains("USEHIGHERPREPPED:NO"));
    assert!(generated.contains("LOADCOMPANIONS:NO"));
    assert!(generated.contains("USETEMPMODS:NO"));
    assert!(generated.contains("SKILLSOUTPUTORDER:0"));
    assert!(generated.contains("SKILLFILTER:2"));
    assert!(generated.contains("IGNORECOST:NO"));
    assert!(generated.contains("ALLOWDEBT:NO"));
    assert!(generated.contains("AUTORESIZEGEAR:YES"));
    assert!(generated.contains("CHARACTERNAME:Sample Hero"));
    assert!(generated.contains("PLAYERNAME:"));
    assert!(generated.contains("HEIGHT:72"));
    assert!(generated.contains("WEIGHT:180"));
    assert!(generated.contains("AGE:35"));
    assert!(generated.contains("HANDED:Right"));
    assert!(generated.contains("STAT:STR"));
    assert!(generated.contains("SCORE:18"));
}

#[test]
fn unparse_preserves_bracketed_pcg_record_shapes() {
    let file = fixture_root().join("inventory_only_pcg_tokens.pcg");
    let parsed = parse_file(&file).expect("parse bracketed pcg fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains(
        "SPELLNAME:Magic Missile|TIMES:1|BOOK:Known Spells|SPELLLEVEL:1|CLASS:Wizard|FEATLIST:[FEAT:Empower Spell]"
    ));
    assert!(generated.contains(
        "DEITY:Pelor|DEITYDOMAINS:[DOMAIN:Good|DOMAIN:Sun]|ALIGNALLOW:LG|HOLYITEM:Sun Disk|DEITYFAVWEAP:[WEAPON:Morningstar]|DEITYALIGN:NG"
    ));
    assert!(generated.contains("DOMAIN:Sun|DOMAINGRANTS:Turn undead as a cleric of higher level."));
    assert!(generated.contains(
        "WEAPONPROF:[WEAPON:Longsword|WEAPON:Dagger|WEAPON:Quarterstaff]"
    ));
}

#[test]
fn unparse_preserves_nested_pcg_progression_records() {
    let file = fixture_root().join("roundtrip_character_progression.pcg");
    let parsed = parse_file(&file).expect("parse character progression fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains(
        "EQUIPNAME:Longsword|OUTPUTORDER:1|COST:15 gp|WT:4 lb.|QUANTITY:1|CUSTOMIZATION:[BASEITEM:Longsword|DATA:EQMOD=STEEL]|NOTE:A trusty blade"
    ));
    assert!(generated.contains(
        "SKILL:Spellcraft|CLASSBOUGHT:[CLASS:Wizard|RANKS:3.0|CLASSSKILL:Y]"
    ));
}

#[test]
fn unparse_emits_structured_ability_migration_tokens() {
    let file = fixture_root().join("roundtrip_ability_migration.lst");
    let parsed = parse_file(&file).expect("parse ability migration fixture");
    let generated = unparse_catalog_to_text(&parsed);

    assert!(generated.contains("ABILITY:FEAT|Mythic Feat ~ Drink Is Life"));
    assert!(generated.contains("MAXVER:6.07.05"));
    assert!(generated.contains("NEWKEY:Drink Is Life"));
    assert!(generated.contains("NEWCATEGORY:Mythic Feat"));
}

#[test]
fn unparse_emits_lower_frequency_structured_tokens() {
    let class_file = fixture_root().join("roundtrip_class.lst");
    let class_generated = unparse_catalog_to_text(&parse_file(&class_file).expect("parse class fixture"));
    assert!(class_generated.contains("SKILLLIST:1|Druid"));

    let equipment_file = fixture_root().join("roundtrip_equipment.lst");
    let equipment_generated =
        unparse_catalog_to_text(&parse_file(&equipment_file).expect("parse equipment fixture"));
    assert!(equipment_generated.contains("ALTCRITICAL:x3"));

    let template_file = fixture_root().join("roundtrip_template.lst");
    let template_generated =
        unparse_catalog_to_text(&parse_file(&template_file).expect("parse template fixture"));
    assert!(template_generated.contains("NONPP:-4"));

    let pcc_file = fixture_root().join("roundtrip_pcc_backlog_tokens.pcc");
    let pcc_generated = unparse_catalog_to_text(&parse_file(&pcc_file).expect("parse pcc fixture"));
    assert!(pcc_generated.contains("HELP:./help_campaign.html"));
}

#[test]
fn roundtrip_fixtures_use_zero_raw_clause_fallback_for_schema_entities() {
    let root = fixture_root();
    let files = collect_all_fixture_files(&root).expect("collect fixtures");

    let mut checked = 0usize;
    for file in files {
        let name = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if !name.starts_with("roundtrip_") {
            continue;
        }

        let parsed = parse_file(&file).expect("parse fixture");
        for entity in &parsed.entities {
            let type_key = entity
                .attributes
                .get("pcgen_entity_type_key")
                .and_then(Value::as_str)
                .unwrap_or("pcgen:type:unresolved");

            let schema = artisan_pcgen::schema::schema_for_entity_type_key(type_key)
                .unwrap_or_else(|| {
                    panic!(
                        "{} / {} has no schema for inferred type key {}",
                        file.display(),
                        entity.name,
                        type_key
                    )
                });

            let fallbacks = fallback_keys_for_entity(entity, schema);
            assert!(
                fallbacks.is_empty(),
                "{} / {} still uses fallback tokens: {:?}",
                file.display(),
                entity.name,
                fallbacks
            );
            checked += 1;
        }
    }

    assert!(
        checked > 0,
        "expected at least one schema-bound roundtrip fixture entity"
    );
}

/// Verify that adjacent bracket groups on a single line — `][` with no pipe between them —
/// are split into separate CLASSBOUGHT clauses, each parsed as a structured array.
///
/// Real corpus example from `Everything.pcg`:
/// ```text
/// SKILL:Perform|OUTPUTORDER:22|CLASSBOUGHT:[CLASS:Bard|RANKS:5.0|COST:1|CLASSSKILL:Y]CLASSBOUGHT:[CLASS:Aristocrat|RANKS:1.0|COST:1|CLASSSKILL:Y]
/// ```
#[test]
fn adjacent_bracket_groups_are_split_into_separate_clauses() {
    // Two CLASSBOUGHT entries with no pipe or whitespace between the closing `]`
    // and the opening `CLASSBOUGHT:[` of the next entry.
    let line = "SKILL:Perform|OUTPUTORDER:22|CLASSBOUGHT:[CLASS:Bard|RANKS:5.0|COST:1|CLASSSKILL:Y]CLASSBOUGHT:[CLASS:Aristocrat|RANKS:1.0|COST:1|CLASSSKILL:Y]";
    let cat = parse_text_to_catalog(line, "test.pcg", "pcg");
    let entity = cat
        .entities
        .iter()
        .find(|e| e.name == "Perform")
        .expect("Perform skill should be parsed");

    // pcgen_classbought should be an array because the token is Repeatable.
    let cb = entity
        .attributes
        .get("pcgen_classbought")
        .expect("pcgen_classbought should be set");
    let arr = cb.as_array().expect("pcgen_classbought should be an array");

    // Two separate bracket groups → two array elements (each itself an array of sub-entries).
    assert_eq!(arr.len(), 2, "should have one entry per CLASSBOUGHT group; got {arr:?}");

    // First group: Bard at rank 5.0
    let first = arr[0].as_array().expect("first CLASSBOUGHT should be an array");
    assert_eq!(first[0]["key"], "CLASS");
    assert_eq!(first[0]["value"], "Bard");
    assert_eq!(first[1]["key"], "RANKS");
    assert_eq!(first[1]["value"], "5.0");

    // Second group: Aristocrat at rank 1.0
    let second = arr[1].as_array().expect("second CLASSBOUGHT should be an array");
    assert_eq!(second[0]["key"], "CLASS");
    assert_eq!(second[0]["value"], "Aristocrat");
    assert_eq!(second[1]["key"], "RANKS");
    assert_eq!(second[1]["value"], "1.0");
}

/// Adjacent bracket groups survive a full parse → emit → reparse cycle with all sub-entries.
#[test]
fn adjacent_bracket_groups_round_trip() {
    let line = "SKILL:Perform|OUTPUTORDER:22|CLASSBOUGHT:[CLASS:Bard|RANKS:5.0|COST:1|CLASSSKILL:Y]CLASSBOUGHT:[CLASS:Aristocrat|RANKS:1.0|COST:1|CLASSSKILL:Y]";

    let first = parse_text_to_catalog(line, "test.pcg", "pcg");
    let emitted = unparse_catalog_to_text(&first);
    let second = parse_text_to_catalog(&emitted, "test.pcg", "pcg");

    let skill = second
        .entities
        .iter()
        .find(|e| e.name == "Perform")
        .expect("Perform should survive round-trip");

    let arr = skill
        .attributes
        .get("pcgen_classbought")
        .and_then(|v| v.as_array())
        .expect("pcgen_classbought should be an array after round-trip");

    assert_eq!(arr.len(), 2, "both CLASSBOUGHT groups should survive round-trip");
    let first_groups = arr[0].as_array().expect("first group should be array");
    assert_eq!(first_groups[0]["value"], "Bard");
    let second_groups = arr[1].as_array().expect("second group should be array");
    assert_eq!(second_groups[0]["value"], "Aristocrat");
}

// ---------------------------------------------------------------------------
// Multi-line LST entity merging
// ---------------------------------------------------------------------------

/// PCGen LST files sometimes split a single logical entity (e.g. `CLASS:Faceman`)
/// across multiple lines.  The parser should merge them into one entity.
#[test]
fn multiline_lst_entity_merges_into_single_entity() {
    // Two CLASS:Faceman lines — first carries structural tokens, second adds
    // skill-related tokens that would overflow the first line.
    let lines = "\
CLASS:Faceman\tHITDIE:10\tTYPE:Base.PC\tMAXLEVEL:20\tABB:Fcm\tSOURCEPAGE:42\n\
CLASS:Faceman\tSTARTSKILLPTS:6\tCSKILL:Bluff|Diplomacy|Disguise";

    let cat = parse_text_to_catalog(lines, "test.lst", "lst");

    // There should be exactly ONE entity, not two.
    let faceman_entities: Vec<_> = cat.entities.iter().filter(|e| e.name == "Faceman").collect();
    assert_eq!(
        faceman_entities.len(), 1,
        "two CLASS:Faceman lines should merge into a single entity"
    );

    let e = faceman_entities[0];

    // Attributes from the first line should be present.
    assert_eq!(
        e.attributes.get("hitdie").and_then(|v| v.as_i64()),
        Some(10),
        "HITDIE from line 1 should be on merged entity"
    );
    assert_eq!(
        e.attributes.get("abbreviation").and_then(|v| v.as_str()),
        Some("Fcm"),
        "ABB from line 1 should be on merged entity"
    );

    // Attributes from the second line should also be present.
    assert_eq!(
        e.attributes.get("pcgen_startskillpts").and_then(|v| v.as_i64()),
        Some(6),
        "STARTSKILLPTS from line 2 should be on merged entity"
    );
}

/// The merged entity has a stable identity — it is the *first* line's id and
/// external_id, not the second.
#[test]
fn multiline_lst_entity_keeps_first_line_identity() {
    let lines = "\
CLASS:Faceman\tHITDIE:10\tMAXLEVEL:20\n\
CLASS:Faceman\tSTARTSKILLPTS:6";

    let cat = parse_text_to_catalog(lines, "identity.lst", "lst");
    let faceman: Vec<_> = cat.entities.iter().filter(|e| e.name == "Faceman").collect();
    assert_eq!(faceman.len(), 1);

    // Line number should be 1 (first occurrence).
    assert_eq!(
        faceman[0].attributes.get("pcgen_line_number").and_then(|v| v.as_i64()),
        Some(1),
        "merged entity should carry line_number from the first occurrence"
    );

    // Only one external_id (from line 1).
    assert_eq!(
        faceman[0].external_ids.len(), 1,
        "merged entity should have exactly one external_id (from first line)"
    );
    assert!(
        faceman[0].external_ids[0].value.ends_with(":1"),
        "external_id should reference line 1, got: {}",
        faceman[0].external_ids[0].value
    );
}

/// The fixture file exercises multi-line merging in the semantic round-trip runner.
#[test]
fn multiline_lst_fixture_produces_single_faceman_entity() {
    let file = fixture_root().join("roundtrip_multiline_class.lst");
    let parsed = parse_file(&file).expect("parse multiline class fixture");

    let faceman: Vec<_> = parsed.entities.iter().filter(|e| e.name == "Faceman").collect();
    assert_eq!(
        faceman.len(), 1,
        "fixture should produce exactly one Faceman entity"
    );
    // Both lines contributed attributes.
    assert!(faceman[0].attributes.contains_key("hitdie"), "line 1 HITDIE should be present");
    assert!(faceman[0].attributes.contains_key("pcgen_startskillpts"), "line 2 STARTSKILLPTS should be present");
}

// ---------------------------------------------------------------------------
// CLASSABILITIESLEVEL relationship annotation
// ---------------------------------------------------------------------------

/// CLASSABILITIESLEVEL lines in PCG files should carry `pcgen_for_class` (the
/// parent class name) and `class_level` (the integer level) derived from
/// the `ClassName=LevelNumber` head value.
#[test]
fn classabilitieslevel_annotates_for_class_and_level() {
    let cat = parse_text_to_catalog(
        "CLASSABILITIESLEVEL:Wizard=5|HITPOINTS:4|SKILLSGAINED:2|SKILLSREMAINING:0",
        "test.pcg",
        "pcg",
    );
    let cal = cat
        .entities
        .iter()
        .find(|e| e.name == "Wizard=5")
        .expect("CLASSABILITIESLEVEL entity should be parsed");

    assert_eq!(
        cal.attributes.get("pcgen_for_class").and_then(|v| v.as_str()),
        Some("Wizard"),
        "pcgen_for_class should name the parent class"
    );
    assert_eq!(
        cal.attributes.get("class_level").and_then(|v| v.as_i64()),
        Some(5),
        "class_level should be the integer level"
    );
    // Raw value preserved for round-trip.
    assert_eq!(
        cal.attributes.get("pcgen_cal_classname_level").and_then(|v| v.as_str()),
        Some("Wizard=5"),
        "raw classname_level should be stored for round-trip fidelity"
    );
}

// ---------------------------------------------------------------------------
// MODIFY canonical effect projection
// ---------------------------------------------------------------------------

/// MODIFY clauses are game-mechanical variable modifications.  They should be
/// projected into the canonical `effects` field (just like BONUS/DEFINE/ADD),
/// with the full `VarName|OP|value` string stored as the effect `target`.
///
/// This test verifies that all three MODIFY operation variants (ADD, SET, SOLVE)
/// produce effects and that the effects survive a parse → emit → reparse cycle.
#[test]
fn modify_clause_projects_to_effects() {
    let lines = "\
CoverageAbility_MODIFY_ADD\tCATEGORY:Internal\tMODIFY:TestVar|ADD|1\n\
CoverageAbility_MODIFY_SET\tCATEGORY:Internal\tMODIFY:Damage|SET|10d6\n\
CoverageAbility_MODIFY_SOLVE\tCATEGORY:Internal\tMODIFY:Score|SOLVE|10\n";

    let cat = parse_text_to_catalog(lines, "test.lst", "lst");

    let find = |name: &str| {
        cat.entities.iter().find(|e| e.name == name)
            .unwrap_or_else(|| panic!("entity {name} not found"))
    };

    // ADD variant: MODIFY:TestVar|ADD|1 → target="TestVar", value=Some("ADD|1")
    let add_entity = find("CoverageAbility_MODIFY_ADD");
    assert_eq!(add_entity.effects.len(), 1, "ADD entity should have 1 effect");
    assert_eq!(add_entity.effects[0].kind, "MODIFY");
    assert_eq!(add_entity.effects[0].target, "TestVar");
    assert_eq!(add_entity.effects[0].value.as_deref(), Some("ADD|1"));

    // SET variant: MODIFY:Damage|SET|10d6 → target="Damage", value=Some("SET|10d6")
    let set_entity = find("CoverageAbility_MODIFY_SET");
    assert_eq!(set_entity.effects.len(), 1, "SET entity should have 1 effect");
    assert_eq!(set_entity.effects[0].kind, "MODIFY");
    assert_eq!(set_entity.effects[0].target, "Damage");
    assert_eq!(set_entity.effects[0].value.as_deref(), Some("SET|10d6"));

    // SOLVE variant: MODIFY:Score|SOLVE|10 → target="Score", value=Some("SOLVE|10")
    let solve_entity = find("CoverageAbility_MODIFY_SOLVE");
    assert_eq!(solve_entity.effects.len(), 1, "SOLVE entity should have 1 effect");
    assert_eq!(solve_entity.effects[0].kind, "MODIFY");
    assert_eq!(solve_entity.effects[0].target, "Score");
    assert_eq!(solve_entity.effects[0].value.as_deref(), Some("SOLVE|10"));

    // The pcgen_modify_* structured attributes should ALSO still be present,
    // since they are set independently by fields.rs.
    assert!(add_entity.attributes.contains_key("pcgen_modify_variable"),
        "pcgen_modify_variable should be set alongside the effect");
}

/// TEMPLATE clauses represent template grants — a game-mechanical effect applied
/// to the bearer.  They should appear in `effects` so canonical consumers don't
/// need to know the `pcgen_template` attribute.
///
/// Note: the ability/feat schemas already declare TEMPLATE as ArtisanMapping::Effect;
/// this test confirms that projection is consistent with that declaration.
#[test]
fn template_clause_projects_to_effects() {
    // TEMPLATE on a CLASS entity — handled via GlobalGroup::Template (attribute path)
    let class_cat = parse_text_to_catalog(
        "CLASS:Coverage_TEMPLATE\tTEMPLATE:Coverage",
        "test.lst",
        "lst",
    );
    let class_entity = class_cat.entities.iter()
        .find(|e| e.name == "Coverage_TEMPLATE")
        .expect("CLASS with TEMPLATE should be parsed");
    let template_effects: Vec<_> = class_entity.effects.iter()
        .filter(|e| e.kind == "TEMPLATE").collect();
    assert_eq!(template_effects.len(), 1, "CLASS TEMPLATE should produce 1 effect");
    assert_eq!(template_effects[0].target, "Coverage");

    // TEMPLATE on an ABILITY entity — handled via ArtisanMapping::Effect (effects path)
    let ability_cat = parse_text_to_catalog(
        "CelestialBlessing\tCATEGORY:Special Ability\tTEMPLATE:Celestial",
        "test.lst",
        "lst",
    );
    let ability_entity = ability_cat.entities.iter()
        .find(|e| e.name == "CelestialBlessing")
        .expect("ABILITY with TEMPLATE should be parsed");
    let ab_template_effects: Vec<_> = ability_entity.effects.iter()
        .filter(|e| e.kind == "TEMPLATE").collect();
    assert_eq!(ab_template_effects.len(), 1, "ABILITY TEMPLATE should produce 1 effect");
    assert_eq!(ab_template_effects[0].target, "Celestial");
}

/// ABILITY grant clauses (ABILITY:Category|AUTO|Name) are game-mechanical grants.
/// They should appear in `effects` so canonical consumers can see them without
/// inspecting the `pcgen_abilities` attribute.
#[test]
fn ability_grant_clause_projects_to_effects() {
    let cat = parse_text_to_catalog(
        "FOLLOWER:AnimalLVL=1\tTYPE:Animal Companion\tABILITY:Special Ability|AUTOMATIC|Companion Bond\tBONUS:STAT|STR|1",
        "test.lst",
        "lst",
    );
    let entity = cat.entities.iter()
        .find(|e| e.name == "AnimalLVL=1")
        .expect("FOLLOWER with ABILITY grant should be parsed");

    let ability_effects: Vec<_> = entity.effects.iter()
        .filter(|e| e.kind == "ABILITY").collect();
    assert_eq!(ability_effects.len(), 1, "ABILITY grant should produce 1 effect");
    assert_eq!(ability_effects[0].target, "Special Ability|AUTOMATIC|Companion Bond");

    // The pcgen_abilities attribute should also still be set (emit path uses it)
    assert!(entity.attributes.contains_key("pcgen_abilities"),
        "pcgen_abilities attribute should be set alongside the effect");
}

/// MODIFY effects survive the full parse → emit → reparse canonical roundtrip.
/// After the roundtrip, the canonical `effects` list should be unchanged.
#[test]
fn modify_effects_survive_canonical_roundtrip() {
    let lines = "\
CoverageAbility_MODIFY_ADD\tCATEGORY:Internal\tMODIFY:TestVar|ADD|1\n\
CoverageAbility_MODIFY_SET\tCATEGORY:Internal\tMODIFY:Damage|SET|10d6\n";

    let first = parse_text_to_catalog(lines, "test.lst", "lst");
    let emitted = unparse_catalog_to_text(&first);
    let second = parse_text_to_catalog(&emitted, "test.lst", "lst");

    let before = core_snapshot(&first);
    let after = core_snapshot(&second);
    assert_eq!(
        before, after,
        "MODIFY effects should survive canonical roundtrip:\n{}",
        diff_core_snapshots(&before, &after)
    );

    // Both first and second parse should have exactly one MODIFY effect per entity.
    for cat in [&first, &second] {
        for entity in &cat.entities {
            let modify_effects: Vec<_> = entity.effects.iter()
                .filter(|e| e.kind == "MODIFY").collect();
            assert_eq!(modify_effects.len(), 1,
                "entity {} should have exactly 1 MODIFY effect after roundtrip; got: {:?}",
                entity.name, modify_effects);
        }
    }
}

/// The character progression fixture contains a `CLASSABILITIESLEVEL:Wizard=5`
/// line; verify it picks up the relationship annotation.
#[test]
fn character_progression_fixture_classabilitieslevel_has_for_class() {
    let file = fixture_root().join("roundtrip_character_progression.pcg");
    let parsed = parse_file(&file).expect("parse character progression fixture");

    let cal = parsed
        .entities
        .iter()
        .find(|e| {
            e.attributes
                .get("pcgen_entity_type_key")
                .and_then(|v| v.as_str())
                .is_some_and(|k| k == "pcgen:pcg:classabilitieslevel")
        })
        .expect("CLASSABILITIESLEVEL entity should be in character progression fixture");

    assert_eq!(
        cal.attributes.get("pcgen_for_class").and_then(|v| v.as_str()),
        Some("Wizard"),
        "fixture CLASSABILITIESLEVEL should have pcgen_for_class=Wizard"
    );
    assert_eq!(
        cal.attributes.get("class_level").and_then(|v| v.as_i64()),
        Some(5),
        "fixture CLASSABILITIESLEVEL should have class_level=5"
    );
}
