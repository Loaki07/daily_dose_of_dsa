pub struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut bytes = s
            .bytes()
            .enumerate()
            .rev()
            .skip_while(|&(_, b)| b == b'0');
        if bytes.next() == Some((0, b'1')) {
            return s.len() as i32 - 1;
        }
        1 + (s.len()
            + bytes.filter(|&(_, b)| b == b'0').count())
            as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::num_steps("1101".to_string()),
            6
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::num_steps("10".to_string()),
            1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::num_steps("1".to_string()), 0);
    }
}
