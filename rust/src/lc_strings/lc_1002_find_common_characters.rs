use std::cmp::min;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn common_chars(
        mut words: Vec<String>,
    ) -> Vec<String> {
        let mut result = Vec::new();
        let mut keys = Vec::new();
        let last = words.pop().unwrap();
        let mut last_map = Solution::mapping(&last);

        for &key in last_map.keys() {
            keys.push(key);
        }

        for word in words {
            let map = Solution::mapping(&word);
            for &k in &keys {
                let y = map.get(&k).unwrap_or(&0);
                let x =
                    last_map.get(&k).unwrap_or(&i8::MAX);
                last_map.insert(k, min(*x, *y));
            }
        }

        for (k, v) in last_map {
            for _ in 0..v {
                result.push(k.to_string());
            }
        }
        result
    }

    fn mapping(word: &str) -> HashMap<char, i8> {
        let mut map: HashMap<char, i8> = HashMap::new();
        for c in word.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let words = vec![
            String::from("bella"),
            String::from("label"),
            String::from("roller"),
        ];
        let mut expected = vec![
            String::from("e"),
            String::from("l"),
            String::from("l"),
        ];
        let mut result = Solution::common_chars(words);
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        let words = vec![
            String::from("cool"),
            String::from("lock"),
            String::from("cook"),
        ];
        let mut expected =
            vec![String::from("c"), String::from("o")];
        let mut result = Solution::common_chars(words);
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}
