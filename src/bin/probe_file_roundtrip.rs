//! Roundtrip probe: investigate attribute drops in a single file.
//!
//! Usage:
//!   probe_file_roundtrip <path/to/file.lst> [attr_to_check]
//!
//! If attr_to_check is given (e.g. "abilities"), prints all entities where that
//! attribute is present before roundtrip but missing after. Also prints a summary
//! of all attributes that changed.
//!
//! If attr_to_check is omitted, prints a full summary of all drops/changes.
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use std::collections::BTreeMap;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: probe_file_roundtrip <path/to/file.lst> [attr_to_check]");
        std::process::exit(1);
    }

    let path = &args[1];
    let attr_filter = args.get(2).map(String::as_str);

    let text = fs::read_to_string(path).expect("file not found");
    let fname = path.split('/').last().unwrap_or(path.as_str());

    let first = parse_text_to_catalog(&text, fname, "lst");
    let emitted = unparse_catalog_to_text(&first);
    let second = parse_text_to_catalog(&emitted, fname, "lst");

    println!("=== Probing: {path} ===");
    println!(
        "Entities: {} before, {} after",
        first.entities.len(),
        second.entities.len()
    );

    let skip: &[&str] = &[
        "head",
        "clauses",
        "line_number",
        "pcgen_line_number",
        "pcgen_record_family",
        "pcgen_record_style",
        "source_format",
        "pcgen_entity_type_key",
        "pcgen_decl_token",
        "pcgen_decl_value",
        "pcgen_mechanical_signals",
        "pcgen_name_open",
        "pcgen_name_pi",
        "pcgen_nameispi",
    ];

    // Build after index: (type_key, name) -> ordered list
    use std::collections::VecDeque;
    let mut after_index: BTreeMap<(String, String), VecDeque<usize>> = BTreeMap::new();
    for (i, e) in second.entities.iter().enumerate() {
        let type_key = e
            .attributes
            .get("pcgen_entity_type_key")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        after_index
            .entry((type_key, e.name.clone()))
            .or_default()
            .push_back(i);
    }

    let mut drop_tally: BTreeMap<String, usize> = BTreeMap::new();
    let mut change_tally: BTreeMap<String, usize> = BTreeMap::new();
    let mut drop_examples: Vec<(String, String, String)> = Vec::new(); // (name, attr, before_val)
    let mut change_examples: Vec<(String, String, String, String)> = Vec::new(); // (name, attr, before, after)

    for before_entity in &first.entities {
        let type_key = before_entity
            .attributes
            .get("pcgen_entity_type_key")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let key = (type_key, before_entity.name.clone());
        let after_idx = after_index.get_mut(&key).and_then(|q| q.pop_front());

        let after_entity = after_idx.map(|i| &second.entities[i]);

        for (attr, before_val) in &before_entity.attributes {
            if skip.contains(&attr.as_str()) {
                continue;
            }
            if let Some(filter) = attr_filter {
                if attr != filter {
                    continue;
                }
            }

            match after_entity.and_then(|e| e.attributes.get(attr)) {
                None => {
                    *drop_tally.entry(attr.clone()).or_insert(0) += 1;
                    if drop_examples.len() < 5 || attr_filter.is_some() {
                        let preview: String = before_val.to_string().chars().take(100).collect();
                        drop_examples.push((before_entity.name.clone(), attr.clone(), preview));
                    }
                }
                Some(after_val) if after_val != before_val => {
                    *change_tally.entry(attr.clone()).or_insert(0) += 1;
                    if change_examples.len() < 5 || attr_filter.is_some() {
                        let b: String = before_val.to_string().chars().take(80).collect();
                        let a: String = after_val.to_string().chars().take(80).collect();
                        change_examples.push((before_entity.name.clone(), attr.clone(), b, a));
                    }
                }
                _ => {}
            }
        }
    }

    if drop_tally.is_empty() && change_tally.is_empty() {
        println!("\n✓ No attribute drops or changes detected.");
    }

    if !drop_tally.is_empty() {
        println!("\n--- Dropped attributes ---");
        let mut sorted: Vec<_> = drop_tally.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (attr, count) in &sorted {
            println!("  {count:5} | {attr}");
        }
        println!("\n--- Drop examples ---");
        for (name, attr, val) in &drop_examples {
            println!("  entity: {name}");
            println!("  attr:   {attr} = {val}");
            for line in emitted.lines() {
                if line.starts_with(name.as_str()) {
                    let preview: String = line.chars().take(200).collect();
                    println!("  emitted: {preview}");
                    break;
                }
            }
            println!();
        }
    }

    if !change_tally.is_empty() {
        println!("\n--- Changed attribute values ---");
        let mut sorted: Vec<_> = change_tally.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (attr, count) in &sorted {
            println!("  {count:5} | {attr}");
        }
        println!("\n--- Change examples ---");
        for (name, attr, before, after) in &change_examples {
            println!("  entity: {name}");
            println!("  attr:   {attr}");
            println!("  before: {before}");
            println!("  after:  {after}");
            println!();
        }
    }
}
