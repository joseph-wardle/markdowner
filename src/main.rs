use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

/// Simple program to convert code files in a directory to a Markdown document
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory to scan for code files
    #[arg(short, long, value_parser)]
    input: PathBuf,

    /// Output Markdown file (optional, defaults to stdout)
    #[arg(short, long, value_parser)]
    output: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Validate input directory
    if !args.input.is_dir() {
        eprintln!("Error: The input path is not a directory or does not exist.");
        std::process::exit(1);
    }

    // Define supported file extensions and their corresponding markdown code block languages
    let mut supported_extensions: HashMap<&str, &str> = HashMap::new();
    supported_extensions.insert("rs", "rust");
    supported_extensions.insert("py", "python");
    // To add more languages in the future, insert here
    // supported_extensions.insert("js", "javascript");

    let mut markdown_output = String::new();

    // Traverse the directory recursively
    for entry in WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if let Some(lang) = supported_extensions.get(ext) {
                // Read file contents
                match fs::read_to_string(path) {
                    Ok(contents) => {
                        // Add separator
                        markdown_output.push_str("---\n\n");

                        // Add file name as a heading
                        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                            markdown_output.push_str(&format!("### {}\n\n", file_name));
                        }

                        // Add code block with language identifier
                        markdown_output.push_str(&format!("```{}\n", lang));
                        markdown_output.push_str(&contents);
                        // Ensure the code block ends with a newline
                        if !contents.ends_with('\n') {
                            markdown_output.push('\n');
                        }
                        markdown_output.push_str("```\n\n");
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Could not read file {}: {}",
                            path.display(),
                            e
                        );
                    }
                }
            }
        }
    }

    // Write the markdown output
    match args.output {
        Some(output_path) => {
            fs::write(&output_path, markdown_output)?;
            println!("Markdown document has been written to {}", output_path.display());
        }
        None => {
            // Write to stdout
            io::stdout().write_all(markdown_output.as_bytes())?;
        }
    }

    Ok(())
}