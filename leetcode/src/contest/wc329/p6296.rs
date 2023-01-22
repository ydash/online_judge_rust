//  6296. Alternating Digit Sum

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut x = 1_i64;
        while x * 10 <= n as i64 {
            x *= 10;
        }
        let mut x = x as i32;
        let mut result = 0;
        let mut sign = 1;
        while x > 0 {
            dbg!(n, x, n / x, sign);
            result += (n / x) * sign;
            n %= x;
            x /= 10;
            sign *= -1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::alternate_digit_sum(521);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::alternate_digit_sum(886996);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::alternate_digit_sum(1);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::alternate_digit_sum(1000000000);
        assert_eq!(1, result)
    }
}
