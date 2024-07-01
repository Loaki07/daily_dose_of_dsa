pub struct Solution;

impl Solution {
    // Fixed sliding window
    // O(n) time | O(1) space
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut window = 0;
        for i in 0..3.min(arr.len()) {
            window += arr[i] % 2;
        }

        if window == 3 {
            return true;
        }

        for i in 3..arr.len() {
            window += arr[i] % 2;
            // slide the window forward by 1 element
            window -= arr[i - 3] % 2;
            if window == 3 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let arr = vec![2, 6, 4, 1];
        assert_eq!(
            Solution::three_consecutive_odds(arr),
            false
        );
    }

    #[test]
    fn test_example_2() {
        let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
        assert_eq!(
            Solution::three_consecutive_odds(arr),
            true
        );
    }
}
