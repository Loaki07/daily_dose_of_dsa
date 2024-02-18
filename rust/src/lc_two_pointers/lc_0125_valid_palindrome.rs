pub fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    let len = s.len();

    for i in 0..(len / 2) {
        if s[i] != s[len - i - 1] {
            return false;
        }
    }

    true
}

// O(n) time | O(n) space
pub fn _is_palindrome(s: String) -> bool {
    let forward = s
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<String>();
    let backward =
        forward.chars().rev().collect::<String>();

    forward == backward
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ))
    }

    #[test]
    fn ex2() {
        assert!(!is_palindrome("race a car".to_string()))
    }

    #[test]
    fn ex3() {
        assert!(is_palindrome(" ".to_string()))
    }
}
