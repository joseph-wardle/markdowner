use chrono::{DateTime, Local};
use glob::Pattern;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::settings::Settings;
use crate::tree::build_directory_tree;

/// Maps file extensions to syntax highlighting languages.
pub struct SupportedExtensions {
    extensions: HashMap<&'static str, &'static str>,
}

impl SupportedExtensions {
    pub fn new() -> Self {
        Self {
            extensions: [
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
                .collect(),
        }
    }

    pub fn get_language(&self, extension: &str) -> Option<&str> {
        self.extensions.get(extension).copied()
    }
}

/// Generates the complete Markdown document based on settings and file contents.
pub fn generate_markdown(
    settings: &Settings,
    extensions: &SupportedExtensions,
) -> io::Result<String> {
    let mut markdown = String::new();
    let input_path = &settings.input_dir;

    // Include the directory tree if the option is enabled
    if settings.include_directory_tree {
        let tree = build_directory_tree(input_path, &settings.ignore_patterns);
        markdown.push_str("```\n");
        markdown.push_str(&tree);
        markdown.push_str("```\n\n");
    }

    // Collect all relevant files based on input directory and ignore patterns
    let file_paths = collect_files(input_path, &settings.ignore_patterns);
    let progress = initialize_progress(file_paths.len());

    let mut processed_files = Vec::new();

    // Process each file
    for path in file_paths {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if let Some(language) = extensions.get_language(extension) {
                // Determine if the file is a Markdown file
                let content = if language == "markdown" {
                    // Process Markdown files differently
                    match process_markdown_file(&path, settings.include_file_info, input_path) {
                        Ok(content) => content,
                        Err(e) => {
                            eprintln!("Warning: Could not read Markdown file {}: {}", path.display(), e);
                            continue;
                        }
                    }
                } else {
                    // Process other code files normally
                    match process_code_file(&path, language, settings.include_file_info, input_path) {
                        Ok(content) => content,
                        Err(e) => {
                            eprintln!("Warning: Could not read file {}: {}", path.display(), e);
                            continue;
                        }
                    }
                };

                processed_files.push(path.clone());
                markdown.push_str(&content);
            }
        }
        progress.inc(1);
        progress.set_message(format!("Processing {}", path.display()));
    }

    // Finish the progress bar
    progress.finish_with_message("Markdown generation complete.");

    // Include the table of contents if the option is enabled
    if settings.include_toc {
        let toc = build_table_of_contents(&processed_files, input_path);
        markdown = format!("## Table of Contents\n\n{}\n\n{}", toc, markdown);
    }

    Ok(markdown)
}

/// Collects all files from the input directory, excluding those that match ignore patterns.
fn collect_files(input_dir: &Path, ignore_patterns: &[String]) -> Vec<PathBuf> {
    WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| !is_ignored(entry.path(), ignore_patterns))
        .map(|entry| entry.into_path())
        .collect()
}

/// Initializes the progress bar for tracking file processing.
fn initialize_progress(total: usize) -> ProgressBar {
    let progress = ProgressBar::new(total as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    progress
}

/// Processes a code file by encapsulating its content within code blocks and adding metadata.
fn process_code_file(path: &Path, language: &str, include_info: bool, base_dir: &Path) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    let metadata = fs::metadata(path)?;
    let modified_time: DateTime<Local> = metadata.modified()?.into();
    let file_size = metadata.len();

    // Compute the full relative path of the file
    let relative_path = path.strip_prefix(base_dir).unwrap_or(path).display().to_string();

    let mut markdown = String::new();
    markdown.push_str("---\n\n");

    // Use the full relative path in the header
    markdown.push_str(&format!("# {}\n\n", relative_path));

    if include_info {
        markdown.push_str(&format!(
            "*Last modified:* `{}` | *Size:* `{}` bytes\n\n",
            modified_time.format("%Y-%m-%d %H:%M:%S"),
            file_size
        ));
    }

    // Encapsulate code content within code fences for syntax highlighting
    markdown.push_str(&format!("```{}\n", language));
    markdown.push_str(&content);
    if !content.ends_with('\n') {
        markdown.push('\n');
    }
    markdown.push_str("```\n\n");

    Ok(markdown)
}

/// Processes a Markdown file by adjusting header levels and adding metadata.
fn process_markdown_file(path: &Path, include_info: bool, base_dir: &Path) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    let metadata = fs::metadata(path)?;
    let modified_time: DateTime<Local> = metadata.modified()?.into();
    let file_size = metadata.len();

    // Compute the full relative path of the file
    let relative_path = path.strip_prefix(base_dir).unwrap_or(path).display().to_string();

    let mut markdown = String::new();
    markdown.push_str("---\n\n");

    // Use the full relative path in the header
    markdown.push_str(&format!("# {}\n\n", relative_path));

    if include_info {
        markdown.push_str(&format!(
            "*Last modified:* `{}` | *Size:* `{}` bytes\n\n",
            modified_time.format("%Y-%m-%d %H:%M:%S"),
            file_size
        ));
    }

    // Adjust header levels within the Markdown content
    let adjusted_content = adjust_markdown_headers(&content);
    markdown.push_str(&adjusted_content);
    markdown.push_str("\n\n");

    Ok(markdown)
}

/// Increases each header level in the Markdown content by one, capping at level 6.
fn adjust_markdown_headers(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            if let Some(header_end) = line.find(|c: char| c != '#') {
                let header_length = header_end;
                // Ensure there's at least one space after the hashes for it to be a valid header
                if header_length > 0 && line[header_length..].starts_with(' ') {
                    let new_level = if header_length < 6 {
                        header_length + 1
                    } else {
                        6
                    };
                    let new_hashes = "#".repeat(new_level);
                    format!("{}{}", new_hashes, &line[header_length..])
                } else {
                    line.to_string()
                }
            } else {
                // Line consists entirely of '#' characters or is empty
                if !line.trim().is_empty() && line.chars().all(|c| c == '#') {
                    "#".repeat(6)
                } else {
                    line.to_string()
                }
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Determines if a given path should be ignored based on the provided patterns.
fn is_ignored(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignore_patterns.iter().any(|pattern| {
        Pattern::new(pattern).map_or(false, |p| p.matches(&path_str))
    })
}

/// Builds the table of contents using the full relative paths of the processed files.
fn build_table_of_contents(file_paths: &[PathBuf], base_dir: &Path) -> String {
    file_paths
        .iter()
        .filter_map(|path| {
            path.strip_prefix(base_dir).ok().map(|relative| {
                let display = relative.display().to_string();
                let anchor = generate_anchor(&display);
                format!("- [{}](#{})", display, anchor)
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generates a valid Markdown anchor from a given display string.
/// This function replaces characters that are not alphanumeric with empty strings
/// and converts the string to lowercase.
fn generate_anchor(display: &str) -> String {
    display
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}