pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(
        mut cost: Vec<i32>,
    ) -> i32 {
        for i in 2..cost.len() {
            cost[i] += cost[i - 1].min(cost[i - 2]);
        }

        let len = cost.len();

        cost[len - 1].min(cost[len - 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let cost = vec![10, 15, 20];
        let output =
            Solution::min_cost_climbing_stairs(cost);
        assert_eq!(output, 15);
    }

    #[test]
    fn test_example_2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let output =
            Solution::min_cost_climbing_stairs(cost);
        assert_eq!(output, 6);
    }
}
