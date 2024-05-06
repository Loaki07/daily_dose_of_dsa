use crate::lc_utils::ListNode;

pub struct Solution;

impl Solution {
    // recursive soln
    pub fn _remove_nodes(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut n) => match Self::remove_nodes(n.next)
            {
                Some(n2) if n2.val > n.val => Some(n2),
                opt => {
                    n.next = opt;
                    Some(n)
                }
            },
        }
    }

    // alternate soln
    // O(n) time | O(1) space
    pub fn remove_nodes(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let reversed_head = Solution::reverse_list(head);
        let new_head = Solution::reverse_and_remove_nodes(
            reversed_head,
        );

        new_head
    }

    pub fn reverse_list(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = head;

        while let Some(mut node) = cur.take() {
            cur = node.next;
            node.next = prev.take();
            prev = Some(node);
        }

        prev
    }

    pub fn reverse_and_remove_nodes(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = head;
        let mut max_val = i32::MIN;

        while let Some(mut node) = cur.take() {
            if node.val > max_val {
                max_val = node.val;
            }

            cur = node.next;

            if node.val < max_val {
                continue;
            }

            node.next = prev.take();
            prev = Some(node);
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let head = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 13,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 8,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 13,
            next: Some(Box::new(ListNode {
                val: 8,
                next: None,
            })),
        }));

        assert_eq!(Solution::remove_nodes(head), expected);
    }

    #[test]
    fn test_example_2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: None,
                    })),
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: None,
                    })),
                })),
            })),
        }));

        assert_eq!(Solution::remove_nodes(head), expected);
    }
}
