use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::{Context, Result};
use rayon::prelude::*;
use walkdir::WalkDir;

/// Reads the content of a file.
///
/// # Arguments
///
/// * `file_path` - Path to the file to read.
///
/// # Returns
///
/// * `Result<String>` - The content of the file or an error.
pub fn read_file_content(file_path: &Path) -> Result<String> {
    let mut file = fs::File::open(file_path)
        .context(format!("Failed to open file: {}", file_path.display()))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .context(format!("Failed to read file: {}", file_path.display()))?;
    Ok(content)
}

/// Processes files and directories specified in the configuration.
///
/// This function processes individual files and directories recursively, excluding
/// directories and files specified in the configuration.
///
/// # Arguments
///
/// * `project_path` - Path to the project root directory.
/// * `files` - List of specific files to process.
/// * `directories` - List of directories to process recursively.
/// * `exclude_directories` - List of directories to exclude from processing.
/// * `exclude_patterns` - List of glob patterns to exclude files.
/// * `max_file_size` - Maximum file size in bytes (optional).
///
/// # Returns
///
/// * `Result<Vec<(PathBuf, String)>>` - A list of file paths and their contents.
pub fn process_files(
    project_path: &PathBuf,
    files: &[PathBuf],
    directories: &[PathBuf],
    exclude_directories: &[String],
    exclude_patterns: &[String],
    max_file_size: Option<u64>,
) -> Result<Vec<(PathBuf, String)>> {
    let mut file_contents = Vec::new();

    // Process individual files
    for file in files {
        let full_path = project_path.join(file);
        if full_path.exists() && full_path.is_file() {
            if should_include_file(&full_path, exclude_patterns, max_file_size, project_path)? {
                let content = read_file_content(&full_path)?;
                file_contents.push((full_path, content));
            }
        }
    }

    // Process files within directories recursively
    for dir in directories {
        let full_dir = project_path.join(dir);
        if full_dir.exists() && full_dir.is_dir() {
            process_directory_parallel(&full_dir, &mut file_contents, exclude_directories, exclude_patterns, max_file_size, project_path)?;
        }
    }

    Ok(file_contents)
}

/// Processes files within a directory in parallel using rayon.
///
/// # Arguments
///
/// * `dir` - Path to the directory to process.
/// * `file_contents` - Vector to store file paths and their contents.
/// * `exclude_directories` - List of directories to exclude from processing.
/// * `exclude_patterns` - List of glob patterns to exclude files.
/// * `max_file_size` - Maximum file size in bytes (optional).
/// * `project_root` - Path to the project root directory.
///
/// # Returns
///
/// * `Result<()>` - Success or error.
fn process_directory_parallel(
    dir: &Path,
    file_contents: &mut Vec<(PathBuf, String)>,
    exclude_directories: &[String],
    exclude_patterns: &[String],
    max_file_size: Option<u64>,
    project_root: &Path,
) -> Result<()> {
    // Collect all file paths first
    let file_paths: Vec<PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !should_exclude_directory(e.path(), exclude_directories))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    // Process files in parallel
    let file_contents_mutex = Arc::new(Mutex::new(Vec::new()));
    
    file_paths.par_iter()
        .filter_map(|path| {
            if should_include_file(path, exclude_patterns, max_file_size, project_root).unwrap_or(false) {
                match read_file_content(path) {
                    Ok(content) => Some((path.clone(), content)),
                    Err(e) => {
                        log::warn!("Failed to read file {}: {}", path.display(), e);
                        None
                    }
                }
            } else {
                None
            }
        })
        .for_each(|(path, content)| {
            file_contents_mutex.lock().unwrap().push((path, content));
        });

    let mut collected = Arc::try_unwrap(file_contents_mutex)
        .unwrap()
        .into_inner()
        .unwrap();
    
    file_contents.append(&mut collected);
    Ok(())
}

/// Determines whether a file should be included based on exclude patterns and size limit.
///
/// # Arguments
///
/// * `file_path` - Path to the file to check.
/// * `exclude_patterns` - List of glob patterns to exclude files.
/// * `max_file_size` - Maximum file size in bytes (optional).
/// * `project_root` - Path to the project root directory.
///
/// # Returns
///
/// * `Result<bool>` - `true` if the file should be included, `false` otherwise.
fn should_include_file(
    file_path: &Path,
    exclude_patterns: &[String],
    max_file_size: Option<u64>,
    project_root: &Path,
) -> Result<bool> {
    // Check file size limit
    if let Some(max_size) = max_file_size {
        let metadata = fs::metadata(file_path)
            .context(format!("Failed to get metadata for: {}", file_path.display()))?;
        if metadata.len() > max_size {
            log::debug!("Skipping large file: {} ({} bytes > {} bytes)", 
                file_path.display(), metadata.len(), max_size);
            return Ok(false);
        }
    }

    // Check exclude patterns
    let relative_path = file_path.strip_prefix(project_root).unwrap_or(file_path);
    let relative_path_str = relative_path.to_string_lossy();
    
    for pattern in exclude_patterns {
        if pattern.contains('*') || pattern.contains('?') {
            // Use glob pattern matching
            if let Ok(compiler) = glob::Pattern::new(pattern) {
                if compiler.matches(&relative_path_str) {
                    log::debug!("Skipping file due to pattern '{}': {}", pattern, file_path.display());
                    return Ok(false);
                }
            }
        } else {
            // Exact match or directory match
            if *relative_path_str == *pattern || relative_path_str.starts_with(pattern.as_str()) {
                log::debug!("Skipping file due to pattern '{}': {}", pattern, file_path.display());
                return Ok(false);
            }
        }
    }

    Ok(true)
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
            return true;
        } else if pattern.starts_with("**/") {
            let dir_name_to_exclude = &pattern[3..];
            let current_dir_name = dir.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("");
            if current_dir_name == dir_name_to_exclude {
                return true;
            }
        } else if pattern.contains('/') || pattern.contains('\\') {
            // Pattern contains path separator, match against relative path
            let rel_path = dir.strip_prefix(Path::new(".")).unwrap_or(dir);
            if rel_path == Path::new(pattern) {
                return true;
            }
        } else {
            // Pattern is just a directory name, match against any component
            let current_dir_name = dir.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("");
            if current_dir_name == pattern {
                return true;
            }
            
            // Also check if any parent component matches
            for component in dir.components() {
                if let std::path::Component::Normal(name) = component {
                    if *name.to_string_lossy() == *pattern {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_read_file_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, World!").unwrap();

        let content = read_file_content(&file_path).unwrap();
        assert_eq!(content, "Hello, World!\n");
    }

    #[test]
    fn test_should_exclude_directory() {
        let dir = Path::new("target/debug");
        assert!(should_exclude_directory(dir, &["target".to_string()]));
        assert!(should_exclude_directory(dir, &["**/debug".to_string()]));
        assert!(!should_exclude_directory(dir, &["src".to_string()]));
    }

    #[test]
    fn test_should_include_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello").unwrap();

        // No exclusions
        assert!(should_include_file(&file_path, &[], None, temp_dir.path()).unwrap());

        // With exclude pattern
        assert!(!should_include_file(&file_path, &["*.txt".to_string()], None, temp_dir.path()).unwrap());

        // With size limit (small file)
        assert!(should_include_file(&file_path, &[], Some(1024), temp_dir.path()).unwrap());

        // With size limit (file too large)
        assert!(!should_include_file(&file_path, &[], Some(1), temp_dir.path()).unwrap());
    }
}
