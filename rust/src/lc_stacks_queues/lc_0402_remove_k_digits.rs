pub struct Solution;

impl Solution {
    // monotonic stack
    // O(n) time | O(n) space
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack = vec![];
        let mut k = k as usize;

        for ch in num.chars() {
            while let Some(&top) = stack.last() {
                if k > 0 && top > ch {
                    k -= 1;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(ch);
        }

        // stack = stack[..stack.len() - k].to_vec();
        while k != 0 {
            stack.pop();
            k -= 1;
        }

        let res: String = stack
            .into_iter()
            .skip_while(|&ch| ch == '0')
            .collect();

        if res.is_empty() {
            "0".to_string()
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_kdigits_example_1() {
        let num = "1432219".to_string();
        let k = 3;
        let expected = "1219".to_string();
        assert_eq!(
            Solution::remove_kdigits(num, k),
            expected
        );
    }

    #[test]
    fn test_remove_kdigits_example_2() {
        let num = "10200".to_string();
        let k = 1;
        let expected = "200".to_string();
        assert_eq!(
            Solution::remove_kdigits(num, k),
            expected
        );
    }

    #[test]
    fn test_remove_kdigits_example_3() {
        let num = "10".to_string();
        let k = 2;
        let expected = "0".to_string();
        assert_eq!(
            Solution::remove_kdigits(num, k),
            expected
        );
    }
}
