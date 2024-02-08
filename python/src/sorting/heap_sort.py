from typing import Any, List


# O(n log(n)) time
# o(1) space
def heap_sort(array: List[Any]):
    build_max_heap(array)
    for end_idx in reversed(range(1, len(array))):
        swap(0, end_idx, array)
        sift_down(0, end_idx - 1, array)

    return array


def build_max_heap(array: List[Any]):
    first_parent_idx = (len(array) - 1) // 2
    for current_idx in reversed(
        range(first_parent_idx + 1)
    ):
        sift_down(current_idx, len(array) - 1, array)


def sift_down(current_idx, end_idx, heap):
    child_one_idx = current_idx * 2 + 1

    while child_one_idx <= end_idx:
        child_two_idx = (
            current_idx * 2 + 2
            if current_idx * 2 + 2 <= end_idx
            else -1
        )

        if (
            child_two_idx > -1
            and heap[child_two_idx] > heap[child_one_idx]
        ):
            idx_to_swap = child_two_idx
        else:
            idx_to_swap = child_one_idx

        if heap[idx_to_swap] > heap[current_idx]:
            swap(current_idx, idx_to_swap, heap)
            current_idx = idx_to_swap
            child_one_idx = current_idx * 2 + 1
        else:
            return


def swap(i, j, array):
    array[i], array[j] = array[j], array[i]


# Test
def test_heap_sort_ex1():
    result = heap_sort([4, 6, 1, 8, 11, 13, 3])
    assert result == [1, 3, 4, 6, 8, 11, 13]
