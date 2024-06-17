pub struct Solution;

impl Solution {
    // O(sqrt(n)) time | O(1) space
    pub fn judge_square_sum(c: i32) -> bool {
        let (mut left, mut right) =
            (0_i64, (c as f64).sqrt() as i64);

        while left <= right {
            let mut sum = left * left + right * right;
            if sum < c as i64 {
                left += 1;
            } else if sum > c as i64 {
                right -= 1;
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_square_sum_example1() {
        assert_eq!(Solution::judge_square_sum(5), true);
    }

    #[test]
    fn test_judge_square_sum_example2() {
        assert_eq!(Solution::judge_square_sum(3), false);
    }
}
