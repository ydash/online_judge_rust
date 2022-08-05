// 377. Combination Sum IV

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let nums = nums.iter().map(|&n| n as usize).collect::<Vec<_>>();
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for i in 0..target {
            for &n in nums.iter().filter(|&n| i + n <= target) {
                dp[i + n] += dp[i];
            }
        }
        dp[target]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::combination_sum4(vec![1, 2, 3], 4);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::combination_sum4(vec![9], 3);
        assert_eq!(0, result)
    }
}
