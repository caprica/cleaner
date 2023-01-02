use std::{path::PathBuf, io::{stdout, Write}, fs, cmp::max, collections::BTreeMap};

use colored::Colorize;

use crate::{error::CleanerResult, art::{get_cover_art_from_file, get_cover_art_from_tag, write_image_to_buffer, write_image_to_file}, tagger::clean_tags, media_files::MediaFiles, audio_file::AudioFile, media_file::MediaFile, image_file::ImageFile};

const UNKNOWN_ARTIST_NAME: &str = "[unknown]";
const UNKNOWN_ALBUM_TITLE: &str = "[unknown]";

pub fn clean_files(root_path: &PathBuf, output_path: &PathBuf, quality: u8) {
    let files = MediaFiles::new(root_path.into());

    let audio_file_map = files.get_audio_file_map();
    let image_file_map = files.get_image_file_map();

    for (source_path, audio_files_in_path) in audio_file_map {
        let audio_files_by_artist = get_audio_files_by_artist(&audio_files_in_path);
        clean_by_artist(&source_path, output_path, quality, &image_file_map, &audio_files_by_artist);
    }
}

fn clean_by_artist(source_path: &PathBuf, output_path: &PathBuf, quality: u8, image_file_map: &BTreeMap<PathBuf, Vec<&ImageFile>>, audio_files_by_artist: &BTreeMap<&str, BTreeMap<&str, Vec<&AudioFile>>>) {
    for (artist_name, audio_files_by_album) in audio_files_by_artist {
        print!(" Artist {} ", artist_name.bright_blue().bold());

        let artist_output_path = output_path.join(artist_name);
        match fs::create_dir_all(&artist_output_path) {
            Ok(_) => println!("{}", "OK".bright_green().bold()),
            Err(err) => {
                println!("{} {}", "ERROR".bright_red().bold(), err.to_string().red());
                continue;
            }
        }

        clean_by_album(source_path, &artist_output_path, quality, image_file_map, audio_files_by_album);
    }
}

fn clean_by_album(source_path: &PathBuf, artist_output_path: &PathBuf, quality: u8, image_file_map: &BTreeMap<PathBuf, Vec<&ImageFile>>, audio_files_by_album: &BTreeMap<&str, Vec<&AudioFile>>) {
    for (album_title, audio_files_in_album) in audio_files_by_album {
        print!("  Album {} ", album_title.bright_cyan().bold());

        let album_output_path = artist_output_path.join(album_title);
        match fs::create_dir_all(&album_output_path) {
            Ok(_) => println!("{}", "OK".bright_green().bold()),
            Err(err) => {
                println!("{} {}", "ERROR".bright_red().bold(), err.to_string().red());
                continue;
            }
        }

        let track_width = get_max_track_num_length(&audio_files_in_album);
        let title_width = track_width + 1 + get_max_title_length(&audio_files_in_album) + 1 + get_max_extension_length(&audio_files_in_album);

        print!("  Cover {:title_width$} ", "cover.jpg".bright_white().bold());

        // Prefer art from an image file in the samae directory, fallback to art embedded in any of the audio files
        let cover_art_image = image_file_map
            .get(source_path)
            .and_then(|image_files| get_cover_art_from_file(&image_files, &audio_files_in_album))
            .or_else(|| get_cover_art_from_tag(&audio_files_in_album));

        let target_image_path = &album_output_path.join("cover.jpg");
        if let Some(image) = &cover_art_image {
            match write_image_to_file(image, target_image_path, quality) {
                Ok(_) => println!("{}", "OK".bright_green().bold()),
                Err(err) => println!("{} {}", "ERROR".bright_red().bold(), err.to_string().red()),
            }
        } else {
            println!("{}", "MISSING".bright_red().bold());
        }

        let cover_art_buffer = cover_art_image
            .as_ref()
            .and_then(|image| write_image_to_buffer(&image, quality).ok());

        let total_tracks: u32 = audio_files_in_album.len().try_into().expect("Failed to get number of tracks");

        for audio_file in audio_files_in_album {
            let meta = audio_file.get_meta();

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

            match clean_audio_file(audio_file, total_tracks, &cover_art_buffer, &album_output_path, target_file_path) {
                Ok(_) => println!("{}", "OK".bright_green().bold()),
                Err(err) => println!("{} {}", "ERROR".bright_red().bold(), err.to_string().red()),
            }
        }
        println!();
    }
}

fn clean_audio_file(audio_file: &AudioFile, total_tracks: u32, cover_art_buffer: &Option<Vec<u8>>, target_directory_path: &PathBuf, target_file_path: &PathBuf) -> CleanerResult<()> {
    fs::create_dir_all(target_directory_path)?;
    fs::copy(audio_file.path(), &target_file_path)?;

    clean_tags(&target_file_path, audio_file.get_meta(), total_tracks, &cover_art_buffer)?;

    Ok(())
}

fn get_audio_files_by_artist<'a>(audio_files: &'a Vec<&AudioFile>) -> BTreeMap<&'a str, BTreeMap<&'a str, Vec<&'a AudioFile>>> {
    audio_files
        .iter()
        .fold(
            BTreeMap::<&str, BTreeMap<&str, Vec<&AudioFile>>>::new(),
            |mut acc, audio_file| {
                let artist_name = audio_file
                    .get_meta()
                    .artist_name()
                    .unwrap_or(UNKNOWN_ARTIST_NAME);

                let album_files = acc.entry(artist_name)
                    .or_insert_with(|| BTreeMap::<&str, Vec<&AudioFile>>::new());

                let album_title = audio_file
                    .get_meta()
                    .album_title()
                    .unwrap_or(UNKNOWN_ALBUM_TITLE);

                album_files.entry(album_title)
                    .or_insert_with(|| Vec::new())
                    .push(audio_file);

                acc
            }
        )
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
