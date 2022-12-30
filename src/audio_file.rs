use std::path::PathBuf;
use lofty::{Accessor, TaggedFileExt, Tag, ItemKey};

use crate::{media_file::MediaFile, audio_file_meta::{AudioFileMeta}, tagger::get_tagged_file, audio_file_path_util::{decompose_artist_path, decompose_file_path, decompose_album_path}};

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

        let tagged_file = get_tagged_file(&path);

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
                .or_else(|| tag.artist().map(|s| s.to_string()))
                .or_else(|| path_artist_name.as_ref().map(|p| p.to_string()));

            // Artist name from artist tag, album artist tag, or artist directory name
            artist_name = tag.artist()
                .map(|s| s.to_string())
                .or_else(|| Self::get_album_artist(tag))
                .or_else(|| path_artist_name.as_ref().map(|s| s.to_owned()));

            // Album title from tag, or album directory name
            album_title = tag.album()
                .map(|s| s.to_string())
                .or_else(|| path_album_title.map(|s| s.to_owned()));

            // Year from tag, or album directory name
            year = tag.year()
                .or_else(|| path_album_year);

            // Track number from tag, or file name
            track_number = tag.track()
                .or_else(|| path_track_number);

            // Track title from tag, or file name
            track_title = tag.title()
                .map(|s| s.to_string())
                .or_else(|| path_track_title.map(|s| s.to_owned()));

            // Genre from tag, no fallback available
            genre = tag.genre()
                .map(|s| s.to_string());
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
            .map(|s| s.to_string())
    }
}

impl MediaFile for AudioFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}
