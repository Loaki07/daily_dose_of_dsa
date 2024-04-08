pub struct Solution;

impl Solution {
    // O(n) time | O(1) space
    pub fn count_students(
        students: Vec<i32>,
        sandwiches: Vec<i32>,
    ) -> i32 {
        let (mut count, n) = (vec![0; 2], students.len());

        for s in students {
            count[s as usize] += 1;
        }

        for (i, &s) in sandwiches.iter().enumerate() {
            count[s as usize] -= 1;

            if count[s as usize] < 0 {
                return (n - i) as i32;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_students_example_1() {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];
        assert_eq!(
            Solution::count_students(students, sandwiches),
            0
        );
    }

    #[test]
    fn test_count_students_example_2() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];
        assert_eq!(
            Solution::count_students(students, sandwiches),
            3
        );
    }
}
