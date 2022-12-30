use std::{path::PathBuf, collections::BTreeSet};

use walkdir::WalkDir;
use zip::result::ZipResult;
use zip_extensions::zip_extract;

pub fn get_archive_paths(path: &PathBuf) -> BTreeSet<PathBuf> {
    let walker = WalkDir::new(&path)
        .min_depth(1)
        .max_depth(1);

    walker.into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                return ext == "zip";
            }
            false
        })
        .map(|e| e.into_path())
        .collect::<BTreeSet<PathBuf>>()
}

pub fn extract_archive(archive_path: &PathBuf, output_path: &PathBuf) -> ZipResult<()> {
    zip_extract(archive_path, output_path)
}
