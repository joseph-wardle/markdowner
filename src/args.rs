use clap::Parser;
use std::path::PathBuf;

/// Converts code files in a directory to a Markdown document
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Input directory to scan for code files (positional argument)
    pub input: PathBuf,

    /// Output Markdown file (optional, defaults to stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Patterns to ignore (files or directories), e.g., "node_modules/*"
    #[arg(long)]
    pub ignore: Vec<String>,

    /// Generate a table of contents
    #[arg(long)]
    pub toc: bool,

    /// Include file information (size, modified date)
    #[arg(long)]
    pub file_info: bool,
}