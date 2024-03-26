pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut rob1, mut rob2) = (0, 0);

        // [rob1, rob2, n, n+1, ...]
        for n in nums {
            let temp = std::cmp::max(n + rob1, rob2);
            rob1 = rob2;
            rob2 = temp;
        }
        rob2
    }

    pub fn _rob(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(
                // (rob1, rob2)
                (0, 0),
                |loot, money| {
                    (loot.1, loot.1.max(loot.0 + money))
                },
            )
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob_example1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);
    }

    #[test]
    fn test_rob_example2() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12);
    }
}
