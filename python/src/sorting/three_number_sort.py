from typing import Any, List


# O(n) time
# O(1) space
def three_number_sort(
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
def test_three_number_sort():
    result = three_number_sort(
        [1, 0, 0, -1, -1, 0, 1, 1], [0, 1, -1]
    )
    assert result == [0, 0, 0, 1, 1, 1, -1, -1]
