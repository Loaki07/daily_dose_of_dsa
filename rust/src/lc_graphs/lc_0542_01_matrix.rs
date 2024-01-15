pub struct Solution;

impl Solution {
    pub fn update_matrix(
        mat: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let width = mat[0].len(); // n -> coloumns
        let height = mat.len(); // m -> rows
        let mut output = vec![vec![0; width]; height];
        let max = (width + height) as i32;

        let mut hs = HashSet::new();

        for row in 0..height {
            for col in 0..width {
                if mat[row][col] == 0 {
                    output[row][col] = 0;
                } else {
                    // setting the output vec to max
                    // possible value for the moment
                    output[row][col] = max;
                    hs.insert((row, col));
                }
            }
        }

        while !hs.is_empty() {
            let entries =
                hs.drain().collect::<Vec<(usize, usize)>>();

            for (row, col) in entries {
                let up = if row == 0 {
                    None
                } else {
                    Some((row - 1, col))
                };
                let down = if row + 1 == height {
                    None
                } else {
                    Some((row + 1, col))
                };
                let left = if col == 0 {
                    None
                } else {
                    Some((row, col - 1))
                };
                let right = if col + 1 == width {
                    None
                } else {
                    Some((row, col + 1))
                };

                let valid = [up, down, left, right]
                    .iter()
                    .filter_map(|&x| x)
                    .collect::<Vec<(usize, usize)>>();

                let min =
                    1 + valid.iter().fold(max, |a, b| {
                        a.min(output[b.0][b.1])
                    });

                if min < output[row][col] {
                    output[row][col] = min;
                    for dir in &valid {
                        hs.insert(*dir);
                    }
                }
            }
        }

        output
    }

    // has an edge case index out of bounds
    // eg: 5 x 1 matrix
    pub fn _update_matrix(
        mat: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let width = mat[0].len(); // n -> coloumns
        let height = mat.len(); // m -> rows
        let mut output = vec![vec![0; width]; height];
        let max = (width + height) as i32;

        for row in 0..height {
            for col in 0..width {
                if mat[row][col] == 0 {
                    output[row][col] = 0;
                } else {
                    // setting the output vec to max
                    // possible value for the moment
                    output[row][col] = max;
                }
            }
        }

        let mut changed = true;
        while changed {
            changed = false;
            for row in 0..height {
                for col in 0..height {
                    let up = if row == 0 {
                        max
                    } else {
                        output[row - 1][col]
                    };
                    let down = if row + 1 == height {
                        max
                    } else {
                        output[row + 1][col]
                    };
                    let left = if col == 0 {
                        max
                    } else {
                        output[row][col - 1]
                    };
                    let right = if col + 1 == width {
                        max
                    } else {
                        output[row][col + 1]
                    };

                    let min = 1 + up
                        .min(down.min(left.min(right)));
                    if output[row][col] > min {
                        output[row][col] = min;
                        changed = true;
                    }
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![1, 1, 1]
            ]),
            vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![1, 2, 1]
            ]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::update_matrix(vec![
                vec![0],
                vec![0],
                vec![0],
                vec![0],
                vec![0],
            ]),
            vec![
                vec![0],
                vec![0],
                vec![0],
                vec![0],
                vec![0],
            ]
        );
    }
}
