from typing import Any, List


# [5, 2, [7, -1], 3, [6, [-13, 8], 4]]
# 12 // calculated as: 5 + 2 + 2 * (7 - 1) + 3 + 2 * (6 + 3 * (13 + 8) + 4)
# O(n) time | O(d) space where d is the depth
def product_sum(
    array: List[Any], multiplier: int = 1
) -> int:
    sum = 0
    for element in array:
        # if type(element) is list:
        if isinstance(element, list):
            sum += product_sum(element, multiplier + 1)
        else:
            sum += element
    return sum * multiplier


# Test
def test_ex1():
    result = product_sum(
        [5, 2, [7, -1], 3, [6, [-13, 8], 4]]
    )
    assert result == 12
