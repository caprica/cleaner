use std::path::PathBuf;

use image_meta::ImageMeta;

use crate::media_file::MediaFile;

pub struct ImageFile {
    path: PathBuf,
    meta: Option<ImageMeta>
}

impl ImageFile {
    pub fn new(path: PathBuf) -> ImageFile {
        let meta = image_meta::load_from_file(&path).ok();

        ImageFile {
            path,
            meta
        }
    }

    pub fn meta(&self) -> Option<&ImageMeta> {
        self.meta.as_ref()
    }
}

impl MediaFile for ImageFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}
