// 279. Perfect Squares

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_squares(n: i32) -> i32 {
        let mut dp: Vec<i32> = (0..=n).collect();
        for i in 0..n as usize {
            let mut x = 2;
            while x * x + i as i32 <= n {
                let di = (x * x) as usize;
                dp[i + di] = dp[i + di].min(dp[i] + 1);
                x += 1;
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_squares(12);
        assert_eq!(result, 3)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_squares(13);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::num_squares(15);
        assert_eq!(4, result)
    }
}
