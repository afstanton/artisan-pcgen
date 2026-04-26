/// Recursively parse PCGen files from one or more directories, build a
/// cumulative ParsedCatalog, and emit the result as organized PCGen files.
///
/// Usage:
///   emit_catalog [--output <dir>] [--manifest <manifest.toml>] <input_dir>...
///
/// Options:
///   --output <dir>      Directory to write emitted files into. Default: ./data
///   --manifest <file>   TOML or JSON EmitManifest override file.
///
/// Input directories are walked recursively. Files with extensions .lst, .pcc,
/// and .pcg are parsed and merged into a single catalog before emission.

use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

use artisan_pcgen::{
    ParsedCatalog, parse_text_to_catalog,
    file_emit::{EmitManifest, emit_catalog},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut input_dirs: Vec<PathBuf> = Vec::new();
    let mut output_dir: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" | "-o" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("emit_catalog: --output requires an argument");
                    std::process::exit(1);
                }
                output_dir = Some(PathBuf::from(&args[i]));
            }
            "--manifest" | "-m" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("emit_catalog: --manifest requires an argument");
                    std::process::exit(1);
                }
                manifest_path = Some(PathBuf::from(&args[i]));
            }
            arg if arg.starts_with('-') => {
                eprintln!("emit_catalog: unknown option {arg:?}");
                print_usage();
                std::process::exit(1);
            }
            path => {
                input_dirs.push(PathBuf::from(path));
            }
        }
        i += 1;
    }

    if input_dirs.is_empty() {
        print_usage();
        std::process::exit(1);
    }

    let out_dir = output_dir.unwrap_or_else(|| PathBuf::from("data"));

    // Load optional manifest.
    let manifest: Option<EmitManifest> = match manifest_path {
        Some(ref mp) => {
            let text = fs::read_to_string(mp).map_err(|e| {
                eprintln!("emit_catalog: cannot read manifest {:?}: {e}", mp);
                e
            })?;
            let ext = mp.extension().and_then(|e| e.to_str()).unwrap_or("toml");
            let m: EmitManifest = if ext == "json" {
                serde_json::from_str(&text).map_err(|e| {
                    eprintln!("emit_catalog: cannot parse manifest JSON: {e}");
                    io::Error::new(io::ErrorKind::InvalidData, e)
                })?
            } else {
                toml::from_str(&text).map_err(|e| {
                    eprintln!("emit_catalog: cannot parse manifest TOML: {e}");
                    io::Error::new(io::ErrorKind::InvalidData, e.to_string())
                })?
            };
            Some(m)
        }
        None => None,
    };

    // Walk all input directories and collect files.
    let mut all_files: Vec<PathBuf> = Vec::new();
    for dir in &input_dirs {
        if !dir.exists() {
            eprintln!("emit_catalog: input directory {:?} does not exist; skipping", dir);
            continue;
        }
        collect_files(dir, &mut all_files)?;
    }

    if all_files.is_empty() {
        eprintln!("emit_catalog: no .lst/.pcc/.pcg files found in the given directories");
        std::process::exit(1);
    }

    eprintln!("emit_catalog: found {} files to parse", all_files.len());

    // Parse and merge into a single catalog.
    let mut merged = ParsedCatalog::default();
    for (idx, file_path) in all_files.iter().enumerate() {
        if idx == 0 || (idx + 1) % 500 == 0 {
            eprintln!(
                "emit_catalog: parsing file {}/{} — {}",
                idx + 1,
                all_files.len(),
                file_path.display()
            );
        }

        let catalog = match parse_file(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("emit_catalog: warning: cannot read {:?}: {e}", file_path);
                continue;
            }
        };

        merge_into(&mut merged, catalog);
    }

    eprintln!(
        "emit_catalog: merged catalog — {} entities, {} sources, {} publishers",
        merged.entities.len(),
        merged.sources.len(),
        merged.publishers.len()
    );

    // Emit.
    eprintln!("emit_catalog: emitting to {:?}", out_dir);

    let file_map = emit_catalog(&merged, manifest.as_ref(), &out_dir).map_err(|e| {
        eprintln!("emit_catalog: emission failed: {e}");
        io::Error::new(io::ErrorKind::Other, e.to_string())
    })?;

    eprintln!(
        "emit_catalog: wrote {} file(s) to {:?}",
        file_map.len(),
        out_dir
    );

    for path in file_map.keys() {
        println!("{}", out_dir.join(path).display());
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn print_usage() {
    eprintln!(
        "Usage: emit_catalog [--output <dir>] [--manifest <file.toml>] <input_dir>..."
    );
    eprintln!("  --output <dir>      Output directory (default: ./data)");
    eprintln!("  --manifest <file>   Optional TOML or JSON EmitManifest override file");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  emit_catalog --output ./out /path/to/PCGen/data");
    eprintln!("  emit_catalog /path/to/PCGen/data /path/to/extra/data");
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out)?;
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_default();

        if matches!(ext.as_str(), "lst" | "pcc" | "pcg") {
            out.push(path);
        }
    }

    Ok(())
}

fn parse_file(path: &Path) -> io::Result<ParsedCatalog> {
    let bytes = fs::read(path)?;
    let text = String::from_utf8_lossy(&bytes).to_string();
    let source_name = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unknown");
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_else(|| "lst".to_string());
    Ok(parse_text_to_catalog(&text, source_name, &ext))
}

fn merge_into(dest: &mut ParsedCatalog, src: ParsedCatalog) {
    dest.publishers.extend(src.publishers);
    dest.sources.extend(src.sources);
    dest.citations.extend(src.citations);
    dest.entity_types.extend(src.entity_types);
    dest.entities.extend(src.entities);
    dest.character_graphs.extend(src.character_graphs);
    dest.identity_links.extend(src.identity_links);
    dest.mapping_records.extend(src.mapping_records);
    dest.projection_maps.extend(src.projection_maps);
    dest.loss_notes.extend(src.loss_notes);
}
