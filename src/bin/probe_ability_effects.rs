//! Quick probe: find ability entities with effects changes
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use std::fs;

fn main() {
    let paths = vec![
        "../../../../externals/PCGen/pcgen/data/3e/wizards_of_the_coast/srd/basics/srd_abilities_common.lst",
        "../../../../externals/PCGen/pcgen/data/3e/wizards_of_the_coast/srd/basics/srd_abilities_racial.lst",
        "../../../../externals/PCGen/pcgen/data/3e/wizards_of_the_coast/srd/basics/srd_ability_class.lst",
        "../../../../externals/PCGen/pcgen/data/3e/wizards_of_the_coast/srd/basics/srd_ability_misc.lst",
        "../../../../externals/PCGen/pcgen/data/3e/wizards_of_the_coast/srd/basics/srd_ability_monster.lst",
        "../../../../externals/PCGen/pcgen/data/3e/wizards_of_the_coast/rsrd/abilities/rsrd_abilities_classfeatures.lst",
        "../../../../externals/PCGen/pcgen/data/3e/wizards_of_the_coast/rsrd/abilities/rsrd_abilities_racial.lst",
    ];
    
    let mut total_changes = 0usize;
    let mut total_abilities = 0usize;
    
    for path in &paths {
        let Ok(text) = fs::read_to_string(path) else { continue; };
        let fname = std::path::Path::new(path).file_name().and_then(|f| f.to_str()).unwrap_or("?");
        let first = parse_text_to_catalog(&text, fname, "lst");
        let emitted = unparse_catalog_to_text(&first);
        let second = parse_text_to_catalog(&emitted, fname, "lst");
        
        let mut type_key_by_id = std::collections::HashMap::new();
        for et in &first.entity_types {
            type_key_by_id.insert(et.id.0.to_string(), et.key.clone());
        }
        let mut type_key_by_id2 = std::collections::HashMap::new();
        for et in &second.entity_types {
            type_key_by_id2.insert(et.id.0.to_string(), et.key.clone());
        }
        
        let after_idx: std::collections::HashMap<(String,String), _> = second.entities.iter()
            .map(|e| {
                let tk = type_key_by_id2.get(&e.entity_type.0.to_string()).cloned().unwrap_or_default();
                ((tk, e.name.clone()), e)
            })
            .collect();
        
        let mut count = 0;
        for entity in &first.entities {
            let tk = type_key_by_id.get(&entity.entity_type.0.to_string()).cloned().unwrap_or_default();
            if tk != "pcgen:entity:ability" { continue; }
            total_abilities += 1;
            let key = (tk.clone(), entity.name.clone());
            if let Some(after_e) = after_idx.get(&key) {
                let before_eff: Vec<_> = entity.effects.iter()
                    .map(|e| (e.kind.as_str(), e.target.as_str(), e.value.as_deref()))
                    .collect();
                let after_eff: Vec<_> = after_e.effects.iter()
                    .map(|e| (e.kind.as_str(), e.target.as_str(), e.value.as_deref()))
                    .collect();
                if before_eff != after_eff {
                    count += 1;
                    if count <= 2 {
                        println!("EFFECTS CHANGE [{fname}]: {}", entity.name);
                        for e in entity.effects.iter().take(3) {
                            println!("  BEFORE: kind={} target={:?} val={:?}", e.kind, e.target, e.value);
                        }
                        for e in after_e.effects.iter().take(3) {
                            println!("  AFTER:  kind={} target={:?} val={:?}", e.kind, e.target, e.value);
                        }
                    }
                }
            } else {
                // Disappeared
                if total_changes < 3 {
                    println!("DISAPPEARED [{fname}]: {} (was ability)", entity.name);
                }
                count += 1;
            }
        }
        if count > 0 {
            println!("[{}] {} ability entities with effects changes/disappearances", fname, count);
            total_changes += count;
        }
    }
    println!("\nTotal: {} ability effects changes out of {} abilities", total_changes, total_abilities);
}
