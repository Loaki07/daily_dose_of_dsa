pub struct Solution;

impl Solution {
    // O(n log n + n log D/m) time where D = pos(n-1) - pos(0)
    // O(1) space
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut position = position;

        position.sort_unstable();

        let mut lo = 1;
        let mut hi = (position.last().unwrap()
            - position[0])
            / (m - 1)
            + 1;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if Self::can_position(&position, m, mid) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        hi - 1
    }

    fn can_position(
        position: &[i32],
        mut m: i32,
        dist: i32,
    ) -> bool {
        let mut last_pos = -dist;

        for &pos in position {
            if pos - last_pos >= dist {
                m -= 1;
                last_pos = pos;
            }
            if m == 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_distance_example1() {
        let position = vec![1, 2, 3, 4, 7];
        let m = 3;
        let result = Solution::max_distance(position, m);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_max_distance_example2() {
        let position = vec![5, 4, 3, 2, 1, 1000000000];
        let m = 2;
        let result = Solution::max_distance(position, m);
        assert_eq!(result, 999999999);
    }
}
