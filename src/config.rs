use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ignore_patterns: Vec<String>,
    pub default_output: Option<String>,
    pub include_directory_tree: bool,
    pub include_toc: bool,
    pub include_file_info: bool,

}

impl Default for Config {
    fn default() -> Self {
        Self {
            ignore_patterns: vec![],
            default_output: None,
            include_directory_tree: false,
            include_toc: false,
            include_file_info: false,
        }
    }
}