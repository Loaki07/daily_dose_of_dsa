use std::collections::HashMap;

pub struct Solution;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Invested {
    Yes,
    No,
    Cooldown,
}

impl Solution {
    pub fn dp(
        prices: &[i32],
        day: usize,
        invested: Invested,
        memo: &mut HashMap<(usize, Invested), i32>,
    ) -> i32 {
        if day == prices.len() {
            0
        } else if let Some(profit) =
            memo.get(&(day, invested))
        {
            *profit
        } else {
            let res = match invested {
                Invested::Yes => Self::dp(
                    prices,
                    day + 1,
                    invested,
                    memo,
                )
                .max(
                    prices[day]
                        + Self::dp(
                            prices,
                            day + 1,
                            Invested::Cooldown,
                            memo,
                        ),
                ),
                Invested::No => Self::dp(
                    prices,
                    day + 1,
                    invested,
                    memo,
                )
                .max(
                    -prices[day]
                        + Self::dp(
                            prices,
                            day + 1,
                            Invested::Yes,
                            memo,
                        ),
                ),
                Invested::Cooldown => Self::dp(
                    prices,
                    day + 1,
                    Invested::No,
                    memo,
                ),
            };

            memo.insert((day, invested), res);
            res
        }
    }

    // dp with memo
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        Self::dp(
            &prices,
            0,
            Invested::No,
            &mut HashMap::new(),
        )
    }

    // simple dp
    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut sell = vec![0; n + 1];
        let mut buy = vec![0; n + 1];

        buy[1] = -prices[0];

        for i in 1..n {
            sell[i + 1] = sell[i].max(buy[i] + prices[i]);
            buy[i + 1] =
                buy[i].max(sell[i - 1] - prices[i]);
        }

        sell[n]
    }

    pub fn _max_profit(prices: Vec<i32>) -> i32 {
        let mut p1 = 0;
        let mut p2 = 0;

        for x in 1..=prices.len() - 1 {
            let temp = p1;
            p1 = std::cmp::max(
                p1 + prices[x] - prices[x - 1],
                p2,
            );
            p2 = std::cmp::max(temp, p2);
        }

        std::cmp::max(p1, p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_example1() {
        let prices = vec![1, 2, 3, 0, 2];
        assert_eq!(Solution::max_profit(prices), 3);
    }

    #[test]
    fn test_max_profit_example2() {
        let prices = vec![1];
        assert_eq!(Solution::max_profit(prices), 0);
    }
}
