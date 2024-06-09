use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let (mut count, mut sum, mut map) = (
            0,
            0,
            HashMap::with_capacity(nums.len() / 2),
        );
        map.insert(0, 1);

        for num in nums {
            sum += num;
            count +=
                map.get(&(sum - k)).copied().unwrap_or(0);
            map.entry(sum)
                .and_modify(|e| *e = *e + 1)
                .or_insert(1);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_subarray_sum() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(Solution::subarray_sum(nums, k), 2);

        let nums = vec![1, 2, 3];
        let k = 3;
        assert_eq!(Solution::subarray_sum(nums, k), 2);
    }
}
