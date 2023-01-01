use std::path::PathBuf;

use lazy_static::lazy_static;
use lofty::{Accessor, TaggedFileExt, Tag, ItemKey};
use regex::Regex;

use crate::{media_file::MediaFile, audio_file_meta::{AudioFileMeta, AudioFileType}, tagger::get_tagged_file};

pub struct AudioFile {
    path: PathBuf,
    meta: AudioFileMeta
}

impl AudioFile {
    pub fn new(root_path: &PathBuf, path: PathBuf) -> AudioFile {
        let meta = Self::build_meta(root_path, &path);

        AudioFile {
            path,
            meta
        }
    }

    pub fn get_meta(&self) -> &AudioFileMeta {
        &self.meta
    }

    fn build_meta(root_path: &PathBuf, path: &PathBuf) -> AudioFileMeta {
        let path_artist_name = decompose_artist_path(root_path, path);

        let (
            path_album_title,
            path_album_year
        ) = decompose_album_path(root_path, path);

        let (
            path_track_number,
            path_track_title,
            audio_file_type
        ) = decompose_file_path(path);

        let tagged_file = get_tagged_file(&path).expect("Failed to get tagged file");

        let album_artist_name: Option<String>;
        let artist_name: Option<String>;
        let album_title: Option<String>;
        let year: Option<u32>;
        let track_number: Option<u32>;
        let track_title: Option<String>;
        let genre: Option<String>;

        if let Some(tag) = tagged_file.primary_tag() {
            // Album artist name from album artist tag, artist tag, or artist directory name
            album_artist_name = Self::get_album_artist(tag)
                .or_else(|| tag.artist().map(|s| s.trim().to_string()))
                .or_else(|| path_artist_name.as_ref().map(|s| s.to_string()));

            // Artist name from artist tag, album artist tag, or artist directory name
            artist_name = tag.artist()
                .map(|s| s.trim().to_string())
                .or_else(|| Self::get_album_artist(tag))
                .or_else(|| path_artist_name.as_ref().map(|s| s.to_owned()));

            // Album title from tag, or album directory name
            album_title = tag.album()
                .map(|s| s.trim().to_string())
                .or_else(|| path_album_title.map(|s| s.to_owned()));

            // Year from tag, or album directory name
            year = tag.year()
                .or_else(|| path_album_year);

            // Track number from tag, or file name
            track_number = tag.track()
                .or_else(|| path_track_number);

            // Track title from tag, or file name
            track_title = tag.title()
                .map(|s| s.trim().to_string())
                .or_else(|| path_track_title.map(|s| s.to_owned()));

            // Genre from tag, no fallback available
            genre = tag.genre()
                .map(|s| s.trim().to_string());
        } else {
            album_artist_name = path_artist_name.as_ref().map(|s| s.to_owned());
            artist_name = path_artist_name.as_ref().map(|s| s.to_owned());
            album_title = path_album_title.map(|s| s.to_owned());
            year = path_album_year;
            track_number = path_track_number;
            track_title = path_track_title.map(|s| s.to_owned());
            genre = None;
        }

        AudioFileMeta::new(
            tagged_file,
            album_artist_name,
            artist_name,
            album_title,
            year,
            track_number,
            track_title,
            genre,
            audio_file_type
        )
    }

    fn get_album_artist(tag: &Tag) -> Option<String> {
        tag
            .get_string(&ItemKey::AlbumArtist)
            .map(|s| s.trim().to_string())
    }
}

impl MediaFile for AudioFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}

fn decompose_artist_path(root_path: &PathBuf, path: &PathBuf) -> Option<String> {
    path
        .parent()
        .and_then(|p| p.parent())
        .filter(|p| p != root_path)
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .map(|s| s.trim().to_string())
}

fn decompose_album_path(root_path: &PathBuf, path: &PathBuf) -> (Option<String>, Option<u32>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.+)\s\(?(\d{4})\)?$").unwrap();
    }

    let album_path_name = path
        .parent()
        .filter(|p| p != root_path)
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str());

    if let Some(album_path_name) = album_path_name {
        if let Some(captures) = RE.captures(album_path_name) {
            let album_title = captures
                .get(1)
                .map(|m| m.as_str())
                .map(|s| s.trim().to_string());

            let album_year = captures
                .get(2)
                .map(|m| m.as_str())
                .and_then(|s| s.parse::<u32>().ok());

            return (album_title, album_year);
        }
    }

    (None::<String>, None::<u32>)
}

fn decompose_file_path(path: &PathBuf) -> (Option<u32>, Option<String>, Option<AudioFileType>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?:(\d+)\.?(?:\s\-\s|\s))?(.+)$").unwrap();
    }

    let audio_file_type = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .and_then(|s| Some(s.parse::<AudioFileType>()))
        .and_then(|r| r.ok());

    if let Some(file_stem) = path
        .file_stem()
        .and_then(|s| s.to_str()) {
            if let Some(captures) = RE.captures(file_stem) {
                let track_number = captures
                    .get(1)
                    .map(|m| m.as_str())
                    .and_then(|s| s.parse::<u32>().ok());

                let track_name = captures
                    .get(2)
                    .map(|m| m.as_str())
                    .map(|s| s.trim().to_string());

                return (track_number, track_name, audio_file_type);
            }
            return (None::<u32>, Some(file_stem.to_string()), audio_file_type);
        }

    (None::<u32>, None::<String>, audio_file_type)
}
