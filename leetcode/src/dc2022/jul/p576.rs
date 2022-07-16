// 576. Out of Boundary Paths
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let divisor = 1000000007;
        let mut prev = vec![vec![0; n]; m];
        let mut current = vec![vec![0; n]; m];
        let mut sum: u32 = 0;
        prev[start_row as usize][start_column as usize] = 1;
        for _ in 0..max_move {
            for i in 0..m {
                for j in 0..n {
                    if prev[i][j] > 0 {
                        if i > 0 {
                            current[i - 1][j] += prev[i][j];
                        } else {
                            sum += prev[i][j];
                        }
                        if j > 0 {
                            current[i][j - 1] += prev[i][j];
                        } else {
                            sum += prev[i][j];
                        }
                        if i < m - 1 {
                            current[i + 1][j] += prev[i][j];
                        } else {
                            sum += prev[i][j];
                        }
                        if j < n - 1 {
                            current[i][j + 1] += prev[i][j];
                        } else {
                            sum += prev[i][j];
                        }
                        sum %= divisor;
                    }
                }
            }
            for i in 0..m {
                for j in 0..n {
                    prev[i][j] = current[i][j] % divisor;
                    current[i][j] = 0;
                }
            }
        }
        sum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_paths(2, 2, 2, 0, 0);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_paths(1, 3, 3, 0, 1);
        assert_eq!(12, result)
    }
}
