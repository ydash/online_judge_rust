// 1770. Maximum Score from Performing Multiplication Operations

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut dp = vec![0; m + 1];
        for left in (0..m).rev() {
            for right in (0..m - left).rev() {
                dp[right] = (multipliers[left + right] * nums[left] + dp[right])
                    .max(multipliers[left + right] * nums[n - right - 1] + dp[right + 1]);
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]);
        assert_eq!(14, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::maximum_score(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6]);
        assert_eq!(102, result)
    }
}
