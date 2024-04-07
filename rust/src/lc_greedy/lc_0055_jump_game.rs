pub struct Solution;

impl Solution {
    // greedy soln
    // O(n) time | O(1) space
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut goal = nums.len() - 1;

        for i in (0..goal).rev() {
            if i + nums[i] as usize >= goal {
                goal = i;
            }
        }

        goal == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump_example_1() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = Solution::can_jump(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_can_jump_example_2() {
        let nums = vec![3, 2, 1, 0, 4];
        let result = Solution::can_jump(nums);
        assert_eq!(result, false);
    }
}
