// 458. Poor Pigs

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let buckets = f64::from(buckets);
        let x = f64::from(minutes_to_test / minutes_to_die + 1);
        buckets.log(x).ceil() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::poor_pigs(1000, 15, 60);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::poor_pigs(4, 15, 15);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::poor_pigs(4, 15, 30);
        assert_eq!(2, result)
    }
}
