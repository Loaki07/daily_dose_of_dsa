from typing import Any, List


def selection_sort(array: List[Any]) -> List[Any]:
    current_idx = 0

    while current_idx < len(array) - 1:
        smallest_idx = current_idx

        for i in range(current_idx + 1, len(array)):
            if array[smallest_idx] > array[i]:
                smallest_idx = i
        swap(current_idx, smallest_idx, array)

        current_idx += 1

    return array


def swap(i, j, array):
    array[i], array[j] = array[j], array[i]


# Test
def test_selection_sort_ex1():
    result = selection_sort([4, 6, 1, 8, 11, 13, 3])
    assert result == [1, 3, 4, 6, 8, 11, 13]
