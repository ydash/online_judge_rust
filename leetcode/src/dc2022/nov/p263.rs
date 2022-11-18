// 263. Ugly Number

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_ugly(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        let mut n = n;
        while n % 2 == 0 {
            n /= 2;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        while n % 5 == 0 {
            n /= 5;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_ugly(6);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_ugly(14);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_ugly(0);
        assert_eq!(false, result)
    }
}
