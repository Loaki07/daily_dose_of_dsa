from typing import Any, List


# O(n) time
# O(1) space
# 1 pass method
def three_number_sort(
    array: List[Any], order: List[Any]
) -> List[Any]:
    first_value = order[0]
    second_value = order[1]

    # keep track of the indicies where the values are stored
    first_idx, second_idx, third_idx = 0, 0, len(array) - 1

    while second_idx <= third_idx:
        value = array[second_idx]

        if value == first_value:
            array[second_idx], array[first_idx] = (
                array[first_idx],
                array[second_idx],
            )
            first_idx += 1
            second_idx += 1

        elif value == second_value:
            second_idx += 1

        else:
            array[second_idx], array[third_idx] = (
                array[third_idx],
                array[second_idx],
            )
            third_idx -= 1

    return array


# O(n) time
# O(1) space
# 2 pass method
def three_number_sort_2(
    array: List[Any], order: List[Any]
) -> List[Any]:
    first_value = order[0]
    third_value = order[2]

    first_idx = 0
    for idx in range(len(array)):
        if array[idx] == first_value:
            array[first_idx], array[idx] = (
                array[idx],
                array[first_idx],
            )
            first_idx += 1

    third_idx = len(array) - 1
    for idx in range(len(array) - 1, -1, -1):
        if array[idx] == third_value:
            array[third_idx], array[idx] = (
                array[idx],
                array[third_idx],
            )
            third_idx -= 1

    return array


# O(n) time
# O(1) space
# bucket sort
def three_number_sort_1(
    array: List[Any], order: List[Any]
) -> List[Any]:
    value_counts = [0, 0, 0]

    for element in array:
        order_idx = order.index(element)
        value_counts[order_idx] += 1

    for i in range(3):
        value = order[i]
        count = value_counts[i]

        num_elements_before = sum(value_counts[:i])
        for n in range(count):
            current_idx = num_elements_before + n
            array[current_idx] = value

    return array


# Test
def test_three_number_sort_1():
    result = three_number_sort_1(
        [1, 0, 0, -1, -1, 0, 1, 1], [0, 1, -1]
    )
    assert result == [0, 0, 0, 1, 1, 1, -1, -1]


def test_three_number_sort_2():
    result = three_number_sort_2(
        [1, 0, 0, -1, -1, 0, 1, 1], [0, 1, -1]
    )
    assert result == [0, 0, 0, 1, 1, 1, -1, -1]


def test_three_number_sort():
    result = three_number_sort(
        [1, 0, 0, -1, -1, 0, 1, 1], [0, 1, -1]
    )
    assert result == [0, 0, 0, 1, 1, 1, -1, -1]
