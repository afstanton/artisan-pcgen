use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::PathBuf,
};

use artisan_core::{
    CanonicalId, ParsedCatalog,
    domain::SubjectRef,
};
use serde_json::Value;

use super::{
    EmitError,
    entity_type_map::{entry_for_type_key, EntityTypeEntry},
    manifest::{EmitManifest, FileManifest, SourceManifest},
    slugify::{derive_abbreviation, game_system_slug, to_slug},
};

// ---------------------------------------------------------------------------
// Public plan types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct EmitPlan {
    pub sources: Vec<SourcePlan>,
    /// Catch-all for entities with no citation.
    pub uncited_file: Option<FilePlan>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SourcePlan {
    pub source_id: CanonicalId,
    pub source_title: String,
    /// Directory path relative to output_root.
    pub directory: PathBuf,
    /// PCC filename relative to directory. `None` if skipped.
    pub pcc_filename: Option<PathBuf>,
    /// Abbreviation used as file prefix.
    pub abbreviation: String,
    /// Raw game mode string (for PCC GAMEMODE token).
    pub gamemode: String,
    pub files: Vec<FilePlan>,
}

#[derive(Debug, Clone)]
pub struct FilePlan {
    /// Filename relative to the source directory.
    pub filename: PathBuf,
    /// PCGen file suffix (e.g. "abilities"). Used for PCC ordering.
    pub file_suffix: String,
    /// Entity type keys included in this file.
    pub entity_type_keys: Vec<String>,
    /// Entity canonical IDs to emit.
    pub entity_ids: Vec<CanonicalId>,
}

// ---------------------------------------------------------------------------
// Planning
// ---------------------------------------------------------------------------

pub fn plan_emission(
    catalog: &ParsedCatalog,
    manifest: Option<&EmitManifest>,
) -> Result<EmitPlan, EmitError> {
    let mut warnings = Vec::new();

    // Index publishers by id.
    let publisher_by_id: HashMap<CanonicalId, &str> = catalog
        .publishers
        .iter()
        .map(|p| (p.id, p.name.as_str()))
        .collect();

    // Index entities by id.
    let entity_by_id: HashMap<CanonicalId, &artisan_core::Entity> = catalog
        .entities
        .iter()
        .map(|e| (e.id, e))
        .collect();

    // Build: source_id → Vec<entity_id> (from citations).
    let mut entities_by_source: HashMap<CanonicalId, Vec<CanonicalId>> = HashMap::new();
    for citation in &catalog.citations {
        if let SubjectRef::Entity(entity_id) = &citation.subject {
            entities_by_source
                .entry(citation.source)
                .or_default()
                .push(*entity_id);
        }
    }

    // Track which entities have at least one citation.
    let cited_entity_ids: HashSet<CanonicalId> = catalog
        .citations
        .iter()
        .filter_map(|c| {
            if let SubjectRef::Entity(id) = &c.subject { Some(*id) } else { None }
        })
        .collect();

    let emit_uncited = manifest
        .and_then(|m| m.emit_uncited)
        .unwrap_or(true);

    // Check for path collisions across sources.
    let mut path_to_sources: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();

    let mut source_plans: Vec<SourcePlan> = Vec::new();

    for source in &catalog.sources {
        // Find matching source manifest override, if any.
        let source_manifest = manifest.and_then(|m| {
            m.sources.iter().find(|sm| {
                sm.source_id.map(|id| CanonicalId(id) == source.id).unwrap_or(false)
                    || sm
                        .source_title
                        .as_deref()
                        .map(|t| t.eq_ignore_ascii_case(&source.title))
                        .unwrap_or(false)
            })
        });

        // Derive gamemode.
        let gamemode = if let Some(sm) = source_manifest.and_then(|m| m.gamemode.as_deref()) {
            sm.to_string()
        } else if let Some(gs) = source.game_systems.first() {
            gs.clone()
        } else {
            warnings.push(format!(
                "source {:?} has no game_systems; defaulting GAMEMODE to \"35e\"",
                source.title
            ));
            "35e".to_string()
        };

        // Derive directory.
        let directory = if let Some(dir) =
            source_manifest.and_then(|m| m.directory.as_ref())
        {
            dir.clone()
        } else {
            derive_directory(source, &publisher_by_id, &mut warnings)
        };

        // Collision check.
        path_to_sources
            .entry(directory.clone())
            .or_default()
            .push(source.title.clone());

        // Derive abbreviation.
        let source_short_hint = entities_by_source
            .get(&source.id)
            .and_then(|ids| ids.first())
            .and_then(|id| entity_by_id.get(id))
            .and_then(|e| e.attributes.get("source_short"))
            .and_then(Value::as_str);

        let abbreviation =
            source_manifest
                .and_then(|m| m.abbreviation.as_deref())
                .map(|s| s.to_string())
                .unwrap_or_else(|| derive_abbreviation(&source.title, source_short_hint));

        // PCC filename.
        let skip_pcc = source_manifest.and_then(|m| m.skip_pcc).unwrap_or(false);
        let pcc_filename = if skip_pcc {
            None
        } else {
            let slug = to_slug(&source.title);
            Some(PathBuf::from(format!("_{slug}.pcc")))
        };

        // Determine entity IDs for this source.
        let all_entity_ids_for_source: Vec<CanonicalId> = entities_by_source
            .get(&source.id)
            .cloned()
            .unwrap_or_default();

        let allowed_ids: Option<HashSet<CanonicalId>> =
            source_manifest.and_then(|m| {
                if m.entity_ids.is_empty() {
                    None
                } else {
                    Some(m.entity_ids.iter().map(|id| CanonicalId(*id)).collect())
                }
            });

        let entity_ids_for_source: Vec<CanonicalId> = all_entity_ids_for_source
            .into_iter()
            .filter(|id| {
                allowed_ids.as_ref().map(|set| set.contains(id)).unwrap_or(true)
            })
            .collect();

        // Build file plans.
        let files = build_file_plans(
            &entity_ids_for_source,
            &entity_by_id,
            &abbreviation,
            source_manifest,
            &mut warnings,
        );

        source_plans.push(SourcePlan {
            source_id: source.id,
            source_title: source.title.clone(),
            directory,
            pcc_filename,
            abbreviation,
            gamemode,
            files,
        });
    }

    // Report path collisions.
    for (path, sources) in &path_to_sources {
        if sources.len() > 1 {
            return Err(EmitError::PathCollision {
                path: path.clone(),
                sources: sources.clone(),
            });
        }
    }

    // Uncited entities catch-all.
    let uncited_file = if emit_uncited {
        let uncited_ids: Vec<CanonicalId> = catalog
            .entities
            .iter()
            .filter(|e| !cited_entity_ids.contains(&e.id))
            .map(|e| e.id)
            .collect();

        if uncited_ids.is_empty() {
            None
        } else {
            warnings.push(format!(
                "{} entities have no citations and will be emitted to _uncited.lst",
                uncited_ids.len()
            ));
            Some(FilePlan {
                filename: PathBuf::from("_uncited.lst"),
                file_suffix: "uncited".to_string(),
                entity_type_keys: vec![],
                entity_ids: uncited_ids,
            })
        }
    } else {
        None
    };

    Ok(EmitPlan { sources: source_plans, uncited_file, warnings })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn derive_directory(
    source: &artisan_core::domain::SourceRecord,
    publisher_by_id: &HashMap<CanonicalId, &str>,
    warnings: &mut Vec<String>,
) -> PathBuf {
    // Game system slug.
    let gs_slug = source
        .game_systems
        .first()
        .map(|s| game_system_slug(s))
        .unwrap_or_else(|| {
            warnings.push(format!(
                "source {:?} has no game_systems; defaulting to \"35e\" for directory",
                source.title
            ));
            "35e".to_string()
        });

    // Publisher slug: prefer publisher_ids lookup, then publisher string field.
    let pub_slug = source
        .publisher_ids
        .first()
        .and_then(|id| publisher_by_id.get(id))
        .copied()
        .or_else(|| source.publisher.as_deref())
        .map(to_slug)
        .unwrap_or_else(|| {
            warnings.push(format!(
                "source {:?} has no publisher; using \"unknown_publisher\" for directory",
                source.title
            ));
            "unknown_publisher".to_string()
        });

    let src_slug = to_slug(&source.title);

    PathBuf::from(format!("data/{gs_slug}/{pub_slug}/{src_slug}"))
}

fn build_file_plans(
    entity_ids: &[CanonicalId],
    entity_by_id: &HashMap<CanonicalId, &artisan_core::Entity>,
    abbreviation: &str,
    source_manifest: Option<&SourceManifest>,
    warnings: &mut Vec<String>,
) -> Vec<FilePlan> {
    match source_manifest.map(|m| m.files.as_slice()) {
        Some(file_manifests) if !file_manifests.is_empty() => build_manifest_file_plans(
            entity_ids,
            entity_by_id,
            abbreviation,
            file_manifests,
            warnings,
        ),
        _ => build_default_file_plans(entity_ids, entity_by_id, abbreviation, warnings),
    }
}

/// Default: one file per effective file suffix, grouping merged types together.
fn build_default_file_plans(
    entity_ids: &[CanonicalId],
    entity_by_id: &HashMap<CanonicalId, &artisan_core::Entity>,
    abbreviation: &str,
    warnings: &mut Vec<String>,
) -> Vec<FilePlan> {
    // suffix → (Vec<entity_id>, Vec<type_key>)
    let mut by_suffix: BTreeMap<String, (Vec<CanonicalId>, Vec<String>)> = BTreeMap::new();

    for &id in entity_ids {
        let entity = match entity_by_id.get(&id) {
            Some(e) => e,
            None => {
                warnings.push(format!("entity {id:?} not found in catalog; skipping"));
                continue;
            }
        };

        let type_key = entity
            .attributes
            .get("pcgen_entity_type_key")
            .and_then(Value::as_str)
            .unwrap_or("");

        let entry = match entry_for_type_key(type_key) {
            Some(e) => e,
            None => {
                warnings.push(format!(
                    "unknown entity type key {:?} for entity {:?}; skipping",
                    type_key, entity.name
                ));
                continue;
            }
        };

        // Resolve effective suffix (accounting for merges).
        let effective_suffix = match (entry.file_suffix, entry.merge_into) {
            (_, Some(into)) => into.to_string(),
            (Some(suffix), None) => suffix.to_string(),
            (None, None) => continue, // skip
        };

        let (ids, keys) = by_suffix.entry(effective_suffix).or_default();
        ids.push(id);
        if !keys.contains(&type_key.to_string()) {
            keys.push(type_key.to_string());
        }
    }

    by_suffix
        .into_iter()
        .map(|(suffix, (ids, keys))| FilePlan {
            filename: PathBuf::from(format!("{abbreviation}_{suffix}.lst")),
            file_suffix: suffix,
            entity_type_keys: keys,
            entity_ids: ids,
        })
        .collect()
}

/// Manifest-driven file plans with optional subtype splitting.
fn build_manifest_file_plans(
    entity_ids: &[CanonicalId],
    entity_by_id: &HashMap<CanonicalId, &artisan_core::Entity>,
    abbreviation: &str,
    file_manifests: &[FileManifest],
    warnings: &mut Vec<String>,
) -> Vec<FilePlan> {
    // Build a lookup: type_key → file manifest index.
    let mut type_key_to_manifest: HashMap<&str, usize> = HashMap::new();
    for (i, fm) in file_manifests.iter().enumerate() {
        for key in &fm.entity_types {
            type_key_to_manifest.insert(key.as_str(), i);
        }
    }

    // For each file manifest, collect matching entities.
    let mut file_entity_ids: Vec<Vec<CanonicalId>> = vec![Vec::new(); file_manifests.len()];

    for &id in entity_ids {
        let entity = match entity_by_id.get(&id) {
            Some(e) => e,
            None => continue,
        };

        let type_key = entity
            .attributes
            .get("pcgen_entity_type_key")
            .and_then(Value::as_str)
            .unwrap_or("");

        // Resolve merge targets.
        let effective_key = match entry_for_type_key(type_key) {
            Some(EntityTypeEntry { merge_into: Some(into), .. }) => {
                // Find the manifest that handles the merge target suffix.
                // We'll just use the type_key as-is and rely on the manifest
                // having declared the primary type.
                type_key_to_manifest
                    .get(type_key)
                    .copied()
                    .or_else(|| {
                        // Try the canonical emit key for the target suffix.
                        file_manifests.iter().enumerate().find_map(|(i, fm)| {
                            fm.entity_types
                                .iter()
                                .any(|k| {
                                    entry_for_type_key(k)
                                        .and_then(|e| e.file_suffix)
                                        == Some(into)
                                })
                                .then_some(i)
                        })
                    })
            }
            Some(EntityTypeEntry { file_suffix: None, merge_into: None, .. }) => continue, // skip
            Some(_) | None => type_key_to_manifest.get(type_key).copied(),
        };

        match effective_key {
            Some(idx) => file_entity_ids[idx].push(id),
            None => warnings.push(format!(
                "entity {:?} (type {type_key:?}) not covered by any file manifest; skipping",
                entity.name
            )),
        }
    }

    let mut plans = Vec::new();

    for (i, fm) in file_manifests.iter().enumerate() {
        let ids = &file_entity_ids[i];
        if ids.is_empty() {
            continue;
        }

        // Derive base suffix from first declared entity type.
        let base_suffix = fm
            .entity_types
            .first()
            .and_then(|k| entry_for_type_key(k))
            .and_then(|e| e.file_suffix)
            .unwrap_or("entities");

        let base_filename = fm
            .filename
            .clone()
            .unwrap_or_else(|| format!("{abbreviation}_{base_suffix}.lst"));

        if let Some(split) = &fm.split_by {
            // Split into buckets.
            let mut buckets: Vec<Vec<CanonicalId>> = vec![Vec::new(); split.buckets.len()];
            let mut other: Vec<CanonicalId> = Vec::new();

            for &id in ids {
                let entity = match entity_by_id.get(&id) {
                    Some(e) => e,
                    None => continue,
                };
                let attr_val = entity
                    .attributes
                    .get(&split.attribute)
                    .and_then(Value::as_str)
                    .unwrap_or("");

                let bucket_idx = split.buckets.iter().position(|b| {
                    b.values.iter().any(|v| v.eq_ignore_ascii_case(attr_val))
                });

                match bucket_idx {
                    Some(idx) => buckets[idx].push(id),
                    None => other.push(id),
                }
            }

            for (b, bucket_ids) in split.buckets.iter().zip(buckets.into_iter()) {
                if bucket_ids.is_empty() {
                    continue;
                }
                let stem = base_filename
                    .trim_end_matches(".lst")
                    .to_string();
                plans.push(FilePlan {
                    filename: PathBuf::from(format!("{}_{}.lst", stem, b.label)),
                    file_suffix: format!("{base_suffix}_{}", b.label),
                    entity_type_keys: fm.entity_types.clone(),
                    entity_ids: bucket_ids,
                });
            }
            if !other.is_empty() {
                let stem = base_filename.trim_end_matches(".lst").to_string();
                plans.push(FilePlan {
                    filename: PathBuf::from(format!("{stem}_other.lst")),
                    file_suffix: format!("{base_suffix}_other"),
                    entity_type_keys: fm.entity_types.clone(),
                    entity_ids: other,
                });
            }
        } else {
            plans.push(FilePlan {
                filename: PathBuf::from(base_filename),
                file_suffix: base_suffix.to_string(),
                entity_type_keys: fm.entity_types.clone(),
                entity_ids: ids.clone(),
            });
        }
    }

    plans
}
