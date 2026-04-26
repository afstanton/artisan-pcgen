use std::path::PathBuf;

#[derive(Debug)]
pub enum EmitError {
    /// Source referenced in manifest not found in catalog.
    SourceNotFound { title: String },

    /// Source has no game_systems; cannot derive GAMEMODE or directory path.
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

impl std::fmt::Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceNotFound { title } => write!(f, "source not found in catalog: {title:?}"),
            Self::MissingGameSystem { source_title } => {
                write!(f, "source {source_title:?} has no game_systems")
            }
            Self::MissingPublisher { source_title } => {
                write!(f, "source {source_title:?} has no publisher")
            }
            Self::PathCollision { path, sources } => {
                write!(f, "path collision at {:?}: {:?}", path, sources)
            }
            Self::FilenameCollision { path } => {
                write!(f, "filename collision at {:?}", path)
            }
            Self::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for EmitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EmitError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
