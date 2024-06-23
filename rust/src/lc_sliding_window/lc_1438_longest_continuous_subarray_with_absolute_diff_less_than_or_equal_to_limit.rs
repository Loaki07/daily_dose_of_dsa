use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    // O(n) time | O(n) space
    pub fn longest_subarray(
        nums: Vec<i32>,
        limit: i32,
    ) -> i32 {
        struct Item {
            num: i32,
            j: usize,
        }

        let mut min_queue = VecDeque::<Item>::new();
        let mut max_queue = VecDeque::<Item>::new();
        let mut max_array = 0;

        let mut i = 0;
        for (j, num) in nums.into_iter().enumerate() {
            // Adjust queue to ensure it remains ascending
            // in `num` value and index
            while min_queue
                .back()
                .map_or(false, |min| min.num > num)
            {
                min_queue.pop_back();
            }

            // Adjust queue to ensure it remains descending
            // in `num` value, but ascending in
            // index
            while max_queue
                .back()
                .map_or(false, |max| max.num < num)
            {
                max_queue.pop_back();
            }

            // Add the current value to the end of each
            // queue.
            min_queue.push_back(Item { num, j });
            max_queue.push_back(Item { num, j });

            // while window condition fails, move `i`
            // forward until enough items leave
            // the window this decreasing the abs diff
            while max_queue[0].num - min_queue[0].num
                > limit
            {
                let min_j = min_queue[0].j;
                let max_j = max_queue[0].j;

                // the next item that may reduce diff.
                i = min_j.min(max_j);

                if min_j == i {
                    min_queue.pop_front();
                }
                if max_j == i {
                    max_queue.pop_front();
                }
                i += 1;
            }
            max_array = max_array.max(j - i + 1);
        }
        max_array as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        let nums = vec![8, 2, 4, 7];
        let limit = 4;
        let result =
            Solution::longest_subarray(nums, limit);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example2() {
        let nums = vec![10, 1, 2, 4, 7, 2];
        let limit = 5;
        let result =
            Solution::longest_subarray(nums, limit);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example3() {
        let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
        let limit = 0;
        let result =
            Solution::longest_subarray(nums, limit);
        assert_eq!(result, 3);
    }
}
