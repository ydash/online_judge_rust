// 300. Longest Increasing Subsequence

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        for i in 0..nums.len() {
            let j = match dp.binary_search(&nums[i]) {
                Ok(j) => j,
                Err(j) => j,
            };
            if j >= dp.len() {
                dp.push(nums[i]);
            } else {
                dp[j] = nums[i];
            }
        }
        dp.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::length_of_lis(vec![1, 2, 3, 4, 5]);
        assert_eq!(5, result)
    }
}
