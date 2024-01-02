pub fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ if n > 0 => fib(n - 1) + fib(n - 2),
        _ => 0, // Return 0 for negative numbers
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(fib(4), 3);
    }
}
