pub struct Solution;

impl Solution {
    // O(n log n) time | O(n) space
    pub fn deck_revealed_increasing(
        mut deck: Vec<i32>,
    ) -> Vec<i32> {
        use std::collections::VecDeque;

        deck.sort_unstable();

        let mut deck = deck.into_iter();

        let mut queue = VecDeque::from_iter(0..deck.len());
        let mut result = vec![0; deck.len()];

        while let Some(i) = queue.pop_front() {
            result[i] = deck.next().unwrap();
            queue.rotate_left(1.min(queue.len()));
        }

        result
    }
    // O(n log n) time | O(n) space
    pub fn _deck_revealed_increasing(
        mut deck: Vec<i32>,
    ) -> Vec<i32> {
        use std::collections::VecDeque;

        deck.sort_unstable();

        let mut res = vec![0; deck.len()];
        let mut queue = VecDeque::from_iter(0..deck.len());

        for n in deck {
            let index = queue.pop_front().unwrap();
            res[index] = n;

            if !queue.is_empty() {
                let push_idx_back =
                    queue.pop_front().unwrap();
                queue.push_back(push_idx_back);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_revealed_increasing_example_1() {
        let deck = vec![17, 13, 11, 2, 3, 5, 7];
        let expected = vec![2, 13, 3, 11, 5, 17, 7];
        assert_eq!(
            Solution::deck_revealed_increasing(deck),
            expected
        );
    }

    #[test]
    fn test_deck_revealed_increasing_example_2() {
        let deck = vec![1, 1000];
        let expected = vec![1, 1000];
        assert_eq!(
            Solution::deck_revealed_increasing(deck),
            expected
        );
    }
}
