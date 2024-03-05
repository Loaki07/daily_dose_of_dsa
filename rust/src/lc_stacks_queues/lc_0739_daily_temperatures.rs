pub struct Solution;

impl Solution {
    pub fn daily_temperatures(
        temperatures: Vec<i32>,
    ) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = vec![];

        for (i, val) in temperatures.iter().enumerate() {
            while !stack.is_empty()
                && *val > stack.last().unwrap().0
            {
                let (_, stack_index) = stack.pop().unwrap();
                res[stack_index] = (i - stack_index) as i32;
            }

            stack.push((*val, i));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let temperatures =
            vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        let result =
            Solution::daily_temperatures(temperatures);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let temperatures = vec![30, 40, 50, 60];
        let expected = vec![1, 1, 1, 0];
        let result =
            Solution::daily_temperatures(temperatures);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_3() {
        let temperatures = vec![30, 60, 90];
        let expected = vec![1, 1, 0];
        let result =
            Solution::daily_temperatures(temperatures);
        assert_eq!(result, expected);
    }
}
