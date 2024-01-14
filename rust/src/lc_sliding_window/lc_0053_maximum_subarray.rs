pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best = i32::MIN;

        let mut curr_sum = 0;
        for n in nums {
            curr_sum = n.max(curr_sum + n);
            best = best.max(curr_sum);
        }
        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::max_sub_array(vec![
                -2, 1, -3, 4, -1, 2, 1, -5, 4
            ]),
            6
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::max_sub_array(vec![5, 4, -1, 7, 8]),
            23
        );
    }
}
