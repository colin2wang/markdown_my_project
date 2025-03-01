# Project Documentation for Markdown My Project

## Project Files

### File: `cargo.toml`

```Text
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

### File: `src\config.rs`

```Rust
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

impl Config {
    /// Loads a project configuration from a YAML file.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the YAML configuration file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn std::error::Error>>` - The loaded configuration or an error.
    pub fn load(config_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(config_path)?;
        let config: Config = serde_yaml::from_str(&config_content)?;
        Ok(config)
    }
}
```

### File: `src\file_processor.rs`

```Rust
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Reads the content of a file.
///
/// # Arguments
///
/// * `file_path` - Path to the file to read.
///
/// # Returns
///
/// * `Result<String, io::Error>` - The content of the file or an error.
pub fn read_file_content(file_path: &Path) -> Result<String, io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Processes files and directories specified in the configuration.
///
/// # Arguments
///
/// * `project_path` - Path to the project root directory.
/// * `files` - List of specific files to process.
/// * `directories` - List of directories to process recursively.
///
/// # Returns
///
/// * `Result<Vec<(PathBuf, String)>, Box<dyn std::error::Error>>` - A list of file paths and their contents.
pub fn process_files(
    project_path: &PathBuf,
    files: &[PathBuf],
    directories: &[PathBuf],
) -> Result<Vec<(PathBuf, String)>, Box<dyn std::error::Error>> {
    let mut file_contents = Vec::new();

    // Process individual files
    for file in files {
        let full_path = project_path.join(file);
        if full_path.exists() && full_path.is_file() {
            let content = read_file_content(&full_path)?;
            file_contents.push((full_path, content));
        }
    }

    // Process files within directories recursively
    for dir in directories {
        let full_dir = project_path.join(dir);
        if full_dir.exists() && full_dir.is_dir() {
            process_directory(&full_dir, &mut file_contents)?;
        }
    }

    Ok(file_contents)
}

/// Recursively processes files within a directory.
///
/// # Arguments
///
/// * `dir` - Path to the directory to process.
/// * `file_contents` - Vector to store file paths and their contents.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error.
fn process_directory(dir: &Path, file_contents: &mut Vec<(PathBuf, String)>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let content = read_file_content(&path)?;
            file_contents.push((path, content));
        } else if path.is_dir() {
            process_directory(&path, file_contents)?;
        }
    }
    Ok(())
}
```

### File: `src\language.rs`

```Rust
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

/// Loads language definitions from a YAML file.
///
/// # Arguments
///
/// * `languages_path` - Path to the YAML file containing language definitions.
///
/// # Returns
///
/// * `Result<HashMap<String, String>, Box<dyn std::error::Error>>` - A map of file extensions to language names.
pub fn load_languages(languages_path: &std::path::Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(languages_path)?;
    let config: LanguageConfig = serde_yaml::from_str(&content)?;
    Ok(config.languages)
}

#[derive(Deserialize)]
struct LanguageConfig {
    languages: HashMap<String, String>,
}
```

### File: `src\logger.rs`

```Rust
use log::{self, LevelFilter};
use simplelog::{CombinedLogger, ConfigBuilder, TermLogger, WriteLogger};
use std::fs::{self, OpenOptions};
use std::path::Path;
use time::macros::format_description;

/// Initializes the logger to output logs to both the console and a file in the `logs` directory.
///
/// # Arguments
///
/// * `level` - The log level to set (e.g., Info, Debug, Error).
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error.
pub fn init_logger(level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the `logs` directory exists
    fs::create_dir_all("logs")?;

    // Define the log file path
    let log_file_path = Path::new("logs").join("project_documentation.log");

    // Define a custom time format with milliseconds (3 digits)
    let log_config = ConfigBuilder::new()
        .set_time_format_custom(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"))
        .set_time_offset_to_local()
        .unwrap()
        .build();

    // Open the log file in append mode
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .map_err(|e| {
            log::error!("Failed to open log file: {:?}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

    // Initialize CombinedLogger to log to both file and console
    CombinedLogger::init(vec![
        TermLogger::new(level, log_config.clone(), simplelog::TerminalMode::Stdout, simplelog::ColorChoice::Auto),
        WriteLogger::new(level, log_config, log_file),
    ])
        .map_err(|e| {
            log::error!("Failed to initialize logger: {:?}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

    log::info!("Logger initialized. Logs are being written to {:?}", log_file_path);
    Ok(())
}
```

### File: `src\main.rs`

```Rust
mod config;
mod file_processor;
mod markdown_generator;
mod logger;
mod language;

use std::fs;
use std::path::{Path, PathBuf};

/// The main entry point of the project documentation generator.
///
/// This function initializes the logger, loads language definitions, processes project configurations,
/// generates Markdown documentation, and writes the output to specified files.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger with the Info level
    logger::init_logger(log::LevelFilter::Info)?;

    log::info!("Starting to generate project documentation...");

    // Load common language definitions from the `languages.yml` file
    let languages_path = PathBuf::from("languages.yml");
    let languages = language::load_languages(&languages_path)?;
    log::info!("Loaded language definitions from languages.yml");

    // Iterate through all project configuration files in the "projects" directory
    let projects_dir = Path::new("projects");
    for entry in fs::read_dir(projects_dir)? {
        let entry = entry?;
        let config_path = entry.path();
        if config_path.is_file() && config_path.extension().unwrap_or_default() == "yml" {
            log::info!("Processing project configuration: {}", config_path.display());

            // Load the project configuration from the YAML file
            let config = config::Config::load(&config_path)?;
            log::info!("Loaded project configuration: {}", config.project_name);

            // Get the project root directory
            let project_root = Path::new(&config.project_path);

            // Process files and directories specified in the configuration
            let files = file_processor::process_files(&config.project_path, &config.files, &config.directories)?;
            log::info!("Processed files for project: {}", config.project_name);

            // Generate Markdown content for the project documentation
            let markdown_content = markdown_generator::generate_markdown(&config.project_name, files, &languages, project_root);

            // Write the generated Markdown content to the output file
            fs::create_dir_all("output")?;
            let output_path = Path::new("output").join(&config.output_file);
            fs::write(&output_path, markdown_content)?;
            log::info!("Generated documentation for project: {} -> {}", config.project_name, output_path.display());
        }
    }

    log::info!("Project documentation generation complete.");
    Ok(())
}
```

### File: `src\markdown_generator.rs`

```Rust
use std::path::{Path, PathBuf};
use std::collections::{BTreeMap, HashMap};

/// Generates Markdown documentation for a project based on its files and directories.
///
/// # Arguments
///
/// * `project_name` - Name of the project.
/// * `files` - List of files with their paths and contents.
/// * `languages` - Mapping of file extensions to language names.
/// * `project_root` - Path to the project root directory.
///
/// # Returns
///
/// * `String` - The generated Markdown content.
pub fn generate_markdown(
    project_name: &str,
    files: Vec<(PathBuf, String)>,
    languages: &HashMap<String, String>,
    project_root: &Path,
) -> String {
    let mut markdown_content = format!("# Project Documentation for {}\n\n", project_name);
    markdown_content.push_str("## Project Files\n\n");

    // Add file contents to the Markdown
    for (file_path, content) in &files {
        // Get the relative path of the file with respect to the project root
        let relative_path = file_path.strip_prefix(project_root).unwrap_or(file_path);
        let display_path = relative_path.display();

        // Determine the file extension and corresponding language
        let extension = file_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        let language = languages.get(&extension).unwrap_or(&"Text".to_string()).clone();

        markdown_content.push_str(&format!(
            "### File: `{}`\n\n```{}\n{}\n```\n\n",
            display_path, // Use relative path
            language,
            content
        ));
    }

    // Add the project file tree to the Markdown
    markdown_content.push_str("\n## Project File Tree\n\n");
    markdown_content.push_str("```\n"); // Start code block
    markdown_content.push_str(&generate_tree(project_name, &files, project_root)); // Generate tree structure
    markdown_content.push_str("```\n"); // End code block

    markdown_content
}

/// Generates a tree-like structure of the project files.
///
/// # Arguments
///
/// * `project_name` - Name of the project.
/// * `files` - List of files with their paths.
/// * `project_root` - Path to the project root directory.
///
/// # Returns
///
/// * `String` - The tree structure as a string.
fn generate_tree(
    project_name: &str,
    files: &[(PathBuf, String)],
    project_root: &Path,
) -> String {
    // Use a BTreeMap to store files and directories hierarchically
    let mut tree_map: BTreeMap<Vec<String>, Vec<String>> = BTreeMap::new();

    for (file_path, _) in files {
        // Get the relative path of the file with respect to the project root
        let relative_path = file_path.strip_prefix(project_root).unwrap_or(file_path);
        let components: Vec<String> = relative_path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();

        // Extract the file name
        let file_name = components.last().unwrap().clone();
        // Extract the directory path
        let dir_components = components[..components.len() - 1].to_vec();

        // Add the file to the corresponding directory in the tree map
        tree_map.entry(dir_components).or_insert_with(Vec::new).push(file_name);
    }

    // Recursively generate the tree structure
    fn generate_tree_recursive(
        tree_map: &BTreeMap<Vec<String>, Vec<String>>,
        current_path: &[String],
        indent: &str,
    ) -> String {
        let mut tree = String::new();

        for (i, (dir_path, files)) in tree_map.iter().enumerate() {
            let is_last = i == tree_map.len() - 1;

            // Skip if the current path is not a subdirectory of the current path
            if dir_path.len() > current_path.len() && dir_path.starts_with(current_path) {
                // Directory name
                let dir_name = &dir_path[current_path.len()];
                // Add directory to the tree
                tree.push_str(&format!(
                    "{}{}── {}\n",
                    indent,
                    if is_last { "└" } else { "├" },
                    dir_name
                ));

                // Recursively generate subdirectories
                let new_indent = if is_last { "    " } else { "│   " };
                tree.push_str(&generate_tree_recursive(tree_map, &dir_path, new_indent));
            } else if dir_path == current_path {
                // Files in the current directory
                for (j, file) in files.iter().enumerate() {
                    let is_last_file = j == files.len() - 1;
                    tree.push_str(&format!(
                        "{}{}── {}\n",
                        indent,
                        if is_last_file { "└" } else { "├" },
                        file
                    ));
                }
            }
        }

        tree
    }

    // Start generating the tree structure from the root directory
    let mut tree = format!("{}:\n", project_name); // Add project name to the first line of the tree
    tree.push_str(&generate_tree_recursive(&tree_map, &[], ""));
    tree
}
```

### File: `projects\project1.yml`

```YAML
# Project Configuration for Project Documentation

# Name of the project
project_name: "Markdown My Project"

# Path to the project root directory
project_path: "F:/Workspaces/JetBrains/RustRover/markdown_my_project"

# Output file path for the generated documentation
output_file: "markdown_my_project.md"

# List of specific files to include in the documentation
files:
  - "cargo.toml"

# List of directories to include in the documentation (files within these directories will be processed recursively)
directories:
  - src
  - test
  - projects
```

### File: `projects\project2.yml`

```YAML
# Project Configuration for My Project

# Name of the project
project_name: "My Project"

# Path to the project root directory
project_path: "path/to/project"

# Output file path for the generated documentation
output_file: "project.md"

# List of specific files to include in the documentation
files:
  - "file1.txt"
  - "file2.txt"

# List of directories to include in the documentation (files within these directories will be processed recursively)
directories:
  - "src"
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
