use std::path::{PathBuf};
use lazy_static::lazy_static;
use lofty::{TaggedFile, Accessor, ItemKey};
use regex::Regex;

use crate::{media_file::MediaFile, audio_file_meta::{AudioFileMeta, AudioFileType}, tagger::get_tagged_file};

pub struct AudioFile {
    path: PathBuf,
    tagged_file: TaggedFile,
    album_path: Option<PathBuf>,
    artist_path: Option<PathBuf>,
}

impl AudioFile {
    pub fn new(root_path: &PathBuf, path: PathBuf) -> AudioFile {
        let tagged_file = get_tagged_file(&path);

        let album_path = path
            .parent()
            .map(|p| p.to_path_buf())
            .filter(|p| p != root_path);

        let artist_path = path
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .filter(|p| p != root_path);

        AudioFile {
            path,
            tagged_file,
            album_path,
            artist_path
        }
    }

    pub fn get_audio_file_meta(&self) -> AudioFileMeta {
        let tag = self.tagged_file.primary_tag();

        let path_artist_name = self.decompose_artist_path();

        let (
            path_album_title,
            path_album_year
        ) = self.decompose_album_path();

        let (
            path_track_number,
            path_track_title,
            audio_file_type
        ) = self.decompose_file_path();

        // Album artist name from album artist tag, artist tag, or artist directory name
        let album_artist_name = tag
            .and_then(|t| t.get_string(&ItemKey::AlbumArtist).or(t.artist()))
            .or_else(|| path_artist_name)
            .map(|s| s.to_string());

        // Artist name from artist tag, album artist tag, or artist directory name
        let artist_name = tag
            .and_then(|t| t.artist().or(t.get_string(&ItemKey::AlbumArtist)))
            .or_else(|| path_artist_name)
            .map(|s| s.to_string());

        // Album title from tag, or album directory name
        let album_title = tag
            .and_then(|t| t.album())
            .or_else(|| path_album_title)
            .map(|s| s.to_string());

        // Year from tag, or album directory name
        let year = tag
            .and_then(|t| t.year())
            .or_else(|| path_album_year);

        // Track number from tag, or file name
        let track_number = tag
            .and_then(|t| t.track())
            .or_else(|| path_track_number);

        // Track title from tag, or file name
        let track_title = tag
            .and_then(|t| t.title())
            .or_else(|| path_track_title)
            .map(|s| s.to_string());

        // Genre from tag, no fallback available
        let genre = tag
            .and_then(|t| t.genre())
            .map(|s| s.to_string());

        AudioFileMeta::new(
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

    fn decompose_artist_path(&self) -> Option<&str> {
        self.artist_path.as_ref()
            .and_then(|p| p.file_name())
            .and_then(|f| f.to_str())
    }

    fn decompose_album_path(&self) -> (Option<&str>, Option<u32>) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+)\s\(?(\d{4})\)?$").unwrap();
        }

        let album_path_name = self.album_path.as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str());

        if let Some(album_path_name) = album_path_name {
            if let Some(captures) = RE.captures(album_path_name) {
                let album_title = captures
                    .get(1)
                    .map(|m| m.as_str());

                let album_year = captures
                    .get(2)
                    .map(|m| m.as_str())
                    .and_then(|s| s.parse::<u32>().ok());

                return (album_title, album_year);
            }
        }

        (None::<&str>, None::<u32>)
    }

    fn decompose_file_path(&self) -> (Option<u32>, Option<&str>, Option<AudioFileType>) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?:(\d+)\.?(?:\s\-\s|\s))?(.+)$").unwrap();
        }

        let audio_file_type = self.path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .and_then(|s| Some(s.parse::<AudioFileType>()))
            .and_then(|r| r.ok());

        if let Some(file_stem) = self.path
            .file_stem()
            .and_then(|s| s.to_str()) {
            if let Some(captures) = RE.captures(file_stem) {
                let track_number = captures
                    .get(1)
                    .map(|m| m.as_str())
                    .and_then(|s| s.parse::<u32>().ok());

                let track_name = captures
                    .get(2)
                    .map(|m| m.as_str());

                return (track_number, track_name, audio_file_type);
            }
            return (None::<u32>, Some(file_stem), audio_file_type);
        }

        (None::<u32>, None::<&str>, audio_file_type)
    }

}

impl MediaFile for AudioFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}
