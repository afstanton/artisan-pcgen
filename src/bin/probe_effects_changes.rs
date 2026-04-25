//! Probe: find non-PCC entities with effects changes after roundtrip.
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

fn effect_snapshot(eff: &artisan_core::domain::rules::Effect) -> Value {
    json!({"kind": eff.kind, "target": eff.target, "value": eff.value})
}

fn main() {
    #[path = "probeutil/config.rs"] mod probe_config;
    let config = probe_config::ProbeConfig::load();

    let mut all_files: Vec<PathBuf> = Vec::new();
    for root in config.scan_roots() {
        scan_dir(Path::new(root), &mut all_files);
    }

    let target_types = ["pcgen:entity:race", "pcgen:entity:template",
                        "pcgen:entity:ability", "pcgen:entity:class",
                        "pcgen:entity:feat", "pcgen:entity:variable-global"];

    let mut examples_shown = 0;
    let max_examples = 15;
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    for path in &all_files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("lst");
        let Ok(text) = fs::read_to_string(path) else { continue };
        let source_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("file");

        let catalog1 = parse_text_to_catalog(&text, source_name, ext);
        let emitted = unparse_catalog_to_text(&catalog1);
        let catalog2 = parse_text_to_catalog(&emitted, source_name, ext);

        if catalog1.entities.len() != catalog2.entities.len() { continue; }

        // Build after index
        let mut after_index: BTreeMap<(String, String), VecDeque<Vec<Value>>> = BTreeMap::new();
        for e in &catalog2.entities {
            let tk = e.attributes.get("pcgen_entity_type_key")
                .and_then(Value::as_str).unwrap_or("?").to_string();
            let mut effects: Vec<Value> = e.effects.iter().map(effect_snapshot).collect();
            effects.sort_by(|a, b|
                a["kind"].as_str().cmp(&b["kind"].as_str())
                    .then(a["target"].as_str().cmp(&b["target"].as_str()))
                    .then(a["value"].as_str().cmp(&b["value"].as_str()))
            );
            after_index.entry((tk, e.name.clone())).or_default().push_back(effects);
        }

        for e in &catalog1.entities {
            let tk = e.attributes.get("pcgen_entity_type_key")
                .and_then(Value::as_str).unwrap_or("?");
            if !target_types.contains(&tk) { continue; }

            let mut before: Vec<Value> = e.effects.iter().map(effect_snapshot).collect();
            before.sort_by(|a, b|
                a["kind"].as_str().cmp(&b["kind"].as_str())
                    .then(a["target"].as_str().cmp(&b["target"].as_str()))
                    .then(a["value"].as_str().cmp(&b["value"].as_str()))
            );

            if let Some(queue) = after_index.get_mut(&(tk.to_string(), e.name.clone()))
                && let Some(after) = queue.pop_front()
            {
                if before != after {
                    *counts.entry(tk.to_string()).or_insert(0) += 1;
                    if examples_shown < max_examples {
                        examples_shown += 1;
                        let short = path.display().to_string();
                        let short = short.split("externals/").last().unwrap_or(&short);
                        println!("FILE: {}", short);
                        println!("  type={} name={:?}", tk, e.name);
                        let maxlen = before.len().max(after.len());
                        for i in 0..maxlen {
                            let b = before.get(i);
                            let a = after.get(i);
                            if b != a {
                                println!("  BEFORE[{}]: {:?}", i, b);
                                println!("  AFTER[{}]:  {:?}", i, a);
                                break;
                            }
                        }
                        println!("  Count: {} → {}", before.len(), after.len());
                        println!();
                    }
                }
            }
        }
    }

    println!("=== Effects changes by type ===");
    for (k, v) in &counts {
        println!("  {} | {}", v, k);
    }
    println!("  Total: {}", counts.values().sum::<usize>());
}
