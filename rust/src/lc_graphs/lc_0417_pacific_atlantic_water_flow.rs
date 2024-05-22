use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // O(n * m) time & space
    pub fn pacific_atlantic(
        heights: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        fn dfs(
            heights: &Vec<Vec<i32>>,
            x: i32,
            y: i32,
            visit: &mut HashSet<(i32, i32)>,
            prev: i32,
        ) {
            if x < 0
                || y < 0
                || x == heights.len() as i32
                || y == heights[0].len() as i32
                || heights[x as usize][y as usize] < prev
                || visit.contains(&(x, y))
            {
                return;
            }
            visit.insert((x, y));
            for (add_x, add_y) in
                [(0, 1), (1, 0), (0, -1), (-1, 0)]
            {
                dfs(
                    heights,
                    x + add_x,
                    y + add_y,
                    visit,
                    heights[x as usize][y as usize],
                );
            }
        }

        let (mut pac, mut atl) =
            (HashSet::new(), HashSet::new());
        let (n_rows, n_cols) =
            (heights.len(), heights[0].len());

        for x in 0..n_rows {
            dfs(
                &heights,
                x as i32,
                0,
                &mut pac,
                heights[x][0],
            );
            dfs(
                &heights,
                x as i32,
                n_cols as i32 - 1,
                &mut atl,
                heights[x][n_cols - 1],
            );
        }

        for y in 0..n_cols {
            dfs(
                &heights,
                0,
                y as i32,
                &mut pac,
                heights[0][y],
            );
            dfs(
                &heights,
                n_rows as i32 - 1,
                y as i32,
                &mut atl,
                heights[n_rows - 1][y],
            );
        }

        (0..n_rows)
            .flat_map(|x| (0..n_cols).map(move |y| (x, y)))
            .filter(|(x, y)| {
                pac.contains(&(*x as i32, *y as i32))
                    && atl.contains(&(*x as i32, *y as i32))
            })
            .map(|(x, y)| vec![x as i32, y as i32])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacific_atlantic_example_1() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let expected = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        assert_eq!(
            Solution::pacific_atlantic(heights),
            expected
        );
    }

    #[test]
    fn test_pacific_atlantic_example_2() {
        let heights = vec![vec![1]];
        let expected = vec![vec![0, 0]];
        assert_eq!(
            Solution::pacific_atlantic(heights),
            expected
        );
    }
}
