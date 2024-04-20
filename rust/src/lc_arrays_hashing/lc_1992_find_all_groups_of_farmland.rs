pub struct Solution;

impl Solution {
    // O(m*n) time | O(k) space, number of groups of
    // farmland
    pub fn find_farmland(
        land: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut land = land;
        (0..land.len()).fold(vec![], |mut r, i| {
            let mut j = 0;
            while j < land[0].len() {
                if land[i][j] == 1 {
                    let mut x = i;
                    let mut y = j;

                    while y < land[0].len()
                        && land[i][y] != 0
                    {
                        y += 1
                    }
                    while x < land.len() && land[x][j] != 0
                    {
                        land[x][j] =
                            y as i32 - j as i32 + 1;
                        x += 1;
                    }
                    r.push(vec![
                        i as i32,
                        j as i32,
                        x as i32 - 1,
                        y as i32 - 1,
                    ]);
                }
                j += if land[i][j] > 1 {
                    land[i][j]
                } else {
                    1
                } as usize;
            }
            r
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_farmland_case_1() {
        let land = vec![
            vec![1, 0, 0],
            vec![0, 1, 1],
            vec![0, 1, 1],
        ];
        let expected =
            vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]];
        assert_eq!(Solution::find_farmland(land), expected);
    }

    #[test]
    fn test_find_farmland_case_2() {
        let land = vec![vec![1, 1], vec![1, 1]];
        let expected = vec![vec![0, 0, 1, 1]];
        assert_eq!(Solution::find_farmland(land), expected);
    }

    #[test]
    fn test_find_farmland_case_3() {
        let land = vec![vec![0]];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::find_farmland(land), expected);
    }

    // Additional test for a more complex scenario
    #[test]
    fn test_find_farmland_complex_case() {
        let land = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
        ];
        let expected =
            vec![vec![0, 0, 0, 0], vec![2, 2, 3, 3]];
        assert_eq!(Solution::find_farmland(land), expected);
    }
}
