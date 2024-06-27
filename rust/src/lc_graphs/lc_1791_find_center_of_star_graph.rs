use std::iter::repeat;

pub struct Solution;

impl Solution {
    // Flat map
    // O(1) time | O(1) space
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        *edges
            .windows(2)
            .flat_map(|w| {
                w[0].iter()
                    .flat_map(|u| repeat(u).zip(&w[1]))
            })
            .find(|(a, b)| a == b)
            .expect("✨🤪✨")
            .0
    }

    // Simple
    // O(1) time | O(1) space
    pub fn _find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0].contains(&edges[1][0]) {
            edges[1][0]
        } else {
            edges[1][1]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let edges =
            vec![vec![1, 2], vec![2, 3], vec![4, 2]];
        assert_eq!(Solution::find_center(edges), 2);
    }

    #[test]
    fn test_example2() {
        let edges = vec![
            vec![1, 2],
            vec![5, 1],
            vec![1, 3],
            vec![1, 4],
        ];
        assert_eq!(Solution::find_center(edges), 1);
    }
}
