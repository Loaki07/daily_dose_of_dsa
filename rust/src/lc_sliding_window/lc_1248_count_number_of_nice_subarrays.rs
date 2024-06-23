pub struct Solution;

impl Solution {
    // 3 pointer sliding window
    // O(n) time | O(1) space
    pub fn number_of_subarrays(
        nums: Vec<i32>,
        k: i32,
    ) -> i32 {
        let (mut res, mut odd) = (0, 0);
        let (mut left, mut mid) = (0, 0);

        for right in 0..nums.len() {
            if nums[right] % 2 != 0 {
                odd += 1;
            }

            while odd > k {
                if nums[left] % 2 != 0 {
                    odd -= 1;
                }
                left += 1;
                mid = left
            }

            if odd == k {
                while nums[mid] % 2 == 0 {
                    mid += 1;
                }
                res += (mid - left) + 1;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![1, 1, 2, 1, 1];
        let k = 3;
        let result = Solution::number_of_subarrays(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example2() {
        let nums = vec![2, 4, 6];
        let k = 1;
        let result = Solution::number_of_subarrays(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example3() {
        let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
        let k = 2;
        let result = Solution::number_of_subarrays(nums, k);
        assert_eq!(result, 16);
    }
}
