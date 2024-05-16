use crate::lc_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn evaluate_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let node = root.as_ref().unwrap().borrow();

        match node.val {
            x @ (0 | 1) => x == 1,
            2 => {
                Solution::evaluate_tree(node.left.clone())
                    || Self::evaluate_tree(
                        node.right.clone(),
                    )
            }
            _ => {
                Solution::evaluate_tree(node.left.clone())
                    && Solution::evaluate_tree(
                        node.right.clone(),
                    )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(
                TreeNode::new(1),
            ))),
            right: Some(Rc::new(RefCell::new(
                TreeNode::new(3),
            ))),
        })));

        assert_eq!(Solution::evaluate_tree(root), true);
    }

    #[test]
    fn test_example2() {
        let root =
            Some(Rc::new(RefCell::new(TreeNode::new(0))));

        assert_eq!(Solution::evaluate_tree(root), false);
    }
}
