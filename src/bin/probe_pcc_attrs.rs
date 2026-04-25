//! Probe: show exact semantic snapshot diffs for a PCC file roundtrip.
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;

const PROVENANCE: &[&str] = &[
    "head", "clauses", "line_number", "pcgen_line_number",
    "pcgen_record_family", "pcgen_record_style", "source_format", "pcgen_source_page",
];
const INFRASTRUCTURE: &[&str] = &[
    "pcgen_decl_token", "pcgen_decl_value", "pcgen_mechanical_signals",
    "pcgen_entity_type_key", "pcgen_name_open", "pcgen_name_pi", "pcgen_nameispi",
];

fn semantic_snapshot(catalog: &artisan_pcgen::ParsedCatalog) -> Value {
    let mut type_key_by_id: BTreeMap<String, String> = BTreeMap::new();
    for et in &catalog.entity_types { type_key_by_id.insert(et.id.0.to_string(), et.key.clone()); }

    let mut source_title_by_id: BTreeMap<String, String> = BTreeMap::new();
    for source in &catalog.sources { source_title_by_id.insert(source.id.0.to_string(), source.title.clone()); }

    let mut entity_name_by_id: BTreeMap<String, String> = BTreeMap::new();
    for entity in &catalog.entities { entity_name_by_id.insert(entity.id.0.to_string(), entity.name.clone()); }

    let mut types = Vec::new();
    for et in &catalog.entity_types { types.push(json!({"key": et.key, "name": et.name})); }

    let mut entities = Vec::new();
    for entity in &catalog.entities {
        let type_key = type_key_by_id.get(&entity.entity_type.0.to_string()).cloned().unwrap_or_default();
        let semantic_attrs: BTreeMap<String, Value> = entity.attributes.iter()
            .filter(|(k, _)| !PROVENANCE.contains(&k.as_str()) && !INFRASTRUCTURE.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let mut effects: Vec<Value> = entity.effects.iter()
            .map(|e| json!({"kind": e.kind, "target": e.target, "value": e.value}))
            .collect();
        effects.sort_by(|a, b| a["kind"].as_str().cmp(&b["kind"].as_str()).then(a["target"].as_str().cmp(&b["target"].as_str())));
        let mut prereqs: Vec<Value> = entity.prerequisites.iter()
            .map(|p| json!({"kind": p.kind, "expression": p.expression}))
            .collect();
        prereqs.sort_by(|a, b| a["kind"].as_str().cmp(&b["kind"].as_str()).then(a["expression"].as_str().cmp(&b["expression"].as_str())));
        entities.push(json!({
            "entity_type": type_key,
            "name": entity.name,
            "attributes": semantic_attrs,
            "effects": effects,
            "prerequisites": prereqs,
            "completeness": format!("{:?}", entity.completeness),
        }));
    }

    let mut publishers: Vec<Value> = catalog.publishers.iter().map(|p| json!({"name": p.name})).collect();
    let mut sources: Vec<Value> = catalog.sources.iter().map(|s| {
        let mut gs = s.game_systems.clone(); gs.sort();
        json!({"title": s.title, "publisher": s.publisher, "edition": s.edition, "game_systems": gs})
    }).collect();
    let mut citations: Vec<Value> = catalog.citations.iter().map(|c| {
        let subject = match &c.subject {
            artisan_core::domain::SubjectRef::Entity(id) => format!("entity:{}", entity_name_by_id.get(&id.0.to_string()).cloned().unwrap_or_default()),
            artisan_core::domain::SubjectRef::EntityType(id) => format!("entity_type:{}", type_key_by_id.get(&id.0.to_string()).cloned().unwrap_or_default()),
        };
        let src = source_title_by_id.get(&c.source.0.to_string()).cloned().unwrap_or_default();
        json!({"subject": subject, "source": src, "locators": c.locators, "verification": c.verification})
    }).collect();

    types.sort_by(|a, b| a["key"].as_str().cmp(&b["key"].as_str()));
    publishers.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    sources.sort_by(|a, b| a["title"].as_str().cmp(&b["title"].as_str()));
    citations.sort_by(|a, b| a["subject"].as_str().cmp(&b["subject"].as_str()).then(a["source"].as_str().cmp(&b["source"].as_str())));
    citations.dedup();
    // Use entity_type (top-level field, not the excluded infrastructure attr)
    entities.sort_by(|a, b| a["entity_type"].as_str().cmp(&b["entity_type"].as_str()).then(a["name"].as_str().cmp(&b["name"].as_str())));

    json!({"publishers": publishers, "sources": sources, "citations": citations, "entity_types": types, "entities": entities})
}

fn print_diff(before: &Value, after: &Value, path: &str) {
    if before == after { return; }
    match (before, after) {
        (Value::Object(b), Value::Object(a)) => {
            let all_keys: std::collections::BTreeSet<&String> = b.keys().chain(a.keys()).collect();
            for key in all_keys {
                let bv = b.get(key).unwrap_or(&Value::Null);
                let av = a.get(key).unwrap_or(&Value::Null);
                print_diff(bv, av, &format!("{}/{}", path, key));
            }
        }
        (Value::Array(b), Value::Array(a)) => {
            if b.len() != a.len() {
                println!("ARRAY_LEN_DIFF at {}: {} → {}", path, b.len(), a.len());
                println!("  BEFORE: {}", serde_json::to_string(before).unwrap());
                println!("  AFTER:  {}", serde_json::to_string(after).unwrap());
            } else {
                for (i, (bv, av)) in b.iter().zip(a.iter()).enumerate() {
                    print_diff(bv, av, &format!("{}[{}]", path, i));
                }
            }
        }
        _ => {
            println!("DIFF at {}:", path);
            println!("  BEFORE: {}", serde_json::to_string(before).unwrap());
            println!("  AFTER:  {}", serde_json::to_string(after).unwrap());
        }
    }
}

fn main() {
    #[path = "probeutil/config.rs"] mod probe_config;
    let config = probe_config::ProbeConfig::load();
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| config.get("default_pcc").to_string());
    let ext = std::path::Path::new(&path).extension().and_then(|e| e.to_str()).unwrap_or("pcc");
    let sn = std::path::Path::new(&path).file_name().and_then(|f| f.to_str()).unwrap_or("file");
    let text = fs::read_to_string(&path).expect("read file");

    let cat1 = parse_text_to_catalog(&text, sn, ext);
    let emitted = unparse_catalog_to_text(&cat1);

    // Debug: print emitted lines that look like metadata directives
    println!("=== Emitted metadata lines ===");
    for (i, line) in emitted.lines().enumerate() {
        let up = line.to_ascii_uppercase();
        if up.starts_with("SOURCELONG") || up.starts_with("SOURCESHORT") || up.starts_with("SOURCE") || up.starts_with("CAMPAIGN") {
            println!("  line {}: {:?}", i + 1, line);
        }
    }
    println!();

    let cat2 = parse_text_to_catalog(&emitted, sn, ext);

    println!("Entities before: {} / after: {}", cat1.entities.len(), cat2.entities.len());
    println!("Sources before: {} / after: {}", cat1.sources.len(), cat2.sources.len());
    println!("Citations before: {} / after: {}", cat1.citations.len(), cat2.citations.len());
    println!();

    println!("=== BEFORE entities ===");
    for e in &cat1.entities { println!("  {:?} type={}", e.name, e.attributes.get("pcgen_entity_type_key").and_then(|v| v.as_str()).unwrap_or("?")); }
    println!();
    println!("=== AFTER entities ===");
    for e in &cat2.entities { println!("  {:?}", e.name); }
    println!();

    let snap1 = semantic_snapshot(&cat1);
    let snap2 = semantic_snapshot(&cat2);

    if snap1 == snap2 {
        println!("Snapshots are IDENTICAL — no diff");
    } else {
        println!("=== Snapshot diffs ===");
        print_diff(&snap1, &snap2, "");
    }
}
