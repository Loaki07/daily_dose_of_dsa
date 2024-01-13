/*
Function: partition
let pivot_value = list[pivot_index];

This line selects the pivot value from the list at the given pivot_index.
list.swap(pivot_index, right);

Swaps the pivot element with the last element of the current range. This step is part of the partitioning process.
let mut store_index = left;

Initializes store_index which will be used to find the right place for the pivot element.
for i in left..right { ... }

A loop that iterates over the array from left to right (exclusive). In each iteration, it compares the current element with the pivot value.
if list[i] < pivot_value { ... }

If the current element is less than the pivot value, it gets swapped with the element at store_index, and store_index is incremented.
list.swap(right, store_index);

After the loop, swap back the pivot to its correct position, which is now store_index.
store_index

Returns the final position of the pivot element, which is now correctly placed.
 */
fn partition(
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

/*
Function: quick_select
if left == right { return list[left]; }

Base case for recursion. If the range has only one element, return it since it's the only candidate for k-th largest.
let mut pivot_index = left + (right - left) / 2;

Chooses a pivot index, here it's the middle element of the range.
pivot_index = partition(list, left, right, pivot_index);

Calls partition with the chosen pivot index. This rearranges the elements and returns the new pivot index.
match k { ... }

A match statement to determine the next step based on the pivot's position relative to k.
x if x == pivot_index => list[k],

If k equals the pivot index, it means we've found the k-th smallest element. Return it.
x if x < pivot_index => quick_select(list, left, pivot_index - 1, k),

If k is less than the pivot index, recursively call quick_select on the left partition.
_ => quick_select(list, pivot_index + 1, right, k),

Otherwise, call it on the right partition.
 */
fn quick_select(
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
        x if x == pivot_index => list[k],
        x if x < pivot_index => {
            quick_select(list, left, pivot_index - 1, k)
        }

        _ => quick_select(list, pivot_index + 1, right, k),
    }
}

/*
Implementation in Solution
let mut nums = nums;

Creates a mutable copy of the input array nums.
let len = nums.len();

Stores the length of the array.
quick_select(&mut nums, 0, len - 1, len - k as usize)

Calls quick_select with the entire array, but adjusts the index to find the k-th largest element (len - k).
*/

/// Best Case: O(n)
/// Worst Case: O(n²)
/// Average Case: O(n)
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    quick_select(&mut nums, 0, len - 1, len - k as usize)
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
