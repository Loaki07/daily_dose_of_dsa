pub struct Solution;

impl Solution {
    // O(n ^ 2) time | O(1) space
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mask = (1 << n) - 1;
        let mut sum = 0;

        while mask != 0 {
            let mut xored = 0;
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    xored ^= nums[i];
                }
            }
            sum += xored;
            mask -= 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![1, 3];
        let result = Solution::subset_xor_sum(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example2() {
        let nums = vec![5, 1, 6];
        let result = Solution::subset_xor_sum(nums);
        assert_eq!(result, 28);
    }
}
