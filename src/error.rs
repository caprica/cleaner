use std::io;

use thiserror::Error;

pub type CleanerResult<T> = Result<T, CleanerError>;

#[derive(Debug, Error)]
pub enum CleanerError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Lofty(#[from] lofty::LoftyError),

    #[error("failed to process rar")]
    Unrar,

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error("missing file extension")]
    MissingFileExtension,

    #[error("unexpected file extension")]
    UnexpectedFileExtension,
}

impl<T> From<unrar::error::UnrarError<T>> for CleanerError {
    fn from(_: unrar::error::UnrarError<T>) -> Self {
        Self::Unrar
    }
}
