// 446. Arithmetic Slices II - Subsequence

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![HashMap::new(); n];
        let mut result = 0;
        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let count = *dp[j].get(&diff).unwrap_or(&0);
                result += count;
                *dp[i].entry(diff).or_insert(0) += count + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]);
        assert_eq!(16, result)
    }
}
