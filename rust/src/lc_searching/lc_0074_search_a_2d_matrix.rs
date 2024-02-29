pub struct Solution;

impl Solution {
    pub fn search_matrix(
        matrix: Vec<Vec<i32>>,
        target: i32,
    ) -> bool {
        let (mut top, mut bot) = (0, matrix.len());
        let mut row = 0;
        while top < bot {
            row = top + (bot - top) / 2;
            let first = matrix[row][0];
            let last = *matrix[row].last().unwrap();
            if target == first || target == last {
                return true;
            } else if target < first {
                bot = row;
            } else if target > last {
                top = row + 1;
            } else {
                break;
            }
        }

        if top > bot {
            return false;
        }

        let (mut left, mut right) = (0, matrix[0].len());
        while left < right {
            let col = left + (right - left) / 2;
            match target.cmp(&matrix[row][col]) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => right = col,
                std::cmp::Ordering::Greater => {
                    left = col + 1
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix_case1() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
        ];
        let target = 3;
        assert_eq!(
            Solution::search_matrix(matrix, target),
            true
        );
    }

    #[test]
    fn test_search_matrix_case2() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
        ];
        let target = 13;
        assert_eq!(
            Solution::search_matrix(matrix, target),
            false
        );
    }
}
