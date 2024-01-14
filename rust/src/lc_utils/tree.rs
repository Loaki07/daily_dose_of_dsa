use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(
        vals: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let mut root = Self::new(vals[0]);

        root.fill(vals, 0);

        Some(Rc::new(RefCell::new(root)))
    }

    fn fill(&mut self, vals: &[i32], index: usize) {
        let left_node_index = index * 2 + 1;
        if left_node_index < vals.len()
            && vals[left_node_index] != i32::MIN
        {
            self.left = Some(Rc::new(RefCell::new(
                Self::new(vals[left_node_index]),
            )));

            self.left
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(vals, left_node_index);
        }

        let right_node_index = left_node_index + 1;
        if right_node_index < vals.len()
            && vals[right_node_index] != i32::MIN
        {
            self.right = Some(Rc::new(RefCell::new(
                Self::new(vals[right_node_index]),
            )));

            self.right
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(vals, right_node_index);
        }
    }

    pub fn into_array(&self) -> Vec<i32> {
        let mut result = Vec::new();
        self.walk(&mut result);
        result
    }

    // in-order level traversal
    fn walk(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.borrow().walk(result);
        }
        result.push(self.val);

        if let Some(right) = &self.right {
            right.borrow().walk(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        assert!(TreeNode::from_vec(&[]).is_none());
    }

    #[test]
    fn fuller_tree() {
        let tree =
            TreeNode::from_vec(&[4, 2, 7, 1, 3, 6, 9]);
        let result = tree.unwrap().borrow().into_array();
        assert_eq!(result, vec![1, 2, 3, 4, 6, 7, 9])
    }

    #[test]
    fn null_entry_tree() {
        let tree = TreeNode::from_vec(&[
            6,
            2,
            8,
            0,
            4,
            7,
            9,
            i32::MIN,
            i32::MIN,
            3,
            5,
        ]);
        let result = tree.unwrap().borrow().into_array();
        assert_eq!(result, vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
