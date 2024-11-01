use glob::Pattern;
use std::fs;
use std::path::Path;

/// Generates a visual representation of the directory tree
pub fn generate_directory_tree(
    current_dir: &Path,
    prefix: &str,
    is_last: bool,
    base_dir: &Path,
    ignore_patterns: &[String],
) -> String {
    let mut tree_str = String::new();

    let dir_name = if current_dir == base_dir {
        format!(
            "{}/",
            current_dir
                .file_name()
                .unwrap_or_else(|| current_dir.as_os_str())
                .to_string_lossy()
        )
    } else {
        current_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    };

    if current_dir == base_dir {
        tree_str.push_str(&format!("{}\n", dir_name));
    } else {
        let connector = if is_last { "└─" } else { "├─" };
        tree_str.push_str(&format!("{}{} {}/\n", prefix, connector, dir_name));
    }

    let mut dir_entries = match fs::read_dir(current_dir) {
        Ok(entries) => entries.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(_) => return tree_str,
    };

    dir_entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    let total_entries = dir_entries.len();
    for (index, entry) in dir_entries.into_iter().enumerate() {
        let path = entry.path();

        if is_ignored(&path, ignore_patterns) {
            continue;
        }

        let is_last_entry = index == total_entries - 1;

        let new_prefix = if current_dir == base_dir {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        if path.is_dir() {
            let subtree = generate_directory_tree(&path, &new_prefix, is_last_entry, base_dir, ignore_patterns);
            tree_str.push_str(&subtree);
        } else {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let connector = if is_last_entry { "└─" } else { "├─" };
            tree_str.push_str(&format!("{}{} {}\n", new_prefix, connector, file_name));
        }
    }

    tree_str
}

fn is_ignored(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignore_patterns.iter().any(|pattern| {
        if let Ok(glob_pattern) = Pattern::new(pattern) {
            glob_pattern.matches(&path_str)
        } else {
            false
        }
    })
}
