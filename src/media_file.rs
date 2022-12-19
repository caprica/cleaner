use std::path::PathBuf;

pub trait MediaFile {
    fn path(&self) -> &PathBuf;
}
