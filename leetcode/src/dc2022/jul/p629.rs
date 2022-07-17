// 629. K Inverse Pairs Array
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let divisor = 1000000007;
        let mut dp = vec![0; k + 1];
        dp[0] = 1;
        for i in 1..=n {
            let mut tmp = vec![0; k + 1];
            tmp[0] = dp[0];
            for j in 1..=k {
                let sum = (dp[j] - if j < i { 0 } else { dp[j - i] } + divisor) % divisor;
                tmp[j] = (sum + tmp[j - 1]) % divisor;
            }
            std::mem::swap(&mut dp, &mut tmp);
        }
        dp[k]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::k_inverse_pairs(3, 0);
        assert_eq!(1, result);
    }

    #[test]
    fn test_case_02() {
        let result = Solution::k_inverse_pairs(3, 1);
        assert_eq!(2, result);
    }

    #[test]
    fn test_case_03() {
        let result = Solution::k_inverse_pairs(3, 2);
        assert_eq!(2, result);
    }

    #[test]
    fn test_case_04() {
        let result = Solution::k_inverse_pairs(4, 3);
        assert_eq!(6, result);
    }

    #[test]
    fn test_case_05() {
        let result = Solution::k_inverse_pairs(4, 6);
        assert_eq!(1, result);
    }
    #[test]
    fn test_case_06() {
        let result = Solution::k_inverse_pairs(1, 1);
        assert_eq!(0, result);
    }
}
