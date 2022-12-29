mod art;
mod audio_file;
mod audio_file_meta;
mod image_file;
mod media_file;
mod other_file;
mod files;
mod tagger;

use clap::Parser;
use files::Files;
use std::{path::{PathBuf, Path}, collections::{BTreeMap}, fs, cmp::max};

use crate::{media_file::MediaFile, tagger::clean_tags, art::{get_cover_art_from_file, get_cover_art_from_tag, write_image_to_buffer, write_image_to_file}};

#[derive(Parser)]
struct Cli {
    path: PathBuf,
    output: PathBuf
}

fn main() {
    let args = Cli::parse();

    if !Path::new(&args.path).exists() {
        println!("Path does not exist");
        return;
    }

    let files = Files::new(args.path);

    println!("PROCESSING FILES IN {}", files.path().to_str().unwrap());
    println!();

    let audio_file_map = files.get_audio_file_map();
    let image_file_map = files.get_image_file_map();
    let other_file_map = files.get_other_file_map();

    let debug = false;

    if debug {
        dump_map(&audio_file_map);
        dump_map(&image_file_map);
        dump_map(&other_file_map);
    }

    let output_path = &args.output;

    for (path, audio_files) in audio_file_map {
        println!("{}", path.to_str().unwrap());

        let cover_art_image = match &image_file_map.get(&path) {
            Some(image_files) => get_cover_art_from_file(&image_files),
            None => get_cover_art_from_tag(&audio_files)
        };

        let cover_art_buffer = cover_art_image.as_ref().map(|image| write_image_to_buffer(&image));

        for audio_file in &audio_files {
            let meta = audio_file.get_audio_file_meta();

            let artist_output_path = output_path.join(meta.album_artist_name().unwrap_or_else(|| "<missing>"));
            let album_output_path = artist_output_path.join(meta.album_title().unwrap_or_else(|| "<missing>"));
            let track_number = meta.track_number().unwrap();
            let track_title = meta.track_title().unwrap();

            let track_field_width = max(2, audio_files.len().to_string().chars().count());

            let track_file_name = format!("{:0width$} {}.{}", track_number, track_title, meta.audio_file_type().expect("Must have a file type").to_extension(), width = track_field_width);
            let target_file_path = &album_output_path.join(track_file_name);

            println!(" create track '{}'", target_file_path.to_str().unwrap());

            if fs::create_dir_all(&album_output_path).is_ok() {
                fs::copy(audio_file.path(), &target_file_path).expect("Failed to copy audio file");
                let total_tracks: u32 = audio_files.len().try_into().unwrap();

                clean_tags(&target_file_path, &meta, total_tracks, &cover_art_buffer);

                let target_image_path = &album_output_path.join("cover.jpg");
                if !target_image_path.exists() {
                    if let Some(ref image) = cover_art_image.as_ref() {
                        write_image_to_file(image, target_image_path);
                    }
                }
            }
        }
        println!();
    }
}

fn dump_map<T: MediaFile>(map: &BTreeMap<PathBuf, Vec<&T>>) {
    for (key, value) in map {
        println!("{}", key.to_str().unwrap());
        for media_file in value {
            println!(" -> {}", media_file.path().file_name().unwrap().to_str().unwrap());
        }
    }
    println!();
}
