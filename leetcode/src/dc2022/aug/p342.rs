// 342. Power of Four

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_power_of_four(16);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_power_of_four(5);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_power_of_four(1);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::is_power_of_four(0);
        assert_eq!(false, result)
    }
}
