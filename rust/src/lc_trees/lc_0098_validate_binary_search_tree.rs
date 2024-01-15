use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::inorder(root)
            .windows(2)
            .all(|window| window[0] < window[1])
    }

    fn inorder(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        if let Some(node) = node {
            let mut res = Self::inorder(
                node.borrow_mut().left.take(),
            );
            res.push(node.borrow().val);
            res.extend(&Self::inorder(
                node.borrow_mut().right.take(),
            ));
            res
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(Solution::is_valid_bst(
            TreeNode::from_vec(&[2, 1, 3])
        ));
    }

    #[test]
    fn ex2() {
        assert!(!Solution::is_valid_bst(
            TreeNode::from_vec(&[
                5,
                1,
                4,
                i32::MAX,
                i32::MAX,
                3,
                6
            ])
        ));
    }
}
