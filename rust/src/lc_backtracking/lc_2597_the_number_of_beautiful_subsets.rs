pub struct Solution;

impl Solution {
    pub fn beautiful_subsets(
        mut nums: Vec<i32>,
        k: i32,
    ) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        let mut visited = [false; 1000];
        Self::backtrack(
            &nums,
            k,
            0,
            &mut vec![],
            &mut visited,
            &mut result,
        );
        result
    }
    pub fn backtrack(
        nums: &Vec<i32>,
        k: i32,
        from: usize,
        path: &mut Vec<i32>,
        visited: &mut [bool],
        result: &mut i32,
    ) {
        if from == nums.len() {
            if !path.is_empty() {
                *result += 1;
            }
            return;
        }
        Self::backtrack(
            nums,
            k,
            from + 1,
            path,
            visited,
            result,
        );
        if !(nums[from] + k < 1001
            && visited
                [nums[from] as usize + k as usize - 1]
            || (nums[from] > k
                && visited
                    [nums[from] as usize - k as usize - 1]))
        {
            path.push(nums[from]);
            visited[nums[from] as usize - 1] = true;
            Self::backtrack(
                nums,
                k,
                from + 1,
                path,
                visited,
                result,
            );
            path.pop();
            visited[nums[from] as usize - 1] = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_beautiful_subsets_example_1() {
        let nums = vec![2, 4, 6];
        let k = 2;
        let result = Solution::beautiful_subsets(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_beautiful_subsets_example_2() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::beautiful_subsets(nums, k);
        assert_eq!(result, 1);
    }
}
