pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn max_satisfied(
        customers: Vec<i32>,
        grumpy: Vec<i32>,
        minutes: i32,
    ) -> i32 {
        let (
            mut left,
            mut current_window,
            mut max_window,
            mut satisfied,
        ) = (0, 0, 0, 0);

        for right in 0..customers.len() {
            if grumpy[right] == 1 {
                current_window += customers[right]
            } else {
                satisfied += customers[right]
            }

            if right - left + 1 > minutes as usize {
                if grumpy[left] == 1 {
                    current_window -= customers[left]
                }
                left += 1
            }
            max_window = max_window.max(current_window);
        }
        satisfied + max_window
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_satisfied_example1() {
        let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
        let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
        let minutes = 3;
        let expected_output = 16;
        let actual_output = Solution::max_satisfied(
            customers, grumpy, minutes,
        );
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_max_satisfied_example2() {
        let customers = vec![1];
        let grumpy = vec![0];
        let minutes = 1;
        let expected_output = 1;
        let actual_output = Solution::max_satisfied(
            customers, grumpy, minutes,
        );
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_max_satisfied_example3() {
        let customers = vec![4, 10, 10];
        let grumpy = vec![1, 1, 0];
        let minutes = 2;
        let expected_output = 24;
        let actual_output = Solution::max_satisfied(
            customers, grumpy, minutes,
        );
        assert_eq!(expected_output, actual_output);
    }
}
