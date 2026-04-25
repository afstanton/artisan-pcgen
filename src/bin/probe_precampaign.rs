//! Temporary investigation tool for PRECAMPAIGN doubling.
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use std::fs;
use serde_json::Value;

fn main() {
    #[path = "probeutil/config.rs"] mod probe_config;
    let config = probe_config::ProbeConfig::load();
    let path = config.get("precampaign_pcc");
    let text = fs::read_to_string(path).expect("file not found");
    
    let first = parse_text_to_catalog(&text, "sggah.pcc", "pcc");
    
    // Find entities with pcgen_precampaign
    println!("=== Parse 1 ===");
    for entity in &first.entities {
        if let Some(v) = entity.attributes.get("pcgen_precampaign") {
            let type_key = entity.attributes.get("pcgen_entity_type_key").and_then(Value::as_str).unwrap_or("?");
            println!("  Entity: {} [{}] → {}", entity.name, type_key, v);
        }
    }
    
    let emitted = unparse_catalog_to_text(&first);
    
    println!("\n=== Emitted lines with PRECAMPAIGN ===");
    for line in emitted.lines() {
        if line.contains("PRECAMPAIGN") {
            println!("  {:?}", line);
        }
    }
    
    let second = parse_text_to_catalog(&emitted, "sggah.pcc", "pcc");
    
    println!("\n=== Parse 2 ===");
    for entity in &second.entities {
        if let Some(v) = entity.attributes.get("pcgen_precampaign") {
            let type_key = entity.attributes.get("pcgen_entity_type_key").and_then(Value::as_str).unwrap_or("?");
            println!("  Entity: {} [{}] → {}", entity.name, type_key, v);
        }
    }
}
