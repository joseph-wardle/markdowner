mod args;
mod code_files;
mod tree;

use args::Args;
use clap::Parser;
use code_files::{generate_markdown, get_supported_extensions};
use std::io::{self, Write};
use std::path::Path;
use tree::generate_directory_tree;

fn main() -> io::Result<()> {
    let args = Args::parse();

    validate_input_directory(&args.input)?;

    let directory_tree = generate_directory_tree(&args.input, "", true, &args.input);

    let supported_extensions = get_supported_extensions();
    let markdown_output = generate_markdown(&args.input, &directory_tree, &supported_extensions);

    write_output(markdown_output, args.output)?;

    Ok(())
}

fn validate_input_directory(input: &Path) -> io::Result<()> {
    if !input.is_dir() {
        eprintln!("Error: The input path is not a directory or does not exist.");
        std::process::exit(1);
    }
    Ok(())
}

fn write_output(markdown_output: String, output_path: Option<std::path::PathBuf>) -> io::Result<()> {
    match output_path {
        Some(path) => {
            std::fs::write(&path, markdown_output)?;
            println!("Markdown document has been written to {}", path.display());
        }
        None => {
            io::stdout().write_all(markdown_output.as_bytes())?;
        }
    }
    Ok(())
}
