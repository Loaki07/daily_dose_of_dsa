/// Sort a mutable slice using heap sort.
///
/// Heap sort is an in-place O(n log n) sorting
/// algorithm. It is based on a max heap, a binary
/// tree data structure whose main feature is that
/// parent nodes are always greater or equal to
/// their child nodes.
///
/// # Max Heap Implementation
///
/// A max heap can be efficiently implemented with
/// an array. For example, the binary tree:
/// ```text
///     1
///  2     3
/// 4 5   6 7
/// ```
///
/// ... is represented by the following array:
/// ```text
/// 1 23 4567
/// ```
///
/// Given the index `i` of a node, parent and
/// child indices can be calculated as follows:
/// ```text
/// parent(i)      = (i-1) / 2
/// left_child(i)  = 2*i + 1
/// right_child(i) = 2*i + 2
/// ```

/// # Algorithm
///
/// Heap sort has two steps:
///   1. Convert the input array to a max heap.
///   2. Partition the array into heap part and
///      sorted part. Initially the heap consists
///      of the whole array and the sorted part is
///      empty: ```text arr: [ heap |] ```
///
///      Repeatedly swap the root (i.e. the
///      largest) element of the heap with
///      the last element of the heap and increase
///      the sorted part by one:      
///     ```text
///      arr: [ root ...   last | sorted ]
///       --> [ last ... | root   sorted ]
///     ```
///     ```
///      After each swap, fix the heap to make it
///      a valid max heap again.      Once the
/// heap is      empty, `arr` is completely
/// sorted.
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // already sorted
    }

    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
}

/// Convert `arr` into a max heap
fn heapify<T: Ord>(arr: &mut [T]) {
    let last_parent = (arr.len() - 1 - 1) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

/// Move the element `root` down until `arr` is a
/// max heap again.
///
/// This assumes that the subtrees under `root`
/// are valid max heaps already.
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

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    heap_sort(&mut nums);
    nums[nums.len() - k as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13, 19]);

        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
    }

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
