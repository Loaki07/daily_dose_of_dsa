pub struct Solution;

impl Solution {
    // O(n * log n) time | O(1) space
    pub fn maximum_happiness_sum(
        mut happiness: Vec<i32>,
        k: i32,
    ) -> i64 {
        let mut decr = 0;
        let mut sum = 0;

        happiness.sort_unstable();

        for val in
            happiness.into_iter().rev().take(k as usize)
        {
            sum += (val - decr).max(0) as i64;
            decr += 1;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::maximum_happiness_sum(
                vec![1, 2, 3],
                2
            ),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::maximum_happiness_sum(
                vec![1, 1, 1, 1],
                2
            ),
            1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::maximum_happiness_sum(
                vec![2, 3, 4, 5],
                1
            ),
            5
        );
    }
}
