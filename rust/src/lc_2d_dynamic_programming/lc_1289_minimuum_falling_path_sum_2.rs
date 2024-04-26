pub struct Solution;

impl Solution {
    // O(n^2) time | O(n) space
    pub fn min_falling_path_sum(
        grid: Vec<Vec<i32>>,
    ) -> i32 {
        if grid.len() == 1 {
            return grid[0][0];
        }

        let rows = grid.len();
        let cols = grid[0].len();

        fn two_smallest(
            row: &Vec<i32>,
        ) -> [(i32, usize); 2] {
            let mut two_smallest =
                [(row[0], 0_usize), (row[1], 1_usize)];
            two_smallest.sort_unstable();

            for (i, val) in row.iter().enumerate().skip(2) {
                if *val < two_smallest[1].0 {
                    two_smallest[1] = (*val, i);
                    two_smallest.sort_unstable();
                }
            }

            two_smallest
        }

        let mut prev = two_smallest(&grid[rows - 1]);
        let mut cur = vec![0; cols];

        for r in (0..rows - 1).rev() {
            for c in 0..cols {
                let min_val = if prev[0].1 != c {
                    prev[0].0
                } else {
                    prev[1].0
                };
                cur[c] = min_val + grid[r][c];
            }
            prev = two_smallest(&cur);
        }

        i32::min(prev[0].0, prev[1].0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_falling_path_sum_case_1() {
        let grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(
            Solution::min_falling_path_sum(grid),
            13
        );
    }

    #[test]
    fn test_min_falling_path_sum_case_2() {
        let grid = vec![vec![7]];
        assert_eq!(Solution::min_falling_path_sum(grid), 7);
    }

    // Additional tests to ensure the function handles
    // various scenarios
    #[test]
    fn test_min_falling_path_sum_large_values() {
        let grid = vec![
            vec![99, 100, 101],
            vec![98, 1, 100],
            vec![1, 97, 99],
        ];
        assert_eq!(
            Solution::min_falling_path_sum(grid),
            101
        ); // 99 + 1 + 1
    }
}
