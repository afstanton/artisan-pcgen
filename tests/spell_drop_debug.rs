/// Diagnostic test for understanding which attributes drop for pcgen:entity:spell
/// during semantic roundtrip.
///
/// Run with: cargo test spell_drop_debug -- --nocapture
use artisan_core::Entity;
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;

fn pcgen_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../externals/PCGen/pcgen")
}

fn srd_spell_file() -> PathBuf {
    pcgen_root().join("data/3e/wizards_of_the_coast/srd/basics/srd_spells.lst")
}

const PROVENANCE_ATTRS: &[&str] = &[
    "head",
    "clauses",
    "line_number",
    "pcgen_line_number",
    "pcgen_record_family",
    "pcgen_record_style",
    "source_format",
    "pcgen_source_page",
];

const INFRA_ATTRS: &[&str] = &[
    "pcgen_decl_token",
    "pcgen_decl_value",
    "pcgen_mechanical_signals",
    "pcgen_entity_type_key",
];

fn type_key_map(catalog: &artisan_pcgen::ParsedCatalog) -> BTreeMap<String, String> {
    catalog
        .entity_types
        .iter()
        .map(|et| (et.id.0.to_string(), et.key.clone()))
        .collect()
}

fn spell_entities(
    catalog: &artisan_pcgen::ParsedCatalog,
    type_keys: &BTreeMap<String, String>,
) -> Vec<(String, String, Vec<(String, serde_json::Value)>)> {
    catalog
        .entities
        .iter()
        .filter_map(|entity| {
            let etype = type_keys
                .get(&entity.entity_type.0.to_string())
                .cloned()
                .unwrap_or_default();
            if etype != "pcgen:entity:spell" {
                return None;
            }
            let attrs: Vec<_> = entity
                .attributes
                .iter()
                .filter(|(k, _)| {
                    !PROVENANCE_ATTRS.contains(&k.as_str()) && !INFRA_ATTRS.contains(&k.as_str())
                })
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            Some((etype, entity.name.clone(), attrs))
        })
        .collect()
}

fn analyze_file(
    path: &std::path::Path,
    drops: &mut HashMap<String, usize>,
    changes: &mut HashMap<String, usize>,
    disappear_types: &mut HashMap<String, usize>,
    total_spells: &mut usize,
    example_drops: &mut Vec<String>,
    example_changes: &mut Vec<String>,
) {
    let filename = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unknown");
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    let first = parse_text_to_catalog(&text, filename, "lst");
    let emitted = unparse_catalog_to_text(&first);
    let second = parse_text_to_catalog(&emitted, filename, "lst");

    let keys1 = type_key_map(&first);
    let keys2 = type_key_map(&second);

    // Build after-index by name (all entity types)
    let mut after_index: HashMap<(String, String), Vec<(String, serde_json::Value)>> =
        HashMap::new();
    for entity in &second.entities {
        let etype = keys2
            .get(&entity.entity_type.0.to_string())
            .cloned()
            .unwrap_or_default();
        let attrs: Vec<_> = entity
            .attributes
            .iter()
            .filter(|(k, _)| {
                !PROVENANCE_ATTRS.contains(&k.as_str()) && !INFRA_ATTRS.contains(&k.as_str())
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        after_index.insert((etype, entity.name.clone()), attrs);
    }

    for entity in &first.entities {
        let etype = keys1
            .get(&entity.entity_type.0.to_string())
            .cloned()
            .unwrap_or_default();
        if etype != "pcgen:entity:spell" {
            continue;
        }
        *total_spells += 1;

        match after_index.get(&(etype.clone(), entity.name.clone())) {
            None => {
                // Disappeared — check if it changed type
                let found_as = after_index
                    .keys()
                    .find(|(_, name)| name == &entity.name)
                    .map(|(t, _)| t.clone())
                    .unwrap_or_else(|| "gone".to_string());
                *disappear_types.entry(found_as).or_insert(0) += 1;
                for (k, _) in entity.attributes.iter() {
                    if !PROVENANCE_ATTRS.contains(&k.as_str()) && !INFRA_ATTRS.contains(&k.as_str()) {
                        *drops.entry(k.clone()).or_insert(0) += 1;
                    }
                }
            }
            Some(after_attrs) => {
                let after_map: HashMap<String, serde_json::Value> =
                    after_attrs.iter().cloned().collect();
                for (k, bval) in entity.attributes.iter() {
                    if PROVENANCE_ATTRS.contains(&k.as_str()) || INFRA_ATTRS.contains(&k.as_str()) {
                        continue;
                    }
                    match after_map.get(k) {
                        None => {
                            *drops.entry(k.clone()).or_insert(0) += 1;
                            if example_drops.len() < 10 {
                                example_drops.push(format!(
                                    "DROPPED  file={:40} spell={:30} attr={:25} was={}",
                                    filename, entity.name, k, bval
                                ));
                            }
                        }
                        Some(aval) if aval != bval => {
                            *changes.entry(k.clone()).or_insert(0) += 1;
                            if example_changes.len() < 10 {
                                example_changes.push(format!(
                                    "CHANGED  file={:40} spell={:30} attr={:25} was={} now={}",
                                    filename, entity.name, k, bval, aval
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[test]
fn spell_drops_all_files() {
    use std::process::Command;

    // Find all spell LST files (have SCHOOL: token)
    let pcgen_roots = [
        PathBuf::from("REDACTED"),
        PathBuf::from(
            "REDACTED",
        ),
    ];

    let mut all_spell_files: Vec<PathBuf> = Vec::new();
    for root in &pcgen_roots {
        if root.exists() {
            collect_lst_files(root, &mut all_spell_files);
        }
    }

    let mut drops: HashMap<String, usize> = HashMap::new();
    let mut changes: HashMap<String, usize> = HashMap::new();
    let mut disappear_types: HashMap<String, usize> = HashMap::new();
    let mut example_drops: Vec<String> = Vec::new();
    let mut example_changes: Vec<String> = Vec::new();
    let mut total_spells = 0usize;
    let mut files_with_spells = 0usize;

    for path in &all_spell_files {
        let before = total_spells;
        analyze_file(
            path,
            &mut drops,
            &mut changes,
            &mut disappear_types,
            &mut total_spells,
            &mut example_drops,
            &mut example_changes,
        );
        if total_spells > before {
            files_with_spells += 1;
        }
    }

    println!("\n=== All-Files Spell Roundtrip Analysis ===");
    println!("Files scanned: {}", all_spell_files.len());
    println!("Files with spell entities: {}", files_with_spells);
    println!("Total spell entities: {}", total_spells);
    println!("Disappeared spell → other type: {:?}", disappear_types);

    let mut drops_sorted: Vec<_> = drops.iter().collect();
    drops_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("\nDropped attributes (spell entities, all files):");
    for (k, v) in &drops_sorted {
        println!("  {:6} | {}", v, k);
    }

    let mut changes_sorted: Vec<_> = changes.iter().collect();
    changes_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("\nChanged attributes (spell entities, all files):");
    for (k, v) in &changes_sorted {
        println!("  {:6} | {}", v, k);
    }

    println!("\nExample drops (first 10):");
    for d in &example_drops {
        println!("  {}", d);
    }
    println!("\nExample changes (first 10):");
    for d in &example_changes {
        println!("  {}", d);
    }
}

fn collect_lst_files(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_lst_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("lst") {
            out.push(path);
        }
    }
}
