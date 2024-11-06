use clap::Parser;
use std::path::PathBuf;

/// Converts code files in a directory to a Markdown document
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Input directory to scan for code files.
    pub input_dir: PathBuf,

    /// Output Markdown file path. Defaults to stdout if not provided.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Glob patterns to ignore (e.g., "node_modules/*").
    #[arg(short, long)]
    pub ignore: Vec<String>,

    /// Include a table of contents.
    #[arg(short, long)]
    pub toc: bool,

    /// Include file metadata (size, last modified).
    #[arg(short, long)]
    pub file_info: bool,

    /// Include the directory tree structure.
    #[arg(short, long)]
    pub directory_tree: bool
}