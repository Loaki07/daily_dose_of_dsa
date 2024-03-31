pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn count_subarrays(
        nums: Vec<i32>,
        min_k: i32,
        max_k: i32,
    ) -> i64 {
        let (mut res, mut a, mut b, mut j) =
            (0, -1, -1, -1);

        for (i, &n) in nums.iter().enumerate() {
            if n == min_k {
                a = i as i64
            }
            if n == max_k {
                b = i as i64
            }
            if n < min_k || n > max_k {
                j = i as i64
            }

            res += (a.min(b) - j).max(0)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays_example_1() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;
        let expected = 2;
        assert_eq!(
            Solution::count_subarrays(nums, min_k, max_k),
            expected
        );
    }

    #[test]
    fn test_count_subarrays_example_2() {
        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;
        let expected = 10;
        assert_eq!(
            Solution::count_subarrays(nums, min_k, max_k),
            expected
        );
    }
}
