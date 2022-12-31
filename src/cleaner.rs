use std::{path::PathBuf, io::{stdout, Write}, fs, cmp::max};

use colored::Colorize;
use image::DynamicImage;

use crate::{error::CleanerResult, art::{get_cover_art_from_file, get_cover_art_from_tag, write_image_to_buffer, write_image_to_file}, tagger::clean_tags, media_files::MediaFiles, audio_file::AudioFile, media_file::MediaFile};

pub fn clean_files(root_path: &PathBuf, output_path: &PathBuf, quality: u8) {
    let files = MediaFiles::new(root_path.into());

    let audio_file_map = files.get_audio_file_map();
    let image_file_map = files.get_image_file_map();

    for (path, audio_files) in audio_file_map {
        let first_path = audio_files.first()
            .map(|f| f.path())
            .and_then(|p| p.parent())
            .map(|p| {
                if p != root_path {
                    p.file_name()
                        .expect("Must have a file name")
                        .to_string_lossy()
                        .to_string()
                } else {
                    String::from("<none>")
                }
        });

        if first_path.is_none() {
            continue;
        }

        println!("Process {}", first_path.unwrap().bright_cyan().bold());

        let track_width = get_max_track_num_length(&audio_files);
        let title_width = track_width + 1 + get_max_title_length(&audio_files) + 1 + get_max_extension_length(&audio_files);

        let cover_art_image = match &image_file_map.get(&path) {
            Some(image_files) => get_cover_art_from_file(&image_files, &audio_files),
            None => get_cover_art_from_tag(&audio_files)
        };

        let cover_art_buffer = cover_art_image.as_ref().map(|image| write_image_to_buffer(&image, quality));

        let total_tracks: u32 = audio_files.len().try_into().expect("Unable to get number of tracks");

        for audio_file in &audio_files {
            let meta = audio_file.get_meta();

            let artist_output_path = output_path.join(meta.album_artist_name().unwrap_or_else(|| "<missing>"));
            let album_output_path = artist_output_path.join(meta.album_title().unwrap_or_else(|| "<missing>"));
            let track_number = meta.track_number().unwrap();
            let track_title = meta.track_title().unwrap();

            let track_file_name = format!("{:0track_width$} {}.{}", track_number, track_title, meta.audio_file_type().expect("Must have a file type").to_extension());

            let target_file_path = &album_output_path.join(track_file_name);

            let target_file_name = target_file_path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
                .expect("Missing file name");

            print!("  Track {:title_width$} ", target_file_name.white().bold());
            stdout().flush().expect("Failed to flush terminal output");

            match clean_audio_file(audio_file, total_tracks, &cover_art_buffer, &cover_art_image, quality, &album_output_path, target_file_path) {
                Ok(_) => println!("{}", "OK".bright_green().bold()),
                Err(err) => println!("{} {}", "ERROR".bright_red().bold(), err.to_string().red()),
            }
        }
        println!();
    }
}

fn get_max_track_num_length(audio_files: &Vec<&AudioFile>) -> usize {
    max(2, audio_files.len().to_string().chars().count())
}

fn get_max_title_length(audio_files: &Vec<&AudioFile>) -> usize {
    audio_files
        .iter()
        .map(|f| f.get_meta())
        .map(|m| m.track_title().unwrap().chars().count())
        .max()
        .unwrap_or(0)
}

fn get_max_extension_length(audio_files: &Vec<&AudioFile>) -> usize {
    audio_files
        .iter()
        .map(|f| f.get_meta())
        .map(|m| m.audio_file_type().unwrap().to_extension().chars().count())
        .max()
        .unwrap_or(0)
}

fn clean_audio_file(audio_file: &AudioFile, total_tracks: u32, cover_art_buffer: &Option<Vec<u8>>, cover_art_image: &Option<DynamicImage>, quality: u8, target_directory_path: &PathBuf, target_file_path: &PathBuf) -> CleanerResult<()> {
    fs::create_dir_all(target_directory_path)?;
    fs::copy(audio_file.path(), &target_file_path)?;

    clean_tags(&target_file_path, audio_file.get_meta(), total_tracks, &cover_art_buffer)?;

    let target_image_path = &target_directory_path.join("cover.jpg");
    if !target_image_path.exists() {
        if let Some(image) = cover_art_image {
            write_image_to_file(image, target_image_path, quality);
        }
    }

    Ok(())
}
