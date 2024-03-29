pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();

        let mut left = 0;
        let mut count = 0;
        let mut res: i64 = 0;

        for val in nums.iter() {
            if *val == max {
                count += 1;
            }

            while count >= k {
                if nums[left] == max {
                    count -= 1;
                }
                left += 1;
            }

            res += left as i64;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 3, 2, 3, 3];
        let k = 2;
        let expected = 6;
        assert_eq!(
            Solution::count_subarrays(nums, k),
            expected
        );
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 4, 2, 1];
        let k = 3;
        let expected = 0;
        assert_eq!(
            Solution::count_subarrays(nums, k),
            expected
        );
    }
}
