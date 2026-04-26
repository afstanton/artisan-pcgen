use std::path::PathBuf;

use artisan_pcgen::{
    file_emit::{EmitManifest, plan_emission, execute_plan, write_files},
    parse_text_to_catalog,
};

fn small_catalog() -> artisan_core::ParsedCatalog {
    // Minimal LST content: one feat, one race.
    let lst = "\
SOURCELNG:Test Source\tSOURCESHORT:TST\tSOURCEDATE:2024\tSOURCEWEB:http://example.com
SOURCELONG:Test Source\tSOURCESHORT:TST\tSOURCEDATE:2024\tSOURCEWEB:http://example.com
Iron Will\tTYPE:General\tDESC:You have a stronger will.
# Race line
Human\tTYPE:Humanoid\tSIZE:M\tMOVE:Walk,30
";
    parse_text_to_catalog(lst, "test/tst_feats.lst", "lst")
}

#[test]
fn plan_emission_succeeds_and_collects_warnings() {
    // A catalog parsed from a basic LST should produce a plan without hard errors.
    // Warnings (missing publisher, etc.) are accumulated rather than aborting.
    let catalog = small_catalog();
    let plan = plan_emission(&catalog, None).expect("plan_emission failed");

    // Any catalog with SourceRecords produces source plans.
    // Source plans may have warnings for missing metadata.
    for plan_source in &plan.sources {
        // Each source plan must have a non-empty title.
        assert!(!plan_source.source_title.is_empty());
    }

    // If there are entities with no citations, an uncited file should be produced
    // (since emit_uncited defaults to true).
    let has_uncited_entities = catalog.entities.iter().any(|e| {
        !catalog.citations.iter().any(|c| {
            matches!(&c.subject, artisan_core::domain::SubjectRef::Entity(id) if *id == e.id)
        })
    });
    if has_uncited_entities {
        assert!(
            plan.uncited_file.is_some(),
            "expected uncited file for entities with no citations"
        );
    }
}

#[test]
fn execute_plan_produces_file_map() {
    let catalog = small_catalog();
    let plan = plan_emission(&catalog, None).expect("plan_emission failed");
    let files = execute_plan(&plan, &catalog).expect("execute_plan failed");
    // Every file in the map should have non-empty content.
    for (path, content) in &files {
        assert!(!content.is_empty(), "file {:?} is empty", path);
    }
}

#[test]
fn write_files_creates_files_on_disk() {
    let catalog = small_catalog();
    let plan = plan_emission(&catalog, None).expect("plan_emission failed");
    let files = execute_plan(&plan, &catalog).expect("execute_plan failed");

    if files.is_empty() {
        return; // nothing to write
    }

    let dir = tempdir();
    write_files(&files, &dir).expect("write_files failed");

    for (rel_path, _) in &files {
        let full_path = dir.join(rel_path);
        assert!(full_path.exists(), "expected file {:?} to exist", full_path);
    }
}

#[test]
fn emit_uncited_false_skips_catch_all() {
    let catalog = small_catalog();
    let manifest = EmitManifest {
        emit_uncited: Some(false),
        ..Default::default()
    };
    let plan = plan_emission(&catalog, Some(&manifest)).expect("plan_emission failed");
    assert!(plan.uncited_file.is_none(), "expected no uncited file when emit_uncited=false");
}

// Minimal tempdir helper (avoids adding a dependency).
fn tempdir() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "artisan_pcgen_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos()
    ));
    std::fs::create_dir_all(&path).unwrap();
    path
}
