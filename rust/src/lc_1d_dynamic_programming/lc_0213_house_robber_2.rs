pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        return match l {
            0 => 0,
            1 => nums[0],
            _ => Self::rob_house(&nums[1..])
                .max(Self::rob_house(&nums[0..l - 1])),
        };
    }

    fn rob_house(nums: &[i32]) -> i32 {
        let (mut rob1, mut rob2) = (0, 0);

        // [rob1, rob2, n, n+1, ...]
        for n in nums {
            let temp = std::cmp::max(n + rob1, rob2);
            rob1 = rob2;
            rob2 = temp;
        }
        rob2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob_example1() {
        let nums = vec![2, 3, 2];
        assert_eq!(Solution::rob(nums), 3);
    }

    #[test]
    fn test_rob_example2() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);
    }

    #[test]
    fn test_rob_example3() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::rob(nums), 3);
    }
}
