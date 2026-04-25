/// Diagnostic probe for the skills entity count drop.
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use std::fs;
use std::collections::BTreeMap;
use serde_json::Value;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "/Users/afstanton/code/afstanton/artisan/externals/PCGen/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/ce_skills.lst".to_string());
    let text = fs::read_to_string(&path).unwrap();
    let fname = path.split('/').last().unwrap_or(&path);
    let cat1 = parse_text_to_catalog(&text, fname, "lst");
    let emitted = unparse_catalog_to_text(&cat1);
    let cat2 = parse_text_to_catalog(&emitted, fname, "lst");

    println!("cat1 entities: {}", cat1.entities.len());
    println!("emitted lines: {}", emitted.lines().filter(|l| !l.trim().is_empty()).count());
    println!("cat2 entities: {}", cat2.entities.len());

    println!("\n=== cat1 entity type distribution ===");
    let mut types1: BTreeMap<String, usize> = BTreeMap::new();
    for e in &cat1.entities {
        let tk = e.attributes.get("pcgen_entity_type_key").and_then(Value::as_str).unwrap_or("?").to_string();
        *types1.entry(tk).or_insert(0) += 1;
    }
    for (k, v) in &types1 { println!("  {:3} | {}", v, k); }

    println!("\n=== cat2 entity type distribution ===");
    let mut types2: BTreeMap<String, usize> = BTreeMap::new();
    for e in &cat2.entities {
        let tk = e.attributes.get("pcgen_entity_type_key").and_then(Value::as_str).unwrap_or("?").to_string();
        *types2.entry(tk).or_insert(0) += 1;
    }
    for (k, v) in &types2 { println!("  {:3} | {}", v, k); }

    // Show some emitted lines from the end (to see if trailing lines are missing)
    let emitted_lines: Vec<&str> = emitted.lines().collect();
    let n = emitted_lines.len();
    println!("\n=== Last 10 emitted lines ===");
    for line in emitted_lines.iter().rev().take(10).rev() {
        println!("  {:?}", &line[..line.len().min(120)]);
    }
    println!("Total emitted lines: {}", n);

    // Show first 30 emitted lines
    println!("\n=== First 30 emitted lines ===");
    for line in emitted_lines.iter().take(30) {
        println!("  {:?}", &line[..line.len().min(120)]);
    }

    // Find cat1 skill names that are NOT in cat2
    let cat2_names: std::collections::BTreeSet<String> = cat2.entities.iter()
        .filter_map(|e| e.attributes.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .collect();
    let missing: Vec<String> = cat1.entities.iter()
        .filter_map(|e| e.attributes.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .filter(|n| !cat2_names.contains(n))
        .collect();
    println!("\n=== First 20 cat1 entities missing from cat2 ===");
    for n in missing.iter().take(20) { println!("  {:?}", n); }
    println!("  (total missing: {})", missing.len());
}
