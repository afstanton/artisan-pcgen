use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: token_inventory <pcgen_root1> [pcgen_root2] ...");
        eprintln!("Example: token_inventory /path/to/PCGen/pcgen /path/to/BahamutDragon/pcgen");
        std::process::exit(1);
    }

    let mut token_counts: HashMap<String, usize> = HashMap::new();
    let mut file_count = 0;
    let mut total_lines = 0;
    let mut fixed_count = 0;

    for root_arg in &args[1..] {
        let root = Path::new(root_arg);

        // Scan data and system directories
        for subdir in &["data", "system"] {
            let path = root.join(subdir);
            if path.exists() && path.is_dir() {
                let (_, fixes) = scan_directory(&path, &mut token_counts, &mut file_count, &mut total_lines)?;
                fixed_count += fixes;
            }
        }
    }

    println!("\n=== Token Inventory Summary ===");
    println!("Files scanned: {}", file_count);
    println!("Total lines processed: {}", total_lines);
    println!("Unique tokens: {}", token_counts.len());
    if fixed_count > 0 {
        println!("Files with UTF-8 encoding issues fixed: {}", fixed_count);
    }
    println!();

    let mut tokens: Vec<_> = token_counts.iter().collect();
    tokens.sort_by(|a, b| b.1.cmp(a.1));

    println!("=== Tokens by Frequency ===");
    for (token, count) in tokens {
        println!("{:6} | {}", count, token);
    }

    Ok(())
}

fn scan_directory(
    path: &Path,
    token_counts: &mut HashMap<String, usize>,
    file_count: &mut usize,
    total_lines: &mut usize,
) -> io::Result<(usize, usize)> {
    let mut fix_count = 0;

    if !path.is_dir() {
        return Ok((0, 0));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Skip hidden directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with('.') {
                    continue;
                }
            }
            let (_, f) = scan_directory(&path, token_counts, file_count, total_lines)?;
            fix_count += f;
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            if ext_str == "lst" || ext_str == "pcc" || ext_str == "pcg" {
                if let Ok(fixed) = process_file(&path, token_counts, file_count, total_lines) {
                    if fixed {
                        fix_count += 1;
                    }
                }
            }
        }
    }

    Ok((0, fix_count))
}

fn process_file(
    path: &Path,
    token_counts: &mut HashMap<String, usize>,
    file_count: &mut usize,
    total_lines: &mut usize,
) -> io::Result<bool> {
    let bytes = fs::read(path)?;
    let content = String::from_utf8_lossy(&bytes);
    let was_fixed = content.contains('\u{FFFD}');

    *file_count += 1;

    for line in content.lines() {
        *total_lines += 1;

        // Skip comments and empty lines
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Parse pipe-delimited tokens
        for token in trimmed.split('|') {
            let token = token.trim();
            if !token.is_empty() {
                // Extract the key part (before any colon or parentheses)
                let token_key = extract_token_key(token);
                if !token_key.is_empty() {
                    *token_counts.entry(token_key).or_insert(0) += 1;
                }
            }
        }
    }

    Ok(was_fixed)
}

fn extract_token_key(token: &str) -> String {
    // Extract the main token key, handling various formats:
    // - ABILITY:CATEGORY=Foo → ABILITY
    // - BONUS:COMBAT|AC|+2 → BONUS
    // - CHOOSE:STRING → CHOOSE
    // - SKILL|Acrobatics|1 → SKILL
    // - Something(text) → Something

    // First, handle cases where there's a tab or multiple spaces
    let token = token.split('\t').next().unwrap_or(token).trim();

    // Extract before:
    if let Some(colon_idx) = token.find(':') {
        return token[..colon_idx].to_uppercase();
    }

    // Extract before (
    if let Some(paren_idx) = token.find('(') {
        return token[..paren_idx].to_uppercase();
    }

    // For pipes within a token, usually the first part is the key
    // But we want to preserve the structure, so just uppercase what we have
    token.to_uppercase()
}
