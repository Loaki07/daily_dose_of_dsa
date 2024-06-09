use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // O(n) time | O(k) space
    pub fn subarrays_div_by_k(
        nums: Vec<i32>,
        k: i32,
    ) -> i32 {
        let (mut prefix_sum, mut res, mut prefix_count) = (
            0,
            0,
            HashMap::with_capacity(nums.len() / 2),
        );

        prefix_count.insert(0, 1);
        for num in nums {
            prefix_sum += num;
            let mut remainder = prefix_sum % k;
            // handle negative numbers
            if remainder < 0 {
                remainder += k;
            }
            res += prefix_count
                .get(&remainder)
                .copied()
                .unwrap_or(0);
            prefix_count
                .entry(remainder)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_subarrays_div_by_k_example1() {
        let nums1 = vec![4, 5, 0, -2, -3, 1];
        let k1 = 5;
        assert_eq!(
            Solution::subarrays_div_by_k(nums1, k1),
            7
        );
    }

    #[test]
    fn test_subarrays_div_by_k_example2() {
        let nums2 = vec![5];
        let k2 = 9;
        assert_eq!(
            Solution::subarrays_div_by_k(nums2, k2),
            0
        );
    }

    #[test]
    fn test_subarrays_div_by_k_example3() {
        let nums = vec![-1, 2, 9];
        let k2 = 2;
        assert_eq!(
            Solution::subarrays_div_by_k(nums, k2),
            2
        );
    }
}
