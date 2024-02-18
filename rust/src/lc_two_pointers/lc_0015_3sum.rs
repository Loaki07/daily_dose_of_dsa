pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering::{Equal, Greater, Less};

        let mut numbers = nums;
        numbers.sort_unstable();

        let mut ans: Vec<Vec<i32>> = Vec::new();

        for i in 0..numbers.len() {
            if i > 0 && numbers[i] == numbers[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, numbers.len() - 1);

            while l < r {
                match (numbers[i] + numbers[l] + numbers[r])
                    .cmp(&0)
                {
                    Less => l += 1,
                    Greater => r -= 1,
                    Equal => {
                        ans.push(vec![
                            numbers[i], numbers[l],
                            numbers[r],
                        ]);

                        l += 1;
                        while numbers[l] == numbers[l - 1]
                            && l < r
                        {
                            l += 1;
                        }

                        r -= 1;
                        while numbers[r] == numbers[r + 1]
                            && l < r
                        {
                            r -= 1;
                        }
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut result = Solution::three_sum(nums);
        let mut expected =
            vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        result.sort(); // Sorting to ensure the order matches for
                       // comparison
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 1, 1];
        let result = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![0, 0, 0];
        let result = Solution::three_sum(nums);
        let expected = vec![vec![0, 0, 0]];

        assert_eq!(result, expected);
    }
}
