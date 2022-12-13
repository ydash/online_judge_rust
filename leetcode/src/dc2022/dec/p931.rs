// 931. Minimum Falling Path Sum

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut dp = matrix.clone();
        for i in 1..n {
            for j in 0..n {
                dp[i][j] = matrix[i][j]
                    + if j == 0 {
                        dp[i - 1][j].min(dp[i - 1][j + 1])
                    } else if j == n - 1 {
                        dp[i - 1][j - 1].min(dp[i - 1][j])
                    } else {
                        dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i - 1][j + 1]))
                    }
            }
        }
        *dp[n - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result =
            Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]);
        assert_eq!(13, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]);
        assert_eq!(-59, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_falling_path_sum(vec![vec![42]]);
        assert_eq!(42, result)
    }
}
