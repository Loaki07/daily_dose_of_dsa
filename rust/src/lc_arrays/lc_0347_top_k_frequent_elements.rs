// Soln using binary heap
// O(k log n) complexity
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::{BinaryHeap, HashMap};

    let mut hm: HashMap<i32, usize> = HashMap::new();

    for num in nums.iter() {
        hm.entry(*num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    hm.into_iter()
        .map(|(num, count)| (count, num))
        .collect::<BinaryHeap<(usize, i32)>>()
        .into_sorted_vec()
        .into_iter()
        .rev()
        .take(k as usize)
        .map(|(_, num)| num)
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let mut result = top_k_frequent(nums, k);
        result.sort();
        let mut expected_res = vec![1, 2];
        expected_res.sort();
        assert_eq!(result, expected_res);
    }

    #[test]
    fn ex2() {
        let nums = vec![1];
        let k = 1;
        let result = top_k_frequent(nums, k);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn ex3() {
        let nums = vec![-1, -1];
        let k = 1;
        let result = top_k_frequent(nums, k);
        assert_eq!(result, vec![-1]);
    }
}
