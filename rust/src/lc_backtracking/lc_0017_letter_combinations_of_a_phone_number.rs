use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn backtrack(
        i: usize,
        curr_str: String,
        result: &mut Vec<String>,
        digits: &String,
        d_c_map: &HashMap<char, &str>,
    ) {
        if curr_str.len() == digits.len() {
            result.push(curr_str);
            return;
        }

        let letters = d_c_map
            .get(&digits.chars().nth(i).unwrap())
            .unwrap()
            .to_string();

        for ch in letters.chars() {
            let mut append_str = curr_str.clone();
            append_str.push(ch);
            Self::backtrack(
                i + 1,
                append_str,
                result,
                digits,
                d_c_map,
            );
        }
    }

    pub fn letter_combinations(
        digits: String,
    ) -> Vec<String> {
        let mut result = vec![];
        let d_c_map = HashMap::from([
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "qprs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]);

        if !digits.is_empty() {
            Solution::backtrack(
                0,
                "".to_string(),
                &mut result,
                &digits,
                &d_c_map,
            );
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations_example1() {
        let digits = "23".to_string();
        let mut result =
            Solution::letter_combinations(digits);
        let mut expected = vec![
            "ad", "ae", "af", "bd", "be", "bf", "cd", "ce",
            "cf",
        ];
        result.sort_unstable();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_letter_combinations_example2() {
        let digits = "".to_string();
        let result = Solution::letter_combinations(digits);
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_letter_combinations_example3() {
        let digits = "2".to_string();
        let mut result =
            Solution::letter_combinations(digits);
        let mut expected = vec!["a", "b", "c"];
        result.sort_unstable();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }
}
