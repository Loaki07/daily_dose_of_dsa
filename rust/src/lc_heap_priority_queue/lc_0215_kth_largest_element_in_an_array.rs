use core::num;

pub fn partition(
    list: &mut [i32],
    left: usize,
    right: usize,
    pivot_index: usize,
) -> usize {
    let pivot_value = list[pivot_index];
    list.swap(pivot_index, right);
    let mut store_index = left;
    for i in left..right {
        if list[i] < pivot_value {
            list.swap(store_index, i);
            store_index += 1;
        }
    }
    list.swap(right, store_index);
    store_index
}

pub fn quick_select(
    list: &mut [i32],
    left: usize,
    right: usize,
    k: usize,
) -> i32 {
    if left == right {
        return list[left];
    }

    let mut pivot_index = left + (right - left) / 2;
    pivot_index = partition(list, left, right, pivot_index);
    match k {
        x if x == pivot_index => list[x],
        x if x < pivot_index => {
            quick_select(list, left, pivot_index - 1, k)
        }
        _ => quick_select(list, pivot_index + 1, right, k),
    }
}

// using quick select algo
pub fn find_kth_largest_1(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    quick_select(&mut nums, 0, len - 1, len - k as usize)
}

// using built in BinaryHeap
// construct a min Heap
pub fn find_kth_largest_2(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let capacity = k as usize;

    // preallocate memory for the capacity k+1, so it
    // doesn't need to be reallocated while function
    // is called
    let mut max_heap: BinaryHeap<Reverse<i32>> =
        BinaryHeap::with_capacity(capacity + 1);

    for num in nums {
        // Reverse is used because we need a MinHeap,
        // instead of a default behavior
        max_heap.push(Reverse(num));

        if max_heap.len() > capacity {
            max_heap.pop();
        }
    }

    // peek the top of the heap, unwrap Reversed value
    // from Option, access the actual value from
    // Reverse via .0
    max_heap.peek().unwrap().0
}

pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    // build a max heap
    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
}

// Convert `arr` into a max heap
fn heapify<T: Ord>(arr: &mut [T]) {
    let last_parent = (arr.len() - 1) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

// Move the element at `root` down until `arr` is
// a max heap again.
//
// this assumes that the subtrees under `root` are
// valid max heaps already.
fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;

    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }

        let right = left + 1;

        let max = if right <= last && arr[right] > arr[left]
        {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}

// using heap sort
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    heap_sort(&mut nums);
    nums[nums.len() - k as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_kth_largest_ex1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        assert_eq!(find_kth_largest(nums, k), 5);
    }

    #[test]
    fn test_for_kth_largest_ex2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        assert_eq!(find_kth_largest(nums, k), 4);
    }
}
