use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut queue = VecDeque::<usize>::new();
        let mut flip = 0;

        for i in 0..nums.len() {
            if queue.front() == Some(&i) {
                queue.pop_front();
                flip ^= 1;
            }

            if nums[i] ^ flip == 0 {
                if i + k as usize > nums.len() {
                    return -1;
                }
                ans += 1;
                flip ^= 1;
                queue.push_back(i + k as usize);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![0, 1, 0];
        let k = 1;
        let expected_output = 2;
        let actual_output =
            Solution::min_k_bit_flips(nums, k);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 1, 0];
        let k = 2;
        let expected_output = -1;
        let actual_output =
            Solution::min_k_bit_flips(nums, k);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![0, 0, 0, 1, 0, 1, 1, 0];
        let k = 3;
        let expected_output = 3;
        let actual_output =
            Solution::min_k_bit_flips(nums, k);
        assert_eq!(actual_output, expected_output);
    }
}
