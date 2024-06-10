use std::iter::repeat;

pub struct Solution;

impl Solution {
    // counting sort
    // O(n) time | O(n + k) space
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut counts = [0; 101];

        heights
            .iter()
            .for_each(|&h| counts[h as usize] += 1);

        counts
            .into_iter()
            .zip(0..)
            .flat_map(|(c, h)| repeat(h).take(c)) // Yeilds heights in order
            .zip(heights)
            .filter(|(expected_h, student_h)| {
                expected_h != student_h
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_height_checker_example1() {
        let heights = vec![1, 1, 4, 2, 1, 3];
        let expected = 3;
        assert_eq!(
            Solution::height_checker(heights),
            expected
        );
    }

    #[test]
    fn test_height_checker_example2() {
        let heights = vec![5, 1, 2, 3, 4];
        let expected = 5;
        assert_eq!(
            Solution::height_checker(heights),
            expected
        );
    }

    #[test]
    fn test_height_checker_example3() {
        let heights = vec![1, 2, 3, 4, 5];
        let expected = 0;
        assert_eq!(
            Solution::height_checker(heights),
            expected
        );
    }
}
