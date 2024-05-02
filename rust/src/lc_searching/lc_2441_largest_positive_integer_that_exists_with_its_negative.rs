pub struct Solution;

impl Solution {
    // O(n log n) time | O(1) space
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        for i in (0..nums.len()).rev() {
            match nums.binary_search(&(-nums[i])) {
                Ok(_) => return nums[i],
                _ => {}
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::find_max_k(vec![-1, 2, -3, 3]),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]),
            7
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::find_max_k(vec![
                -10, 8, 6, 7, -2, -3
            ]),
            -1
        );
    }
}
