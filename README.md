# Markdowner

Markdowner is a Rust command-line tool that converts code files within a specified directory into a single, well-organized Markdown document. It offers customization options such as including a directory tree, table of contents, file metadata, and progress tracking, making it ideal for sharing code snippets in a structured format, such as with a large language model.

## Features

- Comprehensive Conversion: Transforms various code files into Markdown with syntax highlighting.
- Syntax Highlighting: Supports multiple programming languages based on file extensions.
- Directory Tree: Optionally includes a visual representation of the directory structure.
- Table of Contents: Automatically generates a table of contents linking to each code section.
- File Metadata: Adds details like file size and last modified date for each code snippet.
- Ignore Patterns: Allows specifying patterns to exclude certain files or directories from processing.
- Progress Tracking: Displays a progress bar to monitor the conversion process in real-time.

## Installation

To install Markdowner, follow these steps:

1.	Clone the Repository:

```
git clone <repository-url>
cd markdowner
```

2.	Build with Cargo:

```
cargo build --release
```

3.	Run the Executable:

The compiled binary will be located in the target/release directory.

```sh
./target/release/markdowner <input_directory> [OPTIONS]
```

Alternatively, you can install it directly using Cargo:

```
cargo install --path .
```

## Usage

```
markdowner <input_directory> [OPTIONS]
```

#### Arguments

- `<input_directory>`:  Required. The path to the directory containing the code files you want to convert.

#### Options

- `-o, --output <file>`: Specifies the output Markdown file path. If not provided, the output is directed to stdout.
- `-i, --ignore <patterns>`: Glob patterns to ignore specific files or directories. Multiple patterns can be specified by repeating the option.
- `-t, --toc`: Includes a table of contents at the beginning of the Markdown document.
- `-f, --file-info`: Adds file metadata (size, last modified date) under each file’s heading.
- `-d, --directory-tree`: Includes a visual directory tree structure in the Markdown document.

## Examples

1.	Basic Conversion:
      Convert all code files in the src/ directory and output to output.md:

```
markdowner src/ -o output.md
```

2.	Including Table of Contents and File Metadata:

```
markdowner src/ -o documentation.md --toc --file-info
```

3.	Ignoring Specific Patterns:
      Exclude the node_modules directory and all .log files:

```
markdowner src/ -o cleaned.md --ignore "node_modules/*" --ignore "*.log"
```

4.	Including Directory Tree:

```
markdowner src/ -o tree_documentation.md --directory-tree
```

5.	Combining Multiple Options:

```
markdowner src/ -o full_documentation.md --ignore "tests/*" --toc --file-info --directory-tree
```


## Sample Output

After running the command:

```
markdowner src/ -o documentation.md --toc --file-info --directory-tree
```

The documentation.md might look like this:

## Table of Contents

- [main.rs](#mainrs)
- [lib.rs](#librs)

src/
├─ main.rs
└─ lib.rs

---

### main.rs

```rust
fn main() {
    crate::lib::hello_world();
}
```
Last modified: 2024-04-27 10:15:30 | Size: 256 bytes

### lib.rs

```rs
pub fn hello_world() {
    println!("Hello, Markdowner!");
}
```
Last modified: 2024-04-27 10:10:20 | Size: 198 bytes

## Configuration

**Markdowner** uses a configuration file to manage default settings. By default, it looks for a configuration named `markdowner` using the [Confy](https://crates.io/crates/confy) crate, which typically resides in the user's configuration directory.

### Configuration File Structure

```toml
ignore_patterns = ["/.git/*", "*.DS_Store"]
default_output = "README.md"
include_directory_tree = false
include_toc = false
include_file_info = false
```

### Overriding Configuration with Command-Line Arguments

Command-line arguments take precedence over configuration file settings. For instance, specifying --toc in the command line will enable the table of contents regardless of the configuration file.

# License

This project is licensed under the MIT License. See the LICENSE file for details.

# Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your enhancements.

# Contact

For any inquiries or support, please open an issue on the GitHub repository.