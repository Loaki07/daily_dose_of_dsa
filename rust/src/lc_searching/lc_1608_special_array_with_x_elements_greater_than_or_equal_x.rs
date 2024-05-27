pub struct Solution;

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for x in 0..nums.iter().max().unwrap() + 1 {
            let mut pos = nums
                .binary_search(&x)
                .unwrap_or_else(|pos| pos);
            if nums[pos] == x {
                while pos > 0 && nums[pos] == x {
                    pos -= 1;
                }
                if nums[pos] < x {
                    pos += 1;
                }
            }
            if pos < nums.len()
                && nums[pos..nums.len()].len() == x as usize
            {
                return x;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 5];
        let result = Solution::special_array(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 0];
        let result = Solution::special_array(nums);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![0, 4, 3, 0, 4];
        let result = Solution::special_array(nums);
        assert_eq!(result, 3);
    }
}
