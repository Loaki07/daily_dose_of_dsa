pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let (mut count, mut length): (i32, i32) =
            (0, s.len() as i32);

        for i in 0..length {
            // odd length
            let (mut l, mut r) = (i, i);
            while l >= 0
                && r < length
                && s[l as usize] == s[r as usize]
            {
                count += 1;
                l -= 1;
                r += 1;
            }

            // even length
            let (mut l, mut r) = (i, i + 1);

            while l >= 0
                && r < length
                && s[l as usize] == s[r as usize]
            {
                count += 1;
                l -= 1;
                r += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::count_substrings("abc".to_string()),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::count_substrings("aaa".to_string()),
            6
        );
    }
}
