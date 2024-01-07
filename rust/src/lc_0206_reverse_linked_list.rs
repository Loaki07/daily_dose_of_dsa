use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut arr = vec![];

        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }

        let mut rev_ptr = None;

        for val in arr {
            let mut node = ListNode::new(val);
            node.next = rev_ptr;
            rev_ptr = Some(Box::new(node));
        }

        rev_ptr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
        let reverse = Solution::reverse_list(list).unwrap();
        let reverse = reverse.into_array();
        assert_eq!(reverse, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn ex2() {
        let list = ListNode::from_vec(&vec![1, 2]);
        let reverse = Solution::reverse_list(list).unwrap();
        let reverse = reverse.into_array();
        assert_eq!(reverse, vec![2, 1]);
    }

    #[test]
    fn ex3() {
        assert!(Solution::reverse_list(None).is_none());
    }
}
