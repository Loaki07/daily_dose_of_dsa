pub struct Solution;

impl Solution {
    // O(n) time, O(n) space
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut smallest_number = 1;

        for i in 0..nums.len() {
            *map.entry(nums[i]).or_insert(0) += 1;

            if nums[i] == smallest_number {
                while map.get(&smallest_number).is_some() {
                    smallest_number += 1;
                }
            }
        }
        smallest_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive_example1() {
        let nums = vec![1, 2, 0];
        assert_eq!(
            Solution::first_missing_positive(nums),
            3
        );
    }

    #[test]
    fn test_first_missing_positive_example2() {
        let nums = vec![3, 4, -1, 1];
        assert_eq!(
            Solution::first_missing_positive(nums),
            2
        );
    }

    #[test]
    fn test_first_missing_positive_example3() {
        let nums = vec![7, 8, 9, 11, 12];
        assert_eq!(
            Solution::first_missing_positive(nums),
            1
        );
    }
}
