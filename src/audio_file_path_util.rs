use std::path::PathBuf;
use lazy_static::lazy_static;
use regex::Regex;

use crate::audio_file_meta::{AudioFileType};

pub fn decompose_artist_path(root_path: &PathBuf, path: &PathBuf) -> Option<String> {
    path
        .parent()
        .and_then(|p| p.parent())
        .filter(|p| p != root_path)
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

pub fn decompose_album_path(root_path: &PathBuf, path: &PathBuf) -> (Option<String>, Option<u32>) {
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
                .map(|s| s.to_string());

            let album_year = captures
                .get(2)
                .map(|m| m.as_str())
                .and_then(|s| s.parse::<u32>().ok());

            return (album_title, album_year);
        }
    }

    (None::<String>, None::<u32>)
}

pub fn decompose_file_path(path: &PathBuf) -> (Option<u32>, Option<String>, Option<AudioFileType>) {
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
                    .map(|s| s.to_string());

                return (track_number, track_name, audio_file_type);
            }
            return (None::<u32>, Some(file_stem.to_string()), audio_file_type);
        }

    (None::<u32>, None::<String>, audio_file_type)
}
