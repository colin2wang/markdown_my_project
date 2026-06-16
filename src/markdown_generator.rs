use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::Result;
use crate::tree_generator;

/// Returns the localized heading for a given key.
fn localized_text(key: &str, lang: &str) -> String {
    match (key, lang) {
        ("project_documentation", "zh_cn") => "项目文档".to_string(),
        ("project_documentation", _) => "Project Documentation".to_string(),
        ("project_file_tree", "zh_cn") => "项目文件树".to_string(),
        ("project_file_tree", _) => "Project File Tree".to_string(),
        ("project_files", "zh_cn") => "项目文件".to_string(),
        ("project_files", _) => "Project Files".to_string(),
        ("file_label", "zh_cn") => "文件".to_string(),
        ("file_label", _) => "File".to_string(),
        _ => key.to_string(),
    }
}

/// Generates Markdown documentation for a project based on its files and directories.
///
/// # Arguments
///
/// * `project_name` - Name of the project.
/// * `files` - List of files with their paths and contents.
/// * `languages` - Mapping of file extensions to language names.
/// * `project_root` - Path to the project root directory.
/// * `lang` - Output language: "zh_cn" or "en_us" (default).
///
/// # Returns
///
/// * `Result<String>` - The generated Markdown content or an error.
pub fn generate_markdown(
    project_name: &str,
    files: Vec<(PathBuf, String)>,
    languages: &HashMap<String, String>,
    project_root: &Path,
    lang: &str,
) -> Result<String> {
    // Sort files for consistent output
    let mut sorted_files = files;
    sorted_files.sort_by(|a, b| a.0.cmp(&b.0));

    let title = localized_text("project_documentation", lang);
    let mut markdown_content = format!("# {} for {}\n\n", title, project_name);

    // Add the project file tree to the Markdown (at the top)
    let tree_heading = localized_text("project_file_tree", lang);
    markdown_content.push_str(&format!("## {}\n\n", tree_heading));
    markdown_content.push_str("```\n");
    markdown_content.push_str(&format!("{}\n", project_name));
    markdown_content.push_str(&tree_generator::generate_tree(project_name, &sorted_files, project_root)?);
    markdown_content.push_str("```\n\n");

    // Add file contents to the Markdown
    let files_heading = localized_text("project_files", lang);
    let file_label = localized_text("file_label", lang);
    markdown_content.push_str(&format!("## {}\n\n", files_heading));
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
            "### {}: `{}`\n\n```{}\n{}\n```\n\n",
            file_label,
            display_path,
            language,
            content
        ));
    }

    Ok(markdown_content)
}
