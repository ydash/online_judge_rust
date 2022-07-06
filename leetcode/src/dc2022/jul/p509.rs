#[cfg(test)]
mod tests {
    use super::solution::fib;

    #[test]
    fn test_case_01() {
        let result = fib(0);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_02() {
        let result = fib(1);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_03() {
        let result = fib(4);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_04() {
        let result = fib(30);
        assert_eq!(832040, result)
    }
}

// 509. Fibonacci Number
mod solution {
    #[allow(dead_code)]
    pub fn fib(n: i32) -> i32 {
        let mut i = 0;
        let mut cache = (0, 1);
        while i < n {
            let tmp = cache.0 + cache.1;
            cache.0 = cache.1;
            cache.1 = tmp;
            i += 1;
        }
        cache.0
    }
}
