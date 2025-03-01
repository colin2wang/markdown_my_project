# Markdown My Project

## Project Overview

**Markdown My Project** is a command-line tool written in Rust that generates Markdown documentation for software projects. It extracts file contents and directory structures from specified projects and presents them in a user-friendly Markdown format. This tool is designed to help developers quickly generate documentation for projects, including source code, configuration files, and other relevant resources.

## Features

- **Configuration-Driven**: Uses YAML configuration files to specify project details, files to include, and output paths.
- **Recursive File Processing**: Processes files and directories recursively, allowing for comprehensive documentation of project structures.
- **Language Detection**: IdentiExample Project:
└── cargo.toml
├── projects
│   └── example.yml
└── src
    ├── main.rs
    └── utils.rsfies programming languages based on file extensions using a customizable language definition file.
- **Markdown Generation**: Generates well-structured Markdown documentation with syntax highlighting for source code.
- **Logging**: Provides detailed logging during the documentation generation process, aiding in debugging and tracking progress.
- **Multi-Project Support**: Can generate documentation for multiple projects defined in separate configuration files within the `projects` directory.

## Example
For a detailed example of how `Markdown My Project` works, refer to the [Example.md](Example.md) file in the project directory. This file is the generated Markdown documentation for this project in the output directory.

## Installation

1. **Prerequisites**:
    - Rust and Cargo (Rust's package manager) installed on your system. You can install them from [Rust's official website](https://www.rust-lang.org/tools/install).

2. **Build from Source**:
   ```bash
   git clone https://github.com/colin2wang/markdown_my_project.git
   cd markdown_my_project
   cargo build --release
   ```
   Replace `your-repo` with the actual GitHub repository name if applicable.

3. **Executable Path**:
   The compiled binary will be located in the `target/release` directory. Add this directory to your system's PATH for easy access.

## Usage

1. **Project Configuration**:
   Create YAML configuration files in the `projects` directory. An example configuration file `projects/project1.yml` is provided:
   ```yaml
   project_name: "Markdown My Project"
   project_path: "F:/Workspaces/JetBrains/RustRover/markdown_my_project"
   output_file: "markdown_my_project.md"
   files:
     - "cargo.toml"
   directories:
     - src
     - test
     - projects
   ```

2. **Language Definitions**:
   Define language mappings in the `languages.yml` file:
   ```yaml
   languages:
     rs: Rust
     toml: TOML
     yml: YAML
     md: Markdown
   ```

3. **Run the Tool**:
   Execute the binary from the root directory:
   ```bash
   ./target/release/markdown_my_project
   ```
    - The tool will read configuration files from the `projects` directory.
    - Process the specified files and directories.
    - Generate Markdown documentation in the `output` directory based on the configurations.

## Output Structure

The generated Markdown file (`output/markdown_my_project.md`) will look like this:

```markdown
# Project Documentation for Markdown My Project

## Project Files

### File: `cargo.toml`

```toml
[package]
name = "markdown_my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
log = "0.4"
simplelog = "0.12"
time = "0.3.37"
```

### File: `src/config.rs`

```rust
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Configuration structure for a project.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Name of the project.
    pub project_name: String,
    /// Path to the project root directory.
    pub project_path: PathBuf,
    /// Path to the output file where the documentation will be saved.
    pub output_file: PathBuf,
    /// List of specific files to include in the documentation.
    pub files: Vec<PathBuf>,
    /// List of directories to include in the documentation (files within these directories will be processed recursively).
    pub directories: Vec<PathBuf>,
}
```

## Project File Tree

```
Markdown My Project:
└── cargo.toml
├── projects
│   ├── project1.yml
│   └── project2.yml
└── src
    ├── config.rs
    ├── file_processor.rs
    ├── language.rs
    ├── logger.rs
    ├── main.rs
    └── markdown_generator.rs
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contribution

Contributions are welcome! Please open issues or submit pull requests for any improvements or bug fixes.