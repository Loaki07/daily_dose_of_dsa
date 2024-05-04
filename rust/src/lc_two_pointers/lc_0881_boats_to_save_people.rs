pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(
        mut people: Vec<i32>,
        limit: i32,
    ) -> i32 {
        people.sort_unstable();

        let mut res = 0;
        let (mut l, mut r) = (0, people.len() - 1);

        while l < r {
            let remain = limit - people[r];
            if l <= r && remain >= people[l] {
                l += 1;
                r -= 1;
            } else {
                r -= 1;
            }
            res += 1;
        }

        if l == r {
            res += 1;
        }

        res
    }
}

#[test]
fn example1_test() {
    let people = vec![1, 2];
    let limit = 3;
    assert_eq!(
        Solution::num_rescue_boats(people, limit),
        1
    );
}

#[test]
fn example2_test() {
    let people = vec![3, 2, 2, 1];
    let limit = 3;
    assert_eq!(
        Solution::num_rescue_boats(people, limit),
        3
    );
}

#[test]
fn example3_test() {
    let people = vec![3, 5, 3, 4];
    let limit = 5;
    assert_eq!(
        Solution::num_rescue_boats(people, limit),
        4
    );
}
