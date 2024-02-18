pub struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let n = mat.len();

        for i in 0..n {
            // Add the primary diagonal element
            sum += mat[i][i];
            // check if its not the same element of the
            // secondary diagonal
            if i != n - 1 - i {
                // Add the secondary diagonal element
                sum += mat[i][n - 1 - i];
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_matrix() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::diagonal_sum(mat), 10);
        // 1+4+2+3 = 10
    }

    #[test]
    fn test_square_matrix_odd() {
        let mat = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(Solution::diagonal_sum(mat), 25);
        // 1+5+9+3+7 = 25
    }

    #[test]
    fn test_square_matrix_even() {
        let mat = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        assert_eq!(Solution::diagonal_sum(mat), 68);
        // 1+6+11+16+4+8+12+13 = 68
    }

    #[test]
    fn test_single_element_matrix() {
        let mat = vec![vec![5]];
        assert_eq!(Solution::diagonal_sum(mat), 5);
        // Only element is 5
    }
}
