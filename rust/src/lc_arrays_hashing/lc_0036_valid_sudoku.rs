pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        let mut row: HashSet<char> = HashSet::new();
        let mut col: HashSet<char> = HashSet::new();
        let mut bx: HashSet<char> = HashSet::new();

        for i in 0..9 {
            for j in 0..9 {
                let r = board[i][j];
                let c = board[j][i];
                let b = board[i / 3 * 3 + j / 3]
                    [i % 3 * 3 + j % 3];

                if r != '.' {
                    if !row.insert(r) {
                        return false;
                    }
                }

                if c != '.' {
                    if !col.insert(c) {
                        return false;
                    }
                }

                if b != '.' {
                    if !bx.insert(b) {
                        return false;
                    }
                }
            }

            row.clear();
            col.clear();
            bx.clear();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sudoku() {
        let board = vec![
            vec![
                '5', '3', '.', '.', '7', '.', '.', '.', '.',
            ],
            vec![
                '6', '.', '.', '1', '9', '5', '.', '.', '.',
            ],
            vec![
                '.', '9', '8', '.', '.', '.', '.', '6', '.',
            ],
            vec![
                '8', '.', '.', '.', '6', '.', '.', '.', '3',
            ],
            vec![
                '4', '.', '.', '8', '.', '3', '.', '.', '1',
            ],
            vec![
                '7', '.', '.', '.', '2', '.', '.', '.', '6',
            ],
            vec![
                '.', '6', '.', '.', '.', '.', '2', '8', '.',
            ],
            vec![
                '.', '.', '.', '4', '1', '9', '.', '.', '5',
            ],
            vec![
                '.', '.', '.', '.', '8', '.', '.', '7', '9',
            ],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }

    #[test]
    fn test_row_conflict() {
        let board = vec![
            vec![
                '5', '3', '3', '.', '7', '.', '.', '.', '.',
            ], // '3' is duplicated in the first row
            vec![
                '6', '.', '.', '1', '9', '5', '.', '.', '.',
            ],
            vec![
                '.', '9', '8', '.', '.', '.', '.', '6', '.',
            ],
            vec![
                '8', '.', '.', '.', '6', '.', '.', '.', '3',
            ],
            vec![
                '4', '.', '.', '8', '.', '3', '.', '.', '1',
            ],
            vec![
                '7', '.', '.', '.', '2', '.', '.', '.', '6',
            ],
            vec![
                '.', '6', '.', '.', '.', '.', '2', '8', '.',
            ],
            vec![
                '.', '.', '.', '4', '1', '9', '.', '.', '5',
            ],
            vec![
                '.', '.', '.', '.', '8', '.', '.', '7', '9',
            ],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn test_box_conflict() {
        // Adjust the board to simulate a box conflict
        let board = vec![
            vec![
                '5', '3', '.', '.', '7', '.', '.', '.', '.',
            ],
            vec![
                '6', '.', '5', '1', '9', '5', '.', '.', '.',
            ], /* '5' is duplicated within the top-left
                * 3x3 box */
            vec![
                '.', '9', '8', '.', '.', '.', '.', '6', '.',
            ],
            vec![
                '8', '.', '.', '.', '6', '.', '.', '.', '3',
            ],
            vec![
                '4', '.', '.', '8', '.', '3', '.', '.', '1',
            ],
            vec![
                '7', '.', '.', '.', '2', '.', '.', '.', '6',
            ],
            vec![
                '.', '6', '.', '.', '.', '.', '2', '8', '.',
            ],
            vec![
                '.', '.', '.', '4', '1', '9', '.', '.', '5',
            ],
            vec![
                '.', '.', '.', '.', '8', '.', '.', '7', '9',
            ],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn test_valid_empty() {
        let board = vec![
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }
}
