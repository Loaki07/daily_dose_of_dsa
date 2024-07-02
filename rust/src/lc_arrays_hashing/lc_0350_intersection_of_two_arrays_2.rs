pub struct Solution;

impl Solution {
    // O(m+n) time | O(1) space
    pub fn intersect(
        nums1: Vec<i32>,
        mut nums2: Vec<i32>,
    ) -> Vec<i32> {
        let mut counts = [0; 1001];

        for num in nums1 {
            counts[num as usize] += 1;
        }

        nums2.retain(move |&n| {
            counts[n as usize] -= 1;
            counts[n as usize] >= 0
        });

        nums2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect_example1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let expected = vec![2, 2];
        assert_eq!(
            Solution::intersect(nums1, nums2),
            expected
        );
    }

    #[test]
    fn test_intersect_example2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let expected = vec![4, 9].sort();
        assert_eq!(
            Solution::intersect(nums1, nums2).sort(),
            expected
        );
    }
}
