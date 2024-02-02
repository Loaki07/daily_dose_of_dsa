from typing import List

# O(n) time
# O(n) space
def contains_duplicate(nums: List[int]) -> bool:
    hashset = set()

    for n in nums:
        if n in hashset:
            return True
        hashset.add(n)
    return False

#Tests
def test_ex1():
    result = contains_duplicate([1, 2, 3, 1])
    assert result == True