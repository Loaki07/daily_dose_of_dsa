pub struct Solution;

impl Solution {
    fn backtrack(
        first: usize,
        result: &mut Vec<Vec<i32>>,
        nums: &mut Vec<i32>,
    ) {
        if first == nums.len() {
            result.push(nums.to_owned());
            return;
        }

        for i in first..nums.len() {
            nums.swap(first, i);
            Solution::backtrack(first + 1, result, nums);
            nums.swap(first, i);
        }
    }

    fn dfs(
        nums: &[i32],
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if path.len() == nums.len() {
            result.push(path.clone());
            return;
        }

        for i in 0..nums.len() {
            if !used[i] {
                path.push(nums[i]);
                used[i] = true;

                Solution::dfs(nums, path, used, result);

                path.pop();
                used[i] = false;
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        let mut used = vec![false; nums.len()];

        Solution::dfs(
            &nums,
            &mut path,
            &mut used,
            &mut result,
        );
        result
    }

    pub fn _permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut result, mut nums) = (vec![], nums);
        Self::backtrack(0, &mut result, &mut nums);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_example1() {
        let nums = vec![1, 2, 3];
        let mut results = Solution::permute(nums);
        let mut expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        // Sorting to ensure comparison is
        // order-independent
        results.sort();
        expected.sort();

        assert_eq!(results, expected);
    }

    #[test]
    fn test_permute_example2() {
        let nums = vec![0, 1];
        let mut results = Solution::permute(nums);
        let mut expected = vec![vec![0, 1], vec![1, 0]];

        // Sorting to ensure comparison is
        // order-independent
        results.sort();
        expected.sort();

        assert_eq!(results, expected);
    }

    #[test]
    fn test_permute_example3() {
        let nums = vec![1];
        let results = Solution::permute(nums);
        let expected = vec![vec![1]];

        assert_eq!(results, expected);
    }
}
