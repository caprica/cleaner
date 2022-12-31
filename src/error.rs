use std::io;

use thiserror::Error;

pub type CleanerResult<T> = Result<T, CleanerError>;

#[derive(Debug, Error)]
pub enum CleanerError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Lofty(#[from] lofty::LoftyError),

    // #[error(transparent)]
    // Unrar(#[from] unrar::error::UnrarError<T>),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error("missing file extension")]
    MissingFileExtension,

    #[error("unexpected file extension")]
    UnexpectedFileExtension,

    #[error("failed to extract archive")]
    FailedToExtractArchive,
}
