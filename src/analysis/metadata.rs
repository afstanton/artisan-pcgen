use crate::{ParsedClause, ParsedLine, parsing::line_codec::parse_head_key_value};

#[derive(Default)]
pub(crate) struct PcgenMetadata {
    pub(crate) campaign: Option<String>,
    pub(crate) source_title: Option<String>,
    pub(crate) source_short: Option<String>,
    pub(crate) source_web: Option<String>,
    pub(crate) source_date: Option<String>,
    pub(crate) publisher_long: Option<String>,
    pub(crate) publisher_short: Option<String>,
    pub(crate) game_mode: Option<String>,
    pub(crate) setting: Option<String>,
    pub(crate) book_type: Option<String>,
}

pub(crate) fn collect_metadata(parsed_line: &ParsedLine, raw_line: &str, metadata: &mut PcgenMetadata) {
    for (key, value) in extract_metadata_pairs(raw_line) {
        match key.as_str() {
            "CAMPAIGN" if metadata.campaign.is_none() => metadata.campaign = Some(value),
            "SOURCELONG" | "SOURCE" if metadata.source_title.is_none() => {
                metadata.source_title = Some(value)
            }
            "SOURCESHORT" if metadata.source_short.is_none() => metadata.source_short = Some(value),
            "SOURCEWEB" if metadata.source_web.is_none() => metadata.source_web = Some(value),
            "SOURCEDATE" if metadata.source_date.is_none() => metadata.source_date = Some(value),
            "PUBNAMELONG" | "PUBLISHER" | "PUBLISHERNAME" if metadata.publisher_long.is_none() => {
                metadata.publisher_long = Some(value)
            }
            "PUBNAMESHORT" if metadata.publisher_short.is_none() => {
                metadata.publisher_short = Some(value)
            }
            "GAMEMODE" if metadata.game_mode.is_none() => metadata.game_mode = Some(value),
            "SETTING" if metadata.setting.is_none() => metadata.setting = Some(value),
            "BOOKTYPE" if metadata.book_type.is_none() => metadata.book_type = Some(value),
            _ => {}
        }
    }

    if metadata.campaign.is_none()
        && let Some((key, value)) = parse_head_key_value(&parsed_line.head)
        && key.eq_ignore_ascii_case("CAMPAIGN")
    {
        metadata.campaign = Some(value);
    }

    for clause in &parsed_line.clauses {
        if let ParsedClause::KeyValue { key, value } = clause {
            let key_upper = key.to_ascii_uppercase();
            if metadata.source_title.is_none() && (key_upper == "SOURCELONG" || key_upper == "SOURCE") {
                metadata.source_title = Some(value.clone());
            }
            if metadata.source_short.is_none() && key_upper == "SOURCESHORT" {
                metadata.source_short = Some(value.clone());
            }
            if metadata.source_web.is_none() && key_upper == "SOURCEWEB" {
                metadata.source_web = Some(value.clone());
            }
            if metadata.source_date.is_none() && key_upper == "SOURCEDATE" {
                metadata.source_date = Some(value.clone());
            }
            if metadata.publisher_long.is_none()
                && (key_upper == "PUBNAMELONG" || key_upper == "PUBLISHER" || key_upper == "PUBLISHERNAME")
            {
                metadata.publisher_long = Some(value.clone());
            }
            if metadata.publisher_short.is_none() && key_upper == "PUBNAMESHORT" {
                metadata.publisher_short = Some(value.clone());
            }
            if metadata.game_mode.is_none() && key_upper == "GAMEMODE" {
                metadata.game_mode = Some(value.clone());
            }
            if metadata.setting.is_none() && key_upper == "SETTING" {
                metadata.setting = Some(value.clone());
            }
            if metadata.book_type.is_none() && key_upper == "BOOKTYPE" {
                metadata.book_type = Some(value.clone());
            }
        }
    }
}

fn extract_metadata_pairs(line: &str) -> Vec<(String, String)> {
    let keys = [
        "CAMPAIGN",
        "SOURCELONG",
        "SOURCE",
        "SOURCESHORT",
        "SOURCEWEB",
        "SOURCEDATE",
        "PUBNAMELONG",
        "PUBNAMESHORT",
        "PUBLISHER",
        "PUBLISHERNAME",
        "GAMEMODE",
        "BOOKTYPE",
        "SETTING",
    ];

    let mut marks: Vec<(usize, &'static str)> = Vec::new();
    for key in keys {
        let needle = format!("{key}:");
        let mut cursor = 0usize;
        while let Some(pos) = line[cursor..].find(&needle) {
            let start = cursor + pos;
            marks.push((start, key));
            cursor = start + needle.len();
            if cursor >= line.len() {
                break;
            }
        }
    }

    marks.sort_by_key(|(start, _)| *start);
    marks.dedup_by_key(|(start, _)| *start);

    let mut out = Vec::new();
    for (idx, (start, key)) in marks.iter().enumerate() {
        let value_start = start + key.len() + 1;
        let value_end = marks
            .get(idx + 1)
            .map(|(next, _)| *next)
            .unwrap_or(line.len());
        if value_start > line.len() || value_start > value_end {
            continue;
        }
        let value = line[value_start..value_end].trim();
        if !value.is_empty() {
            out.push(((*key).to_string(), value.to_string()));
        }
    }

    out
}
