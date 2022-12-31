use std::{path::PathBuf};

use colored::Colorize;

use crate::cleaner::clean_files;

pub fn process_files(path: &PathBuf, output_path: &PathBuf, quality: u8) {
    println!("Processing files in {} to {}...\n",
        path.to_string_lossy().bright_yellow().bold(),
        output_path.to_string_lossy().bright_yellow().bold()
    );

    clean_files(path, output_path, quality);

    println!("Finished.");
}
