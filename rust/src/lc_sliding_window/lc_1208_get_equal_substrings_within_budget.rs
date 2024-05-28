pub struct Solution;

impl Solution {
    pub fn equal_substring(
        s: String,
        t: String,
        max_cost: i32,
    ) -> i32 {
        let mut window: i32 = 0;
        let mut left: usize = 0;
        let mut ans: i32 = 0;

        let sbytes = s.as_bytes();
        let tbytes = t.as_bytes();

        for right in 0..s.len() {
            window += ((sbytes[right] as i32)
                - (tbytes[right] as i32))
                .abs();
            while window > max_cost {
                window -= ((sbytes[left] as i32)
                    - (tbytes[left] as i32))
                    .abs();
                left += 1;
            }

            ans = ans.max(right as i32 - left as i32 + 1);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("abcd");
        let t = String::from("bcdf");
        let max_cost = 3;
        assert_eq!(
            Solution::equal_substring(s, t, max_cost),
            3
        );
    }

    #[test]
    fn test_example_2() {
        let s = String::from("abcd");
        let t = String::from("cdef");
        let max_cost = 3;
        assert_eq!(
            Solution::equal_substring(s, t, max_cost),
            1
        );
    }

    #[test]
    fn test_example_3() {
        let s = String::from("abcd");
        let t = String::from("acde");
        let max_cost = 0;
        assert_eq!(
            Solution::equal_substring(s, t, max_cost),
            1
        );
    }
}
