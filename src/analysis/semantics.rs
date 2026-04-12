use artisan_core::domain::rules::{Effect, Prerequisite};

use crate::{
    ParsedClause,
    parsing::line_codec::{find_key_value, parse_head_key_value},
};

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

            // --- Numerically-structured bonus effects ---
            // BONUS and TEMPBONUS have a structured format that is split here so that
            // canonical consumers can filter by bonus domain without parsing raw strings.
            //
            //   BONUS:BonusType|SubType(s)|Formula[|TYPE=...][|prereq...]
            //     → target = "BonusType|SubType(s)"
            //     → value  = Some("Formula[|TYPE=...][|...]")
            //
            //   TEMPBONUS:Target|BonusType|SubType|Formula[|...]
            //     → target = "Target|BonusType|SubType"
            //     → value  = Some("Formula[|...]")
            //
            // Emit reconstructs `KIND:target|value` so the original PCGen text is preserved.
            if key == "BONUS" || key == "TEMPBONUS" {
                let split_at = if key == "TEMPBONUS" { 3 } else { 2 };
                let (bonus_target, bonus_value) = split_at_pipe(value, split_at);
                effects.push(Effect {
                    kind: key.clone(),
                    target: bonus_target,
                    value: bonus_value,
                });
                continue;
            }

            // --- Variable definition effects ---
            // DEFINE:VarName|InitialValue
            //   → target = "VarName"   (the variable being defined)
            //   → value  = Some("InitialValue")   (starting formula)
            //
            // DEFINESTAT:StatName|Formula
            //   → target = "StatName"
            //   → value  = Some("Formula")
            //
            // MODIFY:VarName|Operation|Formula
            //   → target = "VarName"   (the variable being modified)
            //   → value  = Some("Operation|Formula")
            if key == "DEFINE" || key == "DEFINESTAT" {
                let (var_name, init) = split_at_pipe(value, 1);
                effects.push(Effect {
                    kind: key.clone(),
                    target: var_name,
                    value: init,
                });
                continue;
            }

            if key == "MODIFY" {
                let (var_name, op_and_formula) = split_at_pipe(value, 1);
                effects.push(Effect {
                    kind: key.clone(),
                    target: var_name,
                    value: op_and_formula,
                });
                continue;
            }

            if key == "ADD"
                || key == "AUTO"
                || key == "CHOOSE"
                || key == "SELECT"
                // TEMPLATE:Name applies a template to the bearer — a game-mechanical effect.
                // Note: ability/feat schemas declare TEMPLATE as ArtisanMapping::Effect, so
                // this entry is required for correct emit on those entity types.
                || key == "TEMPLATE"
                // ABILITY:Category|AUTO|Name grants an ability — a game-mechanical effect.
                // The emit path reads from pcgen_abilities (attribute), so there is no
                // double-emission risk.
                || key == "ABILITY"
                // LANGBONUS:Elvish|Dwarven grants bonus language choices — a canonical grant.
                // Emit path uses GlobalGroup::LangBonus → pcgen_langbonus attribute.
                || key == "LANGBONUS"
                // GRANT:Category|Value is a generic grant (e.g. GRANT:MOVEMENT|Walk).
                // Emit path uses the race schema token def → pcgen_grant attribute.
                || key == "GRANT"
                // --- Physical / combat grants (template and race entities) ---
                // Each of these contributes something to the character's mechanical state
                // when the entity is applied.  Emit paths all use schema Field → pcgen_*
                // attributes, so there is no double-emission risk.
                //
                // VISION:Darkvision (60') — grants/changes vision types.
                || key == "VISION"
                // MOVE:Walk,30,Fly,60 — grants/changes movement modes.
                || key == "MOVE"
                // NATURALATTACKS:Bite,Weapon.Natural... — grants a natural attack.
                || key == "NATURALATTACKS"
                // DR:10/Magic — grants damage reduction.
                || key == "DR"
                // SR:15 — grants spell resistance.
                || key == "SR"
                // --- Character-build grants ---
                // FEAT:Power Attack — grants a feat (when used as a clause, not as a head token).
                || key == "FEAT"
                // FOLLOWERS:Animal Companion|1 — grants follower limit.
                || key == "FOLLOWERS"
                // BONUSSKILLPOINTS:2 — grants bonus skill points per level.
                || key == "BONUSSKILLPOINTS"
                // ADDLEVEL:Fighter|1 — adds a level of the named class.
                || key == "ADDLEVEL"
                // SPELLS:book|TIMES=n|CL=n|spell,DC — grants spell-like ability use.
                || key == "SPELLS"
                // GENDERLOCK:Male — template effect that restricts the character's gender.
                || key == "GENDERLOCK"
                // MOVECLONE:Walk,Swim,/2 — clones a movement mode as a new mode.
                || key == "MOVECLONE"
                // STARTFEATS:2 — grants N additional feats at character creation (race/template).
                || key == "STARTFEATS"
                // WEAPONBONUS:Simple|Martial — grants weapon type proficiency (class/race).
                || key == "WEAPONBONUS"
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

/// Split a pipe-delimited value at the Nth pipe, returning (before, after).
///
/// If the value has fewer than `n` pipes, the entire value is returned as the
/// first element and `None` as the second.
///
/// Examples:
/// ```ignore
/// // BONUS: split at 2nd pipe → domain | formula
/// assert_eq!(split_at_pipe("COMBAT|BASEAB|1", 2), ("COMBAT|BASEAB".into(), Some("1".into())));
/// // DEFINE: split at 1st pipe → varname | initial
/// assert_eq!(split_at_pipe("PsionLevel|classlevel", 1), ("PsionLevel".into(), Some("classlevel".into())));
/// // Too few pipes → entire string in target, None in value
/// assert_eq!(split_at_pipe("NoPipes", 1), ("NoPipes".into(), None));
/// ```
fn split_at_pipe(value: &str, n: usize) -> (String, Option<String>) {
    let mut pipe_count = 0usize;
    for (i, ch) in value.char_indices() {
        if ch == '|' {
            pipe_count += 1;
            if pipe_count == n {
                return (value[..i].to_string(), Some(value[i + 1..].to_string()));
            }
        }
    }
    // Fewer than n pipes — store everything in target.
    (value.to_string(), None)
}

pub(crate) fn infer_entity_type_key(head: &str, clauses: &[ParsedClause]) -> String {
    infer_entity_type_key_for_format(head, clauses, "lst")
}

fn infer_entity_type_key_without_format(head: &str, clauses: &[ParsedClause]) -> String {
    if let Some((head_key, head_value)) = parse_head_key_value(head) {
        if head_key.eq_ignore_ascii_case("ABILITY")
            && looks_like_ability_migration(&head_value, clauses)
        {
            return "pcgen:entity:ability-migration".to_string();
        }

        if let Some(schema) = crate::schema::schema_for_head_token(&head_key) {
            // Skip the migration schema here: migration entities have already
            // been detected above via `looks_like_ability_migration`. If we
            // reach this point the line is NOT a migration record, so don't
            // misclassify it as one just because the migration schema happens
            // to register the "ABILITY" head token.
            if schema.entity_type_key != "pcgen:entity:ability-migration" {
                return schema.entity_type_key.to_string();
            }
        }
    }

    if head
        .trim_start()
        .to_ascii_uppercase()
        .starts_with("CATEGORY=")
    {
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

pub(crate) fn infer_entity_type_key_for_format(
    head: &str,
    clauses: &[ParsedClause],
    source_format: &str,
) -> String {
    if let Some((head_key, _)) = parse_head_key_value(head) {
        let head_key_upper = head_key.to_ascii_uppercase();

        if source_format.eq_ignore_ascii_case("pcg") {
            match head_key_upper.as_str() {
                "CLASS" if looks_like_pcg_class(clauses) => return "pcgen:pcg:class".to_string(),
                "SKILL" if looks_like_pcg_skill(clauses) => return "pcgen:pcg:skill".to_string(),
                _ => {}
            }
        } else {
            match head_key_upper.as_str() {
                "CLASS" => return "pcgen:entity:class".to_string(),
                "SKILL" => return "pcgen:entity:skill".to_string(),
                _ => {}
            }
        }
    }

    infer_entity_type_key_without_format(head, clauses)
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
        "GENRE",
        "ISOGL",
        "INFOTEXT",
        "EQUIPMENT",
        "SPELL",
        "RANK",
        "FACTDEF",
        "STATUS",
        "OPTION",
        "DATAFORMAT",
        "DISPLAYNAME",
        "EXPLANATION",
        "REQUIRED",
        "SELECTABLE",
        "ISMATURE",
        "NAMEISPI",
        "DESCISPI",
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
        "VARIABLE",
        "LICENSE",
        "PCC",
        "MAXDEVVER",
        "HELP",
    ];

    if let Some(key) = head_key(head)
        && pcc_head_keys.iter().any(|k| *k == key)
    {
        return true;
    }

    has_token(clauses, "BOOKTYPE")
        || has_token(clauses, "GENRE")
        || has_token(clauses, "ISOGL")
        || has_token(clauses, "INFOTEXT")
        || has_token(clauses, "EQUIPMENT")
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
        || has_token(clauses, "DESCISPI")
        || has_token(clauses, "MAXVER")
        || has_token(clauses, "NEWKEY")
        || has_token(clauses, "MAXDEVVER")
        || has_token(clauses, "ALLOWDUPES")
        || has_token(clauses, "HIDETYPE")
        || has_token(clauses, "FORWARDREF")
        || has_token(clauses, "PCC")
        || has_token(clauses, "COPYRIGHT")
}

fn looks_like_ability_migration(head_value: &str, clauses: &[ParsedClause]) -> bool {
    head_value.contains('|')
        && (has_token(clauses, "MAXVER")
            || has_token(clauses, "MAXDEVVER")
            || has_token(clauses, "NEWKEY")
            || has_token(clauses, "NEWCATEGORY"))
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
        || has_token(clauses, "EXCLASS")
        || has_token(clauses, "EXCHANGELEVEL")
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
        || (has_token(clauses, "ALIGN") && has_token(clauses, "DOMAINS"))
}

fn looks_like_skill(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "USEUNTRAINED")
        || has_token(clauses, "SITUATION")
        || has_token(clauses, "EXCLUSIVE")
}

fn looks_like_pcg_class(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "LEVEL")
        || has_token(clauses, "SKILLPOOL")
        || has_token(clauses, "SPELLBASE")
        || has_token(clauses, "CANCASTPERDAY")
}

fn looks_like_pcg_skill(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "OUTPUTORDER")
        || has_token(clauses, "CLASSBOUGHT")
        || has_token(clauses, "RANKS")
        || has_token(clauses, "CLASSSKILL")
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
        || has_token(clauses, "PLUS")
        || has_token(clauses, "FORMATCAT")
        || has_token(clauses, "ASSIGNTOALL")
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
        || has_token(clauses, "REGION")
        || has_token(clauses, "SR")
        || has_token(clauses, "!PREMOVE")
        || has_token(clauses, "!PREVISION")
        || has_token(clauses, "PRESRLT")
        || has_token(clauses, "!PREKIT")
        || has_token(clauses, "NONPP")
}

fn looks_like_race(clauses: &[ParsedClause]) -> bool {
    has_token(clauses, "MONSTERCLASS")
        || has_token(clauses, "STARTFEATS")
        || has_token(clauses, "SKILLMULT")
        || has_token(clauses, "RACETYPE")
        || has_token(clauses, "GRANT")
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
        "ABILITY" | "SKILL" | "GEAR" | "CLASS" | "SUBCLASS" | "STARTPACK" | "ABILITYCATEGORY" => {
            Some((key_upper, value))
        }
        _ if crate::schema::schema_for_head_token(&key_upper).is_some() => Some((key_upper, value)),
        _ => None,
    }
}
