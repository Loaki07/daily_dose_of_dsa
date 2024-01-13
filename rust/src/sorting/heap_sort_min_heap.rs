/// Convert `arr` into a min heap.
fn heapify<T: Ord>(arr: &mut [T]) {
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

/// Move the element at `root` down until `arr` is
/// a min heap again.
///
/// This assumes that the subtrees under `root`
/// are valid min heaps already.
fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        let right = left + 1;
        let mut smallest = root;

        if left <= last && arr[left] < arr[smallest] {
            smallest = left;
        }
        if right <= last && arr[right] < arr[smallest] {
            smallest = right;
        }

        // If the root is smaller than both of its
        // children, we're done.
        if smallest == root {
            break;
        }

        // Swap the smallest child with the root.
        arr.swap(root, smallest);

        // Continue sifting down from the child's
        // position.
        root = smallest;
    }
}

/// Perform heap sort using the min heap.
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // already sorted
    }

    // First, turn the array into a min heap.
    heapify(arr);

    // Then, repeatedly swap the root of the heap with
    // the last element, reducing the heap size by
    // one each time, and sift down the new root to
    // maintain the heap property. This effectively
    // sorts the array in place.
    for end in (1..arr.len()).rev() {
        arr.swap(0, end); // Swap the minimum element to its correct
                          // position
        move_down(&mut arr[..end], 0); // Restore the min heap property
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort_min_heap_decending_order() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        heap_sort(&mut v);
        assert_eq!(v, vec![19, 13, 11, 8, 6, 4, 3, 1]);

        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        heap_sort(&mut v);
        assert_eq!(v, vec![14, 13, 12, 9, 7, 6, 2, 1]);
    }
}
