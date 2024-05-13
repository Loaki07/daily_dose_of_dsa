pub struct Solution;

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        for v in grid.iter_mut() {
            if v[0] == 0 {
                for j in 0..m {
                    v[j] ^= 1;
                }
            }
        }

        for j in 0..m {
            for i in 1..n {
                grid[0][j] += grid[i][j];
            }
            grid[0][j] =
                grid[0][j].max(n as i32 - grid[0][j]);
        }

        grid[0]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| x * (1 << i))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let grid = vec![
            vec![0, 0, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(Solution::matrix_score(grid), 39);
    }

    #[test]
    fn test_example2() {
        let grid = vec![vec![0]];
        assert_eq!(Solution::matrix_score(grid), 1);
    }
}
