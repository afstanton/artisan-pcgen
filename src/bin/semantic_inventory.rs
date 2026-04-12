use artisan_pcgen::{ParsedCatalog, parse_text_to_catalog, unparse_catalog_to_text};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const PROVENANCE_ATTRS: &[&str] = &[
    "head",
    "clauses",
    "line_number",
    "pcgen_line_number",
    "pcgen_record_family",
    "pcgen_record_style",
    "source_format",
];

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

#[derive(Debug, Clone)]
struct FileReport {
    path: PathBuf,
    lines: usize,
    entities_before: usize,
    entities_after: usize,
    semantic_ok: bool,
    canonical_ok: bool,
    semantic_diff: Option<String>,
    canonical_diff: Option<String>,
}

#[derive(Default)]
struct CoverageTally {
    total_entities: usize,
    covered_effects_only: usize,
    covered_prereqs_only: usize,
    covered_both: usize,
    covered_canonical_attr_only: usize,
    truly_sparse: usize,
    sparse_pcgen_attr_counts: HashMap<String, usize>,
    canonical_attr_counts: HashMap<String, usize>,
    effect_kind_counts: HashMap<String, usize>,
    prereq_kind_counts: HashMap<String, usize>,
}

#[derive(Default)]
struct Progress {
    files_seen: usize,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut per_file = false;
    let mut failed_files = false;
    let mut roots = Vec::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--per-file" => per_file = true,
            "--failed-files" => failed_files = true,
            _ => roots.push(arg.clone()),
        }
    }

    if roots.is_empty() {
        eprintln!(
            "Usage: semantic_inventory [--per-file] [--failed-files] <pcgen_root1> [pcgen_root2] ..."
        );
        eprintln!(
            "Example: semantic_inventory --failed-files /path/to/PCGen/pcgen /path/to/BahamutDragon/pcgen"
        );
        std::process::exit(1);
    }

    let mut files = Vec::new();
    let mut total_lines = 0usize;
    let mut coverage = CoverageTally::default();
    let mut progress = Progress::default();

    eprintln!("semantic_inventory: scanning {} root(s)", roots.len());
    for root_arg in &roots {
        let root = Path::new(root_arg);
        for subdir in ["data", "system", "characters"] {
            let path = root.join(subdir);
            if path.exists() && path.is_dir() {
                eprintln!("semantic_inventory: scanning {}", path.display());
                scan_directory(
                    &path,
                    &mut files,
                    &mut total_lines,
                    &mut coverage,
                    &mut progress,
                )?;
            }
        }
    }

    let file_count = files.len();
    let semantic_pass = files.iter().filter(|r| r.semantic_ok).count();
    let canonical_pass = files.iter().filter(|r| r.canonical_ok).count();

    let mut report = String::new();
    writeln!(report, "=== Semantic Inventory Summary ===").unwrap();
    writeln!(report, "Files scanned: {}", file_count).unwrap();
    writeln!(report, "Total lines processed: {}", total_lines).unwrap();
    writeln!(
        report,
        "Semantic roundtrip: {} passed / {} failed",
        semantic_pass,
        file_count.saturating_sub(semantic_pass)
    )
    .unwrap();
    writeln!(
        report,
        "Canonical roundtrip: {} passed / {} failed",
        canonical_pass,
        file_count.saturating_sub(canonical_pass)
    )
    .unwrap();
    writeln!(report).unwrap();

    let covered = coverage
        .total_entities
        .saturating_sub(coverage.truly_sparse);
    let pct = if coverage.total_entities > 0 {
        100 * covered / coverage.total_entities
    } else {
        0
    };

    writeln!(report, "=== Canonical Model Coverage Report ===").unwrap();
    writeln!(
        report,
        "Total entities:                    {}",
        coverage.total_entities
    )
    .unwrap();
    writeln!(
        report,
        "  effects + prerequisites:         {}",
        coverage.covered_both
    )
    .unwrap();
    writeln!(
        report,
        "  effects only:                    {}",
        coverage.covered_effects_only
    )
    .unwrap();
    writeln!(
        report,
        "  prerequisites only:              {}",
        coverage.covered_prereqs_only
    )
    .unwrap();
    writeln!(
        report,
        "  canonical attrs / citations:     {}",
        coverage.covered_canonical_attr_only
    )
    .unwrap();
    writeln!(
        report,
        "  truly sparse (pcgen_* only):     {}",
        coverage.truly_sparse
    )
    .unwrap();
    writeln!(
        report,
        "  canonical coverage:              {}% have >=1 canonical field",
        pct
    )
    .unwrap();
    writeln!(report).unwrap();

    write_sorted_counts(
        &mut report,
        "=== Effect Kinds Across All Entities ===",
        &coverage.effect_kind_counts,
        50,
        false,
    );
    write_sorted_counts(
        &mut report,
        "=== Prerequisite Kinds Across All Entities ===",
        &coverage.prereq_kind_counts,
        50,
        false,
    );
    write_sorted_counts(
        &mut report,
        "=== Canonical Attribute Keys On Covered Entities ===",
        &coverage.canonical_attr_counts,
        50,
        false,
    );
    write_sorted_counts(
        &mut report,
        "=== pcgen_* Attributes On Truly Sparse Entities ===",
        &coverage.sparse_pcgen_attr_counts,
        75,
        true,
    );

    if per_file {
        writeln!(report, "=== File Roundtrip Results ===").unwrap();
        for file in &files {
            writeln!(
                report,
                "[{}|{}] {} (lines={}, entities {} -> {})",
                if file.semantic_ok {
                    "semantic:ok"
                } else {
                    "semantic:fail"
                },
                if file.canonical_ok {
                    "canonical:ok"
                } else {
                    "canonical:fail"
                },
                file.path.display(),
                file.lines,
                file.entities_before,
                file.entities_after
            )
            .unwrap();

            if let Some(diff) = &file.semantic_diff {
                writeln!(report, "  semantic diff: {}", diff).unwrap();
            }
            if let Some(diff) = &file.canonical_diff {
                writeln!(report, "  canonical diff: {}", diff).unwrap();
            }
        }
    } else if failed_files {
        writeln!(report, "=== Failed Files ===").unwrap();
        let mut any_failed = false;
        for file in &files {
            if file.semantic_ok && file.canonical_ok {
                continue;
            }
            any_failed = true;
            writeln!(
                report,
                "[{}|{}] {}",
                if file.semantic_ok {
                    "semantic:ok"
                } else {
                    "semantic:fail"
                },
                if file.canonical_ok {
                    "canonical:ok"
                } else {
                    "canonical:fail"
                },
                file.path.display(),
            )
            .unwrap();
            if let Some(diff) = &file.semantic_diff {
                writeln!(report, "  semantic diff: {}", diff).unwrap();
            }
            if let Some(diff) = &file.canonical_diff {
                writeln!(report, "  canonical diff: {}", diff).unwrap();
            }
        }
        if !any_failed {
            writeln!(report, "none").unwrap();
        }
    }

    let output_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("SEMANTIC_INVENTORY.txt");
    fs::write(&output_path, &report)?;
    eprintln!(
        "semantic_inventory: wrote {} (semantic pass={}, canonical pass={}, entities={})",
        output_path.display(),
        semantic_pass,
        canonical_pass,
        coverage.total_entities
    );

    Ok(())
}

fn scan_directory(
    path: &Path,
    files: &mut Vec<FileReport>,
    total_lines: &mut usize,
    coverage: &mut CoverageTally,
    progress: &mut Progress,
) -> io::Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child = entry.path();
        if child.is_dir() {
            scan_directory(&child, files, total_lines, coverage, progress)?;
            continue;
        }

        let ext = child
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_default();
        if ext != "lst" && ext != "pcc" && ext != "pcg" {
            continue;
        }

        let bytes = fs::read(&child)?;
        let text = String::from_utf8_lossy(&bytes).to_string();
        *total_lines += text.lines().count();
        progress.files_seen += 1;
        if progress.files_seen == 1 || progress.files_seen.is_multiple_of(100) {
            eprintln!(
                "semantic_inventory: processed {} file(s); current={}",
                progress.files_seen,
                child.display()
            );
        }

        let source_name = child
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("fixture");
        let first = parse_text_to_catalog(&text, source_name, &ext);
        tally_coverage(&first, coverage);

        let generated = unparse_catalog_to_text(&first);
        let second = parse_text_to_catalog(&generated, source_name, &ext);

        let semantic_before = semantic_snapshot(&first);
        let semantic_after = semantic_snapshot(&second);
        let core_before = core_snapshot(&first);
        let core_after = core_snapshot(&second);

        let semantic_ok = semantic_before == semantic_after;
        let canonical_ok = core_before == core_after;

        let semantic_diff = if semantic_ok {
            None
        } else {
            Some(diff_value_summary(&semantic_before, &semantic_after))
        };
        let canonical_diff = if canonical_ok {
            None
        } else {
            Some(diff_core_snapshots(&core_before, &core_after))
        };

        files.push(FileReport {
            path: child,
            lines: text.lines().count(),
            entities_before: first.entities.len(),
            entities_after: second.entities.len(),
            semantic_ok,
            canonical_ok,
            semantic_diff,
            canonical_diff,
        });
    }

    Ok(())
}

fn semantic_snapshot(catalog: &ParsedCatalog) -> Value {
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

        let mut semantic_attrs: BTreeMap<String, Value> = entity
            .attributes
            .iter()
            .filter(|(k, _)| !PROVENANCE_ATTRS.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

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

fn core_snapshot(catalog: &ParsedCatalog) -> Value {
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

    entities.sort_by(|a, b| {
        a["entity_type"]
            .as_str()
            .cmp(&b["entity_type"].as_str())
            .then_with(|| a["name"].as_str().cmp(&b["name"].as_str()))
    });

    json!({ "entities": entities })
}

fn diff_core_snapshots(before: &Value, after: &Value) -> String {
    let empty = vec![];
    let before_entities = before["entities"].as_array().unwrap_or(&empty);
    let after_entities = after["entities"].as_array().unwrap_or(&empty);

    if before_entities.len() != after_entities.len() {
        return format!(
            "entity count changed: {} -> {}",
            before_entities.len(),
            after_entities.len()
        );
    }

    for (b, a) in before_entities.iter().zip(after_entities.iter()) {
        let name = b["name"].as_str().unwrap_or("?");
        let etype = b["entity_type"].as_str().unwrap_or("?");
        if b["name"] != a["name"] || b["entity_type"] != a["entity_type"] {
            return format!(
                "entity identity changed: ({etype}, {name}) -> ({}, {})",
                a["entity_type"].as_str().unwrap_or("?"),
                a["name"].as_str().unwrap_or("?"),
            );
        }
        if b["effects"] != a["effects"] {
            return format!(
                "effects mismatch for entity ({etype}, {name}): before={} after={}",
                b["effects"], a["effects"]
            );
        }
        if b["prerequisites"] != a["prerequisites"] {
            return format!(
                "prerequisites mismatch for entity ({etype}, {name}): before={} after={}",
                b["prerequisites"], a["prerequisites"]
            );
        }
    }

    String::new()
}

fn diff_value_summary(before: &Value, after: &Value) -> String {
    let before_entities = before["entities"].as_array().map(Vec::len).unwrap_or(0);
    let after_entities = after["entities"].as_array().map(Vec::len).unwrap_or(0);
    if before_entities != after_entities {
        return format!(
            "semantic entity count changed: {} -> {}",
            before_entities, after_entities
        );
    }
    "semantic snapshot differs".to_string()
}

fn tally_coverage(catalog: &ParsedCatalog, coverage: &mut CoverageTally) {
    for entity in &catalog.entities {
        coverage.total_entities += 1;
        let has_effects = !entity.effects.is_empty();
        let has_prereqs = !entity.prerequisites.is_empty();
        let has_canonical_attrs = entity.attributes.keys().any(|k| {
            !k.starts_with("pcgen_")
                && !PROVENANCE_ATTRS.contains(&k.as_str())
                && !INFRASTRUCTURE_ATTRS.contains(&k.as_str())
        });
        let has_citations = !entity.citations.is_empty();
        let is_covered = has_effects || has_prereqs || has_canonical_attrs || has_citations;

        match (has_effects, has_prereqs) {
            (true, true) => coverage.covered_both += 1,
            (true, false) => coverage.covered_effects_only += 1,
            (false, true) => coverage.covered_prereqs_only += 1,
            (false, false) if is_covered => coverage.covered_canonical_attr_only += 1,
            _ => {
                coverage.truly_sparse += 1;
                for key in entity.attributes.keys() {
                    if key.starts_with("pcgen_")
                        && !PROVENANCE_ATTRS.contains(&key.as_str())
                        && !INFRASTRUCTURE_ATTRS.contains(&key.as_str())
                    {
                        *coverage
                            .sparse_pcgen_attr_counts
                            .entry(key.clone())
                            .or_insert(0) += 1;
                    }
                }
            }
        }

        if is_covered {
            for key in entity.attributes.keys() {
                if !key.starts_with("pcgen_")
                    && !PROVENANCE_ATTRS.contains(&key.as_str())
                    && !INFRASTRUCTURE_ATTRS.contains(&key.as_str())
                {
                    *coverage
                        .canonical_attr_counts
                        .entry(key.clone())
                        .or_insert(0) += 1;
                }
            }
        }

        for e in &entity.effects {
            *coverage
                .effect_kind_counts
                .entry(e.kind.clone())
                .or_insert(0) += 1;
        }
        for p in &entity.prerequisites {
            *coverage
                .prereq_kind_counts
                .entry(p.kind.clone())
                .or_insert(0) += 1;
        }
    }
}

fn write_sorted_counts(
    report: &mut String,
    heading: &str,
    counts: &HashMap<String, usize>,
    limit: usize,
    trim_pcgen_prefix: bool,
) {
    let mut items: Vec<_> = counts.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    writeln!(report, "{heading}").unwrap();
    for (key, count) in items.into_iter().take(limit) {
        let display = if trim_pcgen_prefix {
            key.strip_prefix("pcgen_").unwrap_or(key)
        } else {
            key.as_str()
        };
        writeln!(report, "{:6} | {}", count, display).unwrap();
    }
    writeln!(report).unwrap();
}
