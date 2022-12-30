use std::{path::PathBuf, collections::BTreeSet};

use colored::Colorize;
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
                return ext == "zip";
            }
            false
        })
        .map(|e| e.into_path())
        .collect::<BTreeSet<PathBuf>>()
}

pub fn process_archives(path: &PathBuf, output_path: &PathBuf) {
    let archives = get_archive_paths(path);

    let width = archives
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().chars().count())
        .max()
        .unwrap();

    println!("Processing archives from {} to {}:\n",
        path.to_string_lossy().bright_yellow().bold(),
        output_path.to_string_lossy().bright_yellow().bold()
    );

    for archive in archives {
        print!("{:<width$}",
            archive.file_name().unwrap().to_string_lossy().bright_white().bold()
        );

        match extract_archive(&archive, output_path) {
            Ok(_) => println!(" {}", "OK".bright_green().bold()),
            Err(err) => println!(" {} {}", "ERROR".bright_red().bold(), err.to_string().red()),
        }
    }

    println!("\nFinished.");

}

fn extract_archive(archive_path: &PathBuf, output_path: &PathBuf) -> ZipResult<()> {
    zip_extract(archive_path, output_path)
}
