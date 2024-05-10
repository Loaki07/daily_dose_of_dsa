use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    // O(n log k) time | O(k) space
    pub fn kth_smallest_prime_fraction(
        arr: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let n = arr.len();
        let mut fractions =
            Vec::with_capacity(n * (n - 1) / 2);

        // Populate the fractions vector
        for i in 0..n {
            for j in i + 1..n {
                fractions.push((
                    arr[i] as f64 / arr[j] as f64,
                    arr[i],
                    arr[j],
                ));
            }
        }

        // Sort the fractions
        fractions.sort_by(|a, b| {
            a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        });

        // Return the kth smallest fraction
        let (_, numerator, denominator) =
            fractions[(k - 1) as usize];
        vec![numerator, denominator]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        assert_eq!(
            Solution::kth_smallest_prime_fraction(arr, k),
            vec![2, 5]
        );
    }

    #[test]
    fn test_example2() {
        let arr = vec![1, 7];
        let k = 1;
        assert_eq!(
            Solution::kth_smallest_prime_fraction(arr, k),
            vec![1, 7]
        );
    }
}
