pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn tribonacci(n: i32) -> i32 {
        let mut t = [0, 1, 1];

        if n <= 2 {
            return t[n as usize];
        }

        for i in 3..(n + 1) as usize {
            t.swap(0, 1);
            t.swap(1, 2);
            t[2] = t.iter().sum();
        }

        t[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tribonacci_case_1() {
        let n = 4;
        assert_eq!(Solution::tribonacci(n), 4);
    }

    #[test]
    fn test_tribonacci_case_2() {
        let n = 25;
        assert_eq!(Solution::tribonacci(n), 1389537);
    }

    // Additional basic tests to ensure correct
    // computation from the start of the sequence
    #[test]
    fn test_tribonacci_small_values() {
        assert_eq!(Solution::tribonacci(0), 0); // T_0 = 0
        assert_eq!(Solution::tribonacci(1), 1); // T_1 = 1
        assert_eq!(Solution::tribonacci(2), 1); // T_2 = 1
        assert_eq!(Solution::tribonacci(3), 2); // T_3 = 2
    }
}
