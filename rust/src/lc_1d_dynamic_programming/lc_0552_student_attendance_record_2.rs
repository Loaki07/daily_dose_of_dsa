pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn check_record(n: i32) -> i32 {
        const MOD: usize = 1000000007;
        let mut prev: Vec<usize> = vec![1, 1, 0, 1, 0, 0];
        let mut cur = prev.clone();

        for _i in 1..n {
            cur[0] = (prev[0] + prev[1] + prev[2]) % MOD;
            cur[1] = prev[0];
            cur[2] = prev[1];
            cur[3] = prev.iter().sum::<usize>() % MOD;
            cur[4] = prev[3];
            cur[5] = prev[4];
            std::mem::swap(&mut prev, &mut cur);
        }
        (prev.iter().sum::<usize>() % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 2;
        let expected = 8;
        assert_eq!(Solution::check_record(n), expected);
    }

    #[test]
    fn test_case_2() {
        let n = 1;
        let expected = 3;
        assert_eq!(Solution::check_record(n), expected);
    }

    #[test]
    fn test_case_3() {
        let n = 10101;
        let expected = 183236316;
        assert_eq!(Solution::check_record(n), expected);
    }
}
