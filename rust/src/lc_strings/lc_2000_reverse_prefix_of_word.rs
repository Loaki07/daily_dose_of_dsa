pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn reverse_prefix(
        word: String,
        ch: char,
    ) -> String {
        match word.chars().position(|c| c == ch) {
            Some(i) => word[..=i]
                .chars()
                .rev()
                .chain(word[i + 1..].chars())
                .collect(),
            None => word,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let word = "abcdefd".to_string();
        let ch = 'd';
        assert_eq!(
            Solution::reverse_prefix(word, ch),
            "dcbaefd"
        );
    }

    #[test]
    fn test_example_2() {
        let word = "xyxzxe".to_string();
        let ch = 'z';
        assert_eq!(
            Solution::reverse_prefix(word, ch),
            "zxyxxe"
        );
    }

    #[test]
    fn test_example_3() {
        let word = "abcd".to_string();
        let ch = 'z';
        assert_eq!(
            Solution::reverse_prefix(word, ch),
            "abcd"
        );
    }
}
