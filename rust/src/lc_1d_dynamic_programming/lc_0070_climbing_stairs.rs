pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        use std::iter::successors;

        successors(Some((0, 1)), |dp| {
            Some((dp.1, dp.0 + dp.1))
        })
        .take((n + 1) as usize)
        .last()
        .unwrap()
        .1
    }

    pub fn _climb_stairs(n: i32) -> i32 {
        // iterative fib soln
        // dp -> store each result
        let mut fib = vec![1, 1];
        for i in fib.len()..=n as usize {
            fib.push(fib[i - 1] + fib[i - 2])
        }

        fib[n as usize]
    }
}

// its a fibonacci sequence
// 1: (1) -> 1
// 2: (2) -> 1+1, 2
// 3: (3) -> 1+1+1, 2+1, 1+2
// 4: (5) -> 1+1+1+1, 2+1+1, 1+2+1, 1+1+2, 2+2
// 5: (8) -> 1+1+1+1+1, 2+1+1+1, 1+2+1+1, 1+1+2+1,
// 1+1+1+2, 2+2+1, 2+1+2, 1+2+2

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::climb_stairs(6), 13);
    }
}
