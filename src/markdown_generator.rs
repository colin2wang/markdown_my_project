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
    markdown_content.push_str(&*(project_name.to_owned() + "\n")); // Start code block
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
/// Represents a directory in the tree structure.
#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<String>,
    subdirectories: BTreeMap<String, Directory>,
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: Vec::new(),
            subdirectories: BTreeMap::new(),
        }
    }

    fn add_file(&mut self, file: String) {
        self.files.push(file);
    }
}

fn build_directory_tree(files: &[(PathBuf, String)], project_root: &Path) -> Directory {
    let mut root = Directory::new("".to_string());

    for (file_path, _) in files {
        let relative_path = file_path.strip_prefix(project_root).unwrap_or(file_path);
        let components: Vec<String> = relative_path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();

        let mut current_dir = &mut root;
        for (i, component) in components.iter().enumerate() {
            if i < components.len() - 1 {
                let entry = current_dir.subdirectories.entry(component.clone());
                let sub_dir = entry.or_insert_with(|| Directory::new(component.clone()));
                current_dir = sub_dir;
            } else {
                current_dir.add_file(component.clone());
            }
        }
    }

    root
}

fn directory_tree_to_string(
    directory: &Directory,
    indent: &str,
    is_last: bool,
    is_root: bool,
) -> String {
    let mut tree = String::new();
    if !is_root {
        let connector = if is_last { "└── " } else { "├── " };
        let line = format!("{}{}{}", indent, connector, directory.name);
        tree.push_str(&line);
        tree.push('\n');
    }

    let new_indent = if is_last { format!("{}{}", indent, "    ") } else { format!("{}{}", indent, "│   ") };

    // Process files
    for (i, file) in directory.files.iter().enumerate() {
        let is_last_file = i == directory.files.len() - 1;
        let connector = if is_last_file { "└── " } else { "├── " };
        tree.push_str(&format!("{}{}{}\n", new_indent, connector, file));
    }

    // Process subdirectories
    let subdirs: Vec<_> = directory.subdirectories.values().collect();
    for (i, sub_dir) in subdirs.iter().enumerate() {
        let is_last_sub = i == subdirs.len() - 1;
        let sub_tree = directory_tree_to_string(sub_dir, &new_indent, is_last_sub, false);
        tree.push_str(&sub_tree);
    }

    tree
}

fn generate_tree(
    project_name: &str,
    files: &[(PathBuf, String)],
    project_root: &Path,
) -> String {
    let mut root = build_directory_tree(files, project_root);
    root.name = project_name.to_string();

    let tree = directory_tree_to_string(&root, "", true, true);
    // Ensure there's a newline at the end for markdown formatting

    tree
}