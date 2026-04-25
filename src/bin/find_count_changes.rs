/// Probe: find files with large ENTITY_COUNT_CHANGE, sorted by drop magnitude.
use artisan_pcgen::{parse_text_to_catalog, unparse_catalog_to_text};
use std::fs;
use std::path::{Path, PathBuf};

fn scan_dir(dir: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            scan_dir(&p, paths);
        } else if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            if ext == "lst" || ext == "pcc" {
                paths.push(p);
            }
        }
    }
}

fn main() {
    #[path = "probeutil/config.rs"]
    mod probe_config;
    let config = probe_config::ProbeConfig::load();

    let mut all_files: Vec<PathBuf> = Vec::new();
    for root in config.scan_roots() {
        scan_dir(Path::new(root), &mut all_files);
    }

    let mut changes: Vec<(usize, usize, String)> = Vec::new();

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
            let short = path.display().to_string();
            let short = short.split("externals/").last().unwrap_or(&short).to_string();
            changes.push((n1, n2, short));
        }
    }

    // Sort by magnitude of drop (largest drop first)
    changes.sort_by(|a, b| {
        let drop_a = a.0.saturating_sub(a.1);
        let drop_b = b.0.saturating_sub(b.1);
        drop_b.cmp(&drop_a)
    });

    println!("Files with ENTITY_COUNT_CHANGE (sorted by drop magnitude):");
    println!("{:>6} {:>6}  {}", "before", "after", "file");
    for (n1, n2, path) in &changes {
        println!("{:>6} {:>6}  {}", n1, n2, path);
    }
    println!("\nTotal: {} files", changes.len());
}
