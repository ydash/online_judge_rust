// 790. Domino and Tromino Tiling

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_tilings(n: i32) -> i32 {
        if n == 1 {
            1
        } else if n == 2 {
            2
        } else if n == 3 {
            5
        } else {
            let mut dp0 = 1_i64;
            let mut dp1 = 2_i64;
            let mut dp2 = 5_i64;
            let mut result = 0_i64;
            let m = 1000000007_i64;
            for _ in 3..n {
                result = (2_i64 * dp2 + dp0) % m;
                dp0 = dp1;
                dp1 = dp2;
                dp2 = result;
            }
            result as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_tilings(1);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_tilings(2);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::num_tilings(3);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::num_tilings(4);
        assert_eq!(11, result)
    }
}
