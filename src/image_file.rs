use image_meta::ImageMeta;
use std::path::PathBuf;

use crate::media_file::MediaFile;

#[derive(Debug)]
pub enum ImageFileError {
}

#[derive(Debug)]
pub enum ImageMetaError {
    MissingMeta,
}

pub struct ImageFile {
    path: PathBuf,
    meta: Option<ImageMeta>,
    file_errors: Vec<ImageFileError>,
    meta_errors: Vec<ImageMetaError>,
}

impl ImageFile {
    pub fn new(path: PathBuf) -> ImageFile {
        let meta = image_meta::load_from_file(&path).ok();

        ImageFile {
            path,
            meta,
            file_errors: Vec::new(),
            meta_errors: Vec::new()
        }
    }

    pub fn meta(&self) -> &Option<ImageMeta> {
        &self.meta
    }

    pub fn file_errors(&self) -> &Vec<ImageFileError> {
        &self.file_errors
    }

    pub fn meta_errors(&self) -> &Vec<ImageMetaError> {
        &self.meta_errors
    }

    pub fn validate(&mut self) {
        self.reset_validation_state();
        self.validate_file();
        self.validate_meta();
    }

    fn reset_validation_state(&mut self) {
        self.file_errors.clear();
        self.meta_errors.clear();
    }

    fn validate_file(&mut self) {
    }

    fn validate_meta(&mut self) {
        if self.meta.is_none() {
            self.meta_errors.push(ImageMetaError::MissingMeta);
        }
    }
}

impl MediaFile for ImageFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn is_valid(&self) -> bool {
        self.file_errors.is_empty() && self.meta_errors.is_empty()
    }
}
