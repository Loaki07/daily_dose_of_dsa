pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(
        tickets: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut res = 0;
        let k = k as usize;

        for i in 0..tickets.len() {
            if i <= k {
                res += tickets[i].min(tickets[k]);
            } else {
                res += tickets[i].min(tickets[k] - 1);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_required_to_buy_example_1() {
        let tickets = vec![2, 3, 2];
        let k = 2;
        let result =
            Solution::time_required_to_buy(tickets, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_time_required_to_buy_example_2() {
        let tickets = vec![5, 1, 1, 1];
        let k = 0;
        let result =
            Solution::time_required_to_buy(tickets, k);
        assert_eq!(result, 8);
    }
}
