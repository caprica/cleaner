use std::{path::PathBuf, collections::BTreeSet, io::{stdout, Write}};

use colored::Colorize;
use tempfile::{Builder, TempDir};
use unrar::Archive;
use walkdir::WalkDir;
use zip_extensions::zip_extract;

use crate::{error::{CleanerResult, CleanerError}, cleaner::clean_files};

pub fn process_archives(path: &PathBuf, output_path: &PathBuf, quality: u8) {
    println!("Processing archives in {} to {}...\n",
        path.to_string_lossy().bright_yellow().bold(),
        output_path.to_string_lossy().bright_yellow().bold()
    );

    let archives = get_archives(path);

    for archive in archives {
        print!("Extract {} ", archive.file_name().unwrap().to_string_lossy().bright_magenta().bold());

        stdout().flush().expect("Failed to flush terminal output");

        match extract_archive(&archive) {
            Ok(temp_dir) => {
                println!("{}", "OK".bright_green().bold());
                let temp_path = temp_dir.path().to_path_buf();
                clean_files(&temp_path, output_path, quality);
            },
            Err(err) => println!("{} {}", "ERROR".bright_red().bold(), err.to_string().red())
        }
    }

    println!("Finished.");

}

fn get_archives(path: &PathBuf) -> BTreeSet<PathBuf> {
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

fn extract_archive(archive_path: &PathBuf) -> CleanerResult<TempDir> {
    let temp_dir = Builder::new().prefix("cleaner").tempdir()?;
    let temp_path = temp_dir.path().to_path_buf();

    if let Some(ext) = archive_path.extension().and_then(|s| s.to_str()) {
        let result = match ext {
            "rar" => extract_rar_archive(archive_path, &temp_path),
            "zip" => extract_zip_archive(archive_path, &temp_path),
            _ => Err(CleanerError::UnexpectedFileExtension)
        };
        result.map(|_| temp_dir)
    } else {
        Err(CleanerError::MissingFileExtension)
    }
}

fn extract_zip_archive(archive_path: &PathBuf, output_path: &PathBuf) -> CleanerResult<()> {
    zip_extract(archive_path, output_path)?;
    Ok(())
}

fn extract_rar_archive(archive_path: &PathBuf, output_path: &PathBuf) -> CleanerResult<()> {
    let archive_name = archive_path.to_str().unwrap().to_string();
    let output_name = output_path.to_str().unwrap().to_string();
    Archive::new(archive_name)
        .extract_to(output_name)
        .unwrap()
        .process()?;
    Ok(())
}
