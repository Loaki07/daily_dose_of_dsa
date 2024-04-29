pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let get_bits = |n: i32| {
            // The reason we use `n * 2` as the init
            // argument is to make the bodt of
            // the closure a bit cleaner
            // by allowing us to divide n before returning.
            // n % 2.
            //
            // Otherwise we'd have something like:
            // let res = Some(*n % 2);
            // *n /= 2;
            // res
            (0..32).scan(n * 2, |n, _| {
                *n /= 2;
                Some(*n % 2)
            })
        };

        let xor = nums.into_iter().fold(0, |x, n| x ^ n);
        get_bits(xor)
            .zip(get_bits(k))
            .map(|(a, b)| a ^ b)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations_case_1() {
        let nums = vec![2, 1, 3, 4];
        let k = 1;
        assert_eq!(Solution::min_operations(nums, k), 2);
    }

    #[test]
    fn test_min_operations_case_2() {
        let nums = vec![2, 0, 2, 0];
        let k = 0;
        assert_eq!(Solution::min_operations(nums, k), 0);
    }

    // Additional tests for different scenarios
    #[test]
    fn test_min_operations_all_zeros() {
        let nums = vec![0, 0, 0, 0];
        let k = 0;
        assert_eq!(Solution::min_operations(nums, k), 0);
    }

    #[test]
    fn test_min_operations_need_one_operation() {
        let nums = vec![1, 1, 1];
        let k = 0;
        assert_eq!(Solution::min_operations(nums, k), 1);
    }
}
