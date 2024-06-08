use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(
        nums: Vec<i32>,
        k: i32,
    ) -> bool {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        hashmap.insert(0, -1);
        let mut sum = 0;

        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            if let Some(&prev_index) =
                hashmap.get(&(sum % k))
            {
                if i as i32 - prev_index >= 2 {
                    return true;
                }
            } else {
                hashmap.insert(sum % k, i as i32);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![23, 2, 4, 6, 7];
        let k = 6;
        assert_eq!(
            Solution::check_subarray_sum(nums, k),
            true
        );
    }

    #[test]
    fn test_case_2() {
        let nums = vec![23, 2, 6, 4, 7];
        let k = 6;
        assert_eq!(
            Solution::check_subarray_sum(nums, k),
            true
        );
    }

    #[test]
    fn test_case_3() {
        let nums = vec![23, 2, 6, 4, 7];
        let k = 13;
        assert_eq!(
            Solution::check_subarray_sum(nums, k),
            false
        );
    }
}
