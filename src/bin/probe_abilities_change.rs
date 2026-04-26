/// Probe: find entities where `abilities` attribute changes across roundtrip.
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn scan(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() { scan(&p, out); }
        else if matches!(p.extension().and_then(|e| e.to_str()), Some("lst"|"pcc"|"pcg")) {
            out.push(p);
        }
    }
}

fn main() {
    #[path = "probeutil/config.rs"] mod probe_config;
    let config = probe_config::ProbeConfig::load();
    let mut paths = Vec::new();
    for root in config.scan_roots() { scan(Path::new(root), &mut paths); }

    let mut total = 0u32;
    for path in &paths {
        let Ok(text) = fs::read_to_string(path) else { continue };
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("lst");
        let fname = path.file_name().and_then(|f| f.to_str()).unwrap_or("?");
        let cat1 = parse_text_to_catalog(&text, fname, ext);
        let emitted = unparse_catalog_to_text(&cat1);
        let cat2 = parse_text_to_catalog(&emitted, fname, ext);

        for (e1, e2) in cat1.entities.iter().zip(cat2.entities.iter()) {
            let ab1 = e1.attributes.get("abilities");
            let ab2 = e2.attributes.get("abilities");
            if ab1 != ab2 {
                total += 1;
                if total <= 5 {
                    let short = path.display().to_string();
                    let short = short.split("externals/").last().unwrap_or(&short);
                    println!("FILE: {short}");
                    println!("  entity: {} ({})", e1.name,
                        e1.attributes.get("pcgen_entity_type_key").and_then(Value::as_str).unwrap_or("?"));
                    println!("  BEFORE: {}", serde_json::to_string(ab1.unwrap_or(&Value::Null)).unwrap());
                    println!("  AFTER:  {}", serde_json::to_string(ab2.unwrap_or(&Value::Null)).unwrap());
                }
            }
        }
    }
    println!("\nTotal entities with abilities changed: {total}");
}
