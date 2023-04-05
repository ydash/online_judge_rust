// 2439. Minimize Maximum of Array

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut sum: i64 = 0;
        let mut result = 0;
        for (i, n) in nums.into_iter().enumerate() {
            sum += n as i64;
            let i = i as i64;
            result = result.max((sum + i) / (i + 1));
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::minimize_array_value(vec![3, 7, 1, 8]);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::minimize_array_value(vec![10, 1]);
        assert_eq!(10, result)
    }
}
