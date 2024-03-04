pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut bottom = vec![1; n as usize];

        for _ in 0..m - 1 {
            let mut top = vec![1; n as usize];
            for c in (0..n - 1).rev().map(|c| c as usize) {
                top[c] = bottom[c] + top[c + 1];
            }

            bottom = top;
        }
        bottom[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let m = 3;
        let n = 7;
        let result = Solution::unique_paths(m, n);
        assert_eq!(
            result, 28,
            "Test for m = 3 and n = 7 failed"
        );
    }

    #[test]
    fn test_example_2() {
        let m = 3;
        let n = 2;
        let result = Solution::unique_paths(m, n);
        assert_eq!(
            result, 3,
            "Test for m = 3 and n = 2 failed"
        );
    }

    // You can add more tests to cover edge cases or
    // other scenarios
    #[test]
    fn test_small_grid() {
        let m = 1;
        let n = 1;
        let result = Solution::unique_paths(m, n);
        assert_eq!(
            result, 1,
            "Test for m = 1 and n = 1 failed"
        );
    }

    #[test]
    fn test_large_grid() {
        let m = 10;
        let n = 10;
        let result = Solution::unique_paths(m, n);
        // The expected result should be computed and
        // replaced here
        let expected = 48620; // This is just an example, calculate the correct
                              // value.
        assert_eq!(
            result, expected,
            "Test for m = 10 and n = 10 failed"
        );
    }
}
