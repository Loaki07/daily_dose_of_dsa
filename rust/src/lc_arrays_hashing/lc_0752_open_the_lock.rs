pub struct Solution;

impl Solution {
    // bfs
    // O(v+e) time | O(v+e) space
    pub fn open_lock(
        deadends: Vec<String>,
        target: String,
    ) -> i32 {
        use std::collections::{HashSet, VecDeque};

        let mut visited = deadends
            .iter()
            .map(Solution::str2arr)
            .collect::<HashSet<_>>();
        let (target, mut queue) = (
            Solution::str2arr(&target),
            VecDeque::new(),
        );

        queue.push_back((0, [0, 0, 0, 0]));

        if visited.contains(&[0, 0, 0, 0])
            || visited.contains(&target)
        {
            return -1;
        }

        while let Some((turns, u)) = queue.pop_front() {
            if u == target {
                return turns;
            }

            for i in 0..4 {
                let mut v1 = u;
                let mut v2 = u;

                v1[i] = (v1[i] + 1) % 10;
                v2[i] = (v2[i] + 9) % 10;

                if visited.insert(v1) {
                    queue.push_back((turns + 1, v1));
                }

                if visited.insert(v2) {
                    queue.push_back((turns + 1, v2));
                }
            }
        }

        -1
    }

    fn str2arr(s: &String) -> [u8; 4] {
        s.bytes()
            .map(|b| b - b'0')
            .collect::<Vec<_>>()
            .try_into()
            .expect("Conversion from string to arr failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_lock_case_1() {
        let deadends =
            vec!["0201", "0101", "0102", "1212", "2002"]
                .iter()
                .map(|&s| s.to_string())
                .collect();
        let target = "0202".to_string();
        assert_eq!(
            Solution::open_lock(deadends, target),
            6
        );
    }

    #[test]
    fn test_open_lock_case_2() {
        let deadends = vec!["8888"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let target = "0009".to_string();
        assert_eq!(
            Solution::open_lock(deadends, target),
            1
        );
    }

    #[test]
    fn test_open_lock_case_3() {
        let deadends = vec![
            "8887", "8889", "8878", "8898", "8788", "8988",
            "7888", "9888",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();
        let target = "8888".to_string();
        assert_eq!(
            Solution::open_lock(deadends, target),
            -1
        );
    }

    // Additional test for checking behavior when the
    // start "0000" is a deadend
    #[test]
    fn test_open_lock_start_is_deadend() {
        let deadends = vec!["0000"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let target = "8888".to_string();
        assert_eq!(
            Solution::open_lock(deadends, target),
            -1
        );
    }
}
