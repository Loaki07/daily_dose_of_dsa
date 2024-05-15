use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    // O(n ^ 2) time & space
    pub fn maximum_safeness_factor(
        mut grid: Vec<Vec<i32>>,
    ) -> i32 {
        let n = grid.len();
        let in_bounds = |r: usize, c: usize| r < n && c < n;
        let mut queue = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i, j, 1));
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = -1;
                }
            }
        }
        while let Some((i, j, safety)) = queue.pop_front() {
            let expand = [
                (i + 1, j),
                (i.wrapping_sub(1), j),
                (i, j + 1),
                (i, j.wrapping_sub(1)),
            ];
            for (r, c) in expand {
                if in_bounds(r, c) && grid[r][c] == -1 {
                    grid[r][c] = safety;
                    queue.push_back((r, c, safety + 1));
                }
            }
        }
        let mut min_safety = grid[0][0];
        queue.push_back((0, 0, grid[0][0]));
        while let Some((i, j, safety)) = queue.pop_front() {
            min_safety = i32::min(min_safety, safety);
            if (i, j) == (n - 1, n - 1) {
                break;
            };
            let expand = [
                (i + 1, j),
                (i.wrapping_sub(1), j),
                (i, j + 1),
                (i, j.wrapping_sub(1)),
            ];
            for (r, c) in expand {
                if in_bounds(r, c) && grid[r][c] != -1 {
                    let safety = grid[r][c];
                    grid[r][c] = -1;
                    if safety < min_safety {
                        queue.push_back((r, c, safety));
                    } else {
                        queue.push_front((r, c, safety));
                    }
                }
            }
        }
        min_safety
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec![1, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 1],
        ];
        let result =
            Solution::maximum_safeness_factor(grid);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![0, 0, 1],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        let result =
            Solution::maximum_safeness_factor(grid);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_3() {
        let grid = vec![
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let result =
            Solution::maximum_safeness_factor(grid);
        assert_eq!(result, 2);
    }
}
