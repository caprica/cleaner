use std::path::PathBuf;

pub trait MediaFile {
    fn path(&self) -> &PathBuf;

    fn is_valid(&self) -> bool {
        true
    }
}
