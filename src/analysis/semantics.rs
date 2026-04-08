use artisan_core::domain::rules::{Effect, Prerequisite};

use crate::{ParsedClause, parsing::line_codec::{find_key_value, parse_head_key_value}};

pub(crate) fn project_semantics(
    clauses: &[ParsedClause],
    effects: &mut Vec<Effect>,
    prerequisites: &mut Vec<Prerequisite>,
) {
    for clause in clauses {
        if let ParsedClause::KeyValue { key, value } = clause {
            if key.starts_with("PRE") || key.starts_with("!PRE") {
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

            if key == "BONUS"
                || key == "TEMPBONUS"
                || key == "ADD"
                || key == "AUTO"
                || key == "DEFINE"
                || key == "DEFINESTAT"
                || key == "CHOOSE"
                || key == "SELECT"
            {
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
    if let Some((head_key, _)) = parse_head_key_value(head)
        && let Some(schema) = crate::schema::schema_for_head_token(&head_key)
    {
        return schema.entity_type_key.to_string();
    }

    if head.trim_start().to_ascii_uppercase().starts_with("CATEGORY=") {
        return "pcgen:entity:ability".to_string();
    }

    if let Some((decl_key, _)) = declared_entity(head) {
        return format!("pcgen:entity:{}", decl_key.to_ascii_lowercase());
    }

    if looks_like_system_align(clauses) {
        return "pcgen:system:align".to_string();
    }

    if looks_like_system_stat(clauses) {
        return "pcgen:system:stat".to_string();
    }

    if looks_like_class_level(head, clauses) {
        return "pcgen:entity:classlevel".to_string();
    }

    if looks_like_deity(clauses) {
        return "pcgen:entity:deity".to_string();
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
    if looks_like_race(clauses) {
        return "pcgen:entity:race".to_string();
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

    if looks_like_pcc(head, clauses) {
        return "pcgen:entity:pcc".to_string();
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
        "RANK",
        "FACTDEF",
        "STATUS",
        "OPTION",
        "DATAFORMAT",
        "DISPLAYNAME",
        "EXPLANATION",
        "REQUIRED",
        "SELECTABLE",
        "NAMEISPI",
        "URL",
        "ALLOWDUPES",
        "MAXVER",
        "NEWKEY",
        "HIDETYPE",
        "FORWARDREF",
        "ISLICENSED",
        "COVER",
        "LOGO",
        "COPYRIGHT",
        "LICENSE",
    ];

    if let Some(key) = head_key(head)
        && pcc_head_keys.iter().any(|k| *k == key)
    {
        return true;
    }

    has_token(clauses, "BOOKTYPE")
        || has_token(clauses, "FACTDEF")
        || has_token(clauses, "GAMEMODE")
        || has_token(clauses, "SETTING")
        || has_token(clauses, "URL")
        || has_token(clauses, "OPTION")
        || has_token(clauses, "DATAFORMAT")
        || has_token(clauses, "DISPLAYNAME")
        || has_token(clauses, "EXPLANATION")
        || has_token(clauses, "REQUIRED")
        || has_token(clauses, "SELECTABLE")
        || has_token(clauses, "NAMEISPI")
        || has_token(clauses, "MAXVER")
        || has_token(clauses, "NEWKEY")
        || has_token(clauses, "ALLOWDUPES")
        || has_token(clauses, "HIDETYPE")
        || has_token(clauses, "FORWARDREF")
        || has_token(clauses, "COPYRIGHT")
}

fn looks_like_class(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "CAST")
        || has_token(clauses, "KNOWN")
        || has_token(clauses, "MEMORIZE")
        || has_token(clauses, "SPELLSTAT")
        || has_token(clauses, "ADDDOMAINS")
        || has_token(clauses, "DOMAIN")
        || has_token(clauses, "MAXLEVEL")
        || has_token(clauses, "STARTSKILLPTS")
        || has_token(clauses, "SPELLTYPE")
        || has_token(clauses, "SPECIALTYKNOWN")
        || has_token(clauses, "SUBCLASSLEVEL")
        || has_token(clauses, "SUBSTITUTIONCLASS")
        || has_token(clauses, "SUBSTITUTIONLEVEL")
        || has_token(clauses, "PROHIBITCOST")
}

fn looks_like_system_align(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "VALIDFORDEITY") || has_token(clauses, "VALIDFORFOLLOWER")
}

fn looks_like_system_stat(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "STATMOD")
}

fn looks_like_class_level(head: &str, clauses: &[ParsedClause]) -> bool {
    let normalized_head = head.trim();
    !normalized_head.is_empty()
        && normalized_head.chars().all(|ch| ch.is_ascii_digit())
        && (has_token(clauses, "DONOTADD")
            || has_token(clauses, "UDAM")
            || has_token(clauses, "UMULT"))
}

fn looks_like_deity(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "DEITYWEAP")
    || has_token(clauses, "GROUP")
    || (has_token(clauses, "ALIGN") && has_token(clauses, "DOMAINS"))
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
    let Some(category) = find_key_value(clauses, "CATEGORY") else {
        return false;
    };

    let category = category.trim();
    if category.eq_ignore_ascii_case("FEAT") {
        return false;
    }

    has_token(clauses, "ADDSPELLLEVEL")
        || has_token(clauses, "SPELLS")
        || has_token(clauses, "EQMOD")
        || has_token(clauses, "CSKILL")
        || has_token(clauses, "BENEFIT")
        || has_token(clauses, "STACK")
        || has_token(clauses, "MULT")
        || has_token(clauses, "BONUS")
        || has_token(clauses, "DEFINE")
        || has_token(clauses, "SAB")
        || has_token(clauses, "ABILITY")
        || has_token(clauses, "SPELLKNOWN")
        || category.eq_ignore_ascii_case("SPECIAL ABILITY")
        || category.eq_ignore_ascii_case("INTERNAL")
        || category.eq_ignore_ascii_case("TALENT")
        || category.eq_ignore_ascii_case("AFFLICTIONS")
        || category.eq_ignore_ascii_case("CAREER SKILL")
}

fn looks_like_feat(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "MODIFYFEATCHOICE")
        || find_key_value(clauses, "CATEGORY")
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("FEAT"))
}

fn looks_like_template(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "ADDLEVEL")
        || has_token(clauses, "REPEATLEVEL")
        || has_token(clauses, "GENDERLOCK")
        || has_token(clauses, "BONUSSKILLPOINTS")
        || has_token(clauses, "PREAGESET")
        || has_token(clauses, "!PREAGESET")
        || has_token(clauses, "!PREDOMAIN")
        || has_token(clauses, "!PRESPELL")
    || has_token(clauses, "MOVECLONE")
    || has_token(clauses, "MOVE")
    || has_token(clauses, "VISION")
    || has_token(clauses, "SUBRACE")
    || has_token(clauses, "REMOVABLE")
    || has_token(clauses, "SR")
    || has_token(clauses, "!PREMOVE")
    || has_token(clauses, "!PREVISION")
    || has_token(clauses, "PRESRLT")
    || has_token(clauses, "!PREKIT")
}

fn looks_like_race(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "MONSTERCLASS")
        || has_token(clauses, "STARTFEATS")
        || has_token(clauses, "MONCCSKILL")
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
        _ if crate::schema::schema_for_head_token(&key_upper).is_some() => Some((key_upper, value)),
        _ => None,
    }
}
