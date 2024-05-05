use crate::lc_utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: 0,
            next: head.clone(),
        });
        let (mut left, mut right) = (dummy.as_mut(), head);

        let mut n = n;
        while n > 0 && right.is_some() {
            right = right.unwrap().next;
            n -= 1;
        }

        while let Some(r) = right {
            left = left.next.as_mut().unwrap();
            right = r.next;
        }

        left.next = left.next.take().unwrap().next.take();
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        let n = 2;
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: None,
                    })),
                })),
            })),
        }));
        assert_eq!(
            Solution::remove_nth_from_end(head, n),
            expected
        );
    }

    #[test]
    fn test_example2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: None,
        }));
        let n = 1;
        let expected = None;
        assert_eq!(
            Solution::remove_nth_from_end(head, n),
            expected
        );
    }

    #[test]
    fn test_example3() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: None,
            })),
        }));
        let n = 1;
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: None,
        }));
        assert_eq!(
            Solution::remove_nth_from_end(head, n),
            expected
        );
    }
}
