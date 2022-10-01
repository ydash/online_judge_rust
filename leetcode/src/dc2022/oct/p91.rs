// 91. Decode Ways

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_decodings(s: String) -> i32 {
        let nums: Vec<u8> = s.as_bytes().iter().map(|b| b - b'0').collect();
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        dp[1] = if nums[0] == 0 { 0 } else { 1 };
        for i in 1..nums.len() {
            if nums[i] > 0 {
                dp[i + 1] += dp[i];
            }
            if nums[i - 1] > 0 && nums[i - 1] * 10 + nums[i] <= 26 {
                dp[i + 1] += dp[i - 1];
            }
        }
        dp[dp.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_decodings("12".to_owned());
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_decodings("226".to_owned());
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::num_decodings("06".to_owned());
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::num_decodings("105".to_owned());
        assert_eq!(1, result)
    }
}
