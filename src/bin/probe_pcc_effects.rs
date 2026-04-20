//! Probe: find PCC entities with effects changes after roundtrip.
//! Scans ALL file types (same as semantic_inventory).
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use serde_json::{Value, json};
use std::collections::{BTreeMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

fn scan_dir(dir: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            scan_dir(&p, paths);
        } else if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            if ext == "lst" || ext == "pcc" {
                paths.push(p);
            }
        }
    }
}

fn entity_snapshot(entity: &artisan_pcgen::ParsedCatalog) -> BTreeMap<(String, String, usize), Vec<Value>> {
    let mut map: BTreeMap<(String, String, usize), Vec<Value>> = BTreeMap::new();
    let mut count: BTreeMap<(String, String), usize> = BTreeMap::new();

    for e in &entity.entities {
        let tk = e.attributes.get("pcgen_entity_type_key")
            .and_then(Value::as_str).unwrap_or("?").to_string();
        let key = (tk.clone(), e.name.clone());
        let idx = *count.entry(key.clone()).or_insert(0);
        *count.get_mut(&key).unwrap() += 1;

        let mut effects: Vec<Value> = e.effects.iter()
            .map(|eff| json!({"kind": eff.kind, "target": eff.target, "value": eff.value}))
            .collect();
        effects.sort_by(|a, b|
            a["kind"].as_str().cmp(&b["kind"].as_str())
                .then(a["target"].as_str().cmp(&b["target"].as_str()))
                .then(a["value"].as_str().cmp(&b["value"].as_str()))
        );
        map.insert((tk, e.name.clone(), idx), effects);
    }
    map
}

fn main() {
    let roots = [
        "/Users/afstanton/code/afstanton/artisan/externals/PCGen/pcgen",
        "/Users/afstanton/code/afstanton/artisan/externals/BahamutDragon/pcgen",
    ];

    let mut all_files: Vec<PathBuf> = Vec::new();
    for root in &roots {
        scan_dir(Path::new(root), &mut all_files);
    }

    let mut examples_shown = 0;
    let max_examples = 10;
    let mut total_changes = 0usize;

    'outer: for path in &all_files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("lst");
        let Ok(text) = fs::read_to_string(path) else { continue };
        let source_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("file");

        let catalog1 = parse_text_to_catalog(&text, source_name, ext);
        let emitted = unparse_catalog_to_text(&catalog1);
        let catalog2 = parse_text_to_catalog(&emitted, source_name, ext);

        if catalog1.entities.len() != catalog2.entities.len() { continue; }

        // Build VecDeque index for catalog2 (same as inventory)
        let mut after_index: BTreeMap<(String, String), VecDeque<Vec<Value>>> = BTreeMap::new();
        for e in &catalog2.entities {
            let tk = e.attributes.get("pcgen_entity_type_key")
                .and_then(Value::as_str).unwrap_or("?").to_string();
            let mut effects: Vec<Value> = e.effects.iter()
                .map(|eff| json!({"kind": eff.kind, "target": eff.target, "value": eff.value}))
                .collect();
            effects.sort_by(|a, b|
                a["kind"].as_str().cmp(&b["kind"].as_str())
                    .then(a["target"].as_str().cmp(&b["target"].as_str()))
                    .then(a["value"].as_str().cmp(&b["value"].as_str()))
            );
            after_index.entry((tk, e.name.clone())).or_default().push_back(effects);
        }

        for e in &catalog1.entities {
            let tk = e.attributes.get("pcgen_entity_type_key")
                .and_then(Value::as_str).unwrap_or("?").to_string();
            if !tk.starts_with("pcgen:entity:pcc") { continue; }

            let mut before_effects: Vec<Value> = e.effects.iter()
                .map(|eff| json!({"kind": eff.kind, "target": eff.target, "value": eff.value}))
                .collect();
            before_effects.sort_by(|a, b|
                a["kind"].as_str().cmp(&b["kind"].as_str())
                    .then(a["target"].as_str().cmp(&b["target"].as_str()))
                    .then(a["value"].as_str().cmp(&b["value"].as_str()))
            );

            if let Some(queue) = after_index.get_mut(&(tk.clone(), e.name.clone()))
                && let Some(after_effects) = queue.pop_front()
            {
                if before_effects != after_effects {
                    total_changes += 1;
                    if examples_shown < max_examples {
                        examples_shown += 1;
                        let short = path.display().to_string();
                        let short = short.split("externals/").last().unwrap_or(&short);
                        println!("FILE: {}", short);
                        println!("  entity_type={} name={:?}", tk, e.name);
                        let max_len = before_effects.len().max(after_effects.len());
                        for i in 0..max_len {
                            let b = before_effects.get(i);
                            let a = after_effects.get(i);
                            if b != a {
                                println!("  BEFORE[{}]: {:?}", i, b);
                                println!("  AFTER[{}]:  {:?}", i, a);
                                if examples_shown < max_examples { break; }
                            }
                        }
                        println!("  Count: {} before → {} after", before_effects.len(), after_effects.len());
                        println!();
                        if examples_shown >= max_examples { break 'outer; }
                    }
                }
            }
        }
    }

    println!("Total PCC effect changes found: {}", total_changes);
}
