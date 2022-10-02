// 1155. Number of Dice Rolls With Target Sum

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let target = target as usize;
        let mut dp = vec![vec![0; target + 1]; n + 1];
        dp[0][0] = 1;
        for i in 1..=n {
            for x in 1..=k {
                for j in x..=target {
                    dp[i][j] += dp[i - 1][j - x];
                    dp[i][j] %= 1000000007;
                }
            }
        }
        dp[n][target]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_rolls_to_target(1, 6, 3);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_rolls_to_target(2, 6, 7);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::num_rolls_to_target(30, 30, 500);
        assert_eq!(222616187, result)
    }
}
