// 62. Unique Paths
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn unique_paths(_m: i32, _n: i32) -> i32 {
        let m = _m.max(_n) as i64 - 1;
        let n = _m.min(_n) as i64 - 1;
        ((m + 1)..=(m + n))
            .zip(1..=n)
            .fold(1, |acc, (x, y)| -> i64 { (acc * x) / y }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::unique_paths(3, 7);
        assert_eq!(28, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::unique_paths(3, 2);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::unique_paths(51, 9);
        assert_eq!(1916797311, result)
    }
}
