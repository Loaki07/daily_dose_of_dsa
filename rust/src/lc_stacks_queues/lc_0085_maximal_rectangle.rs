pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(
        heights: Vec<i32>,
    ) -> i32 {
        let mut ans = 0;
        let mut stack: Vec<usize> = Vec::new();

        for (i, &height) in
            heights.iter().chain([0].iter()).enumerate()
        {
            while let Some(&top) = stack.last() {
                if height < heights[top] {
                    stack.pop();
                    let h = heights[top];
                    let w = if stack.is_empty() {
                        i as i32
                    } else {
                        (i - stack.last().unwrap() - 1)
                            as i32
                    };
                    ans = ans.max(h * w);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        ans
    }

    // O(m*n) time | O(n) space
    pub fn maximal_rectangle(
        matrix: Vec<Vec<char>>,
    ) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let mut ans = 0;
        let mut hist: Vec<i32> = vec![0; matrix[0].len()];

        for row in matrix.iter() {
            for (i, &c) in row.iter().enumerate() {
                hist[i] =
                    if c == '0' { 0 } else { hist[i] + 1 };
            }
            ans =
                ans.max(Solution::largest_rectangle_area(
                    hist.iter().cloned().collect(),
                ));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        let expected = 6;
        assert_eq!(
            Solution::maximal_rectangle(matrix),
            expected
        );
    }

    #[test]
    fn test_case_2() {
        let matrix = vec![vec!['0']];
        let expected = 0;
        assert_eq!(
            Solution::maximal_rectangle(matrix),
            expected
        );
    }

    #[test]
    fn test_case_3() {
        let matrix = vec![vec!['1']];
        let expected = 1;
        assert_eq!(
            Solution::maximal_rectangle(matrix),
            expected
        );
    }

    // Additional test cases for further coverage
    #[test]
    fn test_case_with_multiple_rows_of_zeros() {
        let matrix =
            vec![vec!['0', '0', '0'], vec!['0', '0', '0']];
        let expected = 0;
        assert_eq!(
            Solution::maximal_rectangle(matrix),
            expected
        );
    }

    #[test]
    fn test_case_with_large_rectangle() {
        let matrix = vec![
            vec!['1', '1', '1', '1'],
            vec!['1', '1', '1', '1'],
            vec!['1', '1', '1', '1'],
        ];
        let expected = 12;
        assert_eq!(
            Solution::maximal_rectangle(matrix),
            expected
        );
    }
}
