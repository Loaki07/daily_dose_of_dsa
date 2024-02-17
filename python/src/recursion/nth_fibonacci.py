from typing import Dict


# Approach 3
# iterative
# O(n) time | O(1) space
def nth_fibonacci(n: int) -> int:
    last_two = [0, 1]
    counter = 3
    while counter <= n:
        next_fib = last_two[0] + last_two[1]
        last_two[0] = last_two[1]
        last_two[1] = next_fib
        counter += 1
    return last_two[1] if n > 1 else last_two[0]


# Approach 2
# memoization
# O(n) time | O(n) space
def nth_fibonacci_2(
    n: int, memoize: Dict[int, int] = {1: 0, 2: 1}
) -> int:
    if n in memoize:
        return memoize[n]
    else:
        memoize[n] = nth_fibonacci(
            n - 1, memoize
        ) + nth_fibonacci(n - 2, memoize)
        return memoize[n]


# Approach 1
# basic recursion
# O(2^n) time | O(n) space
def nth_fibonacci_1(n: int) -> int:
    if n == 2:
        return 1
    elif n == 1:
        return 0
    else:
        return nth_fibonacci(n - 1) + nth_fibonacci(n - 2)


# Test
def test_ex1():
    result = nth_fibonacci(1)
    assert result == 0


def test_ex2():
    result = nth_fibonacci(3)
    assert result == 1


def test_ex3():
    result = nth_fibonacci(4)
    assert result == 2
