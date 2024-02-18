pub fn is_anagram(s: String, t: String) -> bool {
    let mut s1 = s.chars().collect::<Vec<char>>();
    let mut s2 = t.chars().collect::<Vec<char>>();

    s1.sort();
    s2.sort();

    s1 == s2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn ex2() {
        assert!(!is_anagram(
            "rat".to_string(),
            "car".to_string()
        ));
    }
}
