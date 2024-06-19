pub struct Solution;

impl Solution {
    // O(log n) time | O(1) space
    pub fn min_days(
        bloom_day: Vec<i32>,
        m: i32,
        k: i32,
    ) -> i32 {
        let max_day = *bloom_day.iter().max().unwrap();
        let (mut min, mut max) = (0, max_day + 1);
        let mut mid;

        while min < max {
            mid = (min + max) / 2;

            if Self::can_bloom(mid, &bloom_day, m, k) {
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        if min > max_day {
            -1
        } else {
            min
        }
    }

    fn can_bloom(
        day: i32,
        days: &Vec<i32>,
        m: i32,
        k: i32,
    ) -> bool {
        let k = k as usize;
        let (mut prev, mut ans) = (0, 0);

        for i in 0..days.len() {
            if days[i] <= day {
                if 1 + i - prev == k {
                    ans += 1;
                    prev = i + 1;
                }
            } else {
                prev = i + 1;
            }
            if ans == m {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_min_days_example1() {
    let bloom_day = vec![1, 10, 3, 10, 2];
    let m = 3;
    let k = 1;
    let expected_output = 3;
    assert_eq!(
        Solution::min_days(bloom_day, m, k),
        expected_output
    );
}

#[test]
fn test_min_days_example2() {
    let bloom_day = vec![1, 10, 3, 10, 2];
    let m = 3;
    let k = 2;
    let expected_output = -1;
    assert_eq!(
        Solution::min_days(bloom_day, m, k),
        expected_output
    );
}

#[test]
fn test_min_days_example3() {
    let bloom_day = vec![7, 7, 7, 7, 12, 7, 7];
    let m = 2;
    let k = 3;
    let expected_output = 12;
    assert_eq!(
        Solution::min_days(bloom_day, m, k),
        expected_output
    );
}
