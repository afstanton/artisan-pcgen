use indexmap::IndexMap;
use serde_json::{Map, Value, json};

use crate::ParsedClause;
use crate::parsing::parse_modify;

pub(crate) fn project_clause_attributes(
    head_name: &str,
    clauses: &[ParsedClause],
    attributes: &mut IndexMap<String, Value>,
) {
    let mut facts = Vec::new();
        let mut factsets = Vec::new();
    let mut equipment_modifiers = Vec::new();
    let mut class_lists = Vec::new();
    let mut spell_blocks = Vec::new();
    let mut sprop_values = Vec::new();
    let mut page_usage_values = Vec::new();

    for clause in clauses {
        let ParsedClause::KeyValue { key, value } = clause else {
            continue;
        };

        match key.to_ascii_uppercase().as_str() {
            "TYPE" => {
                attributes.insert("pcgen_type".to_string(), Value::String(value.clone()));
            }
            "SOURCE" => {
                attributes.insert("pcgen_source".to_string(), Value::String(value.clone()));
            }
            "SOURCELONG" => {
                attributes.insert("pcgen_source_long".to_string(), Value::String(value.clone()));
            }
            "SOURCESHORT" => {
                attributes.insert("pcgen_source_short".to_string(), Value::String(value.clone()));
            }
            "SOURCEWEB" => {
                attributes.insert("pcgen_source_web".to_string(), Value::String(value.clone()));
            }
            "SOURCELINK" => {
                attributes.insert("pcgen_source_link".to_string(), Value::String(value.clone()));
            }
            "SOURCEDATE" => {
                attributes.insert("pcgen_source_date".to_string(), Value::String(value.clone()));
            }
            "GAMEMODE" => {
                attributes.insert("pcgen_gamemode".to_string(), Value::String(value.clone()));
            }
            "GENRE" => {
                attributes.insert("pcgen_genre".to_string(), Value::String(value.clone()));
            }
            "ISOGL" => {
                attributes.insert("pcgen_isogl".to_string(), parse_yes_no_or_string(value));
            }
            "PUBNAMESHORT" => {
                attributes.insert(
                    "pcgen_publisher_short".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SHOWINMENU" => {
                attributes.insert("pcgen_showinmenu".to_string(), parse_yes_no_or_string(value));
            }
            "SETTING" => {
                attributes.insert("pcgen_setting".to_string(), Value::String(value.clone()));
            }
            "BOOKTYPE" => {
                attributes.insert("pcgen_booktype".to_string(), Value::String(value.clone()));
            }
            "PCC" => append_string_attr(attributes, "pcgen_pcc", value),
            "CHOICE" => append_string_attr(attributes, "pcgen_choice", value),
            "BIOSET" => append_string_attr(attributes, "pcgen_bioset_catalog", value),
            "DATACONTROL" => append_string_attr(attributes, "pcgen_datacontrol_catalog", value),
            "DATATABLE" => append_string_attr(attributes, "pcgen_datatable_catalog", value),
            "COMPANIONMOD" => append_string_attr(attributes, "pcgen_companionmod_catalog", value),
            "ALIGNMENT" => append_string_attr(attributes, "pcgen_alignment_catalog", value),
            "SAVE" => append_string_attr(attributes, "pcgen_save_catalog", value),
            "PRECAMPAIGN" => {
                append_value_attr(attributes, "pcgen_precampaign", parse_pipe_series(value));
            }
            "BASESTATSCORE" => set_i64_or_string(attributes, "pcgen_basestatscore", value),
            "STATRANGE" => set_i64_or_string(attributes, "pcgen_statrange", value),
            "STATMOD" => {
                attributes.insert("pcgen_statmod".to_string(), Value::String(value.clone()));
            }
            "ADDSPELLLEVEL" => set_i64_or_string(attributes, "pcgen_addspelllevel", value),
            "MULT" => {
                attributes.insert("pcgen_mult".to_string(), Value::String(value.clone()));
            }
            "STACK" => {
                attributes.insert("pcgen_stack".to_string(), Value::String(value.clone()));
            }
            "BENEFIT" => {
                attributes.insert("pcgen_benefit".to_string(), Value::String(value.clone()));
            }
            "TEMPDESC" => {
                attributes.insert("pcgen_tempdesc".to_string(), Value::String(value.clone()));
            }
            "DESCISPI" => {
                attributes.insert("pcgen_descispi".to_string(), parse_yes_no_or_string(value));
            }
            "SPELLLEVEL" => {
                attributes.insert("pcgen_spelllevel".to_string(), Value::String(value.clone()));
            }
            "INFO" => {
                attributes.insert("pcgen_info".to_string(), Value::String(value.clone()));
            }
            "APPLY" => {
                attributes.insert("pcgen_apply".to_string(), Value::String(value.clone()));
            }
            "LOOKUP" => append_string_attr(attributes, "pcgen_lookup", value),
            "BASEAGE" => {
                attributes.insert("pcgen_baseage".to_string(), Value::String(value.clone()));
            }
            "MAXAGE" => {
                attributes.insert("pcgen_maxage".to_string(), Value::String(value.clone()));
            }
            "BASEAGEADD" => {
                attributes.insert("pcgen_baseageadd".to_string(), Value::String(value.clone()));
            }
            "AGEDIEROLL" => {
                attributes.insert("pcgen_agedieroll".to_string(), Value::String(value.clone()));
            }
            "SEX" => append_string_attr(attributes, "pcgen_sex", value),
            "HAIR" => {
                attributes.insert("pcgen_hair".to_string(), Value::String(value.clone()));
            }
            "EYES" => {
                attributes.insert("pcgen_eyes".to_string(), Value::String(value.clone()));
            }
            "SKINTONE" => {
                attributes.insert("pcgen_skintone".to_string(), Value::String(value.clone()));
            }
            "ASPECT" => append_string_attr(attributes, "pcgen_aspects", value),
            "RACETYPE" => {
                attributes.insert("pcgen_racetype".to_string(), Value::String(value.clone()));
            }
            "RACESUBTYPE" => append_string_attr(attributes, "pcgen_racesubtype", value),
            "SUBRACE" => {
                attributes.insert("pcgen_subrace".to_string(), Value::String(value.clone()));
            }
            "SUBCLASS" => append_string_attr(attributes, "pcgen_subclass", value),
            "FAVCLASS" => {
                attributes.insert("pcgen_favclass".to_string(), Value::String(value.clone()));
            }
            "FAVOREDCLASS" => {
                attributes.insert("pcgen_favoredclass".to_string(), Value::String(value.clone()));
            }
            "MONSTERCLASS" => {
                attributes.insert("pcgen_monsterclass".to_string(), Value::String(value.clone()));
            }
            "MONCSKILL" => append_string_attr(attributes, "pcgen_moncskill", value),
            "CCSKILL" => append_string_attr(attributes, "pcgen_ccskill", value),
            "MONCCSKILL" => append_string_attr(attributes, "pcgen_monccskill", value),
            "SKILLMULT" => append_string_attr(attributes, "pcgen_skillmult", value),
            "STARTFEATS" => set_i64_or_string(attributes, "pcgen_startfeats", value),
            "HITDICEADVANCEMENT" => {
                attributes.insert(
                    "pcgen_hitdiceadvancement".to_string(),
                    Value::String(value.clone()),
                );
            }
            "LEVELADJUSTMENT" => set_i64_or_string(attributes, "pcgen_leveladjustment", value),
            "XTRASKILLPTSPERLVL" => {
                set_i64_or_string(attributes, "pcgen_xtraskillptsperlvl", value)
            }
            "GENDERLOCK" => {
                attributes.insert("pcgen_genderlock".to_string(), Value::String(value.clone()));
            }
            "BONUSSKILLPOINTS" => set_i64_or_string(attributes, "pcgen_bonusskillpoints", value),
            "ADDLEVEL" => {
                attributes.insert("pcgen_addlevel".to_string(), Value::String(value.clone()));
            }
            "REPEATLEVEL" => append_string_attr(attributes, "pcgen_repeatlevel", value),
            "MOVECLONE" => append_string_attr(attributes, "pcgen_moveclone", value),
            "HD" => set_i64_or_string(attributes, "pcgen_hitdie", value),
            "HITDIE" => set_i64_or_string(attributes, "pcgen_hitdie", value),
            "MAXLEVEL" => {
                attributes.insert("pcgen_maxlevel".to_string(), Value::String(value.clone()));
            }
            "ABB" => {
                attributes.insert("pcgen_abbreviation".to_string(), Value::String(value.clone()));
            }
            "STARTSKILLPTS" => set_i64_or_string(attributes, "pcgen_startskillpts", value),
            "LEVELSPERFEAT" => set_i64_or_string(attributes, "pcgen_levelsperfeat", value),
            "ATTACKCYCLE" => {
                attributes.insert("pcgen_attackcycle".to_string(), Value::String(value.clone()));
            }
            "SPELLTYPE" => {
                attributes.insert("pcgen_spelltype".to_string(), Value::String(value.clone()));
            }
            "SPELLSTAT" => {
                attributes.insert("pcgen_spellstat".to_string(), Value::String(value.clone()));
            }
            "ITEMCREATE" => {
                attributes.insert("pcgen_itemcreate".to_string(), Value::String(value.clone()));
            }
            "BONUSSPELLSTAT" => {
                attributes.insert("pcgen_bonusspellstat".to_string(), Value::String(value.clone()));
            }
            "SPELLBOOK" => {
                attributes.insert("pcgen_spellbook".to_string(), parse_yes_no_or_string(value));
            }
            "MEMORIZE" => {
                attributes.insert("pcgen_memorize".to_string(), parse_yes_no_or_string(value));
            }
            "CAST" => append_string_attr(attributes, "pcgen_cast", value),
            "KNOWN" => append_string_attr(attributes, "pcgen_known", value),
            "KNOWNSPELLS" => {
                attributes.insert("pcgen_knownspells".to_string(), Value::String(value.clone()));
            }
            "SPECIALTYKNOWN" => {
                attributes.insert("pcgen_specialtyknown".to_string(), Value::String(value.clone()));
            }
            "SPELLLIST" => {
                attributes.insert("pcgen_spelllist".to_string(), Value::String(value.clone()));
            }
            "MAXCOST" => set_i64_or_string(attributes, "pcgen_maxcost", value),
            "PROHIBITSPELL" => append_string_attr(attributes, "pcgen_prohibitspell", value),
            "KNOWNSPELLSFROMSPECIALTY" => {
                attributes.insert(
                    "pcgen_knownspellsfromspecialty".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ADDDOMAINS" => append_string_attr(attributes, "pcgen_adddomains", value),
            "DOMAIN" => {
                attributes.insert("pcgen_domains".to_string(), Value::String(value.clone()));
            }
            "ALIGN" => {
                attributes.insert("pcgen_align".to_string(), Value::String(value.clone()));
            }
            "DEITYWEAP" => {
                attributes.insert("pcgen_deityweap".to_string(), Value::String(value.clone()));
            }
            "PANTHEON" => {
                attributes.insert("pcgen_pantheon".to_string(), Value::String(value.clone()));
            }
            "GROUP" => append_string_attr(attributes, "pcgen_group", value),
            "ALLOWBASECLASS" => {
                attributes.insert("pcgen_allowbaseclass".to_string(), parse_yes_no_or_string(value));
            }
            "MODTOSKILLS" => {
                attributes.insert("pcgen_modtoskills".to_string(), parse_yes_no_or_string(value));
            }
            "MONSKILL" => set_i64_or_string(attributes, "pcgen_monskill", value),
            "MONNONSKILLHD" => set_i64_or_string(attributes, "pcgen_monnonskillhd", value),
            "WEAPONBONUS" => append_string_attr(attributes, "pcgen_weaponbonus", value),
            "VISIBLE" => {
                attributes.insert("pcgen_visible".to_string(), Value::String(value.clone()));
            }
            "UDAM" => {
                attributes.insert("pcgen_udam".to_string(), Value::String(value.clone()));
            }
            "UMULT" => set_i64_or_string(attributes, "pcgen_umult", value),
            "DONOTADD" => append_string_attr(attributes, "pcgen_donotadd", value),
            "COMPANIONLIST" => append_string_attr(attributes, "pcgen_companionlist", value),
            "FOLLOWERS" => append_string_attr(attributes, "pcgen_followers", value),
            "REMOVABLE" => {
                attributes.insert("pcgen_removable".to_string(), Value::String(value.clone()));
            }
            "PROHIBITCOST" => set_i64_or_string(attributes, "pcgen_prohibitcost", value),
            "HASSUBCLASS" => {
                attributes.insert("pcgen_hassubclass".to_string(), parse_yes_no_or_string(value));
            }
            "SUBCLASSLEVEL" => {
                attributes.insert("pcgen_subclasslevel".to_string(), Value::String(value.clone()));
            }
            "SUBSTITUTIONCLASS" => {
                attributes.insert(
                    "pcgen_substitutionclass".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SUBSTITUTIONLEVEL" => {
                attributes.insert(
                    "pcgen_substitutionlevel".to_string(),
                    Value::String(value.clone()),
                );
            }
            "EXCLASS" => append_string_attr(attributes, "pcgen_exclass", value),
            "EXCHANGELEVEL" => append_string_attr(attributes, "pcgen_exchangelevel", value),
            "CATEGORY" => {
                attributes.insert("pcgen_category".to_string(), Value::String(value.clone()));
            }
            "DESC" => {
                attributes.insert("pcgen_desc".to_string(), Value::String(value.clone()));
                attributes
                    .entry("description".to_string())
                    .or_insert_with(|| Value::String(value.clone()));
            }
            "KEY" => {
                attributes.insert("pcgen_key".to_string(), Value::String(value.clone()));
            }
            "LANGBONUS" => append_string_attr(attributes, "pcgen_langbonus", value),
            "CSKILL" => append_string_attr(attributes, "pcgen_cskill", value),
            "SAB" => append_string_attr(attributes, "pcgen_sab", value),
            "CHANGEPROF" => append_string_attr(attributes, "pcgen_changeprof", value),
            "SERVESAS" => append_string_attr(attributes, "pcgen_servesas", value),
            "QUALIFY" => append_string_attr(attributes, "pcgen_qualify", value),
            "TEMPLATE" => append_string_attr(attributes, "pcgen_template", value),
            "OUTPUTNAME" => {
                attributes.insert("pcgen_outputname".to_string(), Value::String(value.clone()));
            }
            "RANK" => {
                if let Ok(rank) = value.trim().parse::<i64>() {
                    attributes.insert("pcgen_rank".to_string(), json!(rank));
                } else {
                    attributes.insert("pcgen_rank".to_string(), Value::String(value.clone()));
                }
            }
            "COST" => {
                attributes.insert("pcgen_cost".to_string(), Value::String(value.clone()));
            }
            "COSTPRE" => {
                attributes.insert("pcgen_costpre".to_string(), Value::String(value.clone()));
            }
            "BASEITEM" => {
                attributes.insert("pcgen_baseitem".to_string(), Value::String(value.clone()));
            }
            "WT" => {
                attributes.insert("pcgen_weight".to_string(), Value::String(value.clone()));
            }
            "RANGE" => {
                attributes.insert("pcgen_range".to_string(), Value::String(value.clone()));
            }
            "SCHOOL" => {
                attributes.insert("pcgen_school".to_string(), Value::String(value.clone()));
            }
            "SUBSCHOOL" => {
                attributes.insert("pcgen_subschool".to_string(), Value::String(value.clone()));
            }
            "COMPS" => {
                attributes.insert("pcgen_comps".to_string(), Value::String(value.clone()));
            }
            "CT" => {
                set_i64_or_string(attributes, "pcgen_ct", value);
            }
            "CASTTIME" => {
                attributes.insert("pcgen_casttime".to_string(), Value::String(value.clone()));
            }
            "SORTKEY" => {
                attributes.insert("pcgen_sortkey".to_string(), Value::String(value.clone()));
            }
            "VALIDFORDEITY" => {
                attributes.insert("pcgen_validfordeity".to_string(), Value::String(value.clone()));
            }
            "VALIDFORFOLLOWER" => {
                attributes.insert(
                    "pcgen_validforfollower".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SIZENUM" => {
                attributes.insert("pcgen_sizenum".to_string(), Value::String(value.clone()));
            }
            "ISDEFAULTSIZE" => {
                attributes.insert(
                    "pcgen_isdefaultsize".to_string(),
                    Value::String(value.clone()),
                );
            }
            "TARGETAREA" => {
                attributes.insert("pcgen_targetarea".to_string(), Value::String(value.clone()));
            }
            "DURATION" => {
                attributes.insert("pcgen_duration".to_string(), Value::String(value.clone()));
            }
            "SAVEINFO" => {
                attributes.insert("pcgen_saveinfo".to_string(), Value::String(value.clone()));
            }
            "SPELLRES" => {
                attributes.insert("pcgen_spellres".to_string(), Value::String(value.clone()));
            }
            "TEMPVALUE" => append_string_attr(attributes, "pcgen_tempvalue", value),
            "XPCOST" => {
                attributes.insert("pcgen_xpcost".to_string(), Value::String(value.clone()));
            }
            "DESCRIPTOR" => append_string_attr(attributes, "pcgen_descriptors", value),
            "DOMAINS" => {
                attributes.insert("pcgen_domains".to_string(), Value::String(value.clone()));
            }
            "PPCOST" => set_i64_or_string(attributes, "pcgen_ppcost", value),
            "SPELLPOINTCOST" => {
                attributes.insert("pcgen_spellpointcost".to_string(), Value::String(value.clone()));
            }
            "ITEM" => append_string_attr(attributes, "pcgen_items", value),
            "GEAR" => append_string_attr(attributes, "pcgen_gear", value),
            "KIT" => append_string_attr(attributes, "pcgen_kits", value),
            "ABILITY" => append_string_attr(attributes, "pcgen_abilities", value),
            "ABILITYCATEGORY" => append_string_attr(attributes, "pcgen_abilitycategories", value),
            "FEAT" => append_string_attr(attributes, "pcgen_feats", value),
            "EQUIPMENT" => append_string_attr(attributes, "pcgen_equipment", value),
            "SPELL" => append_string_attr(attributes, "pcgen_spells", value),
            "LICENSE" => append_string_attr(attributes, "pcgen_license", value),
            "INFOTEXT" => append_string_attr(attributes, "pcgen_infotext", value),
            "FORWARDREF" => append_string_attr(attributes, "pcgen_forwardref", value),
            "HIDETYPE" => append_string_attr(attributes, "pcgen_hidetype", value),
            "URL" => append_string_attr(attributes, "pcgen_url", value),
            "OPTION" => {
                attributes.insert("pcgen_option".to_string(), Value::String(value.clone()));
            }
            "MAXVER" => {
                attributes.insert("pcgen_maxver".to_string(), Value::String(value.clone()));
            }
            "MAXDEVVER" => {
                attributes.insert("pcgen_maxdevver".to_string(), Value::String(value.clone()));
            }
            "NEWKEY" => {
                attributes.insert("pcgen_newkey".to_string(), Value::String(value.clone()));
            }
            "NEWCATEGORY" => {
                attributes.insert("pcgen_newcategory".to_string(), Value::String(value.clone()));
            }
            "VALUES" => append_string_attr(attributes, "pcgen_values", value),
            "COPYRIGHT" => append_string_attr(attributes, "pcgen_copyright", value),
            "LSTEXCLUDE" => {
                append_value_attr(attributes, "pcgen_lstexclude", parse_pipe_series(value));
            }
            "FACTDEF" => {
                attributes.insert("pcgen_factdef".to_string(), Value::String(value.clone()));
            }
            "VALUE" => {
                attributes.insert("pcgen_value".to_string(), Value::String(value.clone()));
            }
            "DATAFORMAT" => {
                attributes.insert("pcgen_dataformat".to_string(), Value::String(value.clone()));
            }
            "DISPLAYNAME" => {
                attributes.insert("pcgen_displayname".to_string(), Value::String(value.clone()));
            }
            "DEFAULTDATASET" => {
                attributes.insert(
                    "pcgen_defaultdataset".to_string(),
                    Value::String(value.clone()),
                );
            }
            "EXPLANATION" => {
                attributes.insert("pcgen_explanation".to_string(), Value::String(value.clone()));
            }
            "REQUIRED" => {
                attributes.insert("pcgen_required".to_string(), Value::String(value.clone()));
            }
            "SELECTABLE" => {
                attributes.insert("pcgen_selectable".to_string(), Value::String(value.clone()));
            }
            "NAMEISPI" => {
                attributes.insert("pcgen_nameispi".to_string(), Value::String(value.clone()));
            }
            "ABILITYLIST" => append_string_attr(attributes, "pcgen_abilitylist", value),
            "DISPLAYLOCATION" => {
                attributes.insert(
                    "pcgen_displaylocation".to_string(),
                    Value::String(value.clone()),
                );
            }
            "EDITABLE" => {
                attributes.insert("pcgen_editable".to_string(), Value::String(value.clone()));
            }
            "EDITPOOL" => {
                attributes.insert("pcgen_editpool".to_string(), Value::String(value.clone()));
            }
            "FRACTIONALPOOL" => {
                attributes.insert(
                    "pcgen_fractionalpool".to_string(),
                    Value::String(value.clone()),
                );
            }
            "GAMEMODEKEY" => {
                attributes.insert("pcgen_gamemodekey".to_string(), Value::String(value.clone()));
            }
            "PLURAL" => {
                attributes.insert("pcgen_plural".to_string(), Value::String(value.clone()));
            }
            "POOL" => {
                attributes.insert("pcgen_pool".to_string(), Value::String(value.clone()));
            }
            "PARM" => {
                attributes.insert("pcgen_parm".to_string(), Value::String(value.clone()));
            }
            "VAR" => {
                attributes.insert("pcgen_var".to_string(), Value::String(value.clone()));
            }
            "DEFAULT" => {
                attributes.insert("pcgen_default".to_string(), parse_yes_no_or_string(value));
            }
            "MINXP" => {
                attributes.insert("pcgen_minxp".to_string(), Value::String(value.clone()));
            }
            "CSKILLMAX" => {
                attributes.insert("pcgen_cskillmax".to_string(), Value::String(value.clone()));
            }
            "CCSKILLMAX" => {
                attributes.insert("pcgen_ccskillmax".to_string(), Value::String(value.clone()));
            }
            "EQUIPBUY" => append_string_attr(attributes, "pcgen_equipbuy", value),
            "LOCATION" => {
                attributes.insert("pcgen_location".to_string(), Value::String(value.clone()));
            }
            "QTY" => set_i64_or_string(attributes, "pcgen_qty", value),
            "COUNT" => set_i64_or_string(attributes, "pcgen_count", value),
            "COPYMASTERBAB" => {
                attributes.insert(
                    "pcgen_copymasterbab".to_string(),
                    Value::String(value.clone()),
                );
            }
            "COPYMASTERCHECK" => {
                attributes.insert(
                    "pcgen_copymastercheck".to_string(),
                    Value::String(value.clone()),
                );
            }
            "COPYMASTERHP" => {
                attributes.insert(
                    "pcgen_copymasterhp".to_string(),
                    Value::String(value.clone()),
                );
            }
            "USEMASTERSKILL" => {
                attributes.insert(
                    "pcgen_usemasterskill".to_string(),
                    parse_yes_no_or_string(value),
                );
            }
            "GENDER" => {
                attributes.insert("pcgen_gender".to_string(), Value::String(value.clone()));
            }
            "EXCLUDE" => {
                attributes.insert("pcgen_exclude".to_string(), Value::String(value.clone()));
            }
            "SUBREGION" => {
                attributes.insert("pcgen_subregion".to_string(), Value::String(value.clone()));
            }
            "POINTS" => set_i64_or_string(attributes, "pcgen_points", value),
            "MAXNONEPICLEVEL" => set_i64_or_string(attributes, "pcgen_maxnonepiclevel", value),
            "SKILLCOST_CROSSCLASS" => {
                set_i64_or_string(attributes, "pcgen_skillcost_crossclass", value)
            }
            "SKILLCOST_CLASS" => {
                set_i64_or_string(attributes, "pcgen_skillcost_class", value)
            }
            "SKILLCOST_EXCLUSIVE" => {
                set_i64_or_string(attributes, "pcgen_skillcost_exclusive", value)
            }
            "SPELLBASECONCENTRATION" => {
                attributes.insert(
                    "pcgen_spellbaseconcentration".to_string(),
                    Value::String(value.clone()),
                );
            }
            "XPAWARD" => {
                attributes.insert("pcgen_xpaward".to_string(), Value::String(value.clone()));
            }
            "STATINPUT" => {
                attributes.insert("pcgen_statinput".to_string(), Value::String(value.clone()));
            }
            "PLUSCOST" => {
                attributes.insert("pcgen_pluscost".to_string(), parse_pipe_series(value));
            }
            "RESIZABLEEQUIPTYPE" => {
                attributes.insert(
                    "pcgen_resizableequiptype".to_string(),
                    parse_pipe_series(value),
                );
            }
            "METHOD" => {
                attributes.insert("pcgen_method".to_string(), Value::String(value.clone()));
            }
            "LEVELMSG" => {
                attributes.insert("pcgen_levelmsg".to_string(), Value::String(value.clone()));
            }
            "SHORTRANGE" => set_i64_or_string(attributes, "pcgen_shortrange", value),
            "RANGEPENALTY" => set_i64_or_string(attributes, "pcgen_rangepenalty", value),
            "SQUARESIZE" => set_i64_or_string(attributes, "pcgen_squaresize", value),
            "SKILLMULTIPLIER" => {
                attributes.insert(
                    "pcgen_skillmultiplier".to_string(),
                    Value::String(value.clone()),
                );
            }
            "SPELLBASEDC" => {
                attributes.insert("pcgen_spellbasedc".to_string(), Value::String(value.clone()));
            }
            "WEAPONNONPROFPENALTY" => {
                set_i64_or_string(attributes, "pcgen_weaponnonprofpenalty", value)
            }
            "WEAPONREACH" => {
                attributes.insert("pcgen_weaponreach".to_string(), Value::String(value.clone()));
            }
            "CHARACTERTYPE" => {
                attributes.insert("pcgen_charactertype".to_string(), parse_pipe_series(value));
            }
            "CRTHRESHOLD" => {
                attributes.insert("pcgen_crthreshold".to_string(), Value::String(value.clone()));
            }
            "CRSTEPS" => {
                attributes.insert("pcgen_crsteps".to_string(), Value::String(value.clone()));
            }
            "MONSTERROLES" => {
                attributes.insert("pcgen_monsterroles".to_string(), parse_pipe_series(value));
            }
            "MONSTERROLEDEFAULT" => {
                attributes.insert(
                    "pcgen_monsterroledefault".to_string(),
                    Value::String(value.clone()),
                );
            }
            "XPTABLE" => {
                attributes.insert("pcgen_xptable".to_string(), Value::String(value.clone()));
            }
            "EQSIZEPENALTY" => {
                attributes.insert("pcgen_eqsizepenalty".to_string(), Value::String(value.clone()));
            }
            "DEFAULTUNITSET" => {
                attributes.insert(
                    "pcgen_defaultunitset".to_string(),
                    Value::String(value.clone()),
                );
            }
            "ALLOWEDMODES" => {
                attributes.insert("pcgen_allowedmodes".to_string(), Value::String(value.clone()));
            }
            "BABMAXATT" => set_i64_or_string(attributes, "pcgen_babmaxatt", value),
            "BABMINVAL" => set_i64_or_string(attributes, "pcgen_babminval", value),
            "BABATTCYC" => set_i64_or_string(attributes, "pcgen_babattcyc", value),
            "ACNAME" => {
                attributes.insert("pcgen_acname".to_string(), Value::String(value.clone()));
            }
            "DOMAINFEATURE" => {
                attributes.insert("pcgen_domainfeature".to_string(), parse_yes_no_or_string(value));
            }
            "LOADMULT" => set_i64_or_string(attributes, "pcgen_loadmult", value),
            "NUMSLOTS" => {
                attributes.insert("pcgen_numslots".to_string(), Value::String(value.clone()));
            }
            "HEAD" => set_i64_or_string(attributes, "pcgen_headslots", value),
            "DISTANCEUNIT" => {
                attributes.insert("pcgen_distanceunit".to_string(), Value::String(value.clone()));
            }
            "DISTANCEFACTOR" => {
                attributes.insert("pcgen_distancefactor".to_string(), Value::String(value.clone()));
            }
            "DISTANCEPATTERN" => {
                attributes.insert("pcgen_distancepattern".to_string(), Value::String(value.clone()));
            }
            "HEIGHTUNIT" => {
                attributes.insert("pcgen_heightunit".to_string(), Value::String(value.clone()));
            }
            "HEIGHTFACTOR" => {
                attributes.insert("pcgen_heightfactor".to_string(), Value::String(value.clone()));
            }
            "HEIGHTPATTERN" => {
                attributes.insert("pcgen_heightpattern".to_string(), Value::String(value.clone()));
            }
            "WEIGHTUNIT" => {
                attributes.insert("pcgen_weightunit".to_string(), Value::String(value.clone()));
            }
            "WEIGHTFACTOR" => {
                attributes.insert("pcgen_weightfactor".to_string(), Value::String(value.clone()));
            }
            "WEIGHTPATTERN" => {
                attributes.insert("pcgen_weightpattern".to_string(), Value::String(value.clone()));
            }
            "TOTALCOST" => {
                attributes.insert("pcgen_totalcost".to_string(), Value::String(value.clone()));
            }
            "EQUIPMOD" => {
                append_value_attr(attributes, "pcgen_equipmod_catalog", parse_pipe_series(value));
            }
            "LANGUAGE" => {
                append_value_attr(attributes, "pcgen_language_catalog", parse_pipe_series(value));
            }
            "WEAPONPROF" => {
                append_value_attr(attributes, "pcgen_weaponprof_catalog", parse_pipe_series(value));
            }
            "ARMORPROF" => {
                append_value_attr(attributes, "pcgen_armorprof_catalog", parse_pipe_series(value));
            }
            "SHIELDPROF" => {
                append_value_attr(attributes, "pcgen_shieldprof_catalog", parse_pipe_series(value));
            }
            "DEITY" => {
                attributes.insert("pcgen_deity".to_string(), parse_pipe_series(value));
            }
            "SYMBOL" => {
                attributes.insert("pcgen_symbol".to_string(), Value::String(value.clone()));
            }
            "LANGAUTO" => {
                append_value_attr(attributes, "pcgen_langauto", parse_pipe_series(value));
            }
            "GRANT" => {
                append_value_attr(attributes, "pcgen_grant", parse_pipe_series(value));
            }
            "CRFORMULA" => {
                attributes.insert("pcgen_crformula".to_string(), Value::String(value.clone()));
            }
            "ISMONSTER" => {
                attributes.insert("pcgen_ismonster".to_string(), parse_yes_no_or_string(value));
            }
            "XPPENALTY" => {
                attributes.insert("pcgen_xppenalty".to_string(), parse_yes_no_or_string(value));
            }
            "FREE" => {
                attributes.insert("pcgen_free".to_string(), parse_yes_no_or_string(value));
            }
            "SELECTION" => append_string_attr(attributes, "pcgen_selection", value),
            "VARIANTS" => append_string_attr(attributes, "pcgen_variants", value),
            "SITUATION" => append_string_attr(attributes, "pcgen_situations", value),
            "USEUNTRAINED" => {
                attributes.insert("pcgen_useuntrained".to_string(), parse_yes_no_or_string(value));
            }
            "EXCLUSIVE" => {
                attributes.insert("pcgen_exclusive".to_string(), parse_yes_no_or_string(value));
            }
            "KEYSTAT" => {
                attributes.insert("pcgen_keystat".to_string(), Value::String(value.clone()));
            }
            "SIZE" => {
                attributes.insert("pcgen_size".to_string(), Value::String(value.clone()));
            }
            "FACE" => {
                attributes.insert("pcgen_face".to_string(), Value::String(value.clone()));
            }
            "VISION" => append_string_attr(attributes, "pcgen_vision", value),
            "LEGS" => {
                set_i64_or_string(attributes, "pcgen_legs", value);
            }
            "HANDS" => {
                set_i64_or_string(attributes, "pcgen_hands", value);
            }
            "TORSO" => {
                set_i64_or_string(attributes, "pcgen_torsoslots", value);
            }
            "SHIELD" => {
                set_i64_or_string(attributes, "pcgen_shieldslots", value);
            }
            "DR" => {
                attributes.insert("pcgen_dr".to_string(), Value::String(value.clone()));
            }
            "SR" => {
                attributes.insert("pcgen_sr".to_string(), Value::String(value.clone()));
            }
            "CR" => {
                attributes.insert("pcgen_cr".to_string(), Value::String(value.clone()));
            }
            "CRMOD" => {
                if value.contains('|') {
                    append_value_attr(attributes, "pcgen_crmod", parse_crmod_definition(value));
                } else {
                    set_i64_or_string(attributes, "pcgen_crmod", value);
                }
            }
            "CRMODPRIORITY" => set_i64_or_string(attributes, "pcgen_crmodpriority", value),
            "REGION" => {
                attributes.insert("pcgen_region".to_string(), Value::String(value.clone()));
            }
            "ROLE" => {
                attributes.insert("pcgen_role".to_string(), Value::String(value.clone()));
            }
            "NUMBER" => {
                attributes.insert("pcgen_number".to_string(), Value::String(value.clone()));
            }
            "CONTEXT" => {
                attributes.insert("pcgen_context".to_string(), Value::String(value.clone()));
            }
            "UP" => {
                attributes.insert("pcgen_up".to_string(), Value::String(value.clone()));
            }
            "DOWN" => {
                attributes.insert("pcgen_down".to_string(), Value::String(value.clone()));
            }
            "WIELD" => {
                attributes.insert("pcgen_wield".to_string(), Value::String(value.clone()));
            }
            "EDR" => {
                attributes.insert("pcgen_edr".to_string(), Value::String(value.clone()));
            }
            "SPELLFAILURE" => {
                attributes.insert("pcgen_spellfailure".to_string(), Value::String(value.clone()));
            }
            "ACCHECK" => {
                attributes.insert("pcgen_accheck".to_string(), Value::String(value.clone()));
            }
            "ACHECK" => {
                attributes.insert("pcgen_accheck".to_string(), Value::String(value.clone()));
            }
            "MAXDEX" => {
                attributes.insert("pcgen_maxdex".to_string(), Value::String(value.clone()));
            }
            "SLOTS" => {
                attributes.insert("pcgen_slots".to_string(), Value::String(value.clone()));
            }
            "PART" => append_string_attr(attributes, "pcgen_part", value),
            "DAMAGE" => {
                attributes.insert("pcgen_damage".to_string(), Value::String(value.clone()));
            }
            "ALTDAMAGE" => {
                attributes.insert("pcgen_altdamage".to_string(), Value::String(value.clone()));
            }
            "ALTTYPE" => {
                attributes.insert("pcgen_alttype".to_string(), Value::String(value.clone()));
            }
            "CRITMULT" => {
                attributes.insert("pcgen_critmult".to_string(), Value::String(value.clone()));
            }
            "CRITRANGE" => {
                attributes.insert("pcgen_critrange".to_string(), Value::String(value.clone()));
            }
            "FUMBLERANGE" => {
                attributes.insert("pcgen_fumblerange".to_string(), Value::String(value.clone()));
            }
            "RATEOFFIRE" => {
                attributes.insert("pcgen_rateoffire".to_string(), Value::String(value.clone()));
            }
            "REACH" => {
                attributes.insert("pcgen_reach".to_string(), Value::String(value.clone()));
            }
            "REACHMULT" => {
                attributes.insert("pcgen_reachmult".to_string(), Value::String(value.clone()));
            }
            "ALTCRITMULT" => {
                attributes.insert("pcgen_altcritmult".to_string(), Value::String(value.clone()));
            }
            "ALTCRITRANGE" => {
                attributes.insert("pcgen_altcritrange".to_string(), Value::String(value.clone()));
            }
            "ALTEQMOD" => {
                attributes.insert("pcgen_alteqmod".to_string(), Value::String(value.clone()));
            }
            "PLUS" => set_i64_or_string(attributes, "pcgen_plus", value),
            "ITYPE" => {
                attributes.insert("pcgen_itype".to_string(), Value::String(value.clone()));
            }
            "NAMEOPT" => {
                attributes.insert("pcgen_nameopt".to_string(), Value::String(value.clone()));
            }
            "FORMATCAT" => {
                attributes.insert("pcgen_formatcat".to_string(), Value::String(value.clone()));
            }
            "ASSIGNTOALL" => {
                attributes.insert("pcgen_assigntoall".to_string(), parse_yes_no_or_string(value));
            }
            "SWITCH" => append_string_attr(attributes, "pcgen_switch", value),
            "SIZEDIFF" => set_i64_or_string(attributes, "pcgen_sizediff", value),
            "FINESSABLE" => {
                attributes.insert("pcgen_finessable".to_string(), parse_yes_no_or_string(value));
            }
            "PROFICIENCY" => {
                attributes.insert("pcgen_proficiency".to_string(), parse_pipe_series(value));
            }
            "ARMORTYPE" => {
                append_value_attr(attributes, "pcgen_armortype", parse_transition_pair(value));
            }
            "REPLACES" => {
                attributes.insert("pcgen_replaces".to_string(), Value::String(value.clone()));
            }
            "UNENCUMBEREDMOVE" => {
                append_value_attr(attributes, "pcgen_unencumberedmove", parse_pipe_series(value));
            }
            "CONTAINS" => {
                attributes.insert("pcgen_contains".to_string(), Value::String(value.clone()));
            }
            "BASEQTY" => {
                attributes.insert("pcgen_baseqty".to_string(), Value::String(value.clone()));
            }
            "MODS" => {
                attributes.insert("pcgen_mods".to_string(), Value::String(value.clone()));
            }
            "ICON" => {
                attributes.insert("pcgen_icon".to_string(), Value::String(value.clone()));
            }
            "REMOVE" => append_string_attr(attributes, "pcgen_remove", value),
            "NUMPAGES" => {
                if let Ok(num) = value.trim().parse::<i64>() {
                    attributes.insert("pcgen_numpages".to_string(), json!(num));
                } else {
                    attributes.insert("pcgen_numpages".to_string(), Value::String(value.clone()));
                }
            }
            "QUALITY" => append_string_attr(attributes, "pcgen_qualities", value),
            "SPROP" => sprop_values.push(Value::String(value.clone())),
            "PAGEUSAGE" => page_usage_values.push(Value::String(value.clone())),
            "CHARGES" => append_string_attr(attributes, "pcgen_charges", value),
            "FACT" => facts.push(parse_fact(value)),
            "FACTSET" => factsets.push(parse_fact(value)),
            "EQMOD" => equipment_modifiers.push(parse_pipe_series(value)),
            "CLASSES" => class_lists.push(parse_pipe_series(value)),
            "SPELLS" => spell_blocks.push(parse_spells(value)),
            "SPELLKNOWN" => append_string_attr(attributes, "pcgen_spellknown", value),
            "MOVE" => append_string_attr(attributes, "pcgen_move", value),
            "NATURALATTACKS" => append_string_attr(attributes, "pcgen_naturalattacks", value),
            "MODIFY" => {
                // Parse MODIFY expressions: VarName|Operation|Value
                match parse_modify(value) {
                    Ok(expr) => {
                        append_string_attr(attributes, "pcgen_modify_variable", &expr.variable);
                        append_string_attr(
                            attributes,
                            "pcgen_modify_operation",
                            &expr.operation.to_string(),
                        );
                        append_string_attr(attributes, "pcgen_modify_value", &expr.value);
                    }
                    Err(_) => {
                        // Fallback: store raw MODIFY value if parsing fails
                        append_string_attr(attributes, "pcgen_modify", value);
                    }
                }
            }
            "MODIFYOTHER" => append_string_attr(attributes, "pcgen_modifyother", value),
            _ => {}
        }
    }

    if !facts.is_empty() {
    }
    if !equipment_modifiers.is_empty() {
        attributes.insert("pcgen_eqmods".to_string(), Value::Array(equipment_modifiers));
    }
    if !facts.is_empty() {
        attributes.insert("pcgen_facts".to_string(), Value::Array(facts));
    }
    if !class_lists.is_empty() {
        attributes.insert("pcgen_classes".to_string(), Value::Array(class_lists));
    }
    if !spell_blocks.is_empty() {
        attributes.insert("pcgen_spells".to_string(), Value::Array(spell_blocks));
    }
    if !sprop_values.is_empty() {
        attributes.insert("pcgen_sprop".to_string(), Value::Array(sprop_values));
    }
    if !page_usage_values.is_empty() {
        attributes.insert("pcgen_pageusage".to_string(), Value::Array(page_usage_values));
    }
    if !factsets.is_empty() {
        attributes.insert("pcgen_factsets".to_string(), Value::Array(factsets));
    }

    project_dual_name_fields(head_name, attributes);
}

pub(crate) fn project_decl_token_value(
    decl_token: &str,
    decl_value: &str,
    attributes: &mut IndexMap<String, Value>,
) {
    match decl_token.to_ascii_uppercase().as_str() {
        "ABILITY" => append_string_attr(attributes, "pcgen_abilities", decl_value),
        "ABILITYCATEGORY" => append_string_attr(attributes, "pcgen_abilitycategories", decl_value),
        "FEAT" => append_string_attr(attributes, "pcgen_feats", decl_value),
        "EQUIPMENT" => append_string_attr(attributes, "pcgen_equipment", decl_value),
        "SPELL" => append_string_attr(attributes, "pcgen_spells", decl_value),
        "LICENSE" => append_string_attr(attributes, "pcgen_license", decl_value),
        "INFOTEXT" => append_string_attr(attributes, "pcgen_infotext", decl_value),
        "FORWARDREF" => append_string_attr(attributes, "pcgen_forwardref", decl_value),
        "HIDETYPE" => append_string_attr(attributes, "pcgen_hidetype", decl_value),
        "URL" => append_string_attr(attributes, "pcgen_url", decl_value),
        "LOCAL" => {
            attributes.insert("pcgen_local".to_string(), parse_local_definition(decl_value));
        }
        "GLOBAL" => {
            attributes.insert("pcgen_global".to_string(), parse_global_definition(decl_value));
        }
        "FACTSETDEF" => {
            attributes.insert(
                "pcgen_factsetdef".to_string(),
                parse_factsetdef_definition(decl_value),
            );
        }
        "FUNCTION" => {
            attributes.insert(
                "pcgen_function".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "DYNAMICSCOPE" => {
            attributes.insert(
                "pcgen_dynamicscope".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SPELLRANGE" => {
            attributes.insert(
                "pcgen_spellrange".to_string(),
                parse_spellrange_definition(decl_value),
            );
        }
        "BASEAGEADD" => {
            attributes.insert("pcgen_baseageadd".to_string(), Value::String(decl_value.to_string()));
        }
        "OUTPUTSHEET" => {
            attributes.insert(
                "pcgen_outputsheet".to_string(),
                parse_outputsheet_definition(decl_value),
            );
        }
        "INFOSHEET" => {
            attributes.insert(
                "pcgen_infosheet".to_string(),
                parse_infosheet_definition(decl_value),
            );
        }
        "UNITSET" => {
            attributes.insert("pcgen_unitset".to_string(), Value::String(decl_value.to_string()));
        }
        "ALIGNMENTFEATURE" => {
            attributes.insert(
                "pcgen_alignmentfeature".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "CURRENCYUNITABBREV" => {
            attributes.insert(
                "pcgen_currencyunitabbrev".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "MENUENTRY" => {
            attributes.insert(
                "pcgen_menuentry".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "DISPLAYORDER" => {
            set_i64_or_string(attributes, "pcgen_displayorder", decl_value);
        }
        "DIESIZES" => {
            attributes.insert(
                "pcgen_diesizes".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "LEVELMSG" => {
            attributes.insert(
                "pcgen_levelmsg".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SHORTRANGE" => set_i64_or_string(attributes, "pcgen_shortrange", decl_value),
        "RANGEPENALTY" => set_i64_or_string(attributes, "pcgen_rangepenalty", decl_value),
        "SQUARESIZE" => set_i64_or_string(attributes, "pcgen_squaresize", decl_value),
        "SKILLMULTIPLIER" => {
            attributes.insert(
                "pcgen_skillmultiplier".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "SPELLBASEDC" => {
            attributes.insert(
                "pcgen_spellbasedc".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "WEAPONNONPROFPENALTY" => {
            set_i64_or_string(attributes, "pcgen_weaponnonprofpenalty", decl_value)
        }
        "WEAPONREACH" => {
            attributes.insert(
                "pcgen_weaponreach".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CHARACTERTYPE" => {
            attributes.insert("pcgen_charactertype".to_string(), parse_pipe_series(decl_value));
        }
        "CRTHRESHOLD" => {
            attributes.insert(
                "pcgen_crthreshold".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "CRSTEPS" => {
            attributes.insert("pcgen_crsteps".to_string(), Value::String(decl_value.to_string()));
        }
        "MONSTERROLES" => {
            attributes.insert("pcgen_monsterroles".to_string(), parse_pipe_series(decl_value));
        }
        "MONSTERROLEDEFAULT" => {
            attributes.insert(
                "pcgen_monsterroledefault".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "XPTABLE" => {
            attributes.insert("pcgen_xptable".to_string(), Value::String(decl_value.to_string()));
        }
        "EQSIZEPENALTY" => {
            attributes.insert(
                "pcgen_eqsizepenalty".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "STARTTABLE" => {
            attributes.insert(
                "pcgen_starttable".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "ENDTABLE" => {
            attributes.insert(
                "pcgen_endtable".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "MOVEMENT" => {
            attributes.insert(
                "pcgen_movement".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "DEFAULTDATASET" => {
            attributes.insert(
                "pcgen_defaultdataset".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "GAMEMODEKEY" => {
            attributes.insert(
                "pcgen_gamemodekey".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "LANGAUTO" => {
            attributes.insert("pcgen_langauto".to_string(), parse_pipe_series(decl_value));
        }
        "RESIZABLEEQUIPTYPE" => {
            attributes.insert(
                "pcgen_resizableequiptype".to_string(),
                parse_pipe_series(decl_value),
            );
        }
        "SKILLCOST_CROSSCLASS" => {
            set_i64_or_string(attributes, "pcgen_skillcost_crossclass", decl_value)
        }
        "SKILLCOST_CLASS" => {
            set_i64_or_string(attributes, "pcgen_skillcost_class", decl_value)
        }
        "SKILLCOST_EXCLUSIVE" => {
            set_i64_or_string(attributes, "pcgen_skillcost_exclusive", decl_value)
        }
        "SPELLBASECONCENTRATION" => {
            attributes.insert(
                "pcgen_spellbaseconcentration".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "XPAWARD" => {
            attributes.insert("pcgen_xpaward".to_string(), Value::String(decl_value.to_string()));
        }
        "STATINPUT" => {
            attributes.insert("pcgen_statinput".to_string(), Value::String(decl_value.to_string()));
        }
        "MAXNONEPICLEVEL" => {
            set_i64_or_string(attributes, "pcgen_maxnonepiclevel", decl_value)
        }
        "PLUSCOST" => {
            attributes.insert("pcgen_pluscost".to_string(), parse_pipe_series(decl_value));
        }
        "DEFAULTUNITSET" => {
            attributes.insert(
                "pcgen_defaultunitset".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "ALLOWEDMODES" => {
            attributes.insert(
                "pcgen_allowedmodes".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        "BABMAXATT" => set_i64_or_string(attributes, "pcgen_babmaxatt", decl_value),
        "BABMINVAL" => set_i64_or_string(attributes, "pcgen_babminval", decl_value),
        "BABATTCYC" => set_i64_or_string(attributes, "pcgen_babattcyc", decl_value),
        "ACNAME" => {
            attributes.insert("pcgen_acname".to_string(), Value::String(decl_value.to_string()));
        }
        "DOMAINFEATURE" => {
            attributes.insert(
                "pcgen_domainfeature".to_string(),
                parse_yes_no_or_string(decl_value),
            );
        }
        "LOADMULT" => set_i64_or_string(attributes, "pcgen_loadmult", decl_value),
        "NUMSLOTS" => {
            attributes.insert(
                "pcgen_numslots".to_string(),
                Value::String(decl_value.to_string()),
            );
        }
        _ => {}
    }
}

fn project_dual_name_fields(head_name: &str, attributes: &mut IndexMap<String, Value>) {
    let Some(name_is_pi_raw) = attributes.get("pcgen_nameispi") else {
        return;
    };

    let is_pi_name = bool_like_from_value(name_is_pi_raw).unwrap_or(false);

    if is_pi_name {
        attributes.insert(
            "pcgen_name_pi".to_string(),
            Value::String(head_name.to_string()),
        );

        if let Some(open_name) = open_name_candidate(attributes, head_name) {
            attributes.insert("pcgen_name_open".to_string(), Value::String(open_name));
        }
        return;
    }

    attributes.insert(
        "pcgen_name_open".to_string(),
        Value::String(head_name.to_string()),
    );
}

fn open_name_candidate(attributes: &IndexMap<String, Value>, head_name: &str) -> Option<String> {
    ["pcgen_outputname", "pcgen_key"]
        .iter()
        .filter_map(|key| attributes.get(*key).and_then(Value::as_str))
        .map(str::trim)
        .find(|value| !value.is_empty() && !value.eq_ignore_ascii_case(head_name))
        .map(ToString::to_string)
}

fn bool_like_from_value(value: &Value) -> Option<bool> {
    match value {
        Value::Bool(b) => Some(*b),
        Value::String(s) => match s.trim().to_ascii_uppercase().as_str() {
            "YES" | "Y" | "TRUE" => Some(true),
            "NO" | "N" | "FALSE" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn append_string_attr(attributes: &mut IndexMap<String, Value>, key: &str, value: &str) {
    match attributes.get_mut(key) {
        Some(Value::Array(existing)) => existing.push(Value::String(value.to_string())),
        Some(Value::String(existing)) => {
            let prior = existing.clone();
            attributes.insert(
                key.to_string(),
                Value::Array(vec![Value::String(prior), Value::String(value.to_string())]),
            );
        }
        _ => {
            attributes.insert(
                key.to_string(),
                Value::Array(vec![Value::String(value.to_string())]),
            );
        }
    }
}

fn append_value_attr(attributes: &mut IndexMap<String, Value>, key: &str, value: Value) {
    match attributes.get_mut(key) {
        Some(Value::Array(existing)) => existing.push(value),
        Some(existing) => {
            let prior = existing.clone();
            attributes.insert(key.to_string(), Value::Array(vec![prior, value]));
        }
        None => {
            attributes.insert(key.to_string(), Value::Array(vec![value]));
        }
    }
}

fn set_i64_or_string(attributes: &mut IndexMap<String, Value>, key: &str, value: &str) {
    if let Ok(parsed) = value.trim().parse::<i64>() {
        attributes.insert(key.to_string(), json!(parsed));
    } else {
        attributes.insert(key.to_string(), Value::String(value.to_string()));
    }
}

fn parse_yes_no_or_string(value: &str) -> Value {
    match value.trim().to_ascii_uppercase().as_str() {
        "YES" | "Y" | "TRUE" => Value::Bool(true),
        "NO" | "N" | "FALSE" => Value::Bool(false),
        _ => Value::String(value.to_string()),
    }
}

fn parse_fact(input: &str) -> Value {
    let mut parts = input.splitn(2, '|');
    let fact_key = parts.next().unwrap_or_default().trim();
    let fact_value = parts.next().unwrap_or_default().trim();

    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));
    if !fact_key.is_empty() {
        out.insert("key".to_string(), Value::String(fact_key.to_string()));
    }
    if !fact_value.is_empty() {
        out.insert("value".to_string(), Value::String(fact_value.to_string()));
    }
    Value::Object(out)
}

fn parse_pipe_series(input: &str) -> Value {
    let parts: Vec<Value> = input
        .split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| Value::String(part.to_string()))
        .collect();

    json!({
        "raw": input,
        "parts": parts,
    })
}

fn parse_spells(input: &str) -> Value {
    let parts: Vec<&str> = input
        .split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();

    let mut assignments = Map::new();
    let mut spells = Vec::new();
    let mut mode = None;

    for (index, part) in parts.iter().enumerate() {
        if index == 0 && !part.contains('=') {
            mode = Some((*part).to_string());
            continue;
        }

        if let Some((key, value)) = part.split_once('=') {
            assignments.insert(key.trim().to_ascii_lowercase(), Value::String(value.trim().to_string()));
        } else {
            spells.push(Value::String((*part).to_string()));
        }
    }

    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));
    if let Some(mode) = mode {
        out.insert("mode".to_string(), Value::String(mode));
    }
    if !assignments.is_empty() {
        out.insert("assignments".to_string(), Value::Object(assignments));
    }
    if !spells.is_empty() {
        out.insert("spells".to_string(), Value::Array(spells));
    }
    Value::Object(out)
}

fn parse_local_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let scope = parts.next().unwrap_or_default().trim();
    let binding = parts.next().unwrap_or_default().trim();

    if !scope.is_empty() {
        out.insert("scope".to_string(), Value::String(scope.to_string()));
    }
    if !binding.is_empty() {
        out.insert("binding".to_string(), parse_variable_binding(binding));
    }

    Value::Object(out)
}

fn parse_global_definition(input: &str) -> Value {
    json!({
        "raw": input,
        "binding": parse_variable_binding(input),
    })
}

fn parse_factsetdef_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let subject = parts.next().unwrap_or_default().trim();
    let field = parts.next().unwrap_or_default().trim();

    if !subject.is_empty() {
        out.insert("subject".to_string(), Value::String(subject.to_string()));
    }
    if !field.is_empty() {
        out.insert("field".to_string(), Value::String(field.to_string()));
    }

    Value::Object(out)
}

fn parse_spellrange_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let range_name = parts.next().unwrap_or_default().trim();
    let formula = parts.next().unwrap_or_default().trim();

    if !range_name.is_empty() {
        out.insert("name".to_string(), Value::String(range_name.to_string()));
    }
    if !formula.is_empty() {
        out.insert("formula".to_string(), Value::String(formula.to_string()));
    }

    Value::Object(out)
}

fn parse_transition_pair(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let from = parts.next().unwrap_or_default().trim();
    let to = parts.next().unwrap_or_default().trim();

    if !from.is_empty() {
        out.insert("from".to_string(), Value::String(from.to_string()));
    }
    if !to.is_empty() {
        out.insert("to".to_string(), Value::String(to.to_string()));
    }

    Value::Object(out)
}

fn parse_outputsheet_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let kind = parts.next().unwrap_or_default().trim();
    let path = parts.next().unwrap_or_default().trim();

    if !kind.is_empty() {
        out.insert("kind".to_string(), Value::String(kind.to_string()));
    }
    if !path.is_empty() {
        out.insert("path".to_string(), Value::String(path.to_string()));
    }

    Value::Object(out)
}

fn parse_infosheet_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let kind = parts.next().unwrap_or_default().trim();
    let path = parts.next().unwrap_or_default().trim();

    if !kind.is_empty() {
        out.insert("kind".to_string(), Value::String(kind.to_string()));
    }
    if !path.is_empty() {
        out.insert("path".to_string(), Value::String(path.to_string()));
    }

    Value::Object(out)
}

fn parse_crmod_definition(input: &str) -> Value {
    let mut out = Map::new();
    out.insert("raw".to_string(), Value::String(input.to_string()));

    let mut parts = input.splitn(2, '|');
    let scope = parts.next().unwrap_or_default().trim();
    let modifier = parts.next().unwrap_or_default().trim();

    if !scope.is_empty() {
        let class_types: Vec<Value> = scope
            .split('.')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(|part| Value::String(part.to_string()))
            .collect();
        if !class_types.is_empty() {
            out.insert("class_types".to_string(), Value::Array(class_types));
        }
    }

    if !modifier.is_empty() {
        if let Ok(num) = modifier.parse::<i64>() {
            out.insert("modifier".to_string(), json!(num));
        } else {
            out.insert("modifier".to_string(), Value::String(modifier.to_string()));
        }
    }

    Value::Object(out)
}

fn parse_variable_binding(input: &str) -> Value {
    let trimmed = input.trim();
    if let Some((kind, name)) = trimmed.split_once('=') {
        return json!({
            "raw": trimmed,
            "kind": kind.trim(),
            "name": name.trim(),
        });
    }

    json!({
        "raw": trimmed,
        "name": trimmed,
    })
}
