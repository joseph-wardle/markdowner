use chrono::{DateTime, Local};
use glob::Pattern;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
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
) -> std::io::Result<String> {
    let mut markdown = String::new();
    let input_path = &settings.input_dir;

    if settings.include_directory_tree {
        let tree = build_directory_tree(
            input_path,
            &settings.ignore_patterns,
        );
        markdown.push_str("```\n");
        markdown.push_str(&tree);
        markdown.push_str("```\n\n");
    }

    let file_paths = collect_files(input_path, &settings.ignore_patterns);
    let progress = initialize_progress(file_paths.len());

    let mut processed_files = Vec::new();

    // Process each file
    for path in file_paths {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if let Some(language) = extensions.get_language(extension) {
                if let Ok(content) = process_file(&path, language, settings.include_file_info) {
                    processed_files.push(path.clone());
                    markdown.push_str(&content);
                }
            }
        }
        progress.inc(1);
        progress.set_message(format!("Processing {}", path.display()));
    }

    // Finish progress bar
    progress.finish_with_message("Markdown generation complete.");

    if settings.include_toc {
        let toc = build_table_of_contents(&processed_files, input_path);
        markdown = format!("## Table of Contents\n\n{}\n\n{}", toc, markdown);
    }

    Ok(markdown)
}

fn collect_files(input_dir: &Path, ignore_patterns: &[String]) -> Vec<PathBuf> {
    WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| !is_ignored(entry.path(), ignore_patterns))
        .map(|entry| entry.into_path())
        .collect()
}

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

fn process_file(path: &Path, language: &str, include_info: bool) -> std::io::Result<String> {
    let content = fs::read_to_string(path)?;
    let metadata = fs::metadata(path)?;
    let modified_time: DateTime<Local> = metadata.modified()?.into();
    let file_size = metadata.len();

    let mut markdown = String::new();
    markdown.push_str("---\n\n");

    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        markdown.push_str(&format!("### {}\n\n", file_name));
    }

    if include_info {
        markdown.push_str(&format!(
            "*Last modified:* `{}` | *Size:* `{}` bytes\n\n",
            modified_time.format("%Y-%m-%d %H:%M:%S"),
            file_size
        ));
    }

    markdown.push_str(&format!("```{}\n", language));
    markdown.push_str(&content);
    if !content.ends_with('\n') {
        markdown.push('\n');
    }
    markdown.push_str("```\n\n");

    Ok(markdown)
}

fn is_ignored(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignore_patterns.iter().any(|pattern| {
        Pattern::new(pattern).map_or(false, |p| p.matches(&path_str))
    })
}

fn build_table_of_contents(file_paths: &[PathBuf], base_dir: &Path) -> String {
    file_paths.iter().filter_map(|path| {
        path.strip_prefix(base_dir).ok().map(|relative| {
            let display = relative.display().to_string();
            let anchor = display.replace(['/', '.'], "").to_lowercase();
            format!("- [{}](#{})", display, anchor)
        })
    }).collect::<Vec<_>>().join("\n")
}
