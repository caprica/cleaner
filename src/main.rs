mod audio_file;
mod image_file;
mod media_file;
mod other_file;
mod files;

use clap::Parser;
use files::Files;
use std::{path::{PathBuf, Path}, collections::{BTreeMap}};

use crate::media_file::MediaFile;

#[derive(Parser)]
struct Cli {
    path: PathBuf
}

fn main() {
    let args = Cli::parse();

    if !Path::new(&args.path).exists() {
        println!("Path does not exist");
        return;
    }

    let files = Files::new(args.path);

    println!("PROCESSING FILES IN {}", files.path().to_str().unwrap());

    let debug = false;

    if debug {
        for audio_file in files.audio_files() {
            println!("AUDIO FILE: {}", audio_file.path().to_str().unwrap());
            if !audio_file.is_valid() {
                for err in audio_file.file_errors() {
                    println!(" -> FILE ERROR: {:?}", err);
                }
                for err in audio_file.tag_errors() {
                    println!(" ->  TAG ERROR: {:?}", err);
                }
            }
        }

        for image_file in files.image_files() {
            println!("IMAGE FILE: {}", image_file.path().to_str().unwrap());
            if !image_file.is_valid() {
                for err in image_file.file_errors() {
                    println!(" -> FILE ERROR: {:?}", err);
                }
                for err in image_file.meta_errors() {
                    println!(" -> META ERROR: {:?}", err);
                }
            }
        }

        for other_file in files.other_files() {
            println!("OTHER FILE: {}", other_file.path().to_str().unwrap());
        }

        println!();
    }

    let audio_file_map = files.get_audio_file_map();
    let image_file_map = files.get_image_file_map();
    let other_file_map = files.get_other_file_map();

    dump_map(&audio_file_map);
    dump_map(&image_file_map);
    dump_map(&other_file_map);

}

fn dump_map<T: MediaFile>(map: &BTreeMap<PathBuf, Vec<&T>>) {
    for (key, value) in map {
        println!("{}", key.to_str().unwrap());
        for other_file in value {
            println!(" -> {}", other_file.path().file_name().unwrap().to_str().unwrap());
        }
    }
    println!();
}
