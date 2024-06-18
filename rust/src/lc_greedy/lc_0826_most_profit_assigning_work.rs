pub struct Solution;

#[derive(Debug, Clone, Copy)]
struct Job {
    d: i32,
    p: i32,
}

impl Solution {
    // O(n log n + m log m) time | O(n) space
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut jobs: Vec<Job> = difficulty
            .into_iter()
            .zip(profit.into_iter())
            .map(|(d, p)| Job { d, p })
            .collect();
        jobs.sort_unstable_by_key(|job| job.p);
        worker.sort_unstable_by_key(|&w| -w);

        let mut total = 0;
        for w in worker {
            while jobs.last().is_some_and(|&j| j.d > w) {
                jobs.pop();
            }
            if let Some(&job) = jobs.last() {
                total += job.p;
            } else {
                return total;
            }
        }
        total
    }
}

#[test]
fn test_max_profit_assignment_example1() {
    let difficulty = vec![2, 4, 6, 8, 10];
    let profit = vec![10, 20, 30, 40, 50];
    let worker = vec![4, 5, 6, 7];
    let expected_output = 100;
    let actual_output = Solution::max_profit_assignment(
        difficulty, profit, worker,
    );
    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_max_profit_assignment_example2() {
    let difficulty = vec![85, 47, 57];
    let profit = vec![24, 66, 99];
    let worker = vec![40, 25, 25];
    let expected_output = 0;
    let actual_output = Solution::max_profit_assignment(
        difficulty, profit, worker,
    );
    assert_eq!(expected_output, actual_output);
}
