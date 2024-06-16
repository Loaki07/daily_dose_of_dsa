pub struct Solution;

impl Solution {
    // O(m + log n) time || O(1) space
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut i = 0;
        let mut patches = 0;
        let mut miss: i64 = 1;

        while miss <= n as i64 {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                patches += 1;
            }
        }
        patches
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_patches_example1() {
        let nums = vec![1, 3];
        let n = 6;
        let result = Solution::min_patches(nums, n);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_min_patches_example2() {
        let nums = vec![1, 5, 10];
        let n = 20;
        let result = Solution::min_patches(nums, n);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_min_patches_example3() {
        let nums = vec![1, 2, 2];
        let n = 5;
        let result = Solution::min_patches(nums, n);
        assert_eq!(result, 0);
    }
}
