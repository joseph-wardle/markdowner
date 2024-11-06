use crate::args::Args;
use chrono::prelude::*;
use glob::Pattern;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

/// Returns a mapping of file extensions to language identifiers for syntax highlighting
pub fn get_supported_extensions() -> HashMap<&'static str, &'static str> {
    [
        ("rs", "rust"),
        ("py", "python"),
        ("java", "java"),
        ("c", "c"),
        ("h", "c"),
        ("cpp", "cpp"),
        ("cc", "cpp"),
        ("cxx", "cpp"),
        ("hpp", "cpp"),
        ("cs", "csharp"),
        ("js", "javascript"),
        ("ts", "typescript"),
        ("go", "go"),
        ("rb", "ruby"),
        ("php", "php"),
        ("swift", "swift"),
        ("kt", "kotlin"),
        ("scala", "scala"),
        ("hs", "haskell"),
        ("lua", "lua"),
        ("sh", "bash"),
        ("bash", "bash"),
        ("pl", "perl"),
        ("r", "r"),
        ("m", "matlab"),
        ("mm", "objectivec"),
        ("html", "html"),
        ("htm", "html"),
        ("css", "css"),
        ("xml", "xml"),
        ("json", "json"),
        ("yaml", "yaml"),
        ("yml", "yaml"),
        ("toml", "toml"),
        ("make", "makefile"),
        ("cmake", "cmake"),
        ("gd", "gdscript"),
        ("md", "markdown"),
        ("txt", ""),
    ]
        .iter()
        .cloned()
        .collect()
}


/// Generates the Markdown content by processing code files in the directory
pub fn generate_markdown(
    input_dir: &Path,
    directory_tree: &str,
    supported_extensions: &HashMap<&str, &str>,
    args: &Args,
) -> String {
    let mut markdown_output = String::new();

    // Append directory tree to markdown_output
    markdown_output.push_str("```\n");
    markdown_output.push_str(directory_tree);
    markdown_output.push_str("```\n\n");

    // Collect file headings for TOC
    let mut file_headings = Vec::new();

    // Collect all files to process
    let files_to_process: Vec<PathBuf> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| !is_ignored(e.path(), &args.ignore))
        .collect::<Vec<_>>()
        .iter()
        .map(|e| e.path().to_path_buf())
        .collect();

    // Initialize progress bar
    let progress_bar = ProgressBar::new(files_to_process.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .expect("Failed to set progress bar style")
            .progress_chars("#>-"),
    );

    // Process each file
    for path in files_to_process {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if let Some(lang) = supported_extensions.get(ext) {
                match process_file(&path, lang, args.file_info) {
                    Ok(content) => {
                        file_headings.push(path.clone());
                        markdown_output.push_str(&content);
                    }
                    Err(e) => eprintln!("Warning: Could not read file {}: {}", path.display(), e),
                }
            }
        }

        // Update progress bar
        progress_bar.inc(1);
        progress_bar.set_message(format!("Processing {}", path.display()));
    }

    // Finish progress bar
    progress_bar.finish_with_message("Processing complete");

    if args.toc {
        let toc = generate_toc(&file_headings, input_dir);
        markdown_output = format!("{}\n\n{}", toc, markdown_output);
    }

    markdown_output
}

fn process_file(path: &Path, lang: &str, include_file_info: bool) -> std::io::Result<String> {
    let contents = fs::read_to_string(path)?;
    let mut file_markdown = String::new();

    // Add separator
    file_markdown.push_str("---\n\n");

    // Generate heading with anchor
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        let heading = format!("### {}\n\n", file_name);
        file_markdown.push_str(&heading);
    }

    // Include file information if requested
    if include_file_info {
        let metadata = fs::metadata(path)?;
        let file_size = metadata.len();
        let modified_time = metadata.modified()?;
        let datetime: DateTime<Local> = modified_time.into();
        file_markdown.push_str(&format!(
            "*Last modified:* `{}` &nbsp; | &nbsp; *Size:* `{}` bytes\n\n",
            datetime.format("%Y-%m-%d %H:%M:%S"),
            file_size
        ));
    }

    // Add code block with language identifier
    file_markdown.push_str(&format!("```{}\n", lang));
    file_markdown.push_str(&contents);
    if !contents.ends_with('\n') {
        file_markdown.push('\n');
    }
    file_markdown.push_str("```\n\n");

    Ok(file_markdown)
}

pub fn is_ignored(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignore_patterns.iter().any(|pattern| {
        if let Ok(glob_pattern) = Pattern::new(pattern) {
            glob_pattern.matches(&path_str)
        } else {
            false
        }
    })
}

fn generate_toc(file_paths: &[std::path::PathBuf], base_dir: &Path) -> String {
    let mut toc = String::from("## Table of Contents\n\n");
    for path in file_paths {
        if let Some(relative_path) = path.strip_prefix(base_dir).ok() {
            let display_path = relative_path.display().to_string();
            let anchor = display_path.replace('/', "").replace('.', "");
            toc.push_str(&format!(
                "- [{}](#{})\n",
                display_path,
                anchor.to_lowercase()
            ));
        }
    }
    toc
}
