// 718. Maximum Length of Repeated Subarray

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len = nums1.len();
        let mut dp = vec![0; len + 1];
        let mut result = 0;
        for n1 in nums1.iter().rev() {
            for (j, n2) in nums2.iter().enumerate() {
                dp[j] = if n1 == n2 { dp[j + 1] + 1 } else { 0 };
                result = result.max(dp[j]);
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
        let result = Solution::find_length(vec![0, 0, 0, 0, 1], vec![1, 0, 0, 0, 0]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]);
        assert_eq!(3, result)
    }
}
