// Premium leetcode problem statement
// https://neetcode.io/problems/string-encode-and-decode

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encode(&self, strs: Vec<String>) -> String {
        let mut store = String::new();

        for s in strs {
            let len = s.len() as u8;

            store.push(len as char);
            store.push_str(&s);
        }
        store
    }

    pub fn decode(&self, s: String) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        let mut i = 0;

        let mut res = vec![];

        while i < s.len() {
            let len = s[i] as u8 as usize;
            i += 1;

            let j = i + len;

            if j <= s.len() {
                let slice = &s[i..i + len];
                res.push(
                    slice.into_iter().collect::<String>(),
                );
            }

            i += len;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let codec = Codec::new();
        let input = vec![
            "neet".to_string(),
            "code".to_string(),
            "love".to_string(),
            "you".to_string(),
        ];
        let encoded = codec.encode(input.clone()); // Clone input since encode takes ownership
        let decoded = codec.decode(encoded);
        assert_eq!(decoded, input);
    }

    // #[test]
    // fn test_example_2() {
    //     let codec = Codec::new();
    //     let input = vec![
    //         "we".to_string(),
    //         "say".to_string(),
    //         ":".to_string(),
    //         "yes".to_string(),
    //     ];
    //     let encoded =
    // codec.encode(input.clone()); // Clone input
    // since encode takes ownership
    //     let decoded = codec.decode(encoded);
    //     assert_eq!(decoded, input);
    // }
}
