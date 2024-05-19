pub struct Solution;

impl Solution {
    // O(n log n) time | O(n) space
    pub fn maximum_value_sum(
        nums: Vec<i32>,
        k: i32,
        edges: Vec<Vec<i32>>,
    ) -> i64 {
        let mut delta: Vec<i32> =
            nums.iter().map(|&n| (n ^ k) - n).collect();

        // sort descending
        delta.sort_unstable_by(|a, b| b.cmp(a));
        let mut res = nums.iter().map(|&n| n as i64).sum();

        for i in (0..nums.len()).step_by(2) {
            if i == nums.len() - 1 {
                break;
            }
            let path_delta = delta[i] + delta[i + 1];

            if path_delta <= 0 {
                break;
            }
            res += path_delta as i64;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 1];
        let k = 3;
        let edges = vec![vec![0, 1], vec![0, 2]];
        let result =
            Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 3];
        let k = 7;
        let edges = vec![vec![0, 1]];
        let result =
            Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![7, 7, 7, 7, 7, 7];
        let k = 3;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
        ];
        let result =
            Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(result, 42);
    }
}
