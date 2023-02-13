// 1523. Count Odd Numbers in an Interval Range

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high + 1) / 2 - low / 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::count_odds(3, 7);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::count_odds(8, 10);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::count_odds(1, 8);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::count_odds(2, 7);
        assert_eq!(3, result)
    }
}
