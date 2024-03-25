pub struct Solution;

impl Solution {
    // simple dp
    // O(n^2)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        let mut dp = vec![1; size];

        let mut out = 1;
        for i in 1..size {
            for j in 0..i {
                if nums[i] > nums[j] && dp[i] <= dp[j] {
                    dp[i] = dp[j] + 1;
                    out = std::cmp::max(out, dp[i]);
                }
            }
        }
        out
    }

    // dp solution
    // O(n^2)
    pub fn length_of_lis_1(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        dp[0] = 1;
        let mut max = 1;

        for i in 1..nums.len() {
            dp[i] = nums
                .iter()
                .zip(dp.iter())
                .take(i)
                .filter(|(num, _)| **num < nums[i])
                .map(|(_, dp_entry)| *dp_entry + 1)
                .max()
                .unwrap_or(1);
            max = max.max(dp[i]);
        }

        max
    }

    // Patience Sort
    // O(n log n)
    pub fn _length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = vec![];

        for &num in &nums {
            match tails.binary_search(&num) {
                Ok(_) => {}
                Err(i) => {
                    if i == tails.len() {
                        tails.push(num);
                    } else {
                        tails[i] = num;
                    }
                }
            }
        }
        tails.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis_example1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(Solution::length_of_lis(nums), 4);
    }

    #[test]
    fn test_length_of_lis_example2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(Solution::length_of_lis(nums), 4);
    }

    #[test]
    fn test_length_of_lis_example3() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::length_of_lis(nums), 1);
    }
}
