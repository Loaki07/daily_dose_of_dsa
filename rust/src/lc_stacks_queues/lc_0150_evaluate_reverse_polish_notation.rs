pub struct Solution;

impl Solution {
    // reverse polish notation
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            // Why use &token[..] instead of
            // directly matching token?

            // Rust's match statement can directly work with
            // &str patterns but not with String objects
            // without an explicit conversion. Using
            // &token[..] converts the String into a str
            // slice, which can then be matched against
            // string literals in the match arms.
            // This conversion is a common pattern when you
            // need to perform pattern matching on the
            // contents of a String object without taking
            // ownership of it, allowing for efficient
            // comparisons
            match &token[..] {
                "+" => {
                    let second_operand =
                        stack.pop().unwrap();
                    let first_operand =
                        stack.pop().unwrap();
                    stack.push(
                        first_operand + second_operand,
                    )
                }
                "-" => {
                    let second_operand =
                        stack.pop().unwrap();
                    let first_operand =
                        stack.pop().unwrap();
                    stack.push(
                        first_operand - second_operand,
                    )
                }
                "*" => {
                    let second_operand =
                        stack.pop().unwrap();
                    let first_operand =
                        stack.pop().unwrap();
                    stack.push(
                        first_operand * second_operand,
                    )
                }
                "/" => {
                    let second_operand =
                        stack.pop().unwrap();
                    let first_operand =
                        stack.pop().unwrap();
                    stack.push(
                        first_operand / second_operand,
                    )
                }
                value => stack
                    .push(value.parse::<i32>().unwrap()),
            }
        }

        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tokens = vec!["2", "1", "+", "3", "*"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let expected = 9;
        assert_eq!(Solution::eval_rpn(tokens), expected);
    }

    #[test]
    fn test_example_2() {
        let tokens = vec!["4", "13", "5", "/", "+"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let expected = 6;
        assert_eq!(Solution::eval_rpn(tokens), expected);
    }

    #[test]
    fn test_example_3() {
        let tokens = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*",
            "17", "+", "5", "+",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let expected = 22;
        assert_eq!(Solution::eval_rpn(tokens), expected);
    }
}
