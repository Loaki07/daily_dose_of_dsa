pub struct Solution;

impl Solution {
    pub fn exist(
        board: Vec<Vec<char>>,
        word: String,
    ) -> bool {
        let m = board.len();
        let n = board[0].len();
        let mut board = board;

        let word: Vec<char> = word.chars().collect();

        for i in 0..m {
            for j in 0..n {
                if Solution::dfs(&mut board, &word, i, j) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        word: &[char],
        x: usize,
        y: usize,
    ) -> bool {
        if word.len() == 1 {
            return board[x][y] == word[0];
        }

        if board[x][y] != word[0] {
            return false;
        }
        let dirs = [[-1, 0], [0, 1], [1, 0], [0, -1]];
        board[x][y] = '*';

        for dir in dirs {
            let x = x as i32 + dir[0];
            let y = y as i32 + dir[1];

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= board.len() || y >= board[0].len() {
                continue;
            }

            if Solution::dfs(board, &word[1..], x, y) {
                board[x][y] = word[0];
                return true;
            }
        }
        board[x][y] = word[0];
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist_example1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn test_exist_example2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        assert_eq!(Solution::exist(board, word), true);
    }

    #[test]
    fn test_exist_example3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        assert_eq!(Solution::exist(board, word), false);
    }
}
