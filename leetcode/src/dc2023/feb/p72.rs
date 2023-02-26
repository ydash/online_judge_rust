// 72. Edit Distance

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for j in 0..=n {
            dp[0][j] = j as i32;
        }
        for i in 0..m {
            dp[i + 1][0] = i as i32 + 1;
            for j in 0..n {
                let replace_cost = if word1.as_bytes()[i] == word2.as_bytes()[j] {
                    0
                } else {
                    1
                };
                dp[i + 1][j + 1] = (dp[i][j + 1] + 1)
                    .min(dp[i + 1][j] + 1)
                    .min(dp[i][j] + replace_cost);
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_distance("horse".to_owned(), "ros".to_owned());
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_distance("intention".to_owned(), "execution".to_owned());
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_distance("a".to_owned(), "b".to_owned());
        assert_eq!(1, result)
    }
}
