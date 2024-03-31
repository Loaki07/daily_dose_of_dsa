pub struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(
        nums: Vec<i32>,
        k: i32,
    ) -> i32 {
        use std::collections::HashMap;

        let n = nums.len();
        let (
            mut res,
            mut left_near,
            mut left_far,
            mut distinct,
        ) = (0, 0, 0, 0);
        let mut counts = HashMap::new();

        for right in 0..n {
            *counts.entry(nums[right]).or_insert(0) += 1;
            if *counts.get(&nums[right]).unwrap() == 1 {
                distinct += 1;
            }

            while distinct > k {
                *counts
                    .entry(nums[left_near])
                    .or_insert(0) -= 1;
                if *counts.get(&nums[left_near]).unwrap()
                    == 0
                {
                    distinct -= 1;
                }
                left_near += 1;
                left_far = left_near;
            }

            while *counts.get(&nums[left_near]).unwrap() > 1
            {
                *counts
                    .entry(nums[left_near])
                    .or_insert(0) -= 1;
                left_near += 1;
            }

            if distinct == k {
                res += left_near - left_far + 1;
            }
        }

        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarrays_with_k_distinct_example_1() {
        let nums = vec![1, 2, 1, 2, 3];
        let k = 2;
        let expected = 7;
        assert_eq!(
            Solution::subarrays_with_k_distinct(nums, k),
            expected
        );
    }

    #[test]
    fn test_subarrays_with_k_distinct_example_2() {
        let nums = vec![1, 2, 1, 3, 4];
        let k = 3;
        let expected = 3;
        assert_eq!(
            Solution::subarrays_with_k_distinct(nums, k),
            expected
        );
    }
}
