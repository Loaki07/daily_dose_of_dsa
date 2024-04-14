use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        // if it's left and it's leaf, return the value.
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            is_left: bool,
        ) -> i32 {
            if let Some(n) = node {
                let mut new_n = n.borrow_mut();
                if new_n.left.is_none()
                    && new_n.right.is_none()
                    && is_left
                {
                    return new_n.val;
                }

                return dfs(new_n.left.take(), true)
                    + dfs(new_n.right.take(), false);
            }
            0
        }

        dfs(root, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree_from_slice(
        slice: &[Option<i32>],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;
        let mut deque = VecDeque::new();
        let root = Rc::new(RefCell::new(TreeNode::new(
            slice[0].unwrap(),
        )));
        deque.push_back(Rc::clone(&root));

        let mut i = 1;
        while i < slice.len() {
            if let Some(node) = deque.pop_front() {
                if let Some(val) = slice[i] {
                    let left_child = Rc::new(RefCell::new(
                        TreeNode::new(val),
                    ));
                    node.borrow_mut().left =
                        Some(Rc::clone(&left_child));
                    deque.push_back(left_child);
                }
                i += 1;
                if i < slice.len() {
                    if let Some(val) = slice[i] {
                        let right_child =
                            Rc::new(RefCell::new(
                                TreeNode::new(val),
                            ));
                        node.borrow_mut().right =
                            Some(Rc::clone(&right_child));
                        deque.push_back(right_child);
                    }
                    i += 1;
                }
            }
        }

        Some(root)
    }

    #[test]
    fn test_case_1() {
        let slice = vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ];
        let root = build_tree_from_slice(&slice);
        let expected = 24;
        assert_eq!(
            Solution::sum_of_left_leaves(root),
            expected
        );
    }

    #[test]
    fn test_case_2() {
        let slice = vec![Some(1)];
        let root = build_tree_from_slice(&slice);
        let expected = 0;
        assert_eq!(
            Solution::sum_of_left_leaves(root),
            expected
        );
    }

    // Additional tests for further validation
    #[test]
    fn test_case_with_no_left_leaves() {
        let slice = vec![Some(1), None, Some(2)];
        let root = build_tree_from_slice(&slice);
        let expected = 0;
        assert_eq!(
            Solution::sum_of_left_leaves(root),
            expected
        );
    }

    #[test]
    fn test_case_with_all_left_leaves() {
        let slice = vec![Some(1), Some(2), None, Some(3)];
        let root = build_tree_from_slice(&slice);
        let expected = 3;
        assert_eq!(
            Solution::sum_of_left_leaves(root),
            expected
        );
    }
}
