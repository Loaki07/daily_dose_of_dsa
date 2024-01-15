pub struct Solution;

impl Solution {
    pub fn update_matrix(
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
}
