use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to write {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to run `{program}`: {source}")]
    Spawn {
        program: String,
        #[source]
        source: io::Error,
    },
    #[error("failed to write to `{program}`: {source}")]
    CommandInput {
        program: String,
        #[source]
        source: io::Error,
    },
    #[error("failed to open {path}: {source}")]
    Open {
        path: PathBuf,
        #[source]
        source: opener::OpenError,
    },
    #[error("SVG compression failed: {0}")]
    Compress(String),
    #[error("unknown shell; specify one with --shell")]
    UnknownShell,
    #[error("--open requires --output")]
    OpenWithoutOutput,
}

pub type Result<T> = std::result::Result<T, Error>;
