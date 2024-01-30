from typing import List


# O(n log (n)) time
# O(1) space
# works only for a sorted array
def two_sum_for_sorted_array(
    nums: List[int], target: int
) -> List[int]:
    nums.sort()
    left = 0
    right = len(nums) - 1

    while left < right:
        current_sum = nums[left] + nums[right]

        if current_sum == target:
            return [left, right]
        elif current_sum < target:
            left += 1
        elif current_sum > target:
            right -= 1

    return []


# O(n) time
# O(n) space
def two_sum(nums: List[int], target: int) -> List[int]:
    checked_elements = {}
    for i, v in enumerate(nums):
        needed_v = target - v
        if needed_v in checked_elements:
            return [checked_elements[needed_v], i]
        checked_elements[v] = i
    return []


# O(n^2) time
# O(1) space
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
