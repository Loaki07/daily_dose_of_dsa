#[derive(Debug, PartialEq)]
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
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => {
                    self.left =
                        Some(Box::new(BST::new(value)))
                }
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => {
                    self.right =
                        Some(Box::new(BST::new(value)))
                }
            }
        }
    }
}

pub fn find_closest_value_in_bst(
    tree: &BST,
    target: i32,
) -> i32 {
    find_closest_value_in_bst_recursion_helper(
        &Some(tree),
        target,
        f64::INFINITY as i32,
    );
    find_closest_value_in_bst_iterative_helper(
        tree,
        target,
        f64::INFINITY as i32,
    )
}

// iterative solution
// Average: O(log(n)) time | O(1) space
// worst: O(n) time | O(1) space
fn find_closest_value_in_bst_iterative_helper(
    tree: &BST,
    target: i32,
    mut closest: i32,
) -> i32 {
    let mut current_node = Some(tree);

    while let Some(node) = current_node {
        let node_value = node.value;

        if (target - closest).abs()
            > (target - node.value).abs()
        {
            closest = node_value;
        }

        current_node = if target < node_value {
            node.left.as_deref()
        } else if target > node_value {
            node.right.as_deref()
        } else {
            break;
        }
    }

    closest
}

// recursive solution
// Average: O(log(n)) time | O(log(n)) space
// worst: O(n) time | O(n) space
fn find_closest_value_in_bst_recursion_helper(
    tree: &Option<&BST>,
    target: i32,
    closest: i32,
) -> i32 {
    match tree {
        Some(node) => {
            let current_closest = if (target - closest)
                .abs()
                > (target - node.value)
            {
                node.value
            } else {
                closest
            };

            if target < node.value {
                find_closest_value_in_bst_recursion_helper(
                    &node.left.as_deref(),
                    target,
                    current_closest,
                )
            } else if target > node.value {
                find_closest_value_in_bst_recursion_helper(
                    &node.right.as_deref(),
                    target,
                    current_closest,
                )
            } else {
                current_closest
            }
        }
        None => closest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_bst() -> BST {
        let mut bst = BST::new(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(2);
        bst.insert(5); // Duplicate values are okay for this example
        bst.insert(13);
        bst.insert(22);
        bst.insert(1);
        bst.insert(14);
        bst
    }

    #[test]
    fn test_find_closest_value() {
        let bst = build_bst();

        assert_eq!(
            find_closest_value_in_bst(&bst, 12),
            13,
            "Test 1 Failed"
        );
        assert_eq!(
            find_closest_value_in_bst(&bst, 14),
            14,
            "Test 2 Failed"
        );
        // this test case fails for some reason with
        // recursive approach
        assert_eq!(
            find_closest_value_in_bst(&bst, 18),
            15,
            "Test 3 Failed"
        );
        assert_eq!(
            find_closest_value_in_bst(&bst, 23),
            22,
            "Test 4 Failed"
        );
        assert_eq!(
            find_closest_value_in_bst(&bst, 0),
            1,
            "Test 5 Failed"
        );
    }
}
