use lalrpop_util::ParseError;
use serde_json::{Value, json};

use crate::{ParsedClause, ParsedLine};

use super::parser_tokens;

pub(crate) fn parse_line_internal(line: &str) -> ParsedLine {
    let segments = split_top_level_segments(line);
    let mut iter = segments.into_iter();
    let head = iter.next().unwrap_or_default();
    let clauses = iter.map(|segment| parse_clause(&segment)).collect();
    ParsedLine { head, clauses }
}

pub(crate) fn unparse_line_internal(head: &str, clauses: &[ParsedClause]) -> String {
    let mut parts = vec![escape_head_segment(head)];
    for clause in clauses {
        match clause {
            ParsedClause::Bare(value) => parts.push(escape_segment(value)),
            ParsedClause::KeyValue { key, value } => {
                parts.push(format!("{}:{}", escape_segment(key), escape_segment(value)));
            }
        }
    }
    parts.join("|")
}

pub(crate) fn split_first_key_value(parsed: &ParsedLine) -> Option<(String, String)> {
    if let Some((k, v)) = parse_head_key_value(&parsed.head) {
        return Some((k, v));
    }
    for clause in &parsed.clauses {
        if let ParsedClause::KeyValue { key, value } = clause {
            return Some((key.clone(), value.clone()));
        }
    }
    None
}

pub(crate) fn parse_head_key_value(head: &str) -> Option<(String, String)> {
    let idx = head.find(':')?;
    let key = head[..idx].trim();
    let value = head[idx + 1..].trim();
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key.to_string(), value.to_string()))
}

pub(crate) fn find_key_value(clauses: &[ParsedClause], key: &str) -> Option<String> {
    for clause in clauses {
        if let ParsedClause::KeyValue { key: k, value } = clause
            && k.eq_ignore_ascii_case(key)
        {
            return Some(value.clone());
        }
    }
    None
}

pub(crate) fn clauses_to_json(clauses: &[ParsedClause]) -> Vec<Value> {
    clauses
        .iter()
        .map(|clause| match clause {
            ParsedClause::Bare(value) => json!({"kind": "bare", "value": value}),
            ParsedClause::KeyValue { key, value } => {
                json!({"kind": "key_value", "key": key, "value": value})
            }
        })
        .collect()
}

pub(crate) fn clauses_from_json(value: &Value) -> Option<Vec<ParsedClause>> {
    let array = value.as_array()?;
    let mut out = Vec::new();
    for item in array {
        let kind = item.get("kind")?.as_str()?;
        match kind {
            "bare" => out.push(ParsedClause::Bare(item.get("value")?.as_str()?.to_string())),
            "key_value" => out.push(ParsedClause::KeyValue {
                key: item.get("key")?.as_str()?.to_string(),
                value: item.get("value")?.as_str()?.to_string(),
            }),
            _ => return None,
        }
    }
    Some(out)
}

fn parse_segments_with_generated_parser(line: &str) -> Vec<String> {
    let parser = super::line_grammar::SegmentsParser::new();
    let parse_result: Result<Vec<String>, ParseError<usize, super::parser_tokens::LineToken, String>> =
        parser.parse(parser_tokens::line_tokens(line));
    match parse_result {
        Ok(segments) => normalize_non_empty_segments(segments),
        Err(_) => normalize_non_empty_segments([line.to_string()]),
    }
}

fn split_top_level_segments(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    // Fast path: lines with no whitespace cannot have whitespace token separators.
    if !trimmed.chars().any(char::is_whitespace) {
        return merge_token_prefixed_head_segments(parse_segments_with_generated_parser(trimmed));
    }

    let whitespace_segments = split_on_whitespace_token_starts_trimmed(trimmed);
    if whitespace_segments.len() > 1 {
        return whitespace_segments;
    }

    merge_token_prefixed_head_segments(parse_segments_with_generated_parser(trimmed))
}

fn merge_token_prefixed_head_segments(mut segments: Vec<String>) -> Vec<String> {
    if segments.len() < 2 {
        return segments;
    }

    let Some((head_key, _)) = parse_head_key_value(&segments[0]) else {
        return segments;
    };

    let Some(schema) = crate::schema::schema_for_head_token(&head_key) else {
        return segments;
    };

    if !matches!(schema.head_format, crate::schema::HeadFormat::TokenPrefixed) {
        return segments;
    }

    let mut merged_head = segments.remove(0);
    while let Some(next) = segments.first() {
        if looks_like_token_start(next, 0) {
            break;
        }
        merged_head.push('|');
        merged_head.push_str(&segments.remove(0));
    }

    let mut merged = vec![merged_head];
    merged.extend(segments);
    merged
}

fn split_on_whitespace_token_starts_trimmed(trimmed: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut start = 0usize;
    let mut scan = 0usize;

    while scan < trimmed.len() {
        let next_char = trimmed[scan..].chars().next().expect("valid char boundary");
        if next_char.is_whitespace() {
            let whitespace_start = scan;
            let mut after_ws = scan;
            while after_ws < trimmed.len() {
                let ch = trimmed[after_ws..].chars().next().expect("valid char boundary");
                if !ch.is_whitespace() {
                    break;
                }
                after_ws += ch.len_utf8();
            }

            if looks_like_token_start(trimmed, after_ws) {
                push_trimmed_if_non_empty(&mut segments, &trimmed[start..whitespace_start]);
                start = after_ws;
                scan = after_ws;
                continue;
            }

            scan = after_ws;
            continue;
        }

        scan += next_char.len_utf8();
    }

    push_trimmed_if_non_empty(&mut segments, &trimmed[start..]);

    segments
}

fn normalize_non_empty_segments<I>(segments: I) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    let mut out = Vec::new();
    for segment in segments {
        push_trimmed_if_non_empty(&mut out, &segment);
    }
    out
}

fn push_trimmed_if_non_empty(out: &mut Vec<String>, input: &str) {
    let trimmed = input.trim();
    if !trimmed.is_empty() {
        out.push(trimmed.to_string());
    }
}

fn looks_like_token_start(input: &str, start: usize) -> bool {
    if start >= input.len() {
        return false;
    }

    let mut index = start;
    let mut saw_alpha = false;
    let mut saw_upper = false;

    if input[index..].starts_with('!') {
        index += 1;
    }

    while index < input.len() {
        let ch = input[index..].chars().next().expect("valid char boundary");
        if ch == ':' {
            return saw_alpha && saw_upper;
        }
        if !(ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.')) {
            return false;
        }
        if ch.is_ascii_alphabetic() {
            saw_alpha = true;
            if ch.is_ascii_uppercase() {
                saw_upper = true;
            } else {
                // Token keys in PCGen are uppercase; lowercase strongly suggests
                // this is inline free text (e.g., description prose).
                return false;
            }
        }
        index += ch.len_utf8();
    }

    false
}

fn parse_clause(segment: &str) -> ParsedClause {
    let parser = super::clause_grammar::ClausePiecesParser::new();
    let parse_result: Result<Vec<Option<String>>, ParseError<usize, super::parser_tokens::ClauseToken, String>> =
        parser.parse(parser_tokens::clause_tokens(segment));

    if let Ok(parts) = parse_result {
        return build_clause_from_parts(parts.into_iter().map(ClausePart::from));
    }

    // Keep a conservative fallback to avoid dropping data if the generated parser fails.
    let parts = parser_tokens::clause_tokens(segment)
        .into_iter()
        .map(|(_, token, _)| ClausePart::from(token));

    build_clause_from_parts(parts)
}

#[derive(Debug)]
enum ClausePart {
    Piece(String),
    Colon,
}

impl From<Option<String>> for ClausePart {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(piece) => ClausePart::Piece(piece),
            None => ClausePart::Colon,
        }
    }
}

impl From<parser_tokens::ClauseToken> for ClausePart {
    fn from(token: parser_tokens::ClauseToken) -> Self {
        match token {
            parser_tokens::ClauseToken::Colon => ClausePart::Colon,
            parser_tokens::ClauseToken::Piece(part) => ClausePart::Piece(part),
        }
    }
}

fn build_clause_from_parts(parts: impl IntoIterator<Item = ClausePart>) -> ParsedClause {
    let mut key = String::new();
    let mut value = String::new();
    let mut seen_colon = false;

    for part in parts {
        match part {
            ClausePart::Colon => {
                if seen_colon {
                    value.push(':');
                } else {
                    seen_colon = true;
                }
            }
            ClausePart::Piece(piece) => {
                if seen_colon {
                    value.push_str(&piece);
                } else {
                    key.push_str(&piece);
                }
            }
        }
    }

    if seen_colon {
        let key_trimmed = key.trim().to_string();
        if !key_trimmed.is_empty() {
            return ParsedClause::KeyValue {
                key: key_trimmed,
                value: value.trim().to_string(),
            };
        }
        return ParsedClause::Bare(format!("{}:{}", key, value).trim().to_string());
    }

    ParsedClause::Bare(key.trim().to_string())
}

fn escape_head_segment(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        match ch {
            '\\' | '|' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

fn escape_segment(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        match ch {
            '\\' | '|' | ':' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}
