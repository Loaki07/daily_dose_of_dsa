use std::collections::HashMap;

// Approach 3
// iterative
// O(n) time | O(1) space
pub fn nth_fibonacci_3(n: u64) -> u64 {
    let mut last_two: Vec<u64> = vec![0, 1];
    let mut counter = 3;

    while counter <= n {
        let mut next_fib = last_two[0] + last_two[1];
        last_two[0] = last_two[1];
        last_two[1] = next_fib;
        counter += 1
    }

    if n > 1 {
        last_two[1]
    } else {
        last_two[0]
    }
}

// Approach 2
// memoization
// O(n) time | O(n) space
pub fn nth_fibonacci_2(
    n: u64,
    memoize: &mut HashMap<u64, u64>,
) -> u64 {
    if memoize.is_empty() {
        memoize.insert(1, 0);
        memoize.insert(2, 1);
    }

    match memoize.get(&n) {
        Some(&value) => value,
        None => {
            let res = nth_fibonacci_2(n - 1, memoize)
                + nth_fibonacci_2(n - 2, memoize);
            memoize.insert(n, res);
            res
        }
    }
}

// Approach 1
// basic recursion
// O(2^n) time | O(n) space
pub fn nth_fibonacci_1(n: u64) -> u64 {
    match n {
        2 => 1,
        1 => 0,
        _ => {
            nth_fibonacci_1(n - 1) + nth_fibonacci_1(n - 2)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let result = nth_fibonacci_1(1);
        assert_eq!(result, 0);
        let mut memoize: HashMap<u64, u64> = HashMap::new();
        let result = nth_fibonacci_2(1, &mut memoize);
        assert_eq!(result, 0);
        let result = nth_fibonacci_3(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn ex2() {
        let result = nth_fibonacci_1(3);
        assert_eq!(result, 1);
        let mut memoize: HashMap<u64, u64> = HashMap::new();
        let result = nth_fibonacci_2(3, &mut memoize);
        assert_eq!(result, 1);
        let result = nth_fibonacci_3(3);
        assert_eq!(result, 1);
    }

    #[test]
    fn ex3() {
        let result = nth_fibonacci_1(4);
        assert_eq!(result, 2);
        let mut memoize: HashMap<u64, u64> = HashMap::new();
        let result = nth_fibonacci_2(4, &mut memoize);
        assert_eq!(result, 2);
        let result = nth_fibonacci_3(4);
        assert_eq!(result, 2);
    }

    #[test]
    fn ex4() {
        let result = nth_fibonacci_1(2);
        assert_eq!(result, 1);
        let mut memoize: HashMap<u64, u64> = HashMap::new();
        let result = nth_fibonacci_2(2, &mut memoize);
        assert_eq!(result, 1);
        let result = nth_fibonacci_3(2);
        assert_eq!(result, 1);
    }
}
