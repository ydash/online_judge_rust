// 704. Binary Search

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::search(vec![2], 2);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::search(vec![-1, 0, 3, 5, 9, 12], 2);
        assert_eq!(-1, result)
    }
}
