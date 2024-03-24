pub struct Solution;

impl Solution {
    pub fn car_fleet(
        target: i32,
        position: Vec<i32>,
        speed: Vec<i32>,
    ) -> i32 {
        let mut position_speed_pair: Vec<(f64, f64)> =
            position
                .iter()
                .map(|x| *x as f64)
                .zip(speed.iter().map(|x| *x as f64))
                .collect();

        position_speed_pair
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut stack = vec![];

        for (pos, speed) in position_speed_pair.iter().rev()
        {
            stack.push((target as f64 - pos) / speed);

            if stack.len() >= 2
                && stack.last()
                    <= stack.get(stack.len() - 2)
            {
                stack.pop();
            }
        }

        stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::car_fleet(
                12,
                vec![10, 8, 0, 5, 3],
                vec![2, 4, 1, 1, 3]
            ),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::car_fleet(10, vec![3], vec![3]),
            1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::car_fleet(
                100,
                vec![0, 2, 4],
                vec![4, 2, 1]
            ),
            1
        );
    }
}
