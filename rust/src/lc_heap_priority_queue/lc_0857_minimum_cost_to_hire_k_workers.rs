use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn mincost_to_hire_workers(
        quality: Vec<i32>,
        wage: Vec<i32>,
        k: i32,
    ) -> f64 {
        let k = k as usize;
        let mut workers = quality
            .iter()
            .enumerate()
            .map(|(i, &q)| (wage[i] as f64 / q as f64, q))
            .collect::<Vec<(f64, i32)>>();
        workers.sort_unstable_by(|(a, _), (b, _)| {
            a.partial_cmp(&b).unwrap()
        });

        let mut qsum = 0i32;

        let mut h = BinaryHeap::<i32>::new();

        workers.iter().fold(f64::MAX, |res, &(r, q)| {
            h.push(q);
            qsum += q;
            if h.len() > k {
                qsum -= h.pop().unwrap();
            }
            if h.len() == k {
                res.min(qsum as f64 * r)
            } else {
                res
            }
        }) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let quality = vec![10, 20, 5];
        let wage = vec![70, 50, 30];
        let k = 2;
        assert_eq!(
            Solution::mincost_to_hire_workers(
                quality, wage, k,
            ),
            105.0
        );
    }

    #[test]
    fn test_example2() {
        let quality = vec![3, 1, 10, 10, 1];
        let wage = vec![4, 8, 2, 2, 7];
        let k = 3;
        assert_eq!(
            (Solution::mincost_to_hire_workers(
                quality, wage, k,
            ) * 100.0)
                .round()
                / 100.0,
            30.67
        );
    }
}
