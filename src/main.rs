mod art;
mod archives;
mod audio_file;
mod audio_file_meta;
mod error;
mod files;
mod cleaner;
mod image_file;
mod media_file;
mod media_files;
mod mode;
mod other_file;
mod tagger;

use std::{path::{PathBuf, Path}, process::ExitCode};

use clap::{Parser};
use files::process_files;
use mode::Mode;

use crate::{archives::process_archives};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to process
    path: PathBuf,

    /// Output directory
    output: PathBuf,

    /// Processing mode
    #[arg(short, long, value_enum, default_value_t = Mode::Archives)]
    mode: Mode,

    /// Optional directory to use for temporary files, otherwise use system temporary directory (currently unused)
    #[arg(short, long)]
    temp_dir: Option<PathBuf>,

    /// Quality factor to use when generating JPEG cover art
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..100), default_value_t = 90)]
    quality: u8,
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let source_path = &args.path;
    let output_path = &args.output;
    let quality = args.quality;

    if !Path::new(source_path).exists() {
        println!("Path '{}' does not exist", source_path.to_string_lossy());
        return ExitCode::from(1);
    }

    match args.mode {
        Mode::Archives => process_archives(&source_path, &output_path, quality),
        Mode::Files => process_files(&source_path, &output_path, quality),
    }

    return ExitCode::from(0);
}
