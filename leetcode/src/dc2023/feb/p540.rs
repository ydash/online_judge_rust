// 540. Single Element in a Sorted Array

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() / 2;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[2 * mid] == nums[2 * mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[left * 2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]);
        assert_eq!(10, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::single_non_duplicate(vec![1, 2, 2, 3, 3]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::single_non_duplicate(vec![1, 1, 2, 2, 3]);
        assert_eq!(3, result)
    }
}
