// basic two pointers for the sliding window
// O(n) time | O(1) space
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut l = 0;
    let mut r = 1;
    let mut max_profit = 0;

    while r < prices.len() {
        if prices[l] < prices[r] {
            let profit = prices[r] - prices[l];
            max_profit = profit.max(max_profit);
        } else {
            l = r;
        }
        r += 1;
    }

    max_profit
}

pub fn _max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut max = 0.max(prices[1] - prices[0]);
    let mut min = prices[0];

    for i in 1..prices.len() {
        max = max.max(prices[i] - min);
        min = min.min(prices[i]);
        // dbg!(prices[i], max, min);
    }

    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5)
    }

    #[test]
    fn ex2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
