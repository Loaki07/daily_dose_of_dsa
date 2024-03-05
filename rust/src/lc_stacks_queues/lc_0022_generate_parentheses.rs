pub struct Solution;

impl Solution {
    // only add open parenthesis if open < n
    // only add a closing parenthesis if closed < open
    // valid if open == closed == n
    // solution requires a stack and backtracking
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];

        fn backtrack(
            res: &mut Vec<String>,
            s: String,
            open: i32,
            close: i32,
        ) {
            if open == 0 && close == 0 {
                res.push(s);
                return;
            }

            if open == close {
                backtrack(
                    res,
                    s.clone() + "(",
                    open - 1,
                    close,
                );
            } else {
                if open > 0 {
                    backtrack(
                        res,
                        s.clone() + "(",
                        open - 1,
                        close,
                    )
                }

                if close > 0 {
                    backtrack(
                        res,
                        s.clone() + ")",
                        open,
                        close - 1,
                    );
                }
            }
        }

        backtrack(&mut res, String::from(""), n, n);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 3;
        let mut result = Solution::generate_parenthesis(n);
        let mut expected = vec![
            "((()))", "(()())", "(())()", "()(())",
            "()()()",
        ];
        result.sort(); // Sorting to ensure the order matches for
                       // comparison
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let n = 1;
        let mut result = Solution::generate_parenthesis(n);
        let mut expected = vec!["()"];
        result.sort(); // Sorting is optional here since there's only one
                       // element, but kept for consistency
        expected.sort();
        assert_eq!(result, expected);
    }
}
