use std::collections::{HashMap, HashSet};

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

// for an unsorted array
pub fn find_duplicates_set(nums: Vec<i32>) -> Vec<i32> {
    let mut hm = HashMap::<i32, i32>::new();
    let mut result = HashSet::<i32>::new();
    if nums.len() == 0 {
        return vec![];
    }

    for i in nums {
        let v = hm.entry(i).or_insert(0);
        *v += 1;
        if *v > 1 {
            result.insert(i);
        }
    }

    result.into_iter().collect::<Vec<i32>>()
}

pub fn find_duplicates_allocate(
    nums: Vec<i32>,
) -> Vec<i32> {
    let mut arr = vec![0; nums.len()];
    let mut result = HashSet::<i32>::new();
    if nums.len() == 0 {
        return vec![];
    }

    for i in nums {
        if i <= 0 {
            panic!("Invalid number for problem");
        }

        let a = &mut arr[(i - 1) as usize];
        *a += 1;
        if *a > 1 {
            result.insert(i);
        }
    }

    result.into_iter().collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_to_find_duplicates_with_empty() {
        assert_eq!(find_duplicates(vec![]), vec![]);
        assert_eq!(find_duplicates_set(vec![]), vec![]);
        assert_eq!(
            find_duplicates_allocate(vec![]),
            vec![]
        );
    }

    #[test]
    fn ex1() {
        assert_eq!(
            find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
                .sort(),
            vec![2, 3].sort()
        );
        assert_eq!(
            find_duplicates_set(vec![
                4, 3, 2, 7, 8, 2, 3, 1
            ])
            .sort(),
            vec![2, 3].sort()
        );
        assert_eq!(
            find_duplicates_allocate(vec![
                4, 3, 2, 7, 8, 2, 3, 1
            ])
            .sort(),
            vec![2, 3].sort()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1]);
        assert_eq!(
            find_duplicates_set(vec![1, 1, 2]),
            vec![1]
        );
        assert_eq!(
            find_duplicates_allocate(vec![1, 1, 2]),
            vec![1]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(find_duplicates(vec![1]), vec![]);
        assert_eq!(find_duplicates_set(vec![1]), vec![]);
        assert_eq!(
            find_duplicates_allocate(vec![1]),
            vec![]
        );
    }
}
