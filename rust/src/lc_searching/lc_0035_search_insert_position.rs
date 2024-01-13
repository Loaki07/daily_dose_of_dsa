pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match target.cmp(&nums[mid]) {
            std::cmp::Ordering::Less => right = mid,
            std::cmp::Ordering::Equal => return mid as i32,
            std::cmp::Ordering::Greater => left = mid + 1,
        }
    }

    left as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
