pub struct Solution;

impl Solution {
    pub fn dfs(
        candidates: &[i32],
        target: i32,
        result: &mut Vec<Vec<i32>>,
        curr: &mut Vec<i32>,
    ) {
        let sum: i32 = curr.iter().sum();

        if sum == target {
            result.push(curr.to_owned());
            return;
        } else if sum > target {
            return;
        }

        for (i, &c) in candidates.iter().enumerate() {
            curr.push(c);
            Self::dfs(
                &candidates[i..],
                target,
                result,
                curr,
            );
            curr.pop();
        }
    }

    // O(2^t)
    pub fn combination_sum(
        candidates: Vec<i32>,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let (mut result, mut curr) = (vec![], vec![]);

        Self::dfs(
            &candidates,
            target,
            &mut result,
            &mut curr,
        );

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum_example1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let mut results =
            Solution::combination_sum(candidates, target);
        let mut expected = vec![vec![2, 2, 3], vec![7]];

        // Sorting the vectors to compare because order
        // does not matter
        for combination in &mut results {
            combination.sort_unstable();
        }
        results.sort();
        for combination in &mut expected {
            combination.sort_unstable();
        }
        expected.sort();

        assert_eq!(results, expected);
    }

    #[test]
    fn test_combination_sum_example2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let mut results =
            Solution::combination_sum(candidates, target);
        let mut expected = vec![
            vec![2, 2, 2, 2],
            vec![2, 3, 3],
            vec![3, 5],
        ];

        // Sorting the vectors to compare because order
        // does not matter
        for combination in &mut results {
            combination.sort_unstable();
        }
        results.sort();
        for combination in &mut expected {
            combination.sort_unstable();
        }
        expected.sort();

        assert_eq!(results, expected);
    }

    #[test]
    fn test_combination_sum_example3() {
        let candidates = vec![2];
        let target = 1;
        let results =
            Solution::combination_sum(candidates, target);
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(results, expected);
    }
}
