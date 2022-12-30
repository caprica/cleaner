use std::{io::Cursor, fs::File, path::PathBuf, ffi::OsStr, collections::HashSet};

use image::{DynamicImage, io::Reader};
use lofty::{TaggedFileExt, PictureType};

use crate::{image_file::ImageFile, media_file::MediaFile, audio_file::AudioFile};

const JPEG_QUALITY: u8 = 90;

const COVER_NAME: &str = "cover";
const FRONT_NAME: &str = "front";

const INITIAL_BUFFER_CAPACITY: usize = 256 * 1024;

pub fn get_cover_art_from_file(image_files: &Vec<&ImageFile>, audio_files: &Vec<&AudioFile>) -> Option<DynamicImage> {
    let mut cover_art_file = None::<&ImageFile>;

    // Only one file
    if image_files.len() == 1 {
        cover_art_file = image_files.first().copied();
    }

    // Exact name matches
    if cover_art_file.is_none() {
        for image_file in image_files {
            if let Some(name) = to_comparable_string(image_file.path().file_stem()) {
                if name == COVER_NAME || name == FRONT_NAME {
                    cover_art_file = Some(image_file);
                    break;
                }
            }
        }
    }

    // Partial name matches
    if cover_art_file.is_none() {
        for image_file in image_files {
            if let Some(name) = to_comparable_string(image_file.path().file_stem()) {
                if name.contains(COVER_NAME) || name.contains(FRONT_NAME) {
                    cover_art_file = Some(image_file);
                    break;
                }
            }
        }
    }

    // Partial album name matches
    if cover_art_file.is_none() {
        let album_titles = audio_files.iter()
            .fold(
                HashSet::<String>::new(),
                |mut acc, file| {
                    if let Some(album_title) = file.get_meta().album_title() {
                        acc.insert(album_title.to_lowercase());
                    }
                    acc
                }
            );

        for image_file in image_files {
            if let Some(name) = to_comparable_string(image_file.path().file_stem()) {
                for album_title in &album_titles {
                    if name.contains(album_title) {
                        cover_art_file = Some(image_file);
                        break;
                    }
                }
            }
        }
    }

    // Largest image dimension with aspect ratio 1:1
    if cover_art_file.is_none() {
        let mut current_image_file = None::<&ImageFile>;
        let mut largest = 0;
        for image_file in image_files {
            if let Some(dimensions) = image_file.meta().map(|m| &m.dimensions) {
                let aspect = dimensions.width as f32 / dimensions.height as f32;
                if aspect == 1.0 && dimensions.width > largest {
                    current_image_file = Some(image_file);
                    largest = dimensions.width;
                }
            }
        }
        cover_art_file = current_image_file;
    }

    if let Some(cover_art_file) = cover_art_file {
        if let Some(image) = Reader::open(cover_art_file.path())
            .ok()
            .and_then(|r| r.decode().ok()) {
                return Some(image);
            }
    }

    None
}

pub fn get_cover_art_from_tag(audio_files: &Vec<&AudioFile>) -> Option<DynamicImage> {
    for audio_file in audio_files {
        if let Some(cover_art) = audio_file.get_meta()
            .tagged_file()
            .primary_tag()
            .and_then(|t| t.get_picture_type(PictureType::CoverFront))
            .map(|p| Cursor::new(p.data()))
            .and_then(|c| Reader::new(c).with_guessed_format().ok())
            .and_then(|r| r.decode().ok()) {
                return Some(cover_art);
            }
    }

    None
}

pub fn write_image_to_file(image: &DynamicImage, path: &PathBuf) {
    let mut file = File::create(path).expect("Failed to create image file");
    image.write_to(&mut file, image::ImageOutputFormat::Jpeg(JPEG_QUALITY)).expect("Failed to write image to file");
}

pub fn write_image_to_buffer(image: &DynamicImage) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::with_capacity(INITIAL_BUFFER_CAPACITY));
    image.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(JPEG_QUALITY)).expect("Failed to write image to buffer");
    buffer.into_inner()
}

fn to_comparable_string(s: Option<&OsStr>) -> Option<String> {
    s.and_then(|s| s.to_str()
        .map(|s| s.to_string()))
        .map(|s| s.to_lowercase())
}
