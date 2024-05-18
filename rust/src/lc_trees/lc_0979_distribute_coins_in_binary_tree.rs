use crate::lc_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn distribute_coins(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            moves: &mut i32,
        ) -> i32 {
            if let Some(curr_node) = node {
                let l_extra = dfs(
                    curr_node.borrow().left.clone(),
                    moves,
                );
                let r_extra = dfs(
                    curr_node.borrow().right.clone(),
                    moves,
                );
                *moves += l_extra.abs() + r_extra.abs();
                // when calculating extra_coins
                // we do val - 1 to handle the case,
                // where node has only 1 extra coin,
                // that needs to be defaulted
                curr_node.borrow().val + l_extra + r_extra
                    - 1
            } else {
                0
            }
        }

        let mut moves = 0;
        dfs(root, &mut moves);
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Example 1: Input: root = [3,0,0], Output: 2
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(
                TreeNode::new(0),
            ))),
            right: Some(Rc::new(RefCell::new(
                TreeNode::new(0),
            ))),
        })));

        assert_eq!(Solution::distribute_coins(root), 2);
    }

    #[test]
    fn test_example_2() {
        // Example 2: Input: root = [0,3,0], Output: 3
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(
                TreeNode::new(3),
            ))),
            right: Some(Rc::new(RefCell::new(
                TreeNode::new(0),
            ))),
        })));

        assert_eq!(Solution::distribute_coins(root), 3);
    }
}
