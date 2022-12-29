use std::{io::Cursor, fs::File, path::PathBuf};

use image::{DynamicImage, io::Reader};
use lofty::{PictureType, TaggedFileExt};

use crate::{image_file::ImageFile, media_file::MediaFile, audio_file::AudioFile};

const JPEG_QUALITY: u8 = 90;

pub fn get_cover_art_from_file(image_files: &Vec<&ImageFile>) -> Option<DynamicImage> {
    let mut cover_art_file = None::<&ImageFile>;

    // Only one file
    if image_files.len() == 1 {
        cover_art_file = image_files.first().copied();
    }

    // Exact name matches
    if cover_art_file.is_none() {
        for image_file in image_files {
            if let Some(name) = image_file.path().file_stem() {
                if name == "cover" || name == "front" {
                    cover_art_file = Some(image_file);
                    break;
                }
            }
        }
    }

    // Partial name matches
    if cover_art_file.is_none() {
        for image_file in image_files {
            if let Some(name) = image_file.path().file_stem().and_then(|s| s.to_str()) {
                if name.contains("cover") || name.contains("front") {
                    cover_art_file = Some(image_file);
                    break;
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
        if let Some(tag) = audio_file.get_tagged_file().primary_tag() {
            if let Some(pic) = tag.get_picture_type(PictureType::CoverFront) {
                let cursor = Cursor::new(pic.data());
                if let Some(image) = Reader::new(cursor)
                    .with_guessed_format()
                    .ok()
                    .and_then(|r| r.decode().ok()) {
                        return Some(image);
                    }
            }
        }
    }
    None
}

pub fn write_image_to_file(image: &DynamicImage, path: &PathBuf) {
    let mut file = File::create(path).expect("Failed to create image file");
    image.write_to(&mut file, image::ImageOutputFormat::Jpeg(JPEG_QUALITY)).expect("Failed to write image to file");
}

pub fn write_image_to_buffer(image: &DynamicImage) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(JPEG_QUALITY)).expect("Failed to write image to buffer");
    buffer.into_inner()
}
