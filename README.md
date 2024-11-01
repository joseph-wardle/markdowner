# Markdowner

**Markdowner** is a Rust command-line tool that converts code files within a directory into a single, well-organized Markdown document. It includes options for customizing output with a directory tree, table of contents, file metadata, and progress tracking.

## Features

- Converts code files in a specified directory to Markdown format
- Supports syntax highlighting for multiple programming languages
- Optionally includes a visual directory tree and table of contents
- Can display file metadata (size, last modified date) for each file
- Progress bar for tracking file processing in large directories

## Installation

Clone the repository and build with Cargo:

```bash
git clone <repository-url>
cd markdowner
cargo build --release
```

## Usage

```bash
markdowner --input <directory> [OPTIONS]
```

### Options

- `-i, --input <directory>`: **Required**. Specifies the input directory containing code files.
- `-o, --output <file>`: Specifies the output Markdown file (defaults to stdout).
- `--ignore <patterns>`: Ignores specified files or directories, e.g., `--ignore "node_modules/*"`.
- `--toc`: Includes a table of contents at the beginning of the Markdown document.
- `--file-info`: Adds file metadata (size, last modified date) under each file heading.
- `--progress`: Displays a progress bar for file processing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Example

```bash
markdowner --input src/
```

### Output:

```  
src/  
├─ lib.rs  
├─ main.rs  
└─ module.rs  
```  

---  

### lib.rs

```rust  
pub mod module;
```  
  
---  

### main.rs

```rust  
fn main() {
    crate::module::hello_world();
}
```  


---  

### module.rs

```rust  
pub fn hello_world() {
	println!("Hello world!");
}
```  
  

