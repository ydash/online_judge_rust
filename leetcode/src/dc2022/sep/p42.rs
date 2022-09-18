// 42. Trapping Rain Water

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = height[left];
        let mut right_max = height[right];
        while left < right {
            if height[left] <= height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    result += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    result += right_max - height[right];
                }
                right -= 1
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
        let result = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::trap(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(9, result)
    }
}
