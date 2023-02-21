// 35. Search Insert Position

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid as i32;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 5);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 2);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 7);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::search_insert(vec![1], 0);
        assert_eq!(0, result)
    }
}
