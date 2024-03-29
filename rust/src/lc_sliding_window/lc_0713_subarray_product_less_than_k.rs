pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(
        nums: Vec<i32>,
        k: i32,
    ) -> i32 {
        if k <= 1 {
            return 0;
        }

        let mut start = 0;
        let mut product = 1;
        let mut length = 0;
        let mut end = 0;

        while end < nums.len() {
            product *= nums[end];

            while product >= k && start <= end {
                product /= nums[start];
                start += 1;
            }

            length += end - start + 1;
            end += 1;
        }

        length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_subarray_product_less_than_k_example_1() {
        let nums = vec![10, 5, 2, 6];
        let k = 100;
        let expected = 8;
        assert_eq!(
            Solution::num_subarray_product_less_than_k(
                nums, k
            ),
            expected
        );
    }

    #[test]
    fn test_num_subarray_product_less_than_k_example_2() {
        let nums = vec![1, 2, 3];
        let k = 0;
        let expected = 0;
        assert_eq!(
            Solution::num_subarray_product_less_than_k(
                nums, k
            ),
            expected
        );
    }
}
