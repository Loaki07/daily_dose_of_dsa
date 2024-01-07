pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        for n in nums {
            if !set.insert(n) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(Solution::contains_duplicate(vec![
            1, 2, 3, 1
        ]))
    }
}
