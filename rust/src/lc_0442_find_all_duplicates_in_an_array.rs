// O(n^2)
pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if nums.len() == 0 {
        return result;
    }

    nums.sort();

    for i in 0..(nums.len() - 1) {
        if nums[i] == nums[i + 1] {
            if result.binary_search(&nums[i]).is_err() {
                result.push(nums[i]);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_to_find_duplicates_with_empty() {
        assert_eq!(find_duplicates(vec![]), vec![]);
    }

    #[test]
    fn ex1() {
        assert_eq!(
            find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1]);
    }

    #[test]
    fn ex3() {
        assert_eq!(find_duplicates(vec![1]), vec![]);
    }
}
