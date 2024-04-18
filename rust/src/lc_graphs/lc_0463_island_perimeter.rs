use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // O(n*m) time | O(n*m) space
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut visit: HashSet<(i32, i32)> = HashSet::new();

        let mut perimeter: i32 = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    Solution::dfs(
                        i as i32,
                        j as i32,
                        &mut visit,
                        &grid,
                        &mut perimeter,
                    );
                    break;
                }
            }
        }

        perimeter
    }

    fn dfs(
        i: i32,
        j: i32,
        visit: &mut HashSet<(i32, i32)>,
        grid: &Vec<Vec<i32>>,
        perimeter: &mut i32,
    ) {
        if i >= grid.len() as i32
            || j >= grid[0].len() as i32
            || i < 0
            || j < 0
            || grid[i as usize][j as usize] == 0
        {
            *perimeter += 1;
            return;
        }

        if visit.get(&(i, j)).is_some() {
            *perimeter += 0;
            return;
        }

        visit.insert((i, j));

        Solution::dfs(i, j + 1, visit, &grid, perimeter);
        Solution::dfs(i + 1, j, visit, &grid, perimeter);
        Solution::dfs(i, j - 1, visit, &grid, perimeter);
        Solution::dfs(i - 1, j, visit, &grid, perimeter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_island_perimeter_case_1() {
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(Solution::island_perimeter(grid), 16);
    }

    #[test]
    fn test_island_perimeter_case_2() {
        let grid = vec![vec![1]];
        assert_eq!(Solution::island_perimeter(grid), 4);
    }

    #[test]
    fn test_island_perimeter_case_3() {
        let grid = vec![vec![1, 0]];
        assert_eq!(Solution::island_perimeter(grid), 4);
    }
}
