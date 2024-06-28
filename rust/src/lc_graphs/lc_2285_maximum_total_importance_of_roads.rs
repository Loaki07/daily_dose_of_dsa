use std::iter::repeat;

pub struct Solution;

impl Solution {
    // Using counting sort
    // O(n) time | O(n) space
    pub fn maximum_importance(
        n: i32,
        roads: Vec<Vec<i32>>,
    ) -> i64 {
        let mut degree = vec![0; n as usize];
        let mut counts = vec![0; n as usize + 1];

        for road in roads {
            degree[road[0] as usize] += 1;
            degree[road[1] as usize] += 1;
        }

        for deg in degree {
            counts[deg] += 1;
        }

        (1..)
            .zip((0..)
            .zip(counts)
            .flat_map(|(deg, n)| repeat(deg).take(n)))
            .map(|(vert, deg)| vert * deg)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let n = 5;
        let roads = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![0, 2],
            vec![1, 3],
            vec![2, 4],
        ];
        let expected_output = 43;
        let result = Solution::maximum_importance(n, roads);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_example2() {
        let n = 5;
        let roads =
            vec![vec![0, 3], vec![2, 4], vec![1, 3]];
        let expected_output = 20;
        let result = Solution::maximum_importance(n, roads);
        assert_eq!(result, expected_output);
    }
}
