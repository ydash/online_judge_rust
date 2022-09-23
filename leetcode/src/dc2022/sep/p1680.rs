// 1680. Concatenation of Consecutive Binary Numbers

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn concatenated_binary(n: i32) -> i32 {
        let m = 1000000007;
        let mut result = 0_i64;
        for x in 1..=n {
            result <<= 32 - x.leading_zeros();
            result += x as i64;
            result %= m;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::concatenated_binary(1);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::concatenated_binary(3);
        assert_eq!(27, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::concatenated_binary(12);
        assert_eq!(505379714, result)
    }
}
