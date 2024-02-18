pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        use std::collections::HashMap;

        let s: Vec<char> = s.chars().collect();
        let (mut res, mut l, mut maxf) = (0, 0, 0);
        let mut count: HashMap<char, u64> = HashMap::new();

        for r in 0..s.len() {
            *count.entry(s[r]).or_default() += 1;
            maxf = maxf.max(*count.get(&s[r]).unwrap());

            // len(window) - maxf <= k
            while (r - l + 1) - maxf as usize > k as usize {
                *count.get_mut(&s[l]).unwrap() -= 1;
                l += 1;
            }

            res = res.max(r - l + 1);
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "ABAB".to_string();
        let k = 2;
        assert_eq!(
            Solution::character_replacement(s, k),
            4
        );
    }

    #[test]
    fn test_example_2() {
        let s = "AABABBA".to_string();
        let k = 1;
        assert_eq!(
            Solution::character_replacement(s, k),
            4
        );
    }
}
