pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut max_area, mut left, mut right) =
            (0, 0, height.len() - 1);

        while left < right {
            let area = (right - left) as i32
                * height[left].min(height[right]);
            max_area = area.max(max_area);

            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;
        assert_eq!(Solution::max_area(height), expected);
    }

    #[test]
    fn test_example_2() {
        let height = vec![1, 1];
        let expected = 1;
        assert_eq!(Solution::max_area(height), expected);
    }
}
