use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut checked_elements: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, v) in nums.iter().enumerate() {
            let needed_v = target - v;

            match checked_elements.get(&v) {
                Some(&val) => return vec![i as i32, val],
                None => {
                    checked_elements.insert(needed_v, i as i32);
                }
            }
        }
        vec![]
    }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 0])
    }

    #[test]
    fn ex2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![2, 1])
    }

    #[test]
    fn ex3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![1, 0]);
    }
}