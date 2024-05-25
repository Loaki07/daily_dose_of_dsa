pub struct Solution;

impl Solution {
    pub fn word_break(
        s: String,
        mut word_dict: Vec<String>,
    ) -> Vec<String> {
        word_dict.retain(|w| s.contains(w));
        let mut res = vec![];
        Self::create_sentence(
            &word_dict,
            &mut vec![],
            &s,
            &mut res,
        );
        res
    }
    pub fn create_sentence(
        dict: &Vec<String>,
        words: &mut Vec<String>,
        word: &str,
        res: &mut Vec<String>,
    ) {
        if word.is_empty() {
            res.push(words.join(" "));
            return;
        }
        dict.iter().for_each(|w| {
            if word.starts_with(w) {
                let mut new = words.clone();
                new.push(w.clone());
                Self::create_sentence(
                    dict,
                    &mut new,
                    word.strip_prefix(w).unwrap(),
                    res,
                )
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("catsanddog");
        let word_dict = vec![
            String::from("cat"),
            String::from("cats"),
            String::from("and"),
            String::from("sand"),
            String::from("dog"),
        ];
        let mut expected_output = vec![
            String::from("cats and dog"),
            String::from("cat sand dog"),
        ];
        assert_eq!(
            Solution::word_break(s, word_dict).sort(),
            expected_output.sort()
        );
    }

    #[test]
    fn test_example_2() {
        let s = String::from("pineapplepenapple");
        let word_dict = vec![
            String::from("apple"),
            String::from("pen"),
            String::from("applepen"),
            String::from("pine"),
            String::from("pineapple"),
        ];
        let mut expected_output = vec![
            String::from("pine apple pen apple"),
            String::from("pineapple pen apple"),
            String::from("pine applepen apple"),
        ];
        assert_eq!(
            Solution::word_break(s, word_dict).sort(),
            expected_output.sort()
        );
    }

    #[test]
    fn test_example_3() {
        let s = String::from("catsandog");
        let word_dict = vec![
            String::from("cats"),
            String::from("dog"),
            String::from("sand"),
            String::from("and"),
            String::from("cat"),
        ];
        let expected_output: Vec<String> = vec![];
        assert_eq!(
            Solution::word_break(s, word_dict),
            expected_output
        );
    }
}
