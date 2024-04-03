pub struct Solution;

impl Solution {
    // O(n) time | O(n) space
    pub fn _merge_3(
        intervals: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;

        intervals
            .into_iter()
            .collect::<BinaryHeap<Vec<i32>>>()
            .into_sorted_vec()
            .into_iter()
            .fold(vec![], |mut ans, x| {
                let n = ans.len() - 1;
                if ans.is_empty() || ans[n][1] < x[0] {
                    ans.push(x);
                    ans
                } else {
                    ans[n][1] = ans[n][1].max(x[1]);
                    ans
                }
            })
    }

    // O(n log n) time | O(1) space
    pub fn merge(
        mut intervals: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let acc = vec![intervals.first().unwrap().clone()];

        intervals.into_iter().skip(1).fold(
            acc,
            |mut acc, e| {
                let last = acc.last().unwrap();
                if e[0] <= last[1] {
                    acc.last_mut().unwrap()[1] =
                        last[1].max(e[1]);
                } else {
                    acc.push(e);
                }
                acc
            },
        )
    }

    // O(n log + n) time | O(1) space
    pub fn _merge_2(
        mut intervals: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut res = Vec::new();
        let mut idx = 0;

        for interval in intervals {
            if res.is_empty() {
                res.push(interval);
                continue;
            }

            if res[idx][1] >= interval[0] {
                if res[idx][1] < interval[1] {
                    res[idx][1] = interval[1];
                }
            } else {
                res.push(interval);
                idx += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope.

    #[test]
    fn test_merge_intervals_example1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18],
        ];
        let expected =
            vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(Solution::merge(intervals), expected);
    }

    #[test]
    fn test_merge_intervals_example2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        assert_eq!(Solution::merge(intervals), expected);
    }

    // Additional test case: No overlapping intervals.
    #[test]
    fn test_merge_intervals_no_overlap() {
        let intervals =
            vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let expected =
            vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(Solution::merge(intervals), expected);
    }

    // Additional test case: All intervals overlap
    // into one.
    #[test]
    fn test_merge_intervals_all_overlap() {
        let intervals =
            vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let expected = vec![vec![1, 6]];
        assert_eq!(Solution::merge(intervals), expected);
    }
}
