from typing import List


def two_sum(nums: List[int], target: int) -> List[int]:
    checked_elements = {}
    for i, v in enumerate(nums):
        needed_v = target - v
        if needed_v in checked_elements:
            return [checked_elements[needed_v], i]
        checked_elements[v] = i
    return []


def _two_sum(nums, target):
    for i in range(len(nums)):
        for j in range(i + 1, len(nums)):
            if nums[i] + nums[j] == target:
                return [i, j]
    return []


# Tests
def test_ex1():
    result = two_sum([2, 7, 11, 15], 9)
    assert result == [0, 1] or result == [1, 0]


def test_ex2():
    res = two_sum([3, 2, 4], 6)
    assert res == [1, 2] or res == [2, 1]


def test_ex3():
    res = two_sum([3, 3], 6)
    assert res == [0, 1] or res == [1, 0]
