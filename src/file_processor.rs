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
/// This function processes individual files and directories recursively, excluding
/// directories specified in the `exclude_directories` list.
///
/// # Arguments
///
/// * `project_path` - Path to the project root directory.
/// * `files` - List of specific files to process.
/// * `directories` - List of directories to process recursively.
/// * `exclude_directories` - List of directories to exclude from processing.
///
/// # Returns
///
/// * `Result<Vec<(PathBuf, String)>, Box<dyn std::error::Error>>` - A list of file paths and their contents.
pub fn process_files(
    project_path: &PathBuf,
    files: &[PathBuf],
    directories: &[PathBuf],
    exclude_directories: &[String],
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
            process_directory(&full_dir, &mut file_contents, exclude_directories)?;
        }
    }

    Ok(file_contents)
}

/// Recursively processes files within a directory.
///
/// This function traverses the directory tree, excluding directories specified
/// in the `exclude_directories` list.
///
/// # Arguments
///
/// * `dir` - Path to the directory to process.
/// * `file_contents` - Vector to store file paths and their contents.
/// * `exclude_directories` - List of directories to exclude from processing.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error.
fn process_directory(
    dir: &Path,
    file_contents: &mut Vec<(PathBuf, String)>,
    exclude_directories: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let content = read_file_content(&path)?;
            file_contents.push((path, content));
        } else if path.is_dir() {
            if should_exclude_directory(&path, exclude_directories) {
                continue; // Skip excluded directories
            }
            process_directory(&path, file_contents, exclude_directories)?;
        }
    }
    Ok(())
}

/// Determines whether a directory should be excluded based on the `exclude_directories` list.
///
/// # Arguments
///
/// * `dir` - Path to the directory to check.
/// * `exclude_directories` - List of directories to exclude.
///
/// # Returns
///
/// * `bool` - `true` if the directory should be excluded, `false` otherwise.
fn should_exclude_directory(dir: &Path, exclude_directories: &[String]) -> bool {
    for pattern in exclude_directories {
        if pattern == "**" {
            // Exclude all directories if pattern is "**"
            return true;
        } else if pattern.starts_with("**/") {
            // Handle wildcard pattern (e.g., "**/444")
            let dir_name_to_exclude = &pattern[3..]; // Remove the "**/" prefix
            let current_dir_name = dir.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("");
            if current_dir_name == dir_name_to_exclude {
                return true;
            }
        } else {
            // Handle specific directory path (e.g., "111/222")
            let rel_path = dir.strip_prefix(Path::new(".")).unwrap_or(dir); // Get relative path
            if rel_path == Path::new(pattern) {
                return true;
            }
        }
    }
    false
}