pub struct Solution;

impl Solution {
    // bottom up 2d dp
    // O(m*n)
    pub fn longest_common_subsequence(
        text1: String,
        text2: String,
    ) -> i32 {
        let (text1, text2) =
            (text1.as_bytes(), text2.as_bytes());
        let (l1, l2) = (text1.len(), text2.len());

        let mut matrix = vec![vec![0; l2 + 1]; l1 + 1];

        for i in (0..l1).rev() {
            for j in (0..l2).rev() {
                matrix[i][j] = if text1[i] == text2[j] {
                    // go diagonal if val is equal
                    1 + matrix[i + 1][j + 1]
                } else {
                    // go vertical if not equal
                    matrix[i][j + 1].max(matrix[i + 1][j])
                };
            }
        }

        matrix[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence_example1() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        assert_eq!(
            Solution::longest_common_subsequence(
                text1, text2
            ),
            3
        );
    }

    #[test]
    fn test_longest_common_subsequence_example2() {
        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        assert_eq!(
            Solution::longest_common_subsequence(
                text1, text2
            ),
            3
        );
    }

    #[test]
    fn test_longest_common_subsequence_example3() {
        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        assert_eq!(
            Solution::longest_common_subsequence(
                text1, text2
            ),
            0
        );
    }
}
