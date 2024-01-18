use std::cmp::Ordering;

// Time Complexity:

// HashMap Creation: O(N), where N is the length
// of nums. Each insertion or update operation in
// the hash map is O(1), and you do this for every
// element. Vector Creation from HashMap: O(N).
// Converting a hash map into a vector.
// Quick Select (select_nth_unstable_by_key): The
// average time complexity is O(N), but the
// worst-case is O(N²). However, the worst-case is
// rare. Iterating and Collecting Results: O(K),
// where K is the number of top frequent elements
// you are selecting. Overall, the average time
// complexity is O(N), but the worst-case could go
// up to O(N²) due to the quick select step.

// Space Complexity:

// O(N) for the HashMap.
// O(N) for the vector created from HashMap.
// No additional significant space is used.
// So, the total space complexity is O(N).
// built in rust quick select function
// using quick select, we should ideally be able
// to get O(n) time complexity

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut hm = HashMap::new();

    for val in nums.iter() {
        hm.entry(*val)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut count_vec =
        hm.into_iter().collect::<Vec<(i32, i32)>>();

    let res = match (k as usize).cmp(&count_vec.len()) {
        Ordering::Equal => &count_vec,
        _ => quick_select(&mut count_vec, k),
    };

    res.into_iter().map(|(num, _count)| *num).collect()
}

pub fn top_k_frequent_built_in_quick_select(
    nums: Vec<i32>,
    k: i32,
) -> Vec<i32> {
    use std::collections::HashMap;

    let mut hm = HashMap::new();

    for val in nums.iter() {
        hm.entry(val)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut count_vec = hm.into_iter().collect::<Vec<_>>();
    let len = count_vec.len();

    count_vec.select_nth_unstable_by_key(
        len - k as usize,
        |(_num, count)| *count,
    );
    count_vec
        .into_iter()
        .rev()
        .take(k as usize)
        .map(|(num, _count)| *num)
        .collect::<Vec<i32>>()
}

// Time Complexity:

// HashMap Creation: O(N), same as above.
// Creating Binary Heap: O(N log N). Inserting N
// elements into a binary heap costs O(log N)
// each. Sorting the Binary Heap: O(N log N). The
// into_sorted_vec function sorts the heap.
// Iterating and Collecting Results: O(K), similar
// to the first method. The total time complexity
// is O(N log N) due to the heap operations.

// Space Complexity:

// O(N) for the HashMap.
// O(N) for the Binary Heap.
// So, the total space complexity is also O(N).
// Soln using binary heap
// O(k log n) complexity
pub fn _top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
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

pub fn quick_select(
    // (num, count)
    slice: &mut [(i32, i32)],
    k: i32,
) -> &[(i32, i32)] {
    let (mut pivot, mut i, mut j) = (0, 1, 1);

    for index in 1..slice.len() {
        if slice[index].1 >= slice[pivot].1 {
            slice.swap(index, j);
        } else {
            slice.swap(index, i);
            i += 1;
        }
        j += 1;
    }

    slice.swap(pivot, i - 1);
    pivot = i - 1;

    let larger_items = (j - pivot) as i32;

    match larger_items.cmp(&k) {
        Ordering::Less => quick_select(&mut slice[0..j], k),
        Ordering::Greater => {
            quick_select(&mut slice[pivot + 1..j], k)
        }
        Ordering::Equal => &slice[pivot..j],
    }
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
