// 1335. Minimum Difficulty of a Job Schedule

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;
        if n < d {
            return -1;
        }
        let mut dp = vec![vec![i32::MAX; n]; d];
        let mut max_diff = 0;
        for j in 0..n {
            max_diff = max_diff.max(job_difficulty[j]);
            dp[0][j] = max_diff;
        }
        for i in 1..d {
            for j in i..n {
                let mut max_diff = 0;
                for k in (i..=j).rev() {
                    max_diff = max_diff.max(job_difficulty[k]);
                    dp[i][j] = dp[i][j].min(dp[i - 1][k - 1] + max_diff);
                }
            }
        }
        dp[d - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_difficulty(vec![9, 9, 9], 4);
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_difficulty(vec![1, 1, 1], 3);
        assert_eq!(3, result)
    }
}
