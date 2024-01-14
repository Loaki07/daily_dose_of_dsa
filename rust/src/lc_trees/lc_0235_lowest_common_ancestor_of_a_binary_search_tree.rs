use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p.as_ref().unwrap().borrow().val;
    let q = q.as_ref().unwrap().borrow().val;

    let p_path = find_path(&root, p)
        .into_iter()
        .rev()
        .collect::<Vec<i32>>();
    let q_path = find_path(&root, q)
        .into_iter()
        .rev()
        .collect::<Vec<i32>>();
    dbg!(&p_path, &q_path);
    let mut i = 0;

    while i < p_path.len() && i < q_path.len() {
        if p_path[i] != q_path[i] {
            break;
        }
        i += 1;
    }
    Some(Rc::new(RefCell::new(TreeNode::new(
        p_path[i - 1],
    ))))
}

// dfs to get the path
fn find_path(
    root: &Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Vec<i32> {
    let val = root.as_ref().unwrap().borrow().val;

    if val == target {
        return vec![target];
    }

    let mut path = if val < target {
        find_path(
            &root.as_ref().unwrap().borrow().right,
            target,
        )
    } else {
        find_path(
            &root.as_ref().unwrap().borrow().left,
            target,
        )
    };
    path.push(val);
    return path;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_to_2() {
        let tree = TreeNode::from_vec(&vec![
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
        let path = find_path(&tree, 2);
        assert_eq!(path, vec![2, 6]);
    }

    #[test]
    fn path_to_3() {
        let tree = TreeNode::from_vec(&vec![
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
        let path = find_path(&tree, 3);
        assert_eq!(path, vec![3, 4, 2, 6]);
    }

    #[test]
    fn ex1() {
        let tree = TreeNode::from_vec(&vec![
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
        let p =
            Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q =
            Some(Rc::new(RefCell::new(TreeNode::new(8))));
        assert_eq!(
            lowest_common_ancestor(tree, p, q)
                .unwrap()
                .borrow()
                .val,
            6
        );
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_vec(&vec![
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
        let p =
            Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));
        assert_eq!(
            lowest_common_ancestor(tree, p, q)
                .unwrap()
                .borrow()
                .val,
            2
        );
    }

    #[test]
    fn ex3() {
        let tree = TreeNode::from_vec(&vec![2, 1]);
        let p =
            Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q =
            Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(
            lowest_common_ancestor(tree, p, q)
                .unwrap()
                .borrow()
                .val,
            2
        );
    }
}
