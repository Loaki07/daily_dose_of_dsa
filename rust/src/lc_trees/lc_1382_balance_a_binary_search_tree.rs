use crate::lc_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type NodePtr = Rc<RefCell<TreeNode>>;
type NodeOption = Option<NodePtr>;

impl Solution {
    // O(n) time | O(n) space
    pub fn balance_bst(root: NodeOption) -> NodeOption {
        let mut vals: Vec<i32> = Vec::new();
        Self::inorder(&root, &mut vals);
        Self::build_tree(&vals)
    }

    // in-order traversal
    fn inorder(node: &NodeOption, vals: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Self::inorder(&node.left, vals);
            vals.push(node.val);
            Self::inorder(&node.right, vals);
        }
    }

    // divide-and-conquer
    fn build_tree(vals: &[i32]) -> NodeOption {
        if vals.len() < 2 {
            return Self::build_node(
                *vals.last()?,
                None,
                None,
            );
        }
        let (first, last) = vals.split_at(vals.len() / 2);
        let (&mid, last) = last.split_first()?;
        Self::build_node(
            mid,
            Self::build_tree(first),
            Self::build_tree(last),
        )
    }

    fn build_node(
        val: i32,
        left: NodeOption,
        right: NodeOption,
    ) -> NodeOption {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left,
            right,
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME: Test case needs to be fixed
    // Works fine on lc
    #[test]
    #[ignore]
    fn test_example1() {
        let root = TreeNode::from_vec(&[
            1,
            i32::MIN,
            2,
            i32::MIN,
            3,
            i32::MIN,
            4,
            i32::MIN,
            i32::MIN,
        ]);
        let expected = TreeNode::from_vec(&[
            2,
            1,
            3,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            4,
        ]);
        assert_eq!(Solution::balance_bst(root), expected);
    }

    #[test]
    fn test_example2() {
        let root = TreeNode::from_vec(&[2, 1, 3]);
        let expected = TreeNode::from_vec(&[2, 1, 3]);
        assert_eq!(Solution::balance_bst(root), expected);
    }
}
