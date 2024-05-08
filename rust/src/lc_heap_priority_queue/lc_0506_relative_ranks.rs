use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    // O(n * log(n)) time | O(n) space
    pub fn find_relative_ranks(
        score: Vec<i32>,
    ) -> Vec<String> {
        score
            .clone()
            .into_iter()
            .fold(
                (vec![], {
                    score
                        .into_iter()
                        .collect::<BinaryHeap<i32>>()
                        .into_sorted_vec()
                        .into_iter()
                        .rev()
                        .enumerate()
                        .fold(
                            HashMap::new(),
                            |mut m, (i, x)| {
                                m.entry(x).or_insert(i + 1);
                                m
                            },
                        )
                }),
                |(mut ans, m), x| {
                    if let Some(y) = m.get(&x) {
                        match y {
                            1 => ans.push(
                                "Gold Medal".to_string(),
                            ),
                            2 => ans.push(
                                "Silver Medal".to_string(),
                            ),
                            3 => ans.push(
                                "Bronze Medal".to_string(),
                            ),
                            n => ans.push(format!("{n}")),
                        }
                    }
                    (ans, m)
                },
            )
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let score = vec![5, 4, 3, 2, 1];
        let expected = vec![
            "Gold Medal".to_string(),
            "Silver Medal".to_string(),
            "Bronze Medal".to_string(),
            "4".to_string(),
            "5".to_string(),
        ];
        assert_eq!(
            Solution::find_relative_ranks(score),
            expected
        );
    }

    #[test]
    fn test_example_2() {
        let score = vec![10, 3, 8, 9, 4];
        let expected = vec![
            "Gold Medal".to_string(),
            "5".to_string(),
            "Bronze Medal".to_string(),
            "Silver Medal".to_string(),
            "4".to_string(),
        ];
        assert_eq!(
            Solution::find_relative_ranks(score),
            expected
        );
    }
}
