pub struct Solution;

impl Solution {
    // Count Sort
    // O(n + m) time, m is the max elem | O(1) space
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let (max, len) = (
            nums.iter().max().map(|&n| n as usize).unwrap(),
            nums.len(),
        );
        let mut freq = vec![0; max + len + 1];
        nums.iter()
            .map(|&n| n as usize)
            .for_each(|i| freq[i] += 1);

        freq.into_iter()
            .scan(0, |dup, mut n| {
                n += *dup;
                *dup = (n - 1).max(0);
                Some(*dup)
            })
            .sum()
    }

    // O(n log n) time | O(1) space
    pub fn _min_increment_for_unique(
        mut nums: Vec<i32>,
    ) -> i32 {
        nums.sort_unstable();
        let mut res = 0;

        for i in 1..nums.len() {
            if nums[i - 1] >= nums[i] {
                res += 1 + nums[i - 1] - nums[i];
                nums[i] = nums[i - 1] + 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_increment_for_unique() {
        let nums = vec![1, 2, 2];
        let expected = 1;
        assert_eq!(
            Solution::min_increment_for_unique(nums),
            expected
        );
    }

    #[test]
    fn test_min_increment_for_unique_2() {
        let nums = vec![3, 2, 1, 2, 1, 7];
        let expected = 6;
        assert_eq!(
            Solution::min_increment_for_unique(nums),
            expected
        );
    }
}
