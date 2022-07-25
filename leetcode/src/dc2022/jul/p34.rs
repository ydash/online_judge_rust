// 34. Find First and Last Position of Element in Sorted Array

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];
        if nums.is_empty() {
            return result;
        }

        let mut left = 0;
        let mut right = nums.len();
        left = Self::binary_search(&nums, left, right, |v, mid| v[mid] < target);
        if left >= nums.len() || nums[left] != target {
            return result;
        }
        result[0] = left as i32;

        right = nums.len();
        right = Self::binary_search(&nums, left, right, |v, mid| v[mid] <= target);
        result[1] = right as i32 - 1;

        result
    }

    fn binary_search<F: Fn(&Vec<i32>, usize) -> bool>(
        nums: &Vec<i32>,
        left: usize,
        right: usize,
        pred: F,
    ) -> usize {
        let mut left = left;
        let mut right = right;
        while left < right {
            let size = right - left;
            let mid = left + size / 2;
            if pred(&nums, mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8);
        assert_eq!(vec![3, 4], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6);
        assert_eq!(vec![-1, -1], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::search_range(vec![], 0);
        assert_eq!(vec![-1, -1], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::search_range(vec![-3, -1, 3, 4], -3);
        assert_eq!(vec![0, 0], result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::search_range(vec![1; 10], 1);
        assert_eq!(vec![0, 9], result)
    }

    #[test]
    fn test_case_06() {
        let result = Solution::search_range(vec![1], 0);
        assert_eq!(vec![-1, -1], result)
    }

    #[test]
    fn test_case_07() {
        let result = Solution::search_range(vec![2, 2], 3);
        assert_eq!(vec![-1, -1], result)
    }
}
