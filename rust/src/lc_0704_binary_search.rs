use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut is_asc = true;

    if nums.len() > 1 {
        is_asc = nums[0] < nums[nums.len() - 1]
    }

    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if is_asc {
            match target.cmp(&nums[mid]) {
                Ordering::Less => right = mid,
                Ordering::Equal => return mid as i32,
                Ordering::Greater => left = mid + 1,
            }
        } else {
            match target.cmp(&nums[mid]) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return mid as i32,
                Ordering::Greater => right = mid,
            }
        }
    }
    -1 // if not found return -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn ex3() {
        assert_eq!(search(vec![], 5), -1);
    }

    #[test]
    fn ex4() {
        assert_eq!(search(vec![5], -5), -1);
    }

    #[test]
    fn ex5() {
        assert_eq!(
            search(vec![-1, 0, 3, 5, 9, 12], 13),
            -1
        );
    }

    #[test]
    fn ex6() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 12), 5);
    }
}
