use std::{path::PathBuf, collections::BTreeMap};
use walkdir::WalkDir;

use crate::{audio_file::AudioFile, image_file::ImageFile, media_file::MediaFile, other_file::OtherFile};

pub struct Files {
    path: PathBuf,
    audio_files: Vec<AudioFile>,
    image_files: Vec<ImageFile>,
    other_files: Vec<OtherFile>
}

impl Files {

    pub fn new(path: PathBuf) -> Files {
        let mut files = Files {
            path,
            audio_files: Vec::new(),
            image_files: Vec::new(),
            other_files: Vec::new()
        };
        files.scan();
        files.validate();
        files
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn audio_files(&self) -> &Vec<AudioFile> {
        &self.audio_files
    }

    pub fn image_files(&self) -> &Vec<ImageFile> {
        &self.image_files
    }

    pub fn other_files(&self) -> &Vec<OtherFile> {
        &self.other_files
    }

    pub fn get_audio_file_map(&self) -> BTreeMap<PathBuf, Vec<&AudioFile>> {
        self.get_file_map(&self.audio_files)
    }

    pub fn get_image_file_map(&self) -> BTreeMap<PathBuf, Vec<&ImageFile>> {
        self.get_file_map(&self.image_files)
    }

    pub fn get_other_file_map(&self) -> BTreeMap<PathBuf, Vec<&OtherFile>> {
        self.get_file_map(&self.other_files)
    }

    fn scan(&mut self) {
      for entry in WalkDir::new(&self.path).into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir()) {
            let directory = entry.path().parent();
            if directory.is_some() {
                let ext = entry.path().extension().and_then(|e| e.to_str());
                let file_path = entry.path().to_path_buf();
                match ext {
                    Some("mp3" | "flac") => self.audio_files.push(AudioFile::new(file_path)),
                    Some("png" | "jpg" | "jpeg") => self.image_files.push(ImageFile::new(file_path)),
                    _ => self.other_files.push(OtherFile::new(file_path))
                };
            }
        };

    }

    fn validate(&mut self) {
        self.validate_audio_files();
        self.validate_image_files();
        self.validate_other_files();
    }

    fn validate_audio_files(&mut self) {
        for audio_file in self.audio_files.iter_mut() {
            audio_file.validate(&self.path);
        }
    }

    fn validate_image_files(&mut self) {
        for image_file in self.image_files.iter_mut() {
            image_file.validate();
        }
    }

    fn validate_other_files(&mut self) {
        for other_file in self.other_files.iter_mut() {
            other_file.validate();
        }
    }

    fn get_file_map<'a, T: MediaFile>(&'a self, files: &'a Vec<T>) -> BTreeMap<PathBuf, Vec<&T>> {
        let mut map = files
            .into_iter()
            .fold(
                BTreeMap::<PathBuf, Vec<&T>>::new(),
                |mut acc, file| {
                    let parent = file
                        .path()
                        .parent()
                        .unwrap()
                        .to_path_buf();
                    let key = self.relative_path(parent);
                    acc.entry(key)
                        .or_insert_with(|| Vec::new())
                        .push(file);
                    acc
                }
            );
        map
            .values_mut()
            .for_each(|audio_files| {
                audio_files.sort_unstable_by_key(|v| v.path());
            });
        map
    }

    fn relative_path(&self, path: PathBuf) -> PathBuf {
        let prefix = self.path.to_str().unwrap();
        path.strip_prefix(prefix)
            .ok()
            .map(|p| p.to_path_buf())
            .unwrap()
    }

}
