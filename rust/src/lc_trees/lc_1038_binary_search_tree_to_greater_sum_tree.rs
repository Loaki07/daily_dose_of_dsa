// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::lc_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type NodePtr = Rc<RefCell<TreeNode>>;
type NodeOpt = Option<NodePtr>;

impl Solution {
    pub fn bst_to_gst(root: NodeOpt) -> NodeOpt {
        Self::traverse(root.clone(), 0);
        root
    }

    // Reverse in-order traversal
    // O(n) time | O(h) space
    fn traverse(node: NodeOpt, val: i32) -> i32 {
        if let Some(node_ptr) = node {
            let mut node = node_ptr.borrow_mut();
            node.val +=
                Self::traverse(node.right.clone(), val);
            Self::traverse(node.left.clone(), node.val)
        } else {
            val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let root = TreeNode::from_vec(&[
            4,
            1,
            6,
            0,
            2,
            5,
            7,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            3,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            8,
        ]);

        let expected = TreeNode::from_vec(&[
            30,
            36,
            21,
            36,
            35,
            26,
            15,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            33,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            8,
        ]);

        assert_eq!(Solution::bst_to_gst(root), expected);
    }

    #[test]
    fn test_example2() {
        let root = TreeNode::from_vec(&[0, i32::MIN, 1]);
        let expected =
            TreeNode::from_vec(&[1, i32::MIN, 1]);
        assert_eq!(Solution::bst_to_gst(root), expected);
    }
}
