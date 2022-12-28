use std::path::PathBuf;

use lofty::{Probe, Tag, Accessor, TagExt, TaggedFile, ItemKey, TagType, TaggedFileExt};

use crate::audio_file_meta::AudioFileMeta;

pub fn get_tagged_file(path: &PathBuf) -> TaggedFile {
	Probe::open(path)
        .expect("Failed to open file")
        .read()
        .expect("Failed to read file")
}

pub fn clean_tags(path: &PathBuf, meta: &AudioFileMeta, total_tracks: u32) {
	let mut tagged_file = get_tagged_file(path);

    remove_tags(path, &mut tagged_file);

    // Primarily use ID3v2
    add_tag(&mut tagged_file, TagType::ID3v2, meta, total_tracks)
        .save_to_path(path)
        .expect("Failed to write ID3v2 tag");

    // Add ID3v1 for fallback/compatibility
    add_tag(&mut tagged_file, TagType::ID3v1, meta, total_tracks)
        .save_to_path(path)
        .expect("Failed to write ID3v1 tag");
}

fn remove_tags(path: &PathBuf, tagged_file: &mut TaggedFile) {
    remove_tag(path, tagged_file, TagType::ID3v1);
    remove_tag(path, tagged_file, TagType::ID3v2);
}

fn remove_tag(path: &PathBuf, tagged_file: &mut TaggedFile, tag_type: TagType) {
    if let Some(tag) = tagged_file.remove(tag_type) {
        tag.remove_from_path(path).expect("Failed to remove tag");
    }
}

fn add_tag<'a>(tagged_file: &'a mut TaggedFile, tag_type: TagType, meta: &AudioFileMeta, total_tracks: u32) -> &'a Tag {
    tagged_file
        .insert_tag(Tag::new(tag_type));

    let tag = tagged_file
        .tag_mut(tag_type)
        .expect("Tag must have been inserted");

    // See https://docs.rs/lofty/latest/lofty/enum.ItemKey.html

    if let Some(album_artist_name) = meta.album_artist_name() {
        tag.insert_text(ItemKey::AlbumArtist, album_artist_name.to_string());
    }

    if let Some(artist_name) = meta.artist_name() {
        tag.set_artist(artist_name.to_string());
    }

    if let Some(album_title) = meta.album_title() {
        tag.set_album(album_title.to_string());
    }

    if let Some(year) = meta.year() {
        match tag_type {
            TagType::ID3v1 => {
                tag.insert_text(ItemKey::Year, year.to_string());
            },
            TagType::ID3v2 => {
                tag.set_year(year);
            }
            _ => panic!("Unexpected tag type")
        };
        tag.insert_text(ItemKey::RecordingDate, year.to_string());
    }

    if let Some(track_number) = meta.track_number() {
        match tag_type {
            TagType::ID3v1 => {
                tag.set_track(track_number);
            },
            TagType::ID3v2 => {
                tag.insert_text(ItemKey::TrackNumber, format!("{}/{}", track_number.to_string(), total_tracks));
            }
            _ => panic!("Unexpected tag type")
        };
    }

    if let Some(track_title) = meta.track_title() {
        tag.set_title(track_title.to_string());
    }

    if let Some(genre) = meta.genre() {
        tag.set_genre(genre.to_string());
    }

    tag
}
