pub struct Solution;

impl Solution {
    // O(n) time | O(n) space
    pub fn max_subarray_length(
        nums: Vec<i32>,
        k: i32,
    ) -> i32 {
        use std::collections::HashMap;

        let mut res = 0;
        let mut count = HashMap::new();
        let mut l = 0;

        for r in 0..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;

            while *count.get(&nums[r]).unwrap() > k {
                *count.get_mut(&nums[l]).unwrap() -= 1;
                l += 1;
            }

            res = res.max(r as i32 - l as i32 + 1);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray_length_example_1() {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
        let k = 2;
        let expected = 6;
        assert_eq!(
            Solution::max_subarray_length(nums, k),
            expected
        );
    }

    #[test]
    fn test_max_subarray_length_example_2() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2];
        let k = 1;
        let expected = 2;
        assert_eq!(
            Solution::max_subarray_length(nums, k),
            expected
        );
    }

    #[test]
    fn test_max_subarray_length_example_3() {
        let nums = vec![5, 5, 5, 5, 5, 5, 5];
        let k = 4;
        let expected = 4;
        assert_eq!(
            Solution::max_subarray_length(nums, k),
            expected
        );
    }
}
