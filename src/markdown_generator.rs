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