use crate::lc_utils::ListNode;

pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn double_it(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut hnew = None;
        let mut tail = &mut hnew;

        if head.as_ref().map_or(0, |h| h.val) > 4 {
            let node = ListNode::new(1);
            *tail = Some(Box::new(node));
            tail = &mut tail.as_mut().unwrap().next;
        }

        while let Some(mut node) = head {
            if node.next.as_ref().map_or(0, |n| n.val) > 4 {
                node.val = (node.val * 2 + 1) % 10;
            } else {
                node.val = (node.val * 2) % 10;
            }
            head = node.next.take();
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        hnew
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a linked list from a
    // vector
    fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &value in v.iter().rev() {
            let mut new_node = ListNode::new(value);
            new_node.next = current;
            current = Some(Box::new(new_node));
        }
        current
    }

    // Helper function to collect values from a linked
    // list into a vector
    fn list_to_vec(
        list: Option<Box<ListNode>>,
    ) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    #[test]
    fn test_double_it_example1() {
        let test_list = vec_to_list(vec![1, 8, 9]);
        let expected_output = vec_to_list(vec![3, 7, 8]);
        assert_eq!(
            Solution::double_it(test_list),
            expected_output
        );
    }

    #[test]
    fn test_double_it_example2() {
        let test_list = vec_to_list(vec![9, 9, 9]);
        let expected_output = vec_to_list(vec![1, 9, 9, 8]);
        assert_eq!(
            Solution::double_it(test_list),
            expected_output
        );
    }
}
