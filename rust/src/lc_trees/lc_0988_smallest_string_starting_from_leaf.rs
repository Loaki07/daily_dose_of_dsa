use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub struct Solution;

impl Solution {
    // O(n) time | O(h) space
    pub fn smallest_from_leaf(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> String {
        let mut ans = String::new();
        Solution::dfs(&root, String::new(), &mut ans);
        ans
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        mut path: String,
        ans: &mut String,
    ) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            let ch = char::from_u32(
                (node_ref.val as u32) + ('a' as u32),
            )
            .unwrap();
            path.insert(0, ch);

            if node_ref.left.is_none()
                && node_ref.right.is_none()
            {
                if ans.is_empty() || path < *ans {
                    *ans = path.clone();
                }
            }

            Solution::dfs(
                &node_ref.left,
                path.clone(),
                ans,
            );
            Solution::dfs(
                &node_ref.right,
                path.clone(),
                ans,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Helper function to construct a binary tree from
    // a slice of Option<i32>
    fn build_tree(
        nodes: &[Option<i32>],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node_ptrs: Vec<
            Option<Rc<RefCell<TreeNode>>>,
        > = vec![];

        // Create TreeNode for each Some(i) and push None
        // for None
        for &val in nodes {
            let node = val.map(|v| {
                Rc::new(RefCell::new(TreeNode::new(v)))
            });
            node_ptrs.push(node);
        }

        let mut k = 0; // This index will track the current node to
                       // attach children
        for i in 0..nodes.len() {
            if let Some(node) = node_ptrs[i].as_ref() {
                let left_index = 2 * i + 1; // Index of the left child
                let right_index = 2 * i + 2; // Index of the right child

                // Attach left child if in bounds and not
                // None
                if left_index < nodes.len() {
                    node.borrow_mut().left =
                        node_ptrs[left_index].clone();
                }

                // Attach right child if in bounds and not
                // None
                if right_index < nodes.len() {
                    node.borrow_mut().right =
                        node_ptrs[right_index].clone();
                }
            }
        }

        node_ptrs.get(0).cloned().flatten()
    }

    #[test]
    fn test_case_1() {
        let nodes = vec![
            Some(0),
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(3),
            Some(4),
        ];
        let root = build_tree(&nodes);
        assert_eq!(
            Solution::smallest_from_leaf(root),
            "dba".to_string()
        );
    }

    #[test]
    fn test_case_2() {
        let nodes = vec![
            Some(25),
            Some(1),
            Some(3),
            Some(1),
            Some(3),
            Some(0),
            Some(2),
        ];
        let root = build_tree(&nodes);
        assert_eq!(
            Solution::smallest_from_leaf(root),
            "adz".to_string()
        );
    }

    #[test]
    fn test_case_3() {
        let nodes = vec![
            Some(2),
            Some(2),
            Some(1),
            None,
            Some(1),
            Some(0),
            None,
            Some(0),
        ];
        let root = build_tree(&nodes);
        assert_eq!(
            Solution::smallest_from_leaf(root),
            "abc".to_string()
        );
    }
}
