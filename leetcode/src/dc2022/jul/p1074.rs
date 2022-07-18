// 1074. Number of Submatrices That Sum to Target
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                dp[i][j] = matrix[i][j]
                    + if i == 0 && j == 0 {
                        0
                    } else if i == 0 {
                        dp[i][j - 1]
                    } else if j == 0 {
                        dp[i - 1][j]
                    } else {
                        dp[i][j - 1] + dp[i - 1][j] - dp[i - 1][j - 1]
                    }
            }
        }
        let mut count = 0;
        for x2 in 0..m {
            for y2 in 0..n {
                for x1 in 0..=x2 {
                    for y1 in 0..=y2 {
                        let sum = dp[x2][y2]
                            - if x1 > 0 && y1 > 0 {
                                dp[x1 - 1][y2] + dp[x2][y1 - 1] - dp[x1 - 1][y1 - 1]
                            } else if x1 > 0 {
                                dp[x1 - 1][y2]
                            } else if y1 > 0 {
                                dp[x2][y1 - 1]
                            } else {
                                0
                            };

                        if sum == target {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_submatrix_sum_target(
            vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
            0,
        );
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::num_submatrix_sum_target(vec![vec![42]], 0);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::num_submatrix_sum_target(vec![vec![42]], 42);
        assert_eq!(1, result)
    }
}
