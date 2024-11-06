mod args;
mod code_files;
mod tree;
mod config;
mod settings;

use args::Args;
use clap::Parser;
use code_files::{generate_markdown, SupportedExtensions};
use config::Config;
use settings::Settings;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let args = Args::parse();
    let config: Config = load_config();
    let settings = Settings::from_args_and_config(args, config);

    validate_directory(&settings.input_dir)?;

    let extensions = SupportedExtensions::new();
    let markdown = generate_markdown(&settings, &extensions)?;

    write_output(markdown, &settings.output_path)?;

    Ok(())
}

fn load_config() -> Config {
    confy::load("markdowner", None).unwrap_or_default()
}

fn validate_directory(directory: &Path) -> io::Result<()> {
    if !directory.is_dir() {
        eprintln!("Error: '{}' is not a valid directory", directory.display());
        std::process::exit(1);
    }
    Ok(())
}

fn write_output(markdown_output: String, output_path: &Option<PathBuf>) -> io::Result<()> {
    match output_path {
        Some(path) => {
            fs::write(path, markdown_output)?;
            println!("Markdown document saved to '{}'.", path.display());
        }
        None => {
            io::stdout().write_all(markdown_output.as_bytes())?;
        }
    }
    Ok(())
}
