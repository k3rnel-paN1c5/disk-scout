//! This module provides the logic for generating a treemap layout.
//! It takes a hierarchical `FileSystemNode` tree and calculates the rectangular
//! coordinates and dimensions needed to visualize it.

use crate::scanner::FileSystemNode;

/// Represents a 2D rectangle with floating-point coordinates and dimensions.
/// This is used to define the boundaries for each node in the treemap.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Represents a single, drawable item in our treemap layout.
/// It links the file system data (name, size) to a specific `Rectangle`.
#[derive(Debug)]
pub struct TreemapNode {
    pub rect: Rectangle,
    pub name: String, 
    pub size: u64,
    pub depth: usize,
}

/// Generates a treemap layout from a `FileSystemNode` tree.
///
/// # Arguments
///
/// * `node` - A reference to the root `FileSystemNode` of the tree.
/// * `bounds` - The initial rectangle (e.g., the window) to fit the treemap into.
///
/// # Returns
///
/// A flat vector of `TreemapNode`'s, each representing a rectangle to be drawn.
pub fn generate_treemap(node: &FileSystemNode, bounds: Rectangle) -> Vec<TreemapNode> {
    let mut results = Vec::new();
    // The recursive helper function does the main work.
    calculate_layout(&node.children, bounds, &mut results, true, 1);
    results
}

/// A recursive helper function that implements the "slice-and-dice" treemap algorithm.
///
/// It sorts children by size, then alternates between slicing the `bounds` rectangle
/// vertically and horizontally to position the children.
fn calculate_layout(
    nodes: &[FileSystemNode],
    bounds: Rectangle,
    results: &mut Vec<TreemapNode>,
    slice_vertically: bool,
    depth: usize,
) {
    if nodes.is_empty() {
        return;
    }

    let mut sorted_nodes = nodes.to_vec();
    sorted_nodes.sort_by(|a, b| b.size.cmp(&a.size));

    // Calculate the total size of all nodes at this level.
    let total_size = nodes.iter().map(|n| n.size).sum::<u64>() as f64;
    if total_size == 0.0 {
        return;
    }

     // Keep track of our position as we lay out rectangles.
    let mut current_x = bounds.x;
    let mut current_y = bounds.y;

    for node in nodes {
        // The proportion of the total size this node occupies.
        let proportion = node.size as f64 / total_size;
        let child_bounds;

         // Decide whether to slice horizontally or vertically.
        if slice_vertically {
            // Slice vertically: lay out children from left to right.
            let width = bounds.width * proportion;
            child_bounds = Rectangle {
                x: current_x,
                y: current_y,
                width,
                height: bounds.height,
            };
            current_x += width;
        } else {
            // Slice horizontally: lay out children from top to bottom.
            let height = bounds.height * proportion;
            child_bounds = Rectangle {
                x: current_x,
                y: current_y,
                width: bounds.width,
                height,
            };
            current_y += height;
        }
        
        results.push(TreemapNode {
            rect: child_bounds,
            name: node.name.clone(),
            size: node.size,
            depth,
        });

        // Recursively call for the children, flipping the slice direction.
        if !node.children.is_empty() {
            calculate_layout(&node.children, child_bounds, results, !slice_vertically, depth+1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_treemap() {
        // A simple file system tree for testing.
        let tree = FileSystemNode {
            name: "root".to_string(),
            size: 60,
            children: vec![
                FileSystemNode { name: "a".to_string(), size: 30, children: vec![] },
                FileSystemNode { 
                    name: "b".to_string(), 
                    size: 20, 
                    children: vec![], //vec![FileSystemNode {
                    //     name: "b1".to_string(),
                    //     size: 20,
                    //     children: vec![],
                    // }], 
                },
                FileSystemNode { name: "c".to_string(), size: 10, children: vec![] },
            ],
        };

        let bounds = Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
        let layout = generate_treemap(&tree, bounds);

        // Expected layout:
        // 'a' takes 50% of the width (30/60)
        // 'b' takes 33.3% of the width (20/60)
        // 'c' takes 16.6% of the width (10/60)
        let expected_layout = vec![
            TreemapNode {
                rect: Rectangle { x: 0.0, y: 0.0, width: 50.0, height: 100.0 },
                name: "a".to_string(),
                size: 30,
                depth: 1,
            },
            TreemapNode {
                rect: Rectangle { x: 50.0, y: 0.0, width: 100.0/3.0, height: 100.0 },
                name: "b".to_string(),
                size: 20,
                depth: 1,
            },
            TreemapNode {
                rect: Rectangle { x: 50.0 + 100.0/3.0, y: 0.0, width: 100.0/6.0, height: 100.0 },
                name: "c".to_string(),
                size: 10,
                depth: 1,
            },
        ];
        let expected_depths = vec![
            ("a", 1),
            ("b", 1),
            ("c", 1),
        ];
        
        // Custom comparison to handle floating point inaccuracies
        assert_eq!(layout.len(), expected_layout.len());
        for (i, node) in layout.iter().enumerate() {
            let expected_node = &expected_layout[i];
            assert_eq!(node.name, expected_node.name);
            assert_eq!(node.size, expected_node.size);
            assert!((node.rect.x - expected_node.rect.x).abs() < 1e-9);
            assert!((node.rect.y - expected_node.rect.y).abs() < 1e-9);
            assert!((node.rect.width - expected_node.rect.width).abs() < 1e-9);
            assert!((node.rect.height - expected_node.rect.height).abs() < 1e-9);
        }
        for (name, depth) in expected_depths {
            let node = layout.iter().find(|n| n.name == name).unwrap();
            assert_eq!(node.depth, depth);
        }
    }
}