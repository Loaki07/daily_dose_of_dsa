
pub struct Solution;

impl Solution {
    // O(n * m) time | O(n * m) space
    pub fn find_rotate_steps(
        ring: String,
        key: String,
    ) -> i32 {
        use std::collections::{HashMap, HashSet};

        let nr = ring.len();
        let nk = key.len();

        let cixs = ring.chars().enumerate().fold(
            HashMap::<char, HashSet<usize>>::new(),
            |mut a, (i, x)| {
                a.entry(x)
                    .or_insert_with(HashSet::new)
                    .insert(i);
                a
            },
        );

        let mut layer: Vec<(usize, i32)> = vec![(0, 0)];
        for c in key.chars() {
            let mut next_layer: Vec<(usize, i32)> =
                Vec::new();
            if let Some(ixs) = cixs.get(&c) {
                for &ix in ixs.iter() {
                    let mut min_length = i32::MAX;

                    for &(pix, p_path) in layer.iter() {
                        let diff = if ix < pix {
                            pix - ix
                        } else {
                            ix - pix
                        };
                        let dist = diff.min(nr - diff);
                        min_length = min_length
                            .min(p_path + dist as i32);
                    }

                    next_layer.push((ix, min_length));
                }

                layer = next_layer;
            }
        }

        layer.iter().map(|x| x.1).min().unwrap() + nk as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rotate_steps_case_1() {
        let ring = "godding".to_string();
        let key = "gd".to_string();
        assert_eq!(
            Solution::find_rotate_steps(ring, key),
            4
        );
    }

    #[test]
    fn test_find_rotate_steps_case_2() {
        let ring = "godding".to_string();
        let key = "godding".to_string();
        assert_eq!(
            Solution::find_rotate_steps(ring, key),
            13
        );
    }

    // Additional tests to cover more scenarios
    #[test]
    fn test_find_rotate_steps_single_character_key() {
        let ring = "godding".to_string();
        let key = "g".to_string();
        assert_eq!(
            Solution::find_rotate_steps(ring, key),
            1
        );
    }

    #[test]
    fn test_find_rotate_steps_repeated_character_key() {
        let ring = "godding".to_string();
        let key = "ggggggg".to_string();
        assert_eq!(
            Solution::find_rotate_steps(ring, key),
            7
        ); // Assume all g's are accessible
           // without extra rotations
    }
}
