//! Probe: show what is causing semantic failures.
//! Also checks if 'completeness' field changes or entity_type/name changes.
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

const PROVENANCE: &[&str] = &[
    "head", "clauses", "line_number", "pcgen_line_number",
    "pcgen_record_family", "pcgen_record_style", "source_format", "pcgen_source_page",
];
const INFRASTRUCTURE: &[&str] = &[
    "pcgen_decl_token", "pcgen_decl_value", "pcgen_mechanical_signals",
    "pcgen_entity_type_key", "pcgen_name_open", "pcgen_name_pi", "pcgen_nameispi",
];

fn scan_dir(dir: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() { scan_dir(&p, paths); }
        else if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            if ext == "lst" || ext == "pcc" { paths.push(p); }
        }
    }
}

fn effect_json(e: &artisan_core::domain::rules::Effect) -> Value {
    json!({"kind": e.kind, "target": e.target, "value": e.value})
}

fn main() {
    #[path = "probeutil/config.rs"] mod probe_config;
    let config = probe_config::ProbeConfig::load();

    let mut all_files: Vec<PathBuf> = Vec::new();
    for root in config.scan_roots() { scan_dir(Path::new(root), &mut all_files); }

    let mut diff_type_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut total_semantic_fail = 0usize;
    let mut examples: Vec<String> = Vec::new();

    for path in &all_files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("lst");
        let Ok(text) = fs::read_to_string(path) else { continue };
        let sn = path.file_name().and_then(|f| f.to_str()).unwrap_or("file");

        let cat1 = parse_text_to_catalog(&text, sn, ext);
        let emitted = unparse_catalog_to_text(&cat1);
        let cat2 = parse_text_to_catalog(&emitted, sn, ext);

        let n1 = cat1.entities.len();
        let n2 = cat2.entities.len();

        if n1 != n2 {
            *diff_type_counts.entry(format!("ENTITY_COUNT_CHANGE({n1}→{n2})")).or_insert(0) += 1;
            total_semantic_fail += 1;
            continue;
        }

        // Build semantic snapshots using proper sort key (entity_type from type_key_by_id)
        let mut type_key_by_id1: BTreeMap<String, String> = BTreeMap::new();
        for et in &cat1.entity_types { type_key_by_id1.insert(et.id.0.to_string(), et.key.clone()); }
        let mut type_key_by_id2: BTreeMap<String, String> = BTreeMap::new();
        for et in &cat2.entity_types { type_key_by_id2.insert(et.id.0.to_string(), et.key.clone()); }

        // Build after index by (entity_type, name)
        let mut after_idx: BTreeMap<(String, String), VecDeque<(BTreeMap<String, Value>, Vec<Value>, Vec<Value>, String)>> = BTreeMap::new();
        for e in &cat2.entities {
            let tk = type_key_by_id2.get(&e.entity_type.0.to_string()).cloned().unwrap_or_default();
            let attrs: BTreeMap<String, Value> = e.attributes.iter()
                .filter(|(k, _)| !PROVENANCE.contains(&k.as_str()) && !INFRASTRUCTURE.contains(&k.as_str()))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let mut effs: Vec<Value> = e.effects.iter().map(effect_json).collect();
            effs.sort_by(|a, b| a["kind"].as_str().cmp(&b["kind"].as_str()).then(a["target"].as_str().cmp(&b["target"].as_str())));
            let mut prereqs: Vec<Value> = e.prerequisites.iter().map(|p| json!({"kind": p.kind, "expression": p.expression})).collect();
            prereqs.sort_by(|a, b| a["kind"].as_str().cmp(&b["kind"].as_str()).then(a["expression"].as_str().cmp(&b["expression"].as_str())));
            let completeness = format!("{:?}", e.completeness);
            after_idx.entry((tk, e.name.clone())).or_default().push_back((attrs, effs, prereqs, completeness));
        }

        let mut file_diffs: BTreeSet<String> = BTreeSet::new();
        for e in &cat1.entities {
            let tk = type_key_by_id1.get(&e.entity_type.0.to_string()).cloned().unwrap_or_default();
            let before_attrs: BTreeMap<String, Value> = e.attributes.iter()
                .filter(|(k, _)| !PROVENANCE.contains(&k.as_str()) && !INFRASTRUCTURE.contains(&k.as_str()))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let mut before_effs: Vec<Value> = e.effects.iter().map(effect_json).collect();
            before_effs.sort_by(|a, b| a["kind"].as_str().cmp(&b["kind"].as_str()).then(a["target"].as_str().cmp(&b["target"].as_str())));
            let mut before_prereqs: Vec<Value> = e.prerequisites.iter().map(|p| json!({"kind": p.kind, "expression": p.expression})).collect();
            before_prereqs.sort_by(|a, b| a["kind"].as_str().cmp(&b["kind"].as_str()).then(a["expression"].as_str().cmp(&b["expression"].as_str())));
            let before_completeness = format!("{:?}", e.completeness);

            let Some(queue) = after_idx.get_mut(&(tk.clone(), e.name.clone())) else {
                file_diffs.insert("ENTITY_TYPE_OR_NAME_CHANGE".to_string());
                continue;
            };
            let Some((after_attrs, after_effs, after_prereqs, after_completeness)) = queue.pop_front() else { continue };

            if before_attrs != after_attrs {
                let all_keys: BTreeSet<&str> = before_attrs.keys().chain(after_attrs.keys()).map(|k| k.as_str()).collect();
                for key in all_keys {
                    let bv = before_attrs.get(key);
                    let av = after_attrs.get(key);
                    if bv != av {
                        let label = match (bv, av) {
                            (None, _) => format!("ATTR_ADDED:{}", key),
                            (_, None) => format!("ATTR_DROPPED:{}", key),
                            _ => format!("ATTR_CHANGED:{}", key),
                        };
                        file_diffs.insert(label);
                    }
                }
            }
            if before_effs != after_effs { file_diffs.insert("EFFECTS_CHANGED".to_string()); }
            if before_prereqs != after_prereqs { file_diffs.insert("PREREQS_CHANGED".to_string()); }
            if before_completeness != after_completeness { file_diffs.insert(format!("COMPLETENESS_CHANGED")); }
        }

        if !file_diffs.is_empty() {
            total_semantic_fail += 1;
            for diff in &file_diffs {
                *diff_type_counts.entry(diff.clone()).or_insert(0) += 1;
            }
            if examples.len() < 8 {
                let short = path.display().to_string();
                let short = short.split("externals/").last().unwrap_or(&short);
                examples.push(format!("{}: {:?}", short, file_diffs.iter().take(3).collect::<Vec<_>>()));
            }
        }
    }

    println!("Semantic failures (by this probe): {}", total_semantic_fail);
    println!();
    println!("=== Diff types (files each appears in) ===");
    let mut sorted: Vec<_> = diff_type_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (label, count) in sorted.iter().take(40) {
        println!("  {:5} | {}", count, label);
    }
    println!();
    println!("=== Sample failing files ===");
    for ex in &examples {
        println!("  {}", ex);
    }
}
