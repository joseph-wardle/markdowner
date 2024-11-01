use std::collections::HashMap;
use std::fs;
use std::path::Path;
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
) -> String {
    let mut markdown_output = String::new();

    // Append directory tree to markdown_output
    markdown_output.push_str("```\n");
    markdown_output.push_str(directory_tree);
    markdown_output.push_str("```\n\n");

    // Traverse the directory recursively
    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if let Some(lang) = supported_extensions.get(ext) {
                match process_file(path, lang) {
                    Ok(content) => markdown_output.push_str(&content),
                    Err(e) => eprintln!("Warning: Could not read file {}: {}", path.display(), e),
                }
            }
        }
    }

    markdown_output
}

fn process_file(path: &Path, lang: &str) -> std::io::Result<String> {
    let contents = fs::read_to_string(path)?;
    let mut file_markdown = String::new();

    // Add separator
    file_markdown.push_str("---\n\n");

    // Add file name as a heading
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        file_markdown.push_str(&format!("### {}\n\n", file_name));
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
