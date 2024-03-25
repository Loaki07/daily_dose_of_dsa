pub struct Solution;

impl Solution {
    // backtracking
    // O(n*2^n)
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut temp = Vec::new();

        Self::backtrack(0, &nums, &mut temp, &mut res);
        // Adding the missing []
        res.push(Vec::new());
        res
    }

    fn backtrack(
        i: usize,
        nums: &Vec<i32>,
        temp: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if i >= nums.len() {
            return;
        }

        // decision to include nums[i]
        // left branch of the decision tree
        temp.push(nums[i]);
        res.push(temp.to_vec());
        Self::backtrack(i + 1, nums, temp, res);

        // decision NOT to include nums[i]
        // right branch of the decision tree
        temp.pop();
        Self::backtrack(i + 1, nums, temp, res);
    }

    pub fn _subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();

        Vec::from_iter((0..1 << n).map(|bitmask| {
            Vec::from_iter((0..n).filter_map(|i| {
                match (bitmask >> i) & 1 != 0 {
                    true => Some(nums[i]),
                    false => None,
                }
            }))
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_example1() {
        let nums = vec![1, 2, 3];
        let mut result = Solution::subsets(nums);
        let mut expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];

        // Sort the inner vectors and the outer vector to
        // compare, because the order does not matter
        for subset in &mut result {
            subset.sort();
        }
        result.sort();

        for subset in &mut expected {
            subset.sort();
        }
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subsets_example2() {
        let nums = vec![0];
        let mut result = Solution::subsets(nums);
        let mut expected = vec![vec![], vec![0]];

        // Sort the inner vectors and the outer vector to
        // compare, because the order does not matter
        for subset in &mut result {
            subset.sort();
        }
        result.sort();

        for subset in &mut expected {
            subset.sort();
        }
        expected.sort();

        assert_eq!(result, expected);
    }
}
