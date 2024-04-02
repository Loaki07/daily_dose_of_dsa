pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::{HashMap, HashSet};

        let (s, t) = (s.into_bytes(), t.into_bytes());

        let mut map: HashMap<u8, u8> =
            HashMap::with_capacity(s.len());

        for index in 0..s.len() {
            let val =
                map.entry(s[index]).or_insert(t[index]);

            if *val != t[index] {
                return false;
            }
        }

        map.values().collect::<HashSet<_>>().len()
            == map.keys().collect::<HashSet<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic_example_1() {
        let s = "egg".to_string();
        let t = "add".to_string();
        assert!(Solution::is_isomorphic(s, t));
    }

    #[test]
    fn test_is_isomorphic_example_2() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        assert!(!Solution::is_isomorphic(s, t));
    }

    #[test]
    fn test_is_isomorphic_example_3() {
        let s = "paper".to_string();
        let t = "title".to_string();
        assert!(Solution::is_isomorphic(s, t));
    }
}
