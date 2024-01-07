pub fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ if n > 0 => fib(n - 1) + fib(n - 2),
        _ => 0, // Return 0 for negative numbers
    }
}

pub fn fib_iter(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut res = 1;

    for _ in 1..n {
        res = a + b;
        a = b;
        b = res;
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib_iter(2), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(fib(3), 2);
        assert_eq!(fib_iter(3), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(fib(4), 3);
        assert_eq!(fib_iter(4), 3);
    }
}
