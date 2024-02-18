use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn level_order(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        // Self::do_level_recursive(&root, 0, &mut
        // result);
        Self::do_level_bfs(root, &mut result);
        result
    }

    fn do_level_recursive(
        root: &Option<Rc<RefCell<TreeNode>>>,
        level: usize,
        result: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            if result.len() <= level {
                result.push(Vec::new());
            }

            result[level].push(node.borrow().val);
            Self::do_level_recursive(
                &node.borrow().left,
                level + 1,
                result,
            );
            Self::do_level_recursive(
                &node.borrow().right,
                level + 1,
                result,
            );
        }
    }

    fn do_level_bfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            let mut queue = VecDeque::new();

            queue.push_back(node);

            while !queue.is_empty() {
                let level_size = queue.len();
                let mut current_level = Vec::new();

                for _ in 1..=level_size {
                    if let Some(current) = queue.pop_front()
                    {
                        let current =
                            current.as_ref().borrow();
                        current_level.push(current.val);
                        if let Some(left) =
                            current.left.clone()
                        {
                            queue.push_back(left);
                        }
                        if let Some(right) =
                            current.right.clone()
                        {
                            queue.push_back(right);
                        }
                    }
                }

                if !current_level.is_empty() {
                    result.push(current_level);
                }
            }
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
        let expected =
            vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(tree), expected);
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&vec![1]);
        assert_eq!(
            Solution::level_order(tree),
            vec![vec![1]]
        );
    }

    #[test]
    fn ex3() {
        assert!(Solution::level_order(None).is_empty());
    }
}
