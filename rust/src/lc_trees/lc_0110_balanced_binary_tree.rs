use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

struct Solution;

impl Solution {
    pub fn is_balanced(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::height_if_balanced(&root).is_some()
    }

    // dfs to find the height of each sub tree
    // the difference betweet the left and right
    // should be 1
    fn height_if_balanced(
        root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<i32> {
        if let Some(root) = root {
            let lh = Self::height_if_balanced(
                &root.borrow().left,
            );
            let rh = Self::height_if_balanced(
                &root.borrow().right,
            );

            match (lh, rh) {
                (Some(lh), Some(rh)) => {
                    if (lh - rh).abs() < 2 {
                        // +1 for this branch of the
                        // tree
                        return Some(lh.max(rh) + 1);
                    } else {
                        return None;
                    }
                }

                _ => return None,
            }
        } else {
            Some(0)
        }
    }
}

#[cfg(test)]
mod tests {
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
        assert!(Solution::is_balanced(tree));
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&vec![
            1,
            2,
            2,
            3,
            3,
            i32::MIN,
            i32::MIN,
            4,
            4,
        ]);
        assert!(!Solution::is_balanced(tree));
    }

    #[test]
    fn ex3() {
        assert!(Solution::is_balanced(None));
    }
}
