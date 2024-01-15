use core::num;

pub struct Solution;

impl Solution {
    // O(n) time with O(1) space soln
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];

        for i in 1..nums.len() {
            res[i] = nums[i - 1] * res[i - 1];
        }

        let mut right = 1;

        for (i, n) in res.iter_mut().enumerate().rev() {
            *n = *n * right;
            right *= nums[i];
        }
        res
    }

    // O(n) soln
    pub fn _product_except_self(
        nums: Vec<i32>,
    ) -> Vec<i32> {
        let mut prefix = vec![1; nums.len()];
        let mut suffix = vec![1; nums.len()];

        for i in 1..nums.len() {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }

        let mut i = nums.len() - 1;
        while i > 0 {
            suffix[i - 1] = suffix[i] * nums[i];
            i -= 1;
        }

        prefix
            .iter()
            .zip(suffix.iter())
            .map(|(a, b)| *a * *b)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            Solution::product_except_self(vec![
                -1, 1, 0, -3, 3
            ])
        );
    }
}
