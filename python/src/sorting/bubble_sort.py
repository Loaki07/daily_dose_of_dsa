from typing import Any, List


# O(n^2) time
# O(1) space
def bubble_sort(array: List[Any]) -> List[Any]:
    is_sorted = False
    counter = 0

    while not is_sorted:
        is_sorted = True
        for i in range(len(array) - 1 - counter):
            if array[i] > array[i + 1]:
                swap(i, i + 1, array)
                is_sorted = False

        counter += 1
    return array


def swap(i, j, array):
    array[i], array[j] = array[j], array[i]


# Test
def test_bubble_sort_ex1():
    result = bubble_sort([4, 6, 1, 8, 11, 13, 3])
    assert result == [1, 3, 4, 6, 8, 11, 13]
