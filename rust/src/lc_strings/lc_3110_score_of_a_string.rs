pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|a| a[0].abs_diff(a[1]) as i32)
            .sum()
    }

    // O(n) time | O(1) space
    pub fn _score_of_string(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut res = 0;
        for i in 0..bytes.len() - 1 {
            res += (bytes[i] as i32 - bytes[i + 1] as i32)
                .abs();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("hello");
        assert_eq!(Solution::score_of_string(s), 13);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("zaz");
        assert_eq!(Solution::score_of_string(s), 50);
    }
}
