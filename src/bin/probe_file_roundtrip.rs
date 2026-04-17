//! Quick probe: investigate `abilities` attribute drops by entity type
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const PROVENANCE_ATTRS: &[&str] = &[
    "head", "clauses", "line_number", "pcgen_line_number",
    "pcgen_record_family", "pcgen_record_style", "source_format", "pcgen_source_page",
];

const INFRASTRUCTURE_ATTRS: &[&str] = &[
    "pcgen_entity_type_key", "pcgen_decl_token", "pcgen_mechanical_signals",
    "pcgen_key", "pcgen_nameispi", "pcgen_descispi", "pcgen_name_open", "pcgen_name_pi",
];

fn semantic_snapshot(catalog: &artisan_pcgen::ParsedCatalog) -> Value {
    let mut type_key_by_id: BTreeMap<String, String> = BTreeMap::new();
    for et in &catalog.entity_types {
        type_key_by_id.insert(et.id.0.to_string(), et.key.clone());
    }
    let mut entities = Vec::new();
    for entity in &catalog.entities {
        let type_key = type_key_by_id.get(&entity.entity_type.0.to_string()).cloned().unwrap_or_else(|| "unknown".to_string());
        let semantic_attrs: BTreeMap<String, Value> = entity.attributes.iter()
            .filter(|(k, _)| !PROVENANCE_ATTRS.contains(&k.as_str()) && !INFRASTRUCTURE_ATTRS.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let mut effects: Vec<Value> = entity.effects.iter()
            .map(|e| json!({ "kind": e.kind, "target": e.target, "value": e.value }))
            .collect();
        effects.sort_by(|a, b| a["kind"].as_str().cmp(&b["kind"].as_str())
            .then_with(|| a["target"].as_str().cmp(&b["target"].as_str())));
        entities.push(json!({ "entity_type": type_key, "name": entity.name, "attributes": semantic_attrs, "effects": effects }));
    }
    json!({ "types": catalog.entity_types.len(), "entities": entities })
}

/// Scan all entities in a file, return per-entity-type drop counts
fn probe_file(path: &str) -> BTreeMap<String, (usize, usize, usize, String, String, String)> {
    let mut result: BTreeMap<String, (usize, usize, usize, String, String, String)> = BTreeMap::new();
    let text = match fs::read_to_string(path) { Ok(t) => t, Err(_) => return result };
    let fname = match Path::new(path).file_name().and_then(|f| f.to_str()) { Some(n) => n, None => return result };
    let first = parse_text_to_catalog(&text, fname, "lst");
    let emitted = unparse_catalog_to_text(&first);
    let second = parse_text_to_catalog(&emitted, fname, "lst");

    let before = semantic_snapshot(&first);
    let after = semantic_snapshot(&second);

    let be = match before["entities"].as_array() { Some(a) => a, None => return result };
    let ae = match after["entities"].as_array() { Some(a) => a, None => return result };
    if be.len() != ae.len() { return result; }

    for (b, a) in be.iter().zip(ae.iter()) {
        let bv = &b["attributes"]["abilities"];
        let av = &a["attributes"]["abilities"];
        let etype = b["entity_type"].as_str().unwrap_or("?").to_string();
        let entry = result.entry(etype.clone()).or_insert((0, 0, 0, String::new(), String::new(), String::new()));
        if bv != &Value::Null && av == &Value::Null {
            entry.0 += 1;
            if entry.3.is_empty() {
                entry.3 = b["name"].as_str().unwrap_or("?").to_string();
                entry.4 = bv.to_string();
                entry.5 = av.to_string();
            }
        } else if bv == &Value::Null && av != &Value::Null {
            entry.1 += 1;
        } else if bv != av {
            entry.2 += 1;
            if entry.3.is_empty() {
                entry.3 = b["name"].as_str().unwrap_or("?").to_string();
                entry.4 = bv.to_string();
                entry.5 = av.to_string();
            }
        }
    }
    result.retain(|_, v| v.0 + v.1 + v.2 > 0);
    result
}

fn scan_dir(root: &str) {
    // per-entity-type totals
    let mut totals: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    let mut files_with_ability_entity_drops: Vec<(usize, String, String, String, String)> = Vec::new();

    let mut stack = vec![root.to_string()];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    stack.push(p.to_string_lossy().to_string());
                } else if p.extension().map_or(false, |e| e == "lst") {
                    let per_type = probe_file(&p.to_string_lossy());
                    let fname = p.file_name().and_then(|f| f.to_str()).unwrap_or("?").to_string();
                    for (etype, (d, a, c, name, before, after)) in &per_type {
                        let t = totals.entry(etype.clone()).or_insert((0, 0, 0));
                        t.0 += d; t.1 += a; t.2 += c;
                        if etype == "pcgen:entity:ability" && *d > 0 {
                            files_with_ability_entity_drops.push((*d, fname.clone(), name.clone(), before.clone(), after.clone()));
                        }
                    }
                }
            }
        }
    }

    println!("Root: {}", root);
    println!("Abilities churn by entity type (drop/add/change):");
    let mut sorted_totals: Vec<_> = totals.iter().collect();
    sorted_totals.sort_by(|a, b| (b.1.0 + b.1.1 + b.1.2).cmp(&(a.1.0 + a.1.1 + a.1.2)));
    for (etype, (d, a, c)) in sorted_totals.iter().take(15) {
        println!("  drop={d:5} add={a:5} chg={c:5}  {etype}");
    }
    println!("Files with pcgen:entity:ability abilities drops (top 5):");
    files_with_ability_entity_drops.sort_by(|a, b| b.0.cmp(&a.0));
    for (d, fname, name, before, after) in files_with_ability_entity_drops.iter().take(5) {
        println!("  [{d}] {fname}: name={name} before={before} after={after}");
    }
    println!();
}

/// Detailed single-file probe: show all dropped attributes for gear entities
fn probe_gear_drops(path: &str) {
    let text = match fs::read_to_string(path) { Ok(t) => t, Err(_) => { println!("NOTFOUND: {path}"); return; } };
    let fname = Path::new(path).file_name().and_then(|f| f.to_str()).unwrap_or("?");
    let first = parse_text_to_catalog(&text, fname, "lst");
    let emitted = unparse_catalog_to_text(&first);
    let second = parse_text_to_catalog(&emitted, fname, "lst");

    let before = semantic_snapshot(&first);
    let after = semantic_snapshot(&second);
    let be = before["entities"].as_array().unwrap();
    let ae = after["entities"].as_array().unwrap();

    let mut drops = 0usize;
    for (b, a) in be.iter().zip(ae.iter()) {
        let etype = b["entity_type"].as_str().unwrap_or("");
        if etype != "pcgen:entity:gear" { continue; }
        let b_loc = &b["attributes"]["location"];
        let a_loc = &a["attributes"]["location"];
        if b_loc != &Value::Null && a_loc == &Value::Null {
            drops += 1;
            if drops <= 3 {
                println!("  GEAR DROP: name={} location={}", b["name"], b_loc);
                // Show before/after attrs
                let ba = b["attributes"].as_object().unwrap();
                let aa = a["attributes"].as_object().unwrap();
                for (k, bv) in ba {
                    match aa.get(k) {
                        None => println!("    DROPPED: {}={}", k, bv),
                        Some(av) if av != bv => println!("    CHANGED: {}  was={}  now={}", k, bv, av),
                        _ => {}
                    }
                }
            }
        }
    }
    println!("{}: {} gear location drops", fname, drops);
}

fn main() {
    // Quick gear drop probe
    let kit_paths = [
        "../../../../externals/BahamutDragon/pcgen/data/35e/wizards_of_the_coast/core/players_handbook/ph_kits.lst",
        "../../../../externals/BahamutDragon/pcgen/data/35e/wizards_of_the_coast/core/monster_manual/mm_kits.lst",
    ];
    println!("=== Gear location drop probe ===");
    for p in &kit_paths {
        probe_gear_drops(p);
    }
    println!();

    let base = "../../../../externals/BahamutDragon/pcgen/data";
    for subdir in &["35e"] {
        let root = format!("{base}/{subdir}");
        if Path::new(&root).exists() {
            scan_dir(&root);
        }
    }
}
