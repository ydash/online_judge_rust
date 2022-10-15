// 1531. String Compression II

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        fn length_of(count: usize) -> i32 {
            match count {
                c if c == 0 => 0,
                c if c == 1 => 1,
                c if c < 10 => 2,
                c if c < 100 => 3,
                _ => 4,
            }
        }
        let k = k as usize;
        let n = s.len();
        let chars = s.as_bytes();
        let mut dp = vec![vec![i32::MAX; k + 1]; n + 1];
        for j in 0..=k {
            dp[0][j] = 0;
        }
        for i in 1..=n {
            for j in 0..=k {
                if j > 0 {
                    dp[i][j] = dp[i - 1][j - 1];
                }
                let mut count = 0;
                let mut removed = 0;
                for a in (1..=i).rev() {
                    if chars[a - 1] == chars[i - 1] {
                        count += 1;
                    } else {
                        removed += 1;
                        if removed > j {
                            break;
                        }
                    }
                    dp[i][j] = dp[i][j].min(dp[a - 1][j - removed] + length_of(count));
                }
            }
        }
        dp[n][k]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let result = Solution::get_length_of_optimal_compression("aaabcccd".to_owned(), 2);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_2() {
        let result = Solution::get_length_of_optimal_compression("aabbaa".to_owned(), 2);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_3() {
        let result = Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_owned(), 0);
        assert_eq!(3, result)
    }
}
