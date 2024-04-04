pub struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut res = 0;
        let mut curr = 0;

        for ch in s.chars() {
            if ch == '(' {
                curr += 1;
            } else if ch == ')' {
                curr -= 1;
            }
            res = res.max(curr);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope.

    #[test]
    fn test_max_depth_example1() {
        let s = "(1+(2*3)+((8)/4))+1".to_string();
        assert_eq!(Solution::max_depth(s), 3);
    }

    #[test]
    fn test_max_depth_example2() {
        let s = "(1)+((2))+(((3)))".to_string();
        assert_eq!(Solution::max_depth(s), 3);
    }

    // Additional test case: Empty string.
    #[test]
    fn test_max_depth_empty_string() {
        let s = "".to_string();
        assert_eq!(Solution::max_depth(s), 0);
    }

    // Additional test case: No parentheses.
    #[test]
    fn test_max_depth_no_parentheses() {
        let s = "1 + 2 * 3".to_string();
        assert_eq!(Solution::max_depth(s), 0);
    }

    // Additional test case: Single level of depth.
    #[test]
    fn test_max_depth_single_depth() {
        let s = "(1 + 2 * 3)".to_string();
        assert_eq!(Solution::max_depth(s), 1);
    }

    // Additional test case: Long sequence of nested
    // parentheses.
    #[test]
    fn test_max_depth_long_sequence() {
        let s = "((((((((((10))))))))))".to_string();
        assert_eq!(Solution::max_depth(s), 10);
    }
}
