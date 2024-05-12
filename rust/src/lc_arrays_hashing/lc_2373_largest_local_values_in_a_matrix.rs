pub struct Solution;

impl Solution {
    pub fn largest_local(
        grid: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut res: Vec<Vec<i32>> =
            vec![vec![0; n - 2]; n - 2];

        for i in 0..n - 2 {
            for j in 0..n - 2 {
                // loops for r and c are looking
                // at the 3 x 3 matrix
                for r in i..i + 3 {
                    for c in j..j + 3 {
                        res[i][j] =
                            res[i][j].max(grid[r][c])
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec![
            vec![9, 9, 8, 1],
            vec![5, 6, 2, 6],
            vec![8, 2, 6, 4],
            vec![6, 2, 2, 2],
        ];
        let expected_output = vec![vec![9, 9], vec![8, 6]];
        assert_eq!(
            Solution::largest_local(input),
            expected_output
        );
    }

    #[test]
    fn test_example_2() {
        let input = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 2, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let expected_output = vec![
            vec![2, 2, 2],
            vec![2, 2, 2],
            vec![2, 2, 2],
        ];
        assert_eq!(
            Solution::largest_local(input),
            expected_output
        );
    }
}
