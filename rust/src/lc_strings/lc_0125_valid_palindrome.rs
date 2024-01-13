pub fn is_palindrome(s: String) -> bool {
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
