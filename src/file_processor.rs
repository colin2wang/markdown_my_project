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