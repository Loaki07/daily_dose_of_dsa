pub struct Solution;

impl Solution {
    // O(n^2) time | O(1) space
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut ans = 0 as i32;
        let mut n = arr.len();

        for i in 0..n {
            let mut val = arr[i];
            for k in i + 1..n {
                val ^= arr[k];
                if val == 0 {
                    ans += (k - i) as i32;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let arr = vec![2, 3, 1, 6, 7];
        let result = Solution::count_triplets(arr);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_2() {
        let arr = vec![1, 1, 1, 1, 1];
        let result = Solution::count_triplets(arr);
        assert_eq!(result, 10);
    }
}
