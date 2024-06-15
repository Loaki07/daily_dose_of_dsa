use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    // O(n log n) time | O(n) space
    pub fn find_maximized_capital(
        k: i32,
        w: i32,
        profits: Vec<i32>,
        capital: Vec<i32>,
    ) -> i32 {
        let (mut pq1, mut pq2) =
            (BinaryHeap::new(), BinaryHeap::new());
        let (mut count, mut w) = (0, w);

        for i in 0..capital.len() {
            // pq1 is min heap
            pq1.push(Reverse((capital[i], i)));
        }

        while pq1.is_empty() == false {
            while let Some(Reverse((c, i))) = pq1.peek() {
                if *c > w {
                    break;
                }

                // pq2 is a max heap
                pq2.push(profits[*i]);
                pq1.pop();
            }

            if let Some(p) = pq2.pop() {
                w += p;
                count += 1;
                if count == k {
                    break;
                }
            } else {
                break;
            }
        }

        while let Some(p) = pq2.pop() {
            if count == k {
                break;
            }
            count += 1;
            w += p;
        }

        w
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        let k = 2;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 1];
        let result = Solution::find_maximized_capital(
            k, w, profits, capital,
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example2() {
        let k = 3;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 2];
        let result = Solution::find_maximized_capital(
            k, w, profits, capital,
        );
        assert_eq!(result, 6);
    }
}
