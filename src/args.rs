use clap::Parser;
use std::path::PathBuf;

/// Converts code files in a directory to a Markdown document
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Input directory to scan for code files
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output Markdown file (optional, defaults to stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
