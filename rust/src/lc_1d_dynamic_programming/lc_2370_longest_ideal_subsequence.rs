pub struct Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let (mut res, mut table, k) =
            (1, [0; 200], k as usize);

        for &byte in s.as_bytes() {
            let (mut mx, byte) = (0, byte as usize);

            for v in (byte - k)..(byte + k + 1) {
                mx = mx.max(table[v]);
            }

            table[byte] = mx + 1;
            res = res.max(table[byte]);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_ideal_string_case_1() {
        let s = "acfgbd".to_string();
        let k = 2;
        assert_eq!(Solution::longest_ideal_string(s, k), 4);
    }

    #[test]
    fn test_longest_ideal_string_case_2() {
        let s = "abcd".to_string();
        let k = 3;
        assert_eq!(Solution::longest_ideal_string(s, k), 4);
    }

    #[test]
    fn test_longest_ideal_string_no_valid_subsequence() {
        let s = "zx".to_string();
        let k = 1;
        assert_eq!(Solution::longest_ideal_string(s, k), 1);
    }

    #[test]
    fn test_longest_ideal_string_single_character() {
        let s = "a".to_string();
        let k = 0;
        assert_eq!(Solution::longest_ideal_string(s, k), 1);
    }

    #[test]
    fn test_longest_ideal_string_large_k() {
        let s = "abcdefg".to_string();
        let k = 5;
        assert_eq!(Solution::longest_ideal_string(s, k), 7);
    }
}
