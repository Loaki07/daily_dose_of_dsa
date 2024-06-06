use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(
        hand: Vec<i32>,
        group_size: i32,
    ) -> bool {
        let n = hand.len();
        let m = group_size as usize;
        if n % m != 0 {
            return false;
        }

        let x = hand.len() / m;
        let mut btree = BTreeMap::new();
        for v in hand {
            *btree.entry(v).or_insert(0) += 1;
        }

        for _ in 0..x {
            let (&min, _) = btree.iter().next().unwrap();
            for i in 0..m {
                let cv = min + i as i32;
                if let Some(num) = btree.get_mut(&cv) {
                    *num -= 1;
                    if *num == 0 {
                        btree.remove(&cv);
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let hand = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
        let group_size = 3;
        assert_eq!(
            Solution::is_n_straight_hand(hand, group_size),
            true
        );
    }

    #[test]
    fn test_example2() {
        let hand = vec![1, 2, 3, 4, 5];
        let group_size = 4;
        assert_eq!(
            Solution::is_n_straight_hand(hand, group_size),
            false
        );
    }
}
