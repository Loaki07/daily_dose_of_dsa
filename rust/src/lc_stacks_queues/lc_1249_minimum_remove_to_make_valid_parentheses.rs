pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut open_paren_indexes: Vec<usize> = Vec::new();
        let mut result = String::new();

        for ch in s.chars() {
            match ch {
                '(' => {
                    open_paren_indexes.push(result.len());
                    result.push(ch);
                }
                ')' => {
                    if open_paren_indexes.len() > 0 {
                        result.push(ch);
                        open_paren_indexes.pop();
                    }
                }
                _ => {
                    result.push(ch);
                }
            }
        }

        while let Some(index) = open_paren_indexes.pop() {
            result.remove(index);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "lee(t(c)o)de)".to_string();
        let expected = "lee(t(c)o)de".to_string();
        assert_eq!(
            Solution::min_remove_to_make_valid(input),
            expected
        );
    }

    #[test]
    fn test_case_2() {
        let input = "a)b(c)d".to_string();
        let expected = "ab(c)d".to_string();
        assert_eq!(
            Solution::min_remove_to_make_valid(input),
            expected
        );
    }

    #[test]
    fn test_case_3() {
        let input = "))((".to_string();
        let expected = "".to_string();
        assert_eq!(
            Solution::min_remove_to_make_valid(input),
            expected
        );
    }

    // Additional test cases to ensure more coverage
    #[test]
    fn test_case_with_only_parentheses() {
        let input = "()()".to_string();
        let expected = "()()".to_string();
        assert_eq!(
            Solution::min_remove_to_make_valid(input),
            expected
        );
    }

    #[test]
    fn test_case_with_nested_parentheses() {
        let input = "(a(b(c)d)e)".to_string();
        let expected = "(a(b(c)d)e)".to_string();
        assert_eq!(
            Solution::min_remove_to_make_valid(input),
            expected
        );
    }

    #[test]
    fn test_case_with_unmatched_parentheses() {
        let input = ")(a(b(c)d)e(".to_string();
        let expected = "a(b(c)d)e".to_string();
        assert_eq!(
            Solution::min_remove_to_make_valid(input),
            expected
        );
    }
}
