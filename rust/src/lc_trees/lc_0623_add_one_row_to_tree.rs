use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub struct Solution;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    // O(n) time | O(n) space
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn new_node(
            val: i32,
            left: Node,
            right: Node,
        ) -> Node {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left,
                right,
            })))
        }

        // this recursively performs dfs until
        // it reaches a target depth; there, a new node is
        // added
        fn dfs(node: &Node, val: i32, depth: i32, d: i32) {
            if let Some(node) = node {
                if d + 1 == depth {
                    let mut node = node.borrow_mut();

                    node.left = new_node(
                        val,
                        node.left.take(),
                        None,
                    );
                    node.right = new_node(
                        val,
                        None,
                        node.right.take(),
                    );
                } else {
                    dfs(
                        &node.borrow().left,
                        val,
                        depth,
                        d + 1,
                    );
                    dfs(
                        &node.borrow().right,
                        val,
                        depth,
                        d + 1,
                    );
                }
            }
        }

        if depth == 1 {
            // [1] marginal case
            new_node(val, root, None)
        } else {
            // [2] typical case
            dfs(&root, val, depth, 1);
            root
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree_from_slice(
        slice: &[(i32, Option<usize>, Option<usize>)],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = Vec::with_capacity(slice.len());

        // First pass to create all nodes
        for &(val, _, _) in slice {
            nodes.push(Rc::new(RefCell::new(
                TreeNode::new(val),
            )));
        }

        // Second pass to assign children
        for (i, &(_, left, right)) in
            slice.iter().enumerate()
        {
            if let Some(left_index) = left {
                nodes[i].borrow_mut().left =
                    Some(Rc::clone(&nodes[left_index]));
            }
            if let Some(right_index) = right {
                nodes[i].borrow_mut().right =
                    Some(Rc::clone(&nodes[right_index]));
            }
        }

        nodes.get(0).map(Rc::clone)
    }

    #[test]
    fn test_case_1() {
        let tree_structure = &[
            (4, Some(1), Some(2)), // 0
            (2, Some(3), Some(4)), // 1
            (6, Some(5), None),    // 2
            (3, None, None),       // 3
            (1, None, None),       // 4
            (5, None, None),       // 5
        ];
        let root = build_tree_from_slice(tree_structure);
        let val = 1;
        let depth = 2;
        let modified_tree =
            Solution::add_one_row(root, val, depth);
        // Expected output structure
        let expected_structure = &[
            (4, Some(1), Some(2)), // 0
            (1, Some(3), None),    // 1
            (1, None, Some(4)),    // 2
            (2, Some(5), Some(6)), // 3
            (6, Some(7), None),    // 4
            (3, None, None),       // 5
            (1, None, None),       // 6
            (5, None, None),       // 7
        ];
        let expected_root =
            build_tree_from_slice(expected_structure);
        assert_eq!(modified_tree, expected_root);
    }

    #[test]
    fn test_case_2() {
        let tree_structure = &[
            (4, Some(1), None),    // 0
            (2, Some(2), Some(3)), // 1
            (3, None, None),       // 2
            (1, None, None),       // 3
        ];
        let root = build_tree_from_slice(tree_structure);
        let val = 1;
        let depth = 3;
        let modified_tree =
            Solution::add_one_row(root, val, depth);
        // Expected output structure
        let expected_structure = &[
            (4, Some(1), None),    // 0
            (2, Some(2), Some(3)), // 1
            (1, Some(4), None),    // 2
            (1, None, Some(5)),    // 3
            (3, None, None),       // 4
            (1, None, None),       // 5
        ];
        let expected_root =
            build_tree_from_slice(expected_structure);
        assert_eq!(modified_tree, expected_root);
    }

    // Additional test for root modification
    #[test]
    fn test_add_to_root() {
        let tree_structure = &[
            (1, None, None), // 0
        ];
        let root = build_tree_from_slice(tree_structure);
        let val = 2;
        let depth = 1;
        let modified_tree =
            Solution::add_one_row(root, val, depth);
        // Expected output structure
        let expected_structure = &[
            (2, Some(1), None), // 0
            (1, None, None),    // 1
        ];
        let expected_root =
            build_tree_from_slice(expected_structure);
        assert_eq!(modified_tree, expected_root);
    }
}
