pub struct Solution;

impl Solution {
    fn backtrack(
        mut i: usize,
        result: &mut Vec<Vec<i32>>,
        nums: &[i32],
        subset: &mut Vec<i32>,
    ) {
        if i == nums.len() {
            result.push(subset.to_owned());
            return;
        }

        subset.push(nums[i]);
        Solution::backtrack(i + 1, result, nums, subset);
        subset.pop();

        while i + 1 < nums.len() && nums[i] == nums[i + 1] {
            i += 1;
        }

        Solution::backtrack(i + 1, result, nums, subset);
    }

    // O(2^n)
    pub fn subsets_with_dup(
        nums: Vec<i32>,
    ) -> Vec<Vec<i32>> {
        let (mut nums, mut result) = (nums, vec![]);
        nums.sort();

        Solution::backtrack(
            0_usize,
            &mut result,
            &mut nums,
            &mut vec![],
        );

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_with_dup_example1() {
        let nums = vec![1, 2, 2];
        let mut results = Solution::subsets_with_dup(nums);
        let mut expected = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];

        // Sort the results to ensure the order does not
        // affect the comparison
        for subset in &mut results {
            subset.sort();
        }
        results.sort();

        for subset in &mut expected {
            subset.sort();
        }
        expected.sort();

        assert_eq!(results, expected);
    }

    #[test]
    fn test_subsets_with_dup_example2() {
        let nums = vec![0];
        let mut results = Solution::subsets_with_dup(nums);
        let mut expected = vec![vec![], vec![0]];

        // Direct comparison as sorting is not required
        // for single element vectors
        results.sort();
        expected.sort();
        assert_eq!(results, expected);
    }
}
