use crate::lc_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    // O(n) time | O(n) space
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(mut r) = root {
            let mut rn = r.borrow_mut();

            rn.left = Self::remove_leaf_nodes(
                rn.left.take(),
                target,
            );
            rn.right = Self::remove_leaf_nodes(
                rn.right.take(),
                target,
            );

            if rn.left.is_none()
                && rn.right.is_none()
                && rn.val == target
            {
                return None;
            }

            return Some(r.clone());
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn build_tree(
        values: &[Option<i32>],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(
            values[0].unwrap(),
        )));
        let mut queue = vec![Rc::clone(&root)];
        let mut i = 1;
        while i < values.len() {
            let current = queue.remove(0);
            if let Some(Some(val)) = values.get(i) {
                let left = Rc::new(RefCell::new(
                    TreeNode::new(*val),
                ));
                current.borrow_mut().left =
                    Some(Rc::clone(&left));
                queue.push(left);
            }
            i += 1;
            if let Some(Some(val)) = values.get(i) {
                let right = Rc::new(RefCell::new(
                    TreeNode::new(*val),
                ));
                current.borrow_mut().right =
                    Some(Rc::clone(&right));
                queue.push(right);
            }
            i += 1;
        }
        Some(root)
    }

    fn compare_trees(
        a: &Option<Rc<RefCell<TreeNode>>>,
        b: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (Some(a_node), Some(b_node)) => {
                let a_borrow = a_node.borrow();
                let b_borrow = b_node.borrow();
                a_borrow.val == b_borrow.val
                    && compare_trees(
                        &a_borrow.left,
                        &b_borrow.left,
                    )
                    && compare_trees(
                        &a_borrow.right,
                        &b_borrow.right,
                    )
            }
            (None, None) => true,
            _ => false,
        }
    }

    #[test]
    fn test_example_1() {
        let root = build_tree(&[
            Some(1),
            Some(2),
            Some(3),
            Some(2),
            None,
            Some(2),
            Some(4),
        ]);
        let target = 2;
        let expected = build_tree(&[
            Some(1),
            None,
            Some(3),
            None,
            Some(4),
        ]);
        let result =
            Solution::remove_leaf_nodes(root, target);
        assert!(compare_trees(&result, &expected));
    }

    #[test]
    fn test_example_2() {
        let root = build_tree(&[
            Some(1),
            Some(3),
            Some(3),
            Some(3),
            Some(2),
        ]);
        let target = 3;
        let expected = build_tree(&[
            Some(1),
            Some(3),
            None,
            None,
            Some(2),
        ]);
        let result =
            Solution::remove_leaf_nodes(root, target);
        assert!(compare_trees(&result, &expected));
    }

    #[test]
    fn test_example_3() {
        let root = build_tree(&[
            Some(1),
            Some(2),
            None,
            Some(2),
            None,
            Some(2),
        ]);
        let target = 2;
        let expected = build_tree(&[Some(1)]);
        let result =
            Solution::remove_leaf_nodes(root, target);
        assert!(compare_trees(&result, &expected));
    }
}
