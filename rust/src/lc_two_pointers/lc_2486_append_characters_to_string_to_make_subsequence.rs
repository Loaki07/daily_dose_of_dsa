pub struct Solution;

impl Solution {
    // O(s+t) time | O(1) space
    pub fn append_characters(s: String, t: String) -> i32 {
        let (mut i, mut j) = (0, 0);
        let (mut s_arr, mut t_arr) = (
            s.chars().collect::<Vec<char>>(),
            t.chars().collect::<Vec<char>>(),
        );
        let (s_len, t_len) = (s.len(), t.len());

        while i < s_len && j < t_len {
            if s_arr[i] == t_arr[j] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }

        (t_len - j) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("coaching");
        let t = String::from("coding");
        assert_eq!(Solution::append_characters(s, t), 4);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("abcde");
        let t = String::from("a");
        assert_eq!(Solution::append_characters(s, t), 0);
    }

    #[test]
    fn test_example_3() {
        let s = String::from("z");
        let t = String::from("abcde");
        assert_eq!(Solution::append_characters(s, t), 5);
    }
}
