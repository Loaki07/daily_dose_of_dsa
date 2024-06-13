use std::iter::repeat;

pub struct Solution;

impl Solution {
    pub fn min_moves_to_seat(
        seats: Vec<i32>,
        students: Vec<i32>,
    ) -> i32 {
        let mut seat_counts = [0; 101];
        let mut stud_counts = [0; 101];

        seats
            .into_iter()
            .for_each(|s| seat_counts[s as usize] += 1);
        students
            .into_iter()
            .for_each(|s| stud_counts[s as usize] += 1);

        let seats = (0i32..)
            .zip(seat_counts)
            .flat_map(|(s, c)| repeat(s).take(c));
        let studs = (0i32..)
            .zip(stud_counts)
            .flat_map(|(s, c)| repeat(s).take(c));

        seats
            .zip(studs)
            .map(|(seat, stud)| seat.abs_diff(stud) as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        let seats = vec![3, 1, 5];
        let students = vec![2, 7, 4];
        assert_eq!(
            Solution::min_moves_to_seat(seats, students),
            4
        );
    }

    #[test]
    fn test_example2() {
        let seats = vec![4, 1, 5, 9];
        let students = vec![1, 3, 2, 6];
        assert_eq!(
            Solution::min_moves_to_seat(seats, students),
            7
        );
    }

    #[test]
    fn test_example3() {
        let seats = vec![2, 2, 6, 6];
        let students = vec![1, 3, 2, 6];
        assert_eq!(
            Solution::min_moves_to_seat(seats, students),
            4
        );
    }
}
