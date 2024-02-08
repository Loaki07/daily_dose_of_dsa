from typing import Any, List


# Best: O(n Log(n)) time | O(Log(n)) space
def quick_sort(array: List[Any]) -> List[Any]:
    quick_sort_helper(
        array=array, start_idx=0, end_idx=len(array) - 1
    )
    return array


def quick_sort_helper(array: List[Any], start_idx, end_idx):
    if start_idx >= end_idx:
        return

    pivot_idx = start_idx
    left_idx = start_idx + 1
    right_idx = end_idx

    while right_idx >= left_idx:
        if (
            array[left_idx] > array[pivot_idx]
            and array[right_idx] < array[pivot_idx]
        ):
            swap(left_idx, right_idx, array)

        if array[left_idx] <= array[pivot_idx]:
            left_idx += 1

        if array[right_idx] >= array[pivot_idx]:
            right_idx -= 1

    swap(pivot_idx, right_idx, array)

    left_subarray_is_smaller: bool = (
        right_idx - 1 - start_idx
        < end_idx - (right_idx + 1)
    )

    if left_subarray_is_smaller:
        quick_sort_helper(
            array=array,
            start_idx=start_idx,
            end_idx=right_idx - 1,
        )
        quick_sort_helper(
            array=array,
            start_idx=right_idx + 1,
            end_idx=end_idx,
        )
    else:
        quick_sort_helper(
            array=array,
            start_idx=right_idx + 1,
            end_idx=end_idx,
        )
        quick_sort_helper(
            array=array,
            start_idx=start_idx,
            end_idx=right_idx - 1,
        )


def swap(i, j, array):
    array[i], array[j] = array[j], array[i]


# Test
def test_quick_sort_ex1():
    result = quick_sort([4, 6, 1, 8, 11, 13, 3])
    assert result == [1, 3, 4, 6, 8, 11, 13]
