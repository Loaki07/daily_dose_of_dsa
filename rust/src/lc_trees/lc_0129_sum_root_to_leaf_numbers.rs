use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub struct Solution;

impl Solution {
    // O(n) time, n number of nodes
    // O(h) space, height of the binary tree
    pub fn sum_numbers(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        let mut ans = 0;

        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            path: i32,
            ans: &mut i32,
        ) {
            if let Some(n) = node {
                let n = n.borrow();

                if n.left.is_none() && n.right.is_none() {
                    *ans += path * 10 + n.val;
                    return;
                }

                dfs(n.left.clone(), path * 10 + n.val, ans);
                dfs(
                    n.right.clone(),
                    path * 10 + n.val,
                    ans,
                );
            }
        }

        dfs(root, 0, &mut ans);
        ans
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
            (1, Some(1), Some(2)), // 0
            (2, None, None),       // 1
            (3, None, None),       // 2
        ];
        let root = build_tree_from_slice(tree_structure);
        let expected = 25;
        assert_eq!(Solution::sum_numbers(root), expected);
    }

    #[test]
    fn test_case_2() {
        let tree_structure = &[
            (4, Some(1), Some(2)), // 0
            (9, Some(3), Some(4)), // 1
            (0, None, None),       // 2
            (5, None, None),       // 3
            (1, None, None),       // 4
        ];
        let root = build_tree_from_slice(tree_structure);
        let expected = 1026;
        assert_eq!(Solution::sum_numbers(root), expected);
    }

    // Additional tests for further validation
    #[test]
    fn test_case_single_node() {
        let tree_structure = &[
            (1, None, None), // 0
        ];
        let root = build_tree_from_slice(tree_structure);
        let expected = 1;
        assert_eq!(Solution::sum_numbers(root), expected);
    }
}
