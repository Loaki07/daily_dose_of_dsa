pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut hm = HashMap::with_capacity(nums.len());

    for (i, v) in nums.iter().enumerate() {
        let needed_v = target - v;

        match hm.get(&needed_v) {
            Some(val) => return vec![i as i32, *val],
            None => {
                hm.insert(*v, i as i32);
            }
        }
    }
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert!(
            result == vec![0, 1] || result == vec![1, 0]
        );
    }

    #[test]
    fn ex2() {
        let res = two_sum(vec![3, 2, 4], 6);
        assert!(res == vec![1, 2] || res == vec![2, 1]);
    }

    #[test]
    fn ex3() {
        let res = two_sum(vec![3, 3], 6);
        assert!(res == vec![0, 1] || res == vec![1, 0]);
    }
}
