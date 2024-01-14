use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn max_depth(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        Self::height(&root)
    }

    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = Self::height(&node.borrow().left);
            let right = Self::height(&node.borrow().right);

            left.max(right) + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&vec![
            3,
            9,
            20,
            i32::MIN,
            i32::MIN,
            15,
            7,
        ]);
        assert_eq!(Solution::max_depth(tree), 3);
    }

    #[test]
    fn ex2() {
        let tree =
            TreeNode::from_vec(&vec![1, i32::MIN, 2]);
        assert_eq!(Solution::max_depth(tree), 2);
    }
}
