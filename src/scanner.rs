//! This module is responsible for scanning the file system.
//! It contains the logic to recursively traverse a directory and build a hierarchical
//! tree structure representing its contents.

use std::fs;
use std::path::Path;

/// Represents a node in the file system tree.
/// It can be either a file or a directory, and it owns its data.
#[derive(Debug, Clone, PartialEq)]
pub struct FileSystemNode {
    /// The name of the file or directory (e.g., "src", "main.rs").
    pub name: String,
    /// The total size of the node in bytes. For a file, it's the file size.
    /// For a directory, it's the sum of the sizes of all its children.
    pub size: u64,
    /// A vector of child nodes. This is empty for files.
    pub children: Vec<FileSystemNode>,
}

/// Recursively scans a directory and builds a tree of `FileSystemNode`'s.
///
/// This function walks through the file system starting from the given path.
/// It calculates the size of directories by summing their children's sizes.
///
/// # Arguments
///
/// * `path` - The path to the directory or file to build the tree from.
///
/// # Returns
///
/// A `Result` containing the root `FileSystemNode` of the scanned tree,
/// or an `io::Error` if scanning fails at the root level.
pub fn build_tree(path: &Path) -> Result<FileSystemNode, std::io::Error> {
    let metadata = fs::metadata(path)?;

    // Get the name of the file or directory from the path.
    let name = path
        .file_name()
        .unwrap_or_else(|| path.as_os_str()) // Fallback for paths like "/" or "."
        .to_string_lossy()
        .into_owned();

    if metadata.is_dir() {
        let mut children = Vec::new();
        let mut total_size = 0;

        // Read all entries in the directory.
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            
            // Recursively call build_tree for each child.
            match build_tree(&child_path) {
                Ok(child_node) => {
                    total_size += child_node.size;
                    children.push(child_node);
                }
                Err(e) => {
                    // Log an error for inaccessible files/dirs but continue scanning others.
                    // This makes the scan more resilient to permission errors.
                    eprintln!("Failed to scan {}: {}", child_path.display(), e);
                }
            }
        }
        children.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(FileSystemNode {
            name,
            size: total_size,
            children,
        })
    } else {
        // It's a file, so it has a defined size and no children.
        Ok(FileSystemNode {
            name,
            size: metadata.len(),
            children: Vec::new(), 
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, create_dir_all};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_build_tree() {
        // Create a temporary directory for our test file structure.
        let dir = tempdir().unwrap();
        let root = dir.path();

        // Create a test directory structure:
        // /
        // |- a.txt (10 bytes)
        // |- sub/
        //    |- b.txt (20 bytes)
        create_dir_all(root.join("sub")).unwrap();
        let mut file_a = File::create(root.join("a.txt")).unwrap();
        file_a.write_all(&[0; 10]).unwrap();
        let mut file_b = File::create(root.join("sub").join("b.txt")).unwrap();
        file_b.write_all(&[0; 20]).unwrap();

        // The expected structure.
        let expected = FileSystemNode {
            name: root.file_name().unwrap().to_string_lossy().into_owned(),
            size: 30,
            children: vec![
                FileSystemNode {
                    name: "a.txt".to_string(),
                    size: 10,
                    children: vec![],
                },
                FileSystemNode {
                    name: "sub".to_string(),
                    size: 20,
                    children: vec![
                        FileSystemNode {
                            name: "b.txt".to_string(),
                            size: 20,
                            children: vec![],
                        },
                    ],
                },
            ],
        };

        let result = build_tree(root).unwrap();
        assert_eq!(result, expected);
    }
}