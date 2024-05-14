use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());

        let mut max_gold = 0;
        let mut visit: HashSet<(i32, i32)> = HashSet::new();
        for r in 0..rows {
            for c in 0..cols {
                max_gold = max_gold.max(Self::dfs(
                    r as i32, c as i32, &mut visit, &grid,
                ))
            }
        }

        max_gold
    }

    pub fn dfs(
        row: i32,
        col: i32,
        visit: &mut HashSet<(i32, i32)>,
        grid: &Vec<Vec<i32>>,
    ) -> i32 {
        if row.min(col) < 0
            || row == grid.len() as i32
            || col == grid[0].len() as i32
            || grid[row as usize][col as usize] == 0
            || visit.get(&(row, col)).is_some()
        {
            return 0;
        }

        visit.insert((row, col));
        let mut gold = grid[row as usize][col as usize];

        let neighbours = vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ];

        for (row2, col2) in neighbours {
            gold = gold.max(
                grid[row as usize][col as usize]
                    + Self::dfs(row2, col2, visit, grid),
            );
        }

        visit.remove(&(row, col));
        gold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_maximum_gold_example1() {
        let grid3 = vec![
            vec![0, 6, 0],
            vec![5, 8, 7],
            vec![0, 9, 0],
        ];
        assert_eq!(Solution::get_maximum_gold(grid3), 24);
    }

    #[test]
    fn test_get_maximum_gold_example2() {
        let grid4 = vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20],
        ];
        assert_eq!(Solution::get_maximum_gold(grid4), 28);
    }
}
