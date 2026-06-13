use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::Result;
use crate::tree_generator;

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
/// * `Result<String>` - The generated Markdown content or an error.
pub fn generate_markdown(
    project_name: &str,
    files: Vec<(PathBuf, String)>,
    languages: &HashMap<String, String>,
    project_root: &Path,
) -> Result<String> {
    // Sort files for consistent output
    let mut sorted_files = files;
    sorted_files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut markdown_content = format!("# Project Documentation for {}\n\n", project_name);

    // Add the project file tree to the Markdown (at the top)
    markdown_content.push_str("## Project File Tree\n\n");
    markdown_content.push_str("```\n");
    markdown_content.push_str(&format!("{}\n", project_name));
    markdown_content.push_str(&tree_generator::generate_tree(project_name, &sorted_files, project_root)?);
    markdown_content.push_str("```\n\n");

    // Add file contents to the Markdown
    markdown_content.push_str("## Project Files\n\n");
    for (file_path, content) in &sorted_files {
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
            display_path,
            language,
            content
        ));
    }

    Ok(markdown_content)
}

