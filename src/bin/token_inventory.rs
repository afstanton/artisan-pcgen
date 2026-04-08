use artisan_pcgen::{
    ParsedClause, TokenSupportLevel, classify_clause_handling, classify_token_key_support,
    parse_line,
};
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

    let mut semantic_counts: HashMap<String, usize> = HashMap::new();
    let mut policy_supported_counts: HashMap<String, usize> = HashMap::new();
    let mut unhandled_counts: HashMap<String, usize> = HashMap::new();
    let mut file_count = 0;
    let mut total_lines = 0;
    let mut fixed_count = 0;

    for root_arg in &args[1..] {
        let root = Path::new(root_arg);

        // Scan data and system directories
        for subdir in &["data", "system"] {
            let path = root.join(subdir);
            if path.exists() && path.is_dir() {
                let (_, fixes) = scan_directory(
                    &path,
                    &mut semantic_counts,
                    &mut policy_supported_counts,
                    &mut unhandled_counts,
                    &mut file_count,
                    &mut total_lines,
                )?;
                fixed_count += fixes;
            }
        }
    }

    println!("\n=== Token Inventory Summary ===");
    println!("Files scanned: {}", file_count);
    println!("Total lines processed: {}", total_lines);
    println!(
        "Unique observed token keys: {}",
        semantic_counts.len() + policy_supported_counts.len() + unhandled_counts.len()
    );
    println!("Unique semantically interpreted token keys: {}", semantic_counts.len());
    println!(
        "Unique policy-supported-only token keys: {}",
        policy_supported_counts.len()
    );
    println!("Unique unhandled tokens: {}", unhandled_counts.len());
    if fixed_count > 0 {
        println!("Files with UTF-8 encoding issues fixed: {}", fixed_count);
    }
    println!();

    let mut semantic_tokens: Vec<_> = semantic_counts.iter().collect();
    semantic_tokens.sort_by(|a, b| b.1.cmp(a.1));

    println!("=== Semantically Interpreted Tokens by Frequency ===");
    for (token, count) in semantic_tokens {
        println!("{:6} | {}", count, token);
    }

    let mut policy_tokens: Vec<_> = policy_supported_counts.iter().collect();
    policy_tokens.sort_by(|a, b| b.1.cmp(a.1));

    println!();
    println!("=== Policy-Supported-Only Tokens by Frequency ===");
    for (token, count) in policy_tokens {
        println!("{:6} | {}", count, token);
    }

    let mut unhandled: Vec<_> = unhandled_counts.iter().collect();
    unhandled.sort_by(|a, b| b.1.cmp(a.1));

    println!();
    println!("=== Unhandled Tokens by Frequency ===");
    for (token, count) in unhandled {
        println!("{:6} | {}", count, token);
    }

    Ok(())
}

fn scan_directory(
    path: &Path,
    semantic_counts: &mut HashMap<String, usize>,
    policy_supported_counts: &mut HashMap<String, usize>,
    unhandled_counts: &mut HashMap<String, usize>,
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
            let (_, f) =
                scan_directory(
                    &path,
                    semantic_counts,
                    policy_supported_counts,
                    unhandled_counts,
                    file_count,
                    total_lines,
                )?;
            fix_count += f;
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            if ext_str == "lst" || ext_str == "pcc" || ext_str == "pcg" {
                if let Ok(fixed) =
                    process_file(
                        &path,
                        semantic_counts,
                        policy_supported_counts,
                        unhandled_counts,
                        file_count,
                        total_lines,
                    )
                {
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
    semantic_counts: &mut HashMap<String, usize>,
    policy_supported_counts: &mut HashMap<String, usize>,
    unhandled_counts: &mut HashMap<String, usize>,
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

        let parsed = parse_line(trimmed);

        if let Some(head_key) = extract_head_key(&parsed.head) {
            match classify_token_key_support(&head_key, false) {
                TokenSupportLevel::SemanticallyInterpreted => {
                    *semantic_counts.entry(head_key).or_insert(0) += 1;
                }
                TokenSupportLevel::PolicySupported => {
                    *policy_supported_counts.entry(head_key).or_insert(0) += 1;
                }
                TokenSupportLevel::Unhandled(token_key) => {
                    *unhandled_counts.entry(token_key).or_insert(0) += 1;
                }
                TokenSupportLevel::Artifact => {}
            }
        }

        for (clause_index, clause) in parsed.clauses.iter().enumerate() {
            match classify_clause_handling(clause) {
                TokenSupportLevel::SemanticallyInterpreted => {
                    if let Some(token_key) = extract_token_key(clause, clause_index) {
                        *semantic_counts.entry(token_key).or_insert(0) += 1;
                    }
                }
                TokenSupportLevel::PolicySupported => {
                    if let Some(token_key) = extract_token_key(clause, clause_index) {
                        *policy_supported_counts.entry(token_key).or_insert(0) += 1;
                    }
                }
                TokenSupportLevel::Unhandled(token_key) => {
                    *unhandled_counts.entry(token_key).or_insert(0) += 1;
                }
                TokenSupportLevel::Artifact => {}
            }
        }
    }

    Ok(was_fixed)
}

fn extract_head_key(head: &str) -> Option<String> {
    let token = head.split('\t').next().unwrap_or(head).trim();
    let colon_idx = token.find(':')?;
    normalize_key(&token[..colon_idx])
}

fn extract_token_key(clause: &ParsedClause, clause_index: usize) -> Option<String> {
    match clause {
        ParsedClause::KeyValue { key, .. } => normalize_key(key),
        ParsedClause::Bare(value) => {
            // Only consider first bare clause as a possible directive token.
            // Later bare clauses are usually positional values split from another token.
            if clause_index > 0 {
                return None;
            }
            normalize_bare_directive(value)
        }
    }
}

fn normalize_bare_directive(value: &str) -> Option<String> {
    let token = value.split('\t').next().unwrap_or(value).trim();
    if token.is_empty() || token.contains(' ') {
        return None;
    }

    let upper = token.to_ascii_uppercase();
    if !is_plausible_token_name(&upper) {
        return None;
    }

    if upper.starts_with("PRE") || upper.starts_with("!PRE") {
        return Some(upper);
    }

    match upper.as_str() {
        "AUTOMATIC" | "VISIBLE" | "VIRTUAL" | "PRERULE" | "!PRERULE" | "SET" => Some(upper),
        _ => None,
    }
}

fn normalize_key(raw: &str) -> Option<String> {
    let key = raw.trim();
    if key.is_empty() {
        return None;
    }

    let key = key.trim_matches(|c: char| c == '(' || c == ')');
    if key.is_empty() {
        return None;
    }

    let upper = key.to_ascii_uppercase();
    if !is_plausible_token_name(&upper) {
        return None;
    }

    Some(upper)
}

fn is_plausible_token_name(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    if !token
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '!' | '-' | '.'))
    {
        return false;
    }

    // Exclude value-like fragments such as "1", "23", "10".
    if token.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // Functional token names should contain at least one alphabetic character.
    token.chars().any(|c| c.is_ascii_alphabetic())
}
