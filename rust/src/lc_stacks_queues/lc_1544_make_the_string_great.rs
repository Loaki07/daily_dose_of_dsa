pub struct Solution;

impl Solution {
    // The ASCII value of the capital letter "Z" is 90
    // and the ASCII value of the lowercase letter "z"
    // is 122.
    // the difference is always 32
    // O(n) time | O(n) space
    pub fn make_good(s: String) -> String {
        let mut result = String::with_capacity(s.len());

        for ch in s.chars() {
            if let Some(&last_char) =
                result.as_bytes().last()
            {
                if (last_char as i32 - ch as i32).abs()
                    == 32
                {
                    result.pop();
                    continue;
                }
            }
            result.push(ch);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "leEeetcode".to_string();
        let output = Solution::make_good(input);
        assert_eq!(output, "leetcode".to_string());
    }

    #[test]
    fn test_example_2() {
        let input = "abBAcC".to_string();
        let output = Solution::make_good(input);
        assert_eq!(output, "".to_string());
    }

    #[test]
    fn test_example_3() {
        let input = "s".to_string();
        let output = Solution::make_good(input);
        assert_eq!(output, "s".to_string());
    }
}
