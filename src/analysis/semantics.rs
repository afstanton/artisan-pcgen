use artisan_core::domain::rules::{Effect, Prerequisite};

use crate::{ParsedClause, parsing::line_codec::{find_key_value, parse_head_key_value}};

pub(crate) fn project_semantics(
    clauses: &[ParsedClause],
    effects: &mut Vec<Effect>,
    prerequisites: &mut Vec<Prerequisite>,
) {
    for clause in clauses {
        if let ParsedClause::KeyValue { key, value } = clause {
            if key.starts_with("PRE") {
                prerequisites.push(Prerequisite {
                    kind: key.clone(),
                    expression: if value.is_empty() {
                        None
                    } else {
                        Some(value.clone())
                    },
                });
                continue;
            }

            if key == "BONUS" || key == "AUTO" || key == "DEFINE" || key == "CHOOSE" {
                effects.push(Effect {
                    kind: key.clone(),
                    target: value.clone(),
                    value: None,
                });
            }
        }
    }
}

pub(crate) fn infer_entity_type_key(head: &str, clauses: &[ParsedClause]) -> String {
    if let Some((decl_key, _)) = declared_entity(head) {
        return format!("pcgen:entity:{}", decl_key.to_ascii_lowercase());
    }

    if looks_like_pcc(head, clauses) {
        return "pcgen:entity:pcc".to_string();
    }

    if looks_like_class(clauses) {
        return "pcgen:entity:class".to_string();
    }

    if looks_like_skill(clauses) {
        return "pcgen:entity:skill".to_string();
    }

    if looks_like_spell(clauses) {
        return "pcgen:entity:spell".to_string();
    }
    if looks_like_equipment(clauses) {
        return "pcgen:entity:equipment".to_string();
    }
    if looks_like_ability(clauses) {
        return "pcgen:entity:ability".to_string();
    }
    if looks_like_feat(clauses) {
        return "pcgen:entity:feat".to_string();
    }
    if looks_like_template(clauses) {
        return "pcgen:entity:template".to_string();
    }
    if looks_like_race(clauses) {
        return "pcgen:entity:race".to_string();
    }

    if let Some(value) = find_key_value(clauses, "TYPE") {
        let normalized = value
            .split('.')
            .next()
            .unwrap_or(value.as_str())
            .trim()
            .to_ascii_lowercase()
            .replace(' ', "-");

        if let Some(entity_key) = map_type_root_to_entity_key(&normalized) {
            return entity_key.to_string();
        }

        if !normalized.is_empty() {
            return format!("pcgen:type:{normalized}");
        }
    }
    "pcgen:type:unresolved".to_string()
}

fn has_token(clauses: &[ParsedClause], key: &str) -> bool {
    clauses.iter().any(|clause| {
        matches!(
            clause,
            ParsedClause::KeyValue { key: k, .. } if k.eq_ignore_ascii_case(key)
        )
    })
}

fn head_key(head: &str) -> Option<String> {
    parse_head_key_value(head).map(|(key, _)| key.to_ascii_uppercase())
}

fn looks_like_pcc(head: &str, clauses: &[ParsedClause]) -> bool {
    let pcc_head_keys = [
        "CAMPAIGN",
        "SOURCELONG",
        "SOURCE",
        "SOURCESHORT",
        "SOURCEWEB",
        "SOURCEDATE",
        "PUBNAMELONG",
        "PUBNAMESHORT",
        "PUBNAMEWEB",
        "GAMEMODE",
        "SETTING",
        "BOOKTYPE",
        "STATUS",
        "URL",
        "ALLOWDUPES",
        "HIDETYPE",
        "FORWARDREF",
        "ISLICENSED",
        "COVER",
        "LOGO",
        "LICENSE",
    ];

    if let Some(key) = head_key(head)
        && pcc_head_keys.iter().any(|k| *k == key)
    {
        return true;
    }

    has_token(clauses, "BOOKTYPE")
        || has_token(clauses, "GAMEMODE")
        || has_token(clauses, "SETTING")
        || has_token(clauses, "URL")
        || has_token(clauses, "ALLOWDUPES")
        || has_token(clauses, "HIDETYPE")
        || has_token(clauses, "FORWARDREF")
}

fn looks_like_class(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "CAST")
        || has_token(clauses, "KNOWN")
        || has_token(clauses, "STARTSKILLPTS")
        || has_token(clauses, "SPELLTYPE")
        || has_token(clauses, "SPECIALTYKNOWN")
        || has_token(clauses, "SUBCLASSLEVEL")
        || has_token(clauses, "SUBSTITUTIONCLASS")
        || has_token(clauses, "SUBSTITUTIONLEVEL")
        || has_token(clauses, "PROHIBITCOST")
}

fn looks_like_skill(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "USEUNTRAINED") || has_token(clauses, "SITUATION")
}

fn looks_like_spell(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "SCHOOL")
        || has_token(clauses, "COMPS")
        || has_token(clauses, "CT")
        || has_token(clauses, "SAVEINFO")
        || has_token(clauses, "SPELLRES")
        || has_token(clauses, "TARGETAREA")
        || has_token(clauses, "SPELLPOINTCOST")
        || has_token(clauses, "PPCOST")
}

fn looks_like_equipment(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "WT")
        || has_token(clauses, "WIELD")
        || has_token(clauses, "PROFICIENCY")
        || has_token(clauses, "SPROP")
        || has_token(clauses, "QUALITY")
        || has_token(clauses, "REACH")
        || has_token(clauses, "ALTCRITMULT")
        || has_token(clauses, "SPELLFAILURE")
}

fn looks_like_ability(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "CATEGORY")
        && (has_token(clauses, "ADDSPELLLEVEL")
            || has_token(clauses, "SPELLS")
            || has_token(clauses, "EQMOD")
            || has_token(clauses, "BENEFIT")
            || has_token(clauses, "STACK")
            || has_token(clauses, "MULT"))
}

fn looks_like_feat(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "MODIFYFEATCHOICE")
}

fn looks_like_template(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "ADDLEVEL") || has_token(clauses, "REPEATLEVEL")
}

fn looks_like_race(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "MONSTERCLASS") || has_token(clauses, "STARTFEATS")
}

fn map_type_root_to_entity_key(root: &str) -> Option<&'static str> {
    match root {
        "spell" => Some("pcgen:entity:spell"),
        "feat" => Some("pcgen:entity:feat"),
        "race" => Some("pcgen:entity:race"),
        "template" => Some("pcgen:entity:template"),
        "ability" => Some("pcgen:entity:ability"),
        "class" => Some("pcgen:entity:class"),
        "skill" => Some("pcgen:entity:skill"),
        "equipment" | "gear" | "item" | "weapon" | "armor" | "shield" => {
            Some("pcgen:entity:equipment")
        }
        _ => None,
    }
}

pub(crate) fn derive_entity_name(head: &str, clauses: &[ParsedClause]) -> Option<String> {
    if looks_like_ability(clauses)
        && let Some(key_value) = find_key_value(clauses, "KEY")
    {
        return Some(key_value);
    }

    let (decl_key, decl_value) = declared_entity(head)?;

    match decl_key.as_str() {
        "ABILITY" => {
            if let Some(key_value) = find_key_value(clauses, "KEY") {
                return Some(key_value);
            }
            if let Some(ParsedClause::Bare(value)) = clauses.first() {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
            Some(decl_value)
        }
        _ => Some(decl_value),
    }
}

pub(crate) fn declared_entity(head: &str) -> Option<(String, String)> {
    let (key, value) = parse_head_key_value(head)?;
    let key_upper = key.to_ascii_uppercase();

    match key_upper.as_str() {
        "ABILITY" | "SKILL" | "GEAR" | "CLASS" | "STARTPACK" | "ABILITYCATEGORY" => {
            Some((key_upper, value))
        }
        _ => None,
    }
}
