use crate::ParsedClause;

pub(crate) fn classify_clause_token(clause: &ParsedClause) -> ClauseSupportLevel {
    match clause {
        ParsedClause::KeyValue { key, .. } => {
            classify_token_key(key, false)
        }
        ParsedClause::Bare(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                ClauseSupportLevel::PolicySupported
            } else if is_known_bare_directive(trimmed) {
                ClauseSupportLevel::PolicySupported
            } else {
                ClauseSupportLevel::Artifact
            }
        }
    }
}

pub(crate) fn classify_token_key(input: &str, is_bare: bool) -> ClauseSupportLevel {
    let token = input.trim().to_ascii_uppercase();
    if !is_plausible_token_name(&token) {
        return ClauseSupportLevel::Artifact;
    }

    if !is_bare && token.starts_with("PRE") {
        return ClauseSupportLevel::SemanticallyInterpreted;
    }

    match token.as_str() {
        "TYPE"
        | "BONUS"
        | "AUTO"
        | "DEFINE"
        | "CHOOSE"
        | "SOURCEPAGE"
        | "CAMPAIGN"
        | "SOURCELONG"
        | "SOURCE"
        | "SOURCESHORT"
        | "SOURCEWEB"
        | "SOURCEDATE"
        | "PUBNAMELONG"
        | "PUBNAMESHORT"
        | "PUBLISHER"
        | "PUBLISHERNAME"
        | "GAMEMODE"
        | "SETTING"
        | "BOOKTYPE" => ClauseSupportLevel::SemanticallyInterpreted,
        "AUTOMATIC" | "VISIBLE" | "VIRTUAL" | "PRERULE" | "!PRERULE" | "SET" => {
            ClauseSupportLevel::PolicySupported
        }
        _ if is_bare && (token.starts_with("PRE") || token.starts_with("!PRE")) => {
            ClauseSupportLevel::PolicySupported
        }
        _ => ClauseSupportLevel::Unhandled(token),
    }
}

pub(crate) enum ClauseSupportLevel {
    SemanticallyInterpreted,
    PolicySupported,
    Unhandled(String),
    Artifact,
}

fn is_known_bare_directive(input: &str) -> bool {
    let upper = input.to_ascii_uppercase();
    if upper.starts_with("PRE") || upper.starts_with("!PRE") {
        return true;
    }

    matches!(
        upper.as_str(),
        "AUTOMATIC" | "VISIBLE" | "VIRTUAL" | "PRERULE" | "!PRERULE" | "SET"
    )
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

    if token.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    token.chars().any(|c| c.is_ascii_alphabetic())
}
