use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTIONS: [(isize, isize); 4] =
            [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        let mut minutes = 0;

        // fill the queue with all rotten oranges
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    queue.push_back((
                        i as isize, j as isize, 0,
                    ));
                    seen.insert((i, j));
                }
            }
        }

        // perform bfs and spread rotting orange
        while let Some((i, j, time)) = queue.pop_front() {
            for &(di, dj) in &DIRECTIONS {
                let next_i = i + di;
                let next_j = j + dj;

                if next_i >= 0
                    && next_j >= 0
                    && next_i < grid.len() as isize
                    && next_j < grid[0].len() as isize
                    && !seen.contains(&(
                        next_i as usize,
                        next_j as usize,
                    ))
                    && grid[next_i as usize]
                        [next_j as usize]
                        == 1
                {
                    queue.push_back((
                        next_i,
                        next_j,
                        time + 1,
                    ));
                    seen.insert((
                        next_i as usize,
                        next_j as usize,
                    ));
                    minutes = time + 1;
                }
            }
        }

        // make sure all oranges are rotted; in any aren't
        // then it's impossible to rot all oranges
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1
                    && !seen.contains(&(i, j))
                {
                    return -1;
                }
            }
        }

        minutes
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        let grid = vec![
            vec![2, 1, 1],
            vec![1, 1, 0],
            vec![0, 1, 1],
        ];
        let expected = 4;
        assert_eq!(
            Solution::oranges_rotting(grid),
            expected
        );
    }

    #[test]
    fn test_example2() {
        let grid = vec![
            vec![2, 1, 1],
            vec![0, 1, 1],
            vec![1, 0, 1],
        ];
        let expected = -1;
        assert_eq!(
            Solution::oranges_rotting(grid),
            expected
        );
    }

    #[test]
    fn test_example3() {
        let grid = vec![vec![0, 2]];
        let expected = 0;
        assert_eq!(
            Solution::oranges_rotting(grid),
            expected
        );
    }
}
