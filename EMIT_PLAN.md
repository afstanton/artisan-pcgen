# PCGen File Emission Plan

## Purpose

This document describes the design for emitting a `ParsedCatalog` (artisan-core
`CoreCatalog`) as a set of PCGen `.lst` and `.pcc` files organized according to
PCGen's directory/file conventions, with a standardized default nomenclature and
a manifest-driven override system.

---

## Goals

1. Accept any `ParsedCatalog` regardless of how it was built (parsed from PCGen
   files, loaded from a TOML store, assembled programmatically, etc.).
2. Emit a complete, valid PCGen file tree: one directory per source, LST files
   per entity type, and a wiring PCC file per source.
3. Produce an in-memory file map (`BTreeMap<PathBuf, String>`) as the primary
   artifact — easy to test, preview, and diff.
4. Write that map to disk via a thin wrapper.
5. Allow a `Manifest` to override any default: output root, directory paths,
   file names, entity selection, and subtype splitting.
6. Infer all naming from catalog metadata (source title, publisher, game system,
   SOURCESHORT); fail explicitly if required metadata is missing.

---

## Conceptual Overview

```
ParsedCatalog
  ├─ publishers: Vec<PublisherRecord>
  ├─ sources:    Vec<SourceRecord>        ← one directory per source
  ├─ citations:  Vec<CitationRecord>      ← entity → source association
  └─ entities:   Vec<Entity>             ← grouped by (source, entity_type)

             ▼  plan_emission()

EmitPlan
  └─ Vec<SourcePlan>
       ├─ path: "data/35e/wizards_of_the_coast/complete_divine/"
       ├─ pcc:  "_complete_divine.pcc"
       └─ Vec<FilePlan>
            ├─ "cd_abilities.lst"  ← ability entities
            ├─ "cd_feats.lst"      ← feat entities
            └─ ...

             ▼  execute_plan()

BTreeMap<PathBuf, String>   ← { path → file content }

             ▼  write_files()

files on disk
```

---

## Naming Conventions

### Directory Structure

```
{output_root}/
  data/
    {game_system_slug}/
      {publisher_slug}/
        {source_slug}/
          _{source_slug}.pcc
          {abbrev}_{entity_type_suffix}.lst
          ...
```

**Examples:**
```
data/35e/wizards_of_the_coast/complete_divine/
  _complete_divine.pcc
  cd_abilities.lst
  cd_feats.lst
  cd_classes.lst
  cd_equipment.lst
  cd_spells.lst
  cd_templates.lst
  cd_deities.lst

data/pathfinder/paizo/ultimate_combat/
  _ultimate_combat.pcc
  uc_abilities.lst
  uc_feats.lst
  ...
```

### Slug Derivation

| Field | Source | Rule |
|---|---|---|
| `game_system_slug` | `SourceRecord.game_systems[0]` | Lowercase, strip non-alphanumeric, collapse spaces/dots to nothing: `"3.5e"→"35e"`, `"Pathfinder 1e"→"pathfinder1e"`, `"Pathfinder"→"pathfinder"` |
| `publisher_slug` | `PublisherRecord.name` | Lowercase, replace spaces/punctuation with `_`, collapse runs: `"Wizards of the Coast"→"wizards_of_the_coast"`, `"Paizo Inc."→"paizo"` |
| `source_slug` | `SourceRecord.title` | Same as publisher slug: `"Complete Divine"→"complete_divine"`, `"Ultimate Combat"→"ultimate_combat"` |
| `abbrev` | Entity attribute `source_short`, else derived | Prefer `source_short` (e.g. `"CD"`→`"cd"`). If absent, take first letter of each significant word in the source title, lowercased. Truncate to 8 chars. |

**Collision policy:** publisher slug is always included in the path (it is part
of the canonical directory structure, not just a disambiguator). Two sources
with the same slug under the same publisher are disambiguated by appending the
edition: `complete_divine_35e/` vs `complete_divine_5e/`.

### Entity Type → File Suffix

| `pcgen_entity_type_key` | LST file suffix | PCC token |
|---|---|---|
| `pcgen:entity:ability` | `abilities` | `ABILITY` |
| `pcgen:entity:abilitycategory` | `abilitycategories` | `ABILITYCATEGORY` |
| `pcgen:entity:class` | `classes` | `CLASS` |
| `pcgen:entity:subclass` | `classes` | `CLASS` (same file as classes) |
| `pcgen:entity:classlevel` | _(merged into classes file)_ | — |
| `pcgen:entity:deity` | `deities` | `DEITY` |
| `pcgen:entity:equipment` | `equipment` | `EQUIPMENT` |
| `pcgen:entity:feat` | `feats` | `FEAT` |
| `pcgen:entity:kit` | `kits` | `KIT` |
| `pcgen:entity:language` | `languages` | `LANGUAGE` |
| `pcgen:entity:race` | `races` | `RACE` |
| `pcgen:entity:skill` | `skills` | `SKILL` |
| `pcgen:entity:spell` | `spells` | `SPELL` |
| `pcgen:entity:template` | `templates` | `TEMPLATE` |
| `pcgen:entity:companionmod` | `companionmods` | `COMPANIONMOD` |
| `pcgen:entity:gear` | `equipment` | `EQUIPMENT` (merged with equipment) |
| `pcgen:entity:startpack` | `kits` | `KIT` (merged with kits) |
| `pcgen:entity:variable` | `variables` | `VARIABLE` |
| `pcgen:entity:modify` | _(skip — system-level)_ | — |
| unknown / `pcgen:entity:pcc-*` | _(skip — metadata, not content)_ | — |

Entity types marked "merged" are co-emitted into another file; types marked
"skip" are not emitted into LST files (they drive structure, not content).

### Subtype Splitting (Manifest Override)

By default each entity type maps to a single file. A manifest can request
splitting by the value of a nominated attribute, typically `type`:

```
abilities → split by first TYPE segment:
  cd_abilities_class.lst     (TYPE contains "Class Feature" / "ClassFeature")
  cd_abilities_race.lst      (TYPE contains "Racial")
  cd_abilities_other.lst     (everything else)
```

The split dimension and bucket labels are manifest-specified; the emitter does
not auto-detect them.

---

## Source Attribution

An entity is associated with one or more sources via `CitationRecord`:

```
CitationRecord {
  subject: SubjectRef::Entity(entity_id),
  source:  source_id,
  locators: [{ kind: "page", value: "p.42", canonical: true }],
}
```

**Multi-source rule:** an entity is emitted into the LST file of *every* source
that has a citation for it. This matches PCGen's runtime behavior: users can
enable or disable individual source books, so an entity that exists in two
books must appear in both to remain accessible regardless of which sources the
user loads.

If the entity has no citations at all, it goes into a catch-all file
`_uncited.lst` at the output root, with a warning logged.

The manifest's per-source `entity_ids` filter restricts which entities are
emitted for a given source (useful when a UI lets the user uncheck specific
entries). When no filter is specified, all entities cited by that source are
emitted.

---

## PCC File Structure

Each source directory contains one PCC file named `_{source_slug}.pcc`.

```
# Generated by artisan-pcgen
SOURCELONG:{source.title}	SOURCESHORT:{abbrev}	SOURCEDATE:{source.edition or ""}

GAMEMODE:{gamemode}

ABILITY:{abbrev}_abilities.lst
FEAT:{abbrev}_feats.lst
CLASS:{abbrev}_classes.lst
...
```

`{gamemode}` is the raw `SourceRecord.game_systems[0]` value (PCGen expects the
GAMEMODE string unmodified). If `game_systems` is empty, we emit `GAMEMODE:35e`
with a warning.

PCC lines are emitted in the order defined by the entity-type table above.
Only lines for LST files that actually contain at least one entity are included.

---

## Manifest

The manifest is a serde-serializable Rust struct (TOML/JSON round-trips). Every
field is optional; absent fields fall back to defaults derived from the catalog.

```rust
/// Top-level manifest. Absent = emit everything from all sources.
pub struct EmitManifest {
    /// Root directory for output. Defaults to the current working directory.
    pub output_root: Option<PathBuf>,

    /// Per-source overrides. If empty, all sources are emitted with defaults.
    pub sources: Vec<SourceManifest>,

    /// If true, entities with no citation are emitted to a catch-all file
    /// rather than being silently skipped. Default: true.
    pub emit_uncited: Option<bool>,
}

pub struct SourceManifest {
    /// Identifies the source. At least one field must be populated.
    pub source_title: Option<String>,
    pub source_id: Option<Uuid>,

    /// Override the derived directory path (relative to output_root).
    pub directory: Option<PathBuf>,

    /// Override the derived abbreviation / file prefix.
    pub abbreviation: Option<String>,

    /// Override the GAMEMODE string in the PCC file.
    pub gamemode: Option<String>,

    /// If true, skip generating the PCC file.
    pub skip_pcc: Option<bool>,

    /// Per-file overrides. If empty, default one-file-per-entity-type applies.
    pub files: Vec<FileManifest>,

    /// Explicit entity selection: only emit entities whose canonical ID is in
    /// this list. If empty, all entities cited by this source are emitted.
    /// (Entities cited by multiple sources are always emitted to each source's
    /// files — this filter only restricts which of those are included here.)
    pub entity_ids: Vec<Uuid>,
}

pub struct FileManifest {
    /// Entity types to include in this file (by entity_type_key).
    pub entity_types: Vec<String>,

    /// Override the derived filename (without directory prefix).
    pub filename: Option<String>,

    /// Split this file by the value of an entity attribute.
    /// e.g.: split_by = "type", producing one file per distinct type bucket.
    pub split_by: Option<SplitSpec>,
}

pub struct SplitSpec {
    /// The entity attribute whose value drives splitting (e.g. "type").
    pub attribute: String,

    /// Explicit buckets: { bucket_label → list of values that map to it }.
    /// Values not matched by any bucket go to an "other" file.
    pub buckets: Vec<SplitBucket>,
}

pub struct SplitBucket {
    pub label: String,        // used as the filename suffix, e.g. "class"
    pub values: Vec<String>,  // attribute values that belong to this bucket
}
```

**TOML example:**

```toml
output_root = "output/my_campaign"

[[sources]]
source_title = "Complete Divine"
abbreviation = "cd"

[[sources.files]]
entity_types = ["pcgen:entity:ability"]
split_by = { attribute = "type", buckets = [
  { label = "class",  values = ["CLASS FEATURE", "ClassFeature"] },
  { label = "race",   values = ["Racial"] },
] }
# → cd_abilities_class.lst, cd_abilities_race.lst, cd_abilities_other.lst

[[sources.files]]
entity_types = ["pcgen:entity:feat"]
# → cd_feats.lst (default name, no split)
```

---

## API

```rust
// 1. Build the emission plan (pure, no I/O).
pub fn plan_emission(
    catalog: &ParsedCatalog,
    manifest: Option<&EmitManifest>,
) -> Result<EmitPlan, EmitError>;

// 2. Execute the plan into an in-memory file map (pure, no I/O).
pub fn execute_plan(
    plan: &EmitPlan,
    catalog: &ParsedCatalog,
) -> Result<BTreeMap<PathBuf, String>, EmitError>;

// 3. Write the file map to disk.
pub fn write_files(
    files: &BTreeMap<PathBuf, String>,
    base_dir: &Path,
) -> std::io::Result<()>;

// Convenience: plan + execute + write in one call.
pub fn emit_catalog(
    catalog: &ParsedCatalog,
    manifest: Option<&EmitManifest>,
    base_dir: &Path,
) -> Result<BTreeMap<PathBuf, String>, EmitError>;
```

`EmitPlan` is an intermediate representation that is inspectable (for UI
preview, dry-run, etc.) and testable independently of the I/O layer.

```rust
pub struct EmitPlan {
    pub sources: Vec<SourcePlan>,
    pub uncited_file: Option<FilePlan>,  // catch-all for uncited entities
    pub warnings: Vec<String>,           // non-fatal issues (missing metadata, etc.)
}

pub struct SourcePlan {
    pub source_id: CanonicalId,
    pub directory: PathBuf,          // relative to output_root
    pub pcc_filename: Option<PathBuf>,
    pub files: Vec<FilePlan>,
}

pub struct FilePlan {
    pub filename: PathBuf,           // relative to source directory
    pub entity_type_keys: Vec<String>,
    pub entity_ids: Vec<CanonicalId>,
}
```

---

## Error Handling

```rust
pub enum EmitError {
    /// Source referenced in manifest not found in catalog.
    SourceNotFound { title: String },

    /// Source has no game_systems; cannot derive GAMEMODE or directory.
    MissingGameSystem { source_title: String },

    /// Source has no publisher; cannot derive publisher slug.
    MissingPublisher { source_title: String },

    /// Directory path collision between two sources after slug derivation.
    PathCollision { path: PathBuf, sources: Vec<String> },

    /// Filename collision within a source directory.
    FilenameCollision { path: PathBuf },

    /// I/O error during write_files.
    Io(std::io::Error),
}
```

Warnings (non-fatal) are collected into `EmitPlan::warnings` and logged, but do
not abort emission:
- Entity has no citation → emitted to uncited catch-all file.
- `source_short` attribute missing → abbreviation derived from title initials.
- Source has multiple game systems → first is used, rest logged.

---

## Implementation Modules

```
src/
  emit/                         ← new module
    mod.rs                      ← public API (plan_emission, execute_plan, etc.)
    manifest.rs                 ← EmitManifest, SourceManifest, FileManifest, SplitSpec
    plan.rs                     ← EmitPlan, SourcePlan, FilePlan, planning logic
    slugify.rs                  ← slug/abbreviation derivation helpers
    entity_type_map.rs          ← entity_type_key → (suffix, PCC token) mapping
    pcc.rs                      ← PCC file generation
    error.rs                    ← EmitError
```

The existing `src/emit.rs` (single-entity token emitter) is unchanged; the new
`src/emit/` module sits alongside it and calls through to `unparse_catalog_to_text`
(or the lower-level per-entity emitter) to produce LST file content.

---

## Testing Strategy

1. **Unit tests on slug derivation:** property-style tests for all the known
   publisher/source/game-system strings in the corpus to validate slug output.

2. **Unit tests on `entity_type_map`:** every entity type key maps to exactly
   one (suffix, PCC token) pair; no duplicates.

3. **`plan_emission` roundtrip test:** parse the PCGen corpus → `plan_emission`
   → check that the resulting `SourcePlan` directories and file lists match the
   original corpus layout for well-known source books.

4. **`execute_plan` content tests:** parse a small fixture source → plan →
   execute → re-parse the emitted LST files → check entity count and attribute
   equality (reusing the existing semantic roundtrip infrastructure).

5. **Manifest override tests:** verify that manifest-specified filenames,
   entity selections, and subtype splits are respected.

6. **`write_files` integration test:** write to a `tempdir`, scan resulting
   files, assert presence of expected paths.

---

## Open Questions / Future Work

- **Ordering within LST files:** PCGen files often have entities in a particular
  order (alphabetical by name, or by level/CR). The default emission order is
  the order entities appear in `catalog.entities`. A manifest field
  `sort_by: Option<String>` could be added later.
- **Dependency ordering across sources:** some PCC files `INCLUDE` others. If
  the catalog contains cross-source prerequisite relationships, a future
  `include_sources: Vec<String>` field in `SourceManifest` could emit those
  includes in the PCC.
- **Custom LST header comments:** PCGen files conventionally begin with a
  `# CVS $Revision:` or similar comment block and column-label comment lines.
  A manifest field for custom header text could be added.
- **COVER art and LOGO tokens in PCC:** some PCC files reference image assets.
  These are not in the `ParsedCatalog` domain model today and are out of scope.
