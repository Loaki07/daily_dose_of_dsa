from typing import Any, List


# O(n^2) time
# O(1) space
def insertion_sort(array: List[Any]) -> List[Any]:
    for i in range(1, len(array)):
        j = i
        while j > 0 and array[j] < array[j - 1]:
            swap(j, j - 1, array)
            j -= 1
    return array


def swap(i, j, array):
    array[i], array[j] = array[j], array[i]


# Test
def test_insertion_sort_ex1():
    result = insertion_sort([4, 6, 1, 8, 11, 13, 3])
    assert result == [1, 3, 4, 6, 8, 11, 13]
