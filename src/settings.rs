use crate::args::Args;
use crate::config::Config;
use std::path::PathBuf;

/// Combines command-line arguments and configuration file settings.
#[derive(Debug)]
pub struct Settings {
    pub input_dir: PathBuf,
    pub output_path: Option<PathBuf>,
    pub ignore_patterns: Vec<String>,
    pub include_toc: bool,
    pub include_file_info: bool,
    pub include_directory_tree: bool,
}

impl Settings {
    pub fn from_args_and_config(args: Args, config: Config) -> Self {
        let mut ignore_patterns = args.ignore.clone();
        ignore_patterns.extend(config.ignore_patterns.clone());

        Settings {
            input_dir: args.input_dir,
            output_path: args.output.or_else(|| config.default_output.map(PathBuf::from)),
            ignore_patterns,
            include_toc: args.toc || config.include_toc,
            include_file_info: args.file_info || config.include_file_info,
            include_directory_tree: args.directory_tree || config.include_directory_tree,
        }
    }
}