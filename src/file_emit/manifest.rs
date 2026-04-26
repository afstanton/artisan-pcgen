use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Top-level emit manifest. All fields are optional; absent fields fall back to
/// defaults derived from the catalog.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmitManifest {
    /// Root directory for output. Defaults to the current working directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_root: Option<PathBuf>,

    /// Per-source overrides. Empty = emit all sources with defaults.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceManifest>,

    /// If true, entities with no citation are emitted to a catch-all file.
    /// Default: true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_uncited: Option<bool>,
}

/// Per-source override. At least one of `source_title` or `source_id` must be set.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceManifest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<Uuid>,

    /// Override the derived directory path (relative to output_root).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<PathBuf>,

    /// Override the derived abbreviation / file prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,

    /// Override the GAMEMODE string in the PCC file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamemode: Option<String>,

    /// If true, skip generating the PCC file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_pcc: Option<bool>,

    /// Per-file overrides. Empty = one file per entity type (default).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileManifest>,

    /// Explicit entity selection by canonical ID. Empty = all entities cited by
    /// this source.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_ids: Vec<Uuid>,
}

/// Per-file override within a source.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileManifest {
    /// Entity type keys to include in this file (e.g. "pcgen:entity:ability").
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_types: Vec<String>,

    /// Override filename (without directory prefix).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Split this file by the value of an entity attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_by: Option<SplitSpec>,
}

/// Describes how to split a file into multiple files by an entity attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitSpec {
    /// Entity attribute whose value drives splitting (e.g. "type").
    pub attribute: String,

    /// Explicit buckets. Values not matched by any bucket go to an "other" file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buckets: Vec<SplitBucket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitBucket {
    /// Used as the filename suffix (e.g. "class" → `{abbrev}_abilities_class.lst`).
    pub label: String,

    /// Attribute values that map to this bucket.
    pub values: Vec<String>,
}
