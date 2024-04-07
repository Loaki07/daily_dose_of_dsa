pub struct Solution;

impl Solution {
    // greedy solution
    //  O(n) time | O(1) space
    pub fn check_valid_string(s: String) -> bool {
        let (mut left_min, mut left_max) = (0, 0);

        for ch in s.chars() {
            match ch {
                '(' => {
                    left_min += 1;
                    left_max += 1;
                }
                ')' => {
                    left_min -= 1;
                    left_max -= 1;
                }
                _ => {
                    left_min -= 1;
                    left_max += 1;
                }
            }

            if left_max < 0 {
                return false;
            }

            if left_min < 0 {
                left_min = 0;
            }
        }

        left_min == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = String::from("()");
        let output = Solution::check_valid_string(input);
        assert_eq!(output, true);
    }

    #[test]
    fn test_case_2() {
        let input = String::from("(*)");
        let output = Solution::check_valid_string(input);
        assert_eq!(output, true);
    }

    #[test]
    fn test_case_3() {
        let input = String::from("(*))");
        let output = Solution::check_valid_string(input);
        assert_eq!(output, true);
    }
}
