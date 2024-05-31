pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor_all = 0;
        for &num in &nums {
            xor_all ^= num;
        }

        let set_bit = xor_all & !xor_all + 1;
        let (mut a, mut b) = (0, 0);

        for &num in &nums {
            if num & set_bit != 0 {
                a ^= num;
            } else {
                b ^= num;
            }
        }
        vec![a, b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let result = Solution::single_number(nums);
        assert!(
            result == vec![3, 5] || result == vec![5, 3]
        );
    }

    #[test]
    fn test_example2() {
        let nums = vec![-1, 0];
        let result = Solution::single_number(nums);
        assert!(
            result == vec![-1, 0] || result == vec![0, -1]
        );
    }

    #[test]
    fn test_example3() {
        let nums = vec![0, 1];
        let result = Solution::single_number(nums);
        assert!(
            result == vec![0, 1] || result == vec![1, 0]
        );
    }
}
