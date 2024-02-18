#[derive(Debug, Clone, PartialEq)]
pub struct BST {
    value: i32,
    left: Option<Box<BST>>,
    right: Option<Box<BST>>,
}

impl BST {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: i32) {
        let mut current = self;

        loop {
            if value < current.value {
                if let Some(ref mut left) = current.left {
                    current = left;
                } else {
                    current.left =
                        Some(Box::new(BST::new(value)));
                    break;
                }
            } else {
                if let Some(ref mut right) = current.right {
                    current = right
                } else {
                    current.right =
                        Some(Box::new(BST::new(value)));
                    break;
                }
            }
        }
    }

    pub fn contains(&self, value: i32) -> bool {
        let mut current = Some(self);

        while let Some(node) = current {
            if value < node.value {
                current = node.left.as_deref();
            } else if value > node.value {
                current = node.right.as_deref();
            } else {
                return true;
            }
        }

        false
    }

    pub fn remove(&mut self, value: i32) {
        self.remove_node(value, None);
    }

    // TODO: Remove Node needs to be fixed
    fn remove_node(
        &mut self,
        value: i32,
        parent: Option<&mut Box<BST>>,
    ) {
        // if value < self.value {
        //     if let Some(ref mut left) = self.left {
        //         let parent = Some(&mut
        // Box::new(*self));         left.
        // remove_node(value, parent);     }
        // } else if value > self.value {
        //     if let Some(ref mut right) = self.right {
        //         let parent = Some(&mut
        // Box::new(*self));         right.
        // remove_node(value, parent);     }
        // } else {
        //     // Node with two children
        //     if self.left.is_some() &&
        // self.right.is_some() {         let
        // min_val = self             .right
        //             .as_ref()
        //             .unwrap()
        //             .get_min_value();
        //         self.value = min_val;
        //         self.right.as_mut().unwrap().
        // remove_node(             min_val,
        //             Some(&mut Box::new(self)),
        //         );
        //     }
        //     // Node with one or no child
        //     else {
        //         let new_child = if self.left.is_some()
        // {             self.left.take()
        //         } else {
        //             self.right.take()
        //         };

        //         if let Some(p) = parent {
        //             if p.left
        //                 .as_ref()
        //                 .map_or(false, |node| {
        //                     node.value == value
        //                 })
        //             {
        //                 p.left = new_child;
        //             } else {
        //                 p.right = new_child;
        //             }
        //         } else {
        //             if let Some(new_child) = new_child
        // {                 self.value =
        // new_child.value;
        // self.left = new_child.left;
        // self.right = new_child.right;
        //             } else {
        //                 // This is a leaf node
        //                 // removal, handle
        //                 // accordingly
        //                 // For root with no
        //                 // children, this might
        //                 // mean setting the root
        //                 // to a specific state
        //             }
        //         }
        //     }
        todo!()
    }

    pub fn get_min_value(&self) -> i32 {
        let mut current = self;

        while let Some(ref left) = current.left {
            current = left;
        }

        current.value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut bst = BST::new(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(2);
        bst.insert(14);
        bst.insert(22);

        assert!(bst.contains(10));
        assert!(bst.contains(5));
        assert!(bst.contains(15));
        assert!(bst.contains(2));
        assert!(bst.contains(14));
        assert!(bst.contains(22));
        assert!(!bst.contains(99));
    }

    #[test]
    fn test_get_min_value() {
        let mut bst = BST::new(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(1);
        bst.insert(2);
        bst.insert(14);
        bst.insert(22);

        assert_eq!(bst.get_min_value(), 1);
    }

    // #[test]
    // fn test_bst_remove_leaf_node() {
    //     let mut bst = BST::new(10);
    //     bst.insert(5);
    //     bst.insert(15);
    //     bst.remove(15);
    //     assert!(!bst.contains(15));
    // }

    // #[test]
    // fn test_bst_remove_node_with_one_child() {
    //     let mut bst = BST::new(10);
    //     bst.insert(5);
    //     bst.insert(15);
    //     bst.insert(12);
    //     bst.remove(15);
    //     assert!(!bst.contains(15));
    //     assert!(bst.contains(12));
    // }

    // #[test]
    // fn test_bst_remove_node_with_two_children()
    // {     let mut bst = BST::new(10);
    //     bst.insert(5);
    //     bst.insert(15);
    //     bst.insert(12);
    //     bst.insert(17);
    //     bst.remove(15);
    //     assert!(!bst.contains(15));
    //     assert!(bst.contains(12));
    //     assert!(bst.contains(17));
    // }

    // #[test]
    // fn test_bst_remove_root_node() {
    //     let mut bst = BST::new(10);
    //     bst.insert(5);
    //     bst.insert(15);
    //     bst.remove(10);
    //     assert!(!bst.contains(10));
    //     // Verifies that the BST still contains
    // other elements     assert!(bst.
    // contains(5));     assert!(bst.
    // contains(15)); }
}
