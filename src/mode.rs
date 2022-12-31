use std::fmt::Display;

use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Process all archives in the given directory
    Archives,
    /// Process all files, recursively, in the given directory
    Files,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Archives => f.write_str("archives"),
            Mode::Files => f.write_str("Files"),
        }
    }
}
