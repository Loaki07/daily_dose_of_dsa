pub struct Solution;

impl Solution {
    // time O(m*n) - space O(n)
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let n = amount as usize;
        let mut dp = vec![0; n + 1];

        dp[0] = 1;
        let mut c: usize;

        for coin in coins {
            c = coin as usize;
            for i in c..=n {
                dp[i] += dp[i - c];
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_example1() {
        let amount = 5;
        let coins = vec![1, 2, 5];
        assert_eq!(Solution::change(amount, coins), 4);
    }

    #[test]
    fn test_change_example2() {
        let amount = 3;
        let coins = vec![2];
        assert_eq!(Solution::change(amount, coins), 0);
    }

    #[test]
    fn test_change_example3() {
        let amount = 10;
        let coins = vec![10];
        assert_eq!(Solution::change(amount, coins), 1);
    }
}
