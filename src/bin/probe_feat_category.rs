//! Probe: test feat CATEGORY roundtrip
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use std::fs;

fn main() {
    let path = "../../../../externals/BahamutDragon/pcgen/data/35e/wizards_of_the_coast/campaign_settings/forgotten_realms/lost_empires_of_faerun/le_feats.lst";
    let text = fs::read_to_string(path).expect("read file");
    
    // Count lines with CATEGORY:Feat
    let cat_feat_count = text.lines().filter(|l| !l.starts_with('#') && l.contains("CATEGORY:Feat")).count();
    println!("Lines with CATEGORY:Feat: {}", cat_feat_count);
    
    let fname = "le_feats.lst";
    let first = parse_text_to_catalog(&text, fname, "lst");
    let emitted = unparse_catalog_to_text(&first);
    let second = parse_text_to_catalog(&emitted, fname, "lst");
    
    println!("First parse entities: {}", first.entities.len());
    println!("Second parse entities: {}", second.entities.len());
    
    // Check category attribute  
    let mut type_key_by_id = std::collections::HashMap::new();
    for et in &first.entity_types {
        type_key_by_id.insert(et.id.0.to_string(), et.key.clone());
    }
    let mut type_key_by_id2 = std::collections::HashMap::new();
    for et in &second.entity_types {
        type_key_by_id2.insert(et.id.0.to_string(), et.key.clone());
    }
    
    // Check if any feat entity has category
    let feat_with_cat = first.entities.iter().filter(|e| {
        let tk = type_key_by_id.get(&e.entity_type.0.to_string()).map(|s| s.as_str()).unwrap_or("");
        tk == "pcgen:entity:feat" && e.attributes.get("category").is_some()
    }).count();
    println!("Feat entities with category (before): {}", feat_with_cat);
    
    // Show 2 examples
    for entity in first.entities.iter().filter(|e| {
        let tk = type_key_by_id.get(&e.entity_type.0.to_string()).map(|s| s.as_str()).unwrap_or("");
        tk == "pcgen:entity:feat" && e.attributes.get("category").is_some()
    }).take(2) {
        println!("  Feat '{}': category={:?}", entity.name, entity.attributes.get("category"));
    }
    
    // Check if emitted text has CATEGORY:Feat
    let emitted_cat_count = emitted.lines().filter(|l| l.contains("CATEGORY:Feat")).count();
    println!("Emitted lines with CATEGORY:Feat: {}", emitted_cat_count);
    
    // Check second parse
    let feat_with_cat2 = second.entities.iter().filter(|e| {
        let tk = type_key_by_id2.get(&e.entity_type.0.to_string()).map(|s| s.as_str()).unwrap_or("");
        tk == "pcgen:entity:feat" && e.attributes.get("category").is_some()
    }).count();
    println!("Feat entities with category (after): {}", feat_with_cat2);
}
