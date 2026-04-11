use artisan_pcgen::{
    ParsedClause, TokenSupportLevel, classify_clause_handling, classify_token_key_support,
    emittable_keys_for_entity, fallback_keys_for_entity, parse_line, parse_text_to_catalog,
};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
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
    let mut fully_structured_counts: HashMap<String, usize> = HashMap::new();
    let mut fallback_needed_counts: HashMap<String, usize> = HashMap::new();
    let mut policy_supported_counts: HashMap<String, usize> = HashMap::new();
    let mut unhandled_counts: HashMap<String, usize> = HashMap::new();
    let fixture_token_counts = collect_fixture_token_counts()?;
    let mut file_count = 0;
    let mut total_lines = 0;
    let mut fixed_count = 0;

    eprintln!("token_inventory: scanning {} root(s)", args.len() - 1);

    for root_arg in &args[1..] {
        let root = Path::new(root_arg);

        // Scan data and system directories
        for subdir in &["data", "system", "characters"] {
            let path = root.join(subdir);
            if path.exists() && path.is_dir() {
                eprintln!("token_inventory: scanning {}", path.display());
                let (_, fixes) = scan_directory(
                    &path,
                    &mut semantic_counts,
                    &mut fully_structured_counts,
                    &mut fallback_needed_counts,
                    &mut policy_supported_counts,
                    &mut unhandled_counts,
                    &mut file_count,
                    &mut total_lines,
                )?;
                fixed_count += fixes;
            }
        }
    }

    let mut report = String::new();
    let corpus_token_counts =
        combined_token_counts(&semantic_counts, &policy_supported_counts, &unhandled_counts);
    let fixture_token_set: HashSet<String> = fixture_token_counts.keys().cloned().collect();
    let corpus_token_set: HashSet<String> = corpus_token_counts.keys().cloned().collect();

    writeln!(report, "\n=== Token Inventory Summary ===").unwrap();
    writeln!(report, "Files scanned: {}", file_count).unwrap();
    writeln!(report, "Total lines processed: {}", total_lines).unwrap();
    writeln!(
        report,
        "Unique observed token keys: {}",
        semantic_counts.len() + policy_supported_counts.len() + unhandled_counts.len()
    )
    .unwrap();
    writeln!(
        report,
        "Unique semantically interpreted token keys: {}",
        semantic_counts.len()
    )
    .unwrap();
    writeln!(
        report,
        "Unique fully-structured canonical token keys: {}",
        fully_structured_counts.len()
    )
    .unwrap();
    writeln!(
        report,
        "Unique semantically interpreted but not fully-structured token keys: {}",
        semantic_counts
            .keys()
            .filter(|k| !fully_structured_counts.contains_key(*k))
            .count()
    )
    .unwrap();
    writeln!(
        report,
        "Unique fallback-needed token keys: {}",
        fallback_needed_counts.len()
    )
    .unwrap();
    writeln!(
        report,
        "Unique policy-supported-only token keys: {}",
        policy_supported_counts.len()
    )
    .unwrap();
    writeln!(
        report,
        "Unique unhandled tokens: {}",
        unhandled_counts.len()
    )
    .unwrap();
    writeln!(
        report,
        "Unique tokens represented in fixture files: {}",
        fixture_token_counts.len()
    )
    .unwrap();
    if fixed_count > 0 {
        writeln!(
            report,
            "Files with UTF-8 encoding issues fixed: {}",
            fixed_count
        )
        .unwrap();
    }
    writeln!(report).unwrap();

    let mut fixture_tokens: Vec<_> = fixture_token_counts.iter().collect();
    fixture_tokens.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    writeln!(report, "=== Fixture Tokens by Frequency ===").unwrap();
    for (token, count) in fixture_tokens {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    writeln!(report).unwrap();

    let mut corpus_not_fixtures: Vec<_> = corpus_token_counts
        .iter()
        .filter(|(token, _)| !fixture_token_set.contains(*token))
        .collect();
    corpus_not_fixtures.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    writeln!(report, "=== Tokens Found In The Corpus But Not The Fixtures ===").unwrap();
    for (token, count) in corpus_not_fixtures {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    writeln!(report).unwrap();

    let mut fixtures_not_corpus: Vec<_> = fixture_token_counts
        .iter()
        .filter(|(token, _)| !corpus_token_set.contains(*token))
        .collect();
    fixtures_not_corpus.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    writeln!(report, "=== Tokens Found In The Fixtures But Not The Corpus ===").unwrap();
    for (token, count) in fixtures_not_corpus {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    writeln!(report).unwrap();

    let mut semantic_tokens: Vec<_> = semantic_counts.iter().collect();
    semantic_tokens.sort_by(|a, b| b.1.cmp(a.1));

    let mut structured_tokens: Vec<_> = fully_structured_counts.iter().collect();
    structured_tokens.sort_by(|a, b| b.1.cmp(a.1));

    let mut fallback_tokens: Vec<_> = fallback_needed_counts.iter().collect();
    fallback_tokens.sort_by(|a, b| b.1.cmp(a.1));

    let mut semantic_not_fully_structured_tokens: Vec<_> = semantic_counts
        .iter()
        .filter(|(token, _)| !fully_structured_counts.contains_key(*token))
        .collect();
    semantic_not_fully_structured_tokens.sort_by(|a, b| b.1.cmp(a.1));

    writeln!(
        report,
        "=== Semantically Interpreted Tokens by Frequency ==="
    )
    .unwrap();
    for (token, count) in semantic_tokens {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    writeln!(report).unwrap();
    writeln!(
        report,
        "=== Fully-Structured Canonical Tokens by Frequency ==="
    )
    .unwrap();
    for (token, count) in structured_tokens {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    writeln!(report).unwrap();
    writeln!(report, "=== Fallback-Needed Tokens by Frequency ===").unwrap();
    for (token, count) in fallback_tokens {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    writeln!(report).unwrap();
    writeln!(
        report,
        "=== Semantically Interpreted But Not Fully-Structured Tokens by Frequency ==="
    )
    .unwrap();
    for (token, count) in semantic_not_fully_structured_tokens {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    let mut policy_tokens: Vec<_> = policy_supported_counts.iter().collect();
    policy_tokens.sort_by(|a, b| b.1.cmp(a.1));

    writeln!(report).unwrap();
    writeln!(report, "=== Policy-Supported-Only Tokens by Frequency ===").unwrap();
    for (token, count) in policy_tokens {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    let mut unhandled: Vec<_> = unhandled_counts.iter().collect();
    unhandled.sort_by(|a, b| b.1.cmp(a.1));

    writeln!(report).unwrap();
    writeln!(report, "=== Unhandled Tokens by Frequency ===").unwrap();
    for (token, count) in unhandled {
        writeln!(report, "{:6} | {}", count, token).unwrap();
    }

    let output_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("TOKEN_INVENTORY.txt");
    fs::write(&output_path, &report)?;
    eprintln!(
        "token_inventory: wrote {} (interpreted={}, structured={}, unhandled={}, fixtures={})",
        output_path.display(),
        semantic_counts.len(),
        fully_structured_counts.len(),
        unhandled_counts.len(),
        fixture_token_counts.len()
    );

    Ok(())
}

fn scan_directory(
    path: &Path,
    semantic_counts: &mut HashMap<String, usize>,
    fully_structured_counts: &mut HashMap<String, usize>,
    fallback_needed_counts: &mut HashMap<String, usize>,
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
            let (_, f) = scan_directory(
                &path,
                semantic_counts,
                fully_structured_counts,
                fallback_needed_counts,
                policy_supported_counts,
                unhandled_counts,
                file_count,
                total_lines,
            )?;
            fix_count += f;
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            if ext_str == "lst" || ext_str == "pcc" || ext_str == "pcg" {
                if let Ok(fixed) = process_file(
                    &path,
                    semantic_counts,
                    fully_structured_counts,
                    fallback_needed_counts,
                    policy_supported_counts,
                    unhandled_counts,
                    file_count,
                    total_lines,
                ) {
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
    fully_structured_counts: &mut HashMap<String, usize>,
    fallback_needed_counts: &mut HashMap<String, usize>,
    policy_supported_counts: &mut HashMap<String, usize>,
    unhandled_counts: &mut HashMap<String, usize>,
    file_count: &mut usize,
    total_lines: &mut usize,
) -> io::Result<bool> {
    let bytes = fs::read(path)?;
    let content = String::from_utf8_lossy(&bytes);
    let was_fixed = content.contains('\u{FFFD}');

    let source_name = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("inventory");
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_else(|| "lst".to_string());

    let parsed_catalog = parse_text_to_catalog(&content, source_name, &ext);
    let mut emittable_by_line: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut fallback_by_line: HashMap<usize, HashSet<String>> = HashMap::new();
    for entity in &parsed_catalog.entities {
        let Some(line_number) = entity
            .attributes
            .get("pcgen_line_number")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize)
        else {
            continue;
        };

        let Some(type_key) = entity
            .attributes
            .get("pcgen_entity_type_key")
            .and_then(|v| v.as_str())
        else {
            continue;
        };

        let Some(schema) = artisan_pcgen::schema::schema_for_entity_type_key(type_key) else {
            continue;
        };

        let emittable: HashSet<String> = emittable_keys_for_entity(entity, schema)
            .into_iter()
            .map(|k| k.to_ascii_uppercase())
            .collect();
        emittable_by_line
            .entry(line_number)
            .or_default()
            .extend(emittable);

        let fallback: HashSet<String> = fallback_keys_for_entity(entity, schema)
            .into_iter()
            .map(|k| k.to_ascii_uppercase())
            .collect();

        fallback_by_line
            .entry(line_number)
            .or_default()
            .extend(fallback);
    }

    *file_count += 1;

    for (line_idx, line) in content.lines().enumerate() {
        let line_number = line_idx + 1;
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
                    *semantic_counts.entry(head_key.clone()).or_insert(0) += 1;
                    if emittable_by_line
                        .get(&line_number)
                        .is_some_and(|set| set.contains(&head_key))
                    {
                        *fully_structured_counts.entry(head_key).or_insert(0) += 1;
                    } else if fallback_by_line
                        .get(&line_number)
                        .is_some_and(|set| set.contains(&head_key))
                    {
                        *fallback_needed_counts.entry(head_key).or_insert(0) += 1;
                    }
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
                        *semantic_counts.entry(token_key.clone()).or_insert(0) += 1;
                        if emittable_by_line
                            .get(&line_number)
                            .is_some_and(|set| set.contains(&token_key))
                        {
                            *fully_structured_counts.entry(token_key).or_insert(0) += 1;
                        } else if fallback_by_line
                            .get(&line_number)
                            .is_some_and(|set| set.contains(&token_key))
                        {
                            *fallback_needed_counts.entry(token_key).or_insert(0) += 1;
                        }
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

fn collect_fixture_token_counts() -> io::Result<HashMap<String, usize>> {
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pcgen");
    let mut counts = HashMap::new();

    if !fixture_root.exists() || !fixture_root.is_dir() {
        return Ok(counts);
    }

    collect_fixture_tokens_in_dir(&fixture_root, &mut counts)?;
    Ok(counts)
}

fn collect_fixture_tokens_in_dir(
    path: &Path,
    counts: &mut HashMap<String, usize>,
) -> io::Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child = entry.path();

        if child.is_dir() {
            collect_fixture_tokens_in_dir(&child, counts)?;
            continue;
        }

        let Some(ext) = child.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        let ext = ext.to_ascii_lowercase();
        if ext != "lst" && ext != "pcc" && ext != "pcg" {
            continue;
        }

        collect_fixture_tokens_from_file(&child, counts)?;
    }

    Ok(())
}

fn collect_fixture_tokens_from_file(
    path: &Path,
    counts: &mut HashMap<String, usize>,
) -> io::Result<()> {
    let bytes = fs::read(path)?;
    let content = String::from_utf8_lossy(&bytes);

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let parsed = parse_line(trimmed);

        if let Some(head_key) = extract_head_key(&parsed.head) {
            *counts.entry(head_key).or_insert(0) += 1;
        }

        for (clause_index, clause) in parsed.clauses.iter().enumerate() {
            if let Some(token_key) = extract_token_key(clause, clause_index) {
                *counts.entry(token_key).or_insert(0) += 1;
            }
        }
    }

    Ok(())
}

fn combined_token_counts(
    semantic_counts: &HashMap<String, usize>,
    policy_supported_counts: &HashMap<String, usize>,
    unhandled_counts: &HashMap<String, usize>,
) -> HashMap<String, usize> {
    let mut combined = HashMap::new();

    for counts in [semantic_counts, policy_supported_counts, unhandled_counts] {
        for (token, count) in counts {
            *combined.entry(token.clone()).or_insert(0) += count;
        }
    }

    combined
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

    // Bare FEAT is not a valid top-level directive in live corpus data and
    // only appears as malformed text or documentation residue.
    if upper == "FEAT" {
        return None;
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

    if key.chars().any(|c| c.is_ascii_lowercase()) {
        return None;
    }

    let upper = key.to_ascii_uppercase();
    if is_standalone_roman_numeral(&upper) {
        return None;
    }
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

fn is_standalone_roman_numeral(token: &str) -> bool {
    matches!(
        token,
        "I" | "II" | "III" | "IV" | "V" | "VI" | "VII" | "VIII" | "IX" | "X" | "XI" | "XII"
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{combined_token_counts, normalize_bare_directive, normalize_key};

    #[test]
    fn normalize_key_rejects_mixed_case_prose_keys() {
        assert_eq!(normalize_key("Feat"), None);
        assert_eq!(normalize_key("Type"), None);
    }

    #[test]
    fn normalize_key_accepts_uppercase_tokens() {
        assert_eq!(normalize_key("FEAT"), Some("FEAT".to_string()));
        assert_eq!(normalize_key("!PRETYPE"), Some("!PRETYPE".to_string()));
    }

    #[test]
    fn normalize_key_rejects_standalone_roman_numerals() {
        assert_eq!(normalize_key("I"), None);
        assert_eq!(normalize_key("II"), None);
        assert_eq!(normalize_key("IV"), None);
    }

    #[test]
    fn normalize_bare_directive_still_ignores_bare_feat() {
        assert_eq!(normalize_bare_directive("FEAT"), None);
    }

    #[test]
    fn combined_token_counts_merges_source_maps() {
        let semantic = HashMap::from([
            ("FEAT".to_string(), 2usize),
            ("CLASS".to_string(), 1usize),
        ]);
        let policy = HashMap::from([("EQUIPMENT.PART".to_string(), 3usize)]);
        let unhandled = HashMap::from([
            ("NOTE".to_string(), 4usize),
            ("FEAT".to_string(), 1usize),
        ]);

        let combined = combined_token_counts(&semantic, &policy, &unhandled);

        assert_eq!(combined.get("FEAT"), Some(&3usize));
        assert_eq!(combined.get("CLASS"), Some(&1usize));
        assert_eq!(combined.get("EQUIPMENT.PART"), Some(&3usize));
        assert_eq!(combined.get("NOTE"), Some(&4usize));
    }
}
