// 326. Power of Three

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_power_of_three(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_power_of_three(27);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_power_of_three(0);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_power_of_three(9);
        assert_eq!(true, result)
    }
}
