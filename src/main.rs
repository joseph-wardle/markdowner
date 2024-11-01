use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
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

fn generate_directory_tree(
    dir: &Path,
    prefix: &str,
    is_last: bool,
    base_dir: &Path,
) -> String {
    let mut tree = String::new();

    let dir_name = if dir == base_dir {
        format!("{}/", dir.file_name().unwrap_or(dir.as_os_str()).to_string_lossy())
    } else {
        dir.file_name().unwrap().to_string_lossy().into_owned()
    };

    if dir == base_dir {
        // Root directory, print its name
        tree.push_str(&format!("{}\n", dir_name));
    } else {
        if is_last {
            tree.push_str(&format!("{}└─ {}/\n", prefix, dir_name));
        } else {
            tree.push_str(&format!("{}├─ {}/\n", prefix, dir_name));
        }
    }

    let mut entries = match fs::read_dir(dir) {
        Ok(entries) => entries.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(_) => return tree,
    };

    // Sort directories before files, and alphabetically
    entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        if a_is_dir == b_is_dir {
            a.file_name().cmp(&b.file_name())
        } else if a_is_dir {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let len = entries.len();
    for (i, entry) in entries.into_iter().enumerate() {
        let path = entry.path();
        let is_last_entry = i == len - 1;

        let new_prefix = if dir == base_dir {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        if path.is_dir() {
            tree.push_str(&generate_directory_tree(
                &path,
                &new_prefix,
                is_last_entry,
                base_dir,
            ));
        } else {
            let file_name = path.file_name().unwrap().to_string_lossy();
            if is_last_entry {
                tree.push_str(&format!("{}└─ {}\n", new_prefix, file_name));
            } else {
                tree.push_str(&format!("{}├─ {}\n", new_prefix, file_name));
            }
        }
    }

    tree
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
    supported_extensions.insert("java", "java");
    supported_extensions.insert("c", "c");
    supported_extensions.insert("h", "c");
    supported_extensions.insert("cpp", "cpp");
    supported_extensions.insert("cc", "cpp");
    supported_extensions.insert("cxx", "cpp");
    supported_extensions.insert("hpp", "cpp");
    supported_extensions.insert("cs", "csharp");
    supported_extensions.insert("js", "javascript");
    supported_extensions.insert("ts", "typescript");
    supported_extensions.insert("go", "go");
    supported_extensions.insert("rb", "ruby");
    supported_extensions.insert("php", "php");
    supported_extensions.insert("swift", "swift");
    supported_extensions.insert("kt", "kotlin");
    supported_extensions.insert("scala", "scala");
    supported_extensions.insert("hs", "haskell");
    supported_extensions.insert("lua", "lua");
    supported_extensions.insert("sh", "bas");
    supported_extensions.insert("bash", "bash");
    supported_extensions.insert("pl", "perl");
    supported_extensions.insert("r", "r");
    supported_extensions.insert("m", "matlab"); // Note: 'm' can also be Objective-C
    supported_extensions.insert("mm", "objectivec");
    supported_extensions.insert("html", "html");
    supported_extensions.insert("htm", "html");
    supported_extensions.insert("css", "css");
    supported_extensions.insert("xml", "xml");
    supported_extensions.insert("json", "json");
    supported_extensions.insert("yaml", "yaml");
    supported_extensions.insert("yml", "yaml");
    supported_extensions.insert("toml", "toml");
    supported_extensions.insert("make", "makefile");
    supported_extensions.insert("cmake", "cmake");
    supported_extensions.insert("gd", "gdscript");
    supported_extensions.insert("md", "markdown");






    // Generate directory tree
    let directory_tree = generate_directory_tree(&args.input, "", true, &args.input);

    let mut markdown_output = String::new();

    // Append directory tree to markdown_output
    markdown_output.push_str("```\n");
    markdown_output.push_str(&directory_tree);
    markdown_output.push_str("```\n\n");

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
