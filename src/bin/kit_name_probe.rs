fn main() {
    let path = std::env::args().nth(1)
        .expect("Usage: kit_name_probe <file>");
    let ext = path.rsplit('.').next().unwrap_or("lst");
    let text = std::fs::read_to_string(&path).unwrap();
    let fname = path.split('/').last().unwrap_or(&path);
    let cat1 = artisan_pcgen::parse_text_to_catalog(&text, fname, ext);
    let emitted = artisan_pcgen::unparse_catalog_to_text(&cat1);
    let cat2 = artisan_pcgen::parse_text_to_catalog(&emitted, fname, ext);

    let before: Vec<_> = cat1.entities.iter().map(|e| {
        let tk = e.attributes.get("pcgen_entity_type_key").and_then(|v| v.as_str()).unwrap_or("?").to_string();
        (tk, e.name.clone())
    }).collect();
    let after: Vec<_> = cat2.entities.iter().map(|e| {
        let tk = e.attributes.get("pcgen_entity_type_key").and_then(|v| v.as_str()).unwrap_or("?").to_string();
        (tk, e.name.clone())
    }).collect();

    println!("=== Name/type changes ===");
    let mut found_change = false;
    for (i, (bt, bn)) in before.iter().enumerate() {
        let found = after.iter().any(|(at, an)| at == bt && an == bn);
        if !found {
            println!("MISSING AFTER[{}]: type={} name={:?}", i, bt, bn);
            found_change = true;
        }
    }
    for (i, (at, an)) in after.iter().enumerate() {
        let found = before.iter().any(|(bt, bn)| at == bt && an == bn);
        if !found {
            println!("NEW IN AFTER[{}]: type={} name={:?}", i, at, an);
            found_change = true;
        }
    }
    if !found_change {
        println!("✓ No entity name/type changes.");
    }

    // Print emitted lines
    println!("\n=== Emitted lines ===");
    for (i, line) in emitted.lines().enumerate() {
        println!("[{}]: {:?}", i+1, line);
    }

    // Print entity 0 full attributes
    println!("\n=== Entity 0 (before) attributes ===");
    let e = &cat1.entities[0];
    println!("name: {:?}", e.name);
    for (k, v) in &e.attributes {
        println!("  {}: {:?}", k, v);
    }
}
