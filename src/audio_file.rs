use std::path::{PathBuf};
use audiotags::{AudioTag, Tag};
use lazy_static::lazy_static;
use regex::Regex;

use crate::media_file::MediaFile;

#[derive(Debug)]
pub enum AudioFileError {
    MissingAlbumDirectory,
    MissingArtistDirectory,
    InvalidFilename,
    MissingFilename,
}

#[derive(Debug)]
pub enum AudioTagError {
    MissingAlbumName,
    MissingArtistName,
    MissingGenre,
    MissingTag,
    MissingTrackTitle,
    MissingTrackNumber,
    MissingYear,
}

pub struct AudioFile {
    path: PathBuf,
    tag: Option<Box<dyn AudioTag>>,
    file_errors: Vec<AudioFileError>,
    tag_errors: Vec<AudioTagError>
}

impl AudioFile {
    pub fn new(path: PathBuf) -> AudioFile {
        let tag = Tag::new()
            .read_from_path(&path)
            .ok();

        AudioFile {
            path,
            tag,
            file_errors: Vec::new(),
            tag_errors: Vec::new()
        }
    }

    pub fn tag(&self) -> &Option<Box<dyn AudioTag>> {
        &self.tag
    }

    pub fn file_errors(&self) -> &Vec<AudioFileError> {
        &self.file_errors
    }

    pub fn tag_errors(&self) -> &Vec<AudioTagError> {
        &self.tag_errors
    }

    pub fn validate(&mut self, root_path: &PathBuf) {
        self.reset_validation_state();
        self.validate_file(root_path);
        self.validate_tag();
    }

    fn reset_validation_state(&mut self) {
        self.file_errors.clear();
        self.tag_errors.clear();
    }

    fn validate_file(&mut self, root_path: &PathBuf) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{2,3}\s.+$").unwrap();
        }

        let album_path = self.path.parent();
        if album_path.is_none() || album_path.unwrap() == root_path {
            self.file_errors.push(AudioFileError::MissingAlbumDirectory);
        }

        album_path.and_then(|p| p.parent())
            .map(|artist_path| {
                if artist_path == root_path {
                    self.file_errors.push(AudioFileError::MissingArtistDirectory);
                }
            })
            .unwrap_or_else(|| self.file_errors.push(AudioFileError::MissingArtistDirectory));

        self.path.file_stem().map(|file_stem| {
            let filename = file_stem.to_str();
            if !filename.is_some() {
                self.file_errors.push(AudioFileError::InvalidFilename);
            } else if !RE.is_match(filename.unwrap()) {
                self.file_errors.push(AudioFileError::InvalidFilename);
            }
        })
        .unwrap_or_else(|| self.file_errors.push(AudioFileError::MissingFilename));
    }

    fn validate_tag(&mut self) {
        self.tag.as_ref()
            .map(|tag| {
                if !tag.track_number().is_some() {
                    self.tag_errors.push(AudioTagError::MissingTrackNumber);
                }

                if !tag.title().is_some() {
                    self.tag_errors.push(AudioTagError::MissingTrackTitle);
                }

                if !tag.album_title().is_some() {
                    self.tag_errors.push(AudioTagError::MissingAlbumName);
                }

                if !tag.artist().is_some() {
                    self.tag_errors.push(AudioTagError::MissingArtistName);
                }

                if !tag.year().is_some() {
                    self.tag_errors.push(AudioTagError::MissingYear);
                }

                if !tag.genre().is_some() {
                    self.tag_errors.push(AudioTagError::MissingGenre);
                }
            })
            .unwrap_or_else(|| self.tag_errors.push(AudioTagError::MissingTag));
    }
}

impl MediaFile for AudioFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn is_valid(&self) -> bool {
        self.file_errors.is_empty() && self.tag_errors.is_empty()
    }
}
