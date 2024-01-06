pub fn is_valid(s: String) -> bool {
    let mut stack = "".to_string();

    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                if let Some(m) = stack.pop() {
                    if m != c {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn test_02() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_03() {
        assert!(!is_valid("(]".to_string()));
    }
}
