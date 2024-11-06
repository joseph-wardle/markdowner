use glob::Pattern;
use std::fs;
use std::path::Path;

/// Builds a visual directory tree excluding ignored patterns.
pub fn build_directory_tree(
    base_dir: &Path,
    ignore_patterns: &[String],
) -> String {
    let mut tree = String::new();
    let dir_name = if base_dir.file_name().is_some() {
        base_dir.file_name().unwrap().to_string_lossy()
    } else {
        base_dir.to_string_lossy()
    };
    tree.push_str(&format!("{}/\n", dir_name));
    build_subtree(base_dir, "", base_dir, ignore_patterns, &mut tree);
    tree
}

fn build_subtree(
    current_dir: &Path,
    prefix: &str,
    base_dir: &Path,
    ignore_patterns: &[String],
    tree: &mut String,
) {
    let entries = match fs::read_dir(current_dir) {
        Ok(read_dir) => read_dir.filter_map(Result::ok).collect::<Vec<_>>(),
        Err(_) => return,
    };

    let mut sorted_entries = entries; // Ownership transferred; no cloning.

    sorted_entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    let total = sorted_entries.len();
    for (index, entry) in sorted_entries.into_iter().enumerate() {
        let path = entry.path();
        if is_path_ignored(&path, ignore_patterns) {
            continue;
        }

        let last_entry = index == total - 1;
        let connector = if last_entry { "└─" } else { "├─" };
        let name = path.file_name().unwrap().to_string_lossy();

        tree.push_str(&format!("{}{} {}\n", prefix, connector, name));

        if path.is_dir() {
            let new_prefix = if last_entry {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            build_subtree(&path, &new_prefix, base_dir, ignore_patterns, tree);
        }
    }
}

fn is_path_ignored(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignore_patterns.iter().any(|pattern| {
        Pattern::new(pattern).map_or(false, |p| p.matches(&path_str))
    })
}