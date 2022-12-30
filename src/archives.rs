use std::{path::PathBuf, collections::BTreeSet, io::{stdout, Write}};

use colored::Colorize;
use tempfile::Builder;
use unrar::{Archive, error::UnrarResult, archive::Entry};
use walkdir::WalkDir;
use zip::result::ZipResult;
use zip_extensions::zip_extract;

pub fn get_archive_paths(path: &PathBuf) -> BTreeSet<PathBuf> {
    let walker = WalkDir::new(&path)
        .min_depth(1)
        .max_depth(1);

    walker.into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                return ext == "zip" || ext == "rar";
            }
            false
        })
        .map(|e| e.into_path())
        .collect::<BTreeSet<PathBuf>>()
}

pub fn process_archives(path: &PathBuf) {
    let archives = get_archive_paths(path);

    let width = archives
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().chars().count())
        .max()
        .unwrap();

    println!("Processing archives in {}...\n",
        path.to_string_lossy().bright_yellow().bold()
    );

    for archive in archives {
        print!("{:<width$} ",
            archive.file_name().unwrap().to_string_lossy().bright_white().bold()
        );

        stdout().flush().expect("Failed to flush terminal output");

        let temp_dir = Builder::new().prefix("cleaner").tempdir().expect("Failed to create temporary directory");

        let output_path = temp_dir.path().to_path_buf();

        if let Some(ext) = archive.extension() {
            if ext == "zip" {
                match extract_zip_archive(&archive, &output_path) {
                    Ok(_) => println!("{}", "OK".bright_green().bold()),
                    Err(err) => println!("{} {}", "ERROR".bright_red().bold(), err.to_string().red()),
                }
            } else if ext == "rar" {
                match extract_rar_archive(&archive, &output_path) {
                    Ok(_) => println!("{}", "OK".bright_green().bold()),
                    Err(err) => println!("{} {}", "ERROR".bright_red().bold(), err.to_string().red()),
                }
            }
        }
    }

    println!("\nFinished.");

}

fn extract_zip_archive(archive_path: &PathBuf, output_path: &PathBuf) -> ZipResult<()> {
    zip_extract(archive_path, output_path)
}

fn extract_rar_archive(archive_path: &PathBuf, output_path: &PathBuf) ->  UnrarResult<Vec<Entry>> {
    let archive_name = archive_path.to_str().unwrap().to_string();
    let output_name = output_path.to_str().unwrap().to_string();

    Archive::new(archive_name)
        .extract_to(output_name)
        .unwrap()
        .process()
}
