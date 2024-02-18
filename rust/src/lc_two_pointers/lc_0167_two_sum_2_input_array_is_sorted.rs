pub struct Solution;

impl Solution {
    pub fn two_sum(
        numbers: Vec<i32>,
        target: i32,
    ) -> Vec<i32> {
        use std::cmp::Ordering::{Equal, Greater, Less};

        let (mut l, mut r) = (0, numbers.len() - 1);

        while l < r {
            match (numbers[l] + numbers[r]).cmp(&target) {
                Less => l += 1,
                Greater => r -= 1,
                Equal => {
                    // +1 becase the lc problem has index
                    // starting with 1
                    return vec![
                        l as i32 + 1,
                        r as i32 + 1,
                    ];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(
            Solution::two_sum(numbers, target),
            vec![1, 2]
        );
    }

    #[test]
    fn test_example_2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        assert_eq!(
            Solution::two_sum(numbers, target),
            vec![1, 3]
        );
    }

    #[test]
    fn test_example_3() {
        let numbers = vec![-1, 0];
        let target = -1;
        assert_eq!(
            Solution::two_sum(numbers, target),
            vec![1, 2]
        );
    }
}
