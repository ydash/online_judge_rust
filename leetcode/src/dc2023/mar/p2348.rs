// 2348. Number of Zero-Filled Subarrays

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut result: i64 = 0;
        let mut left = 0;
        while left < nums.len() {
            while left < nums.len() && nums[left] != 0 {
                left += 1;
            }
            let mut right = left;
            while right < nums.len() && nums[right] == 0 {
                right += 1;
            }
            let zeros = right - left;
            result += ((zeros + 1) * zeros / 2) as i64;
            left = right + 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]);
        assert_eq!(9, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::zero_filled_subarray(vec![2, 10, 2019]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::zero_filled_subarray(vec![0]);
        assert_eq!(1, result)
    }
}
