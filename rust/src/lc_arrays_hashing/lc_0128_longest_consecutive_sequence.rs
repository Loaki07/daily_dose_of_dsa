pub struct Solution;

impl Solution {
    // O(n) time | O(n) space
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::{
            collections::HashSet, iter::FromIterator,
        };

        let mut set: HashSet<i32> =
            HashSet::from_iter(nums.into_iter());

        let mut max_cnt = 0;

        for n in &set {
            if !set.contains(&(n - 1)) {
                let mut next = n + 1;
                let mut cnt = 1;

                while set.contains(&next) {
                    cnt += 1;
                    next += 1;
                }

                max_cnt = max_cnt.max(cnt);
            }
        }
        max_cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
        // The longest consecutive sequence is [1,
        // 2, 3, 4]
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
        // The longest consecutive sequence is [0,
        // 1, 2, 3, 4, 5, 6, 7, 8]
    }
}
