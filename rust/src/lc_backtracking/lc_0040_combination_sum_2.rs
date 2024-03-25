pub struct Solution;

impl Solution {
    fn dfs(
        candidates: &[i32],
        cur: &mut Vec<i32>,
        dp: &mut Vec<Vec<i32>>,
        target: i32,
    ) {
        if target == 0 {
            dp.push(cur.clone());
            return;
        }

        for i in 0..candidates.len() {
            if candidates[i] > target {
                break;
            }

            if i != 0 && candidates[i] == candidates[i - 1]
            {
                continue;
            }

            cur.push(candidates[i]);
            Self::dfs(
                &candidates[i + 1..],
                cur,
                dp,
                target - candidates[i],
            );
            cur.pop();
        }
    }

    pub fn combination_sum2(
        mut candidates: Vec<i32>,
        target: i32,
    ) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        let mut res = vec![];
        Self::dfs(
            &candidates,
            &mut vec![],
            &mut res,
            target,
        );
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum2_example1() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let mut results =
            Solution::combination_sum2(candidates, target);
        let mut expected = vec![
            vec![1, 1, 6],
            vec![1, 2, 5],
            vec![1, 7],
            vec![2, 6],
        ];

        // Sort the vectors to ensure the comparison does
        // not depend on the order
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
    fn test_combination_sum2_example2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let mut results =
            Solution::combination_sum2(candidates, target);
        let mut expected = vec![vec![1, 2, 2], vec![5]];

        // Sort the vectors to ensure the comparison does
        // not depend on the order
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
}
