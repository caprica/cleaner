use std::path::PathBuf;

use crate::media_file::MediaFile;

pub struct OtherFile {
    path: PathBuf
}

impl OtherFile {
    pub fn new(path: PathBuf) -> OtherFile {
        OtherFile {
            path
        }
    }

    pub fn validate(&self) {
    }
}

impl MediaFile for OtherFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}
