use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
};

use artisan_core::ParsedCatalog;

mod entity_type_map;
mod error;
mod manifest;
mod pcc;
mod plan;
mod slugify;

pub use error::EmitError;
pub use manifest::{EmitManifest, FileManifest, SourceManifest, SplitBucket, SplitSpec};
pub use plan::{EmitPlan, FilePlan, SourcePlan};

use crate::emit_entity_auto;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Build an emission plan from a catalog and optional manifest (pure, no I/O).
pub fn plan_emission(
    catalog: &ParsedCatalog,
    manifest: Option<&EmitManifest>,
) -> Result<EmitPlan, EmitError> {
    plan::plan_emission(catalog, manifest)
}

/// Execute a plan into an in-memory file map (pure, no I/O).
///
/// Keys are paths relative to `output_root` (or the current directory if none
/// was specified in the manifest).
pub fn execute_plan(
    plan: &EmitPlan,
    catalog: &ParsedCatalog,
) -> Result<BTreeMap<PathBuf, String>, EmitError> {
    let entity_by_id: HashMap<artisan_core::CanonicalId, &artisan_core::Entity> = catalog
        .entities
        .iter()
        .map(|e| (e.id, e))
        .collect();

    let mut file_map: BTreeMap<PathBuf, String> = BTreeMap::new();

    for source_plan in &plan.sources {
        // Emit each LST file.
        for file_plan in &source_plan.files {
            if file_plan.entity_ids.is_empty() {
                continue;
            }

            let mut lines: Vec<String> = Vec::new();
            for &id in &file_plan.entity_ids {
                let entity = match entity_by_id.get(&id) {
                    Some(e) => e,
                    None => continue,
                };
                if let Some(line) = emit_entity_auto(entity) {
                    lines.push(line);
                }
            }

            if lines.is_empty() {
                continue;
            }

            let path = source_plan.directory.join(&file_plan.filename);
            file_map.insert(path, lines.join("\n") + "\n");
        }

        // Emit PCC file.
        if let Some(pcc_filename) = &source_plan.pcc_filename {
            let pcc_content = pcc::generate_pcc(source_plan);
            let path = source_plan.directory.join(pcc_filename);
            file_map.insert(path, pcc_content);
        }
    }

    // Emit uncited catch-all.
    if let Some(uncited) = &plan.uncited_file {
        let mut lines: Vec<String> = Vec::new();
        for &id in &uncited.entity_ids {
            if let Some(entity) = entity_by_id.get(&id) {
                if let Some(line) = emit_entity_auto(entity) {
                    lines.push(line);
                }
            }
        }
        if !lines.is_empty() {
            file_map.insert(uncited.filename.clone(), lines.join("\n") + "\n");
        }
    }

    Ok(file_map)
}

/// Write a file map to disk under `base_dir`.
pub fn write_files(
    files: &BTreeMap<PathBuf, String>,
    base_dir: &Path,
) -> std::io::Result<()> {
    for (rel_path, content) in files {
        let full_path = base_dir.join(rel_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full_path, content)?;
    }
    Ok(())
}

/// Convenience: plan + execute + write in one call.
///
/// Returns the in-memory file map (same as `execute_plan`) in addition to
/// writing files to disk.
pub fn emit_catalog(
    catalog: &ParsedCatalog,
    manifest: Option<&EmitManifest>,
    base_dir: &Path,
) -> Result<BTreeMap<PathBuf, String>, EmitError> {
    let plan = plan_emission(catalog, manifest)?;
    let files = execute_plan(&plan, catalog)?;
    write_files(&files, base_dir)?;
    Ok(files)
}
