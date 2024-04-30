pub struct Solution;

impl Solution {
    // O(2 ^ A) time | O(n * A) space
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut cnt: [i64; 1024] = [0; 1024];
        let (mut total, mut prefix) = (0, 0);
        cnt[0] = 1;

        for c in word.bytes() {
            prefix ^= (1 << (c - 97)) as usize;
            total += cnt[prefix];

            for i in 0..10 {
                total += cnt[prefix ^ (1 << i)];
            }

            cnt[prefix] += 1;
        }

        return total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wonderful_substrings_case_1() {
        let word = "aba".to_string();
        assert_eq!(Solution::wonderful_substrings(word), 4);
    }

    #[test]
    fn test_wonderful_substrings_case_2() {
        let word = "aabb".to_string();
        assert_eq!(Solution::wonderful_substrings(word), 9);
    }

    #[test]
    fn test_wonderful_substrings_case_3() {
        let word = "he".to_string();
        assert_eq!(Solution::wonderful_substrings(word), 2);
    }

    // Additional tests to cover more scenarios
    #[test]
    fn test_wonderful_substrings_all_same_characters() {
        let word = "aaaa".to_string();
        assert_eq!(
            Solution::wonderful_substrings(word),
            10
        ); // "a", "aa", "aaa", "aaaa" each
           // considered once + each individual
           // "a"
    }

    #[test]
    fn test_wonderful_substrings_no_repeats() {
        let word = "abcd".to_string();
        assert_eq!(Solution::wonderful_substrings(word), 4);
        // Each individual letter is wonderful
    }
}
