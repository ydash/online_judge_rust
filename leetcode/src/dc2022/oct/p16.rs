// 16. 3Sum Closest

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut min_diff = i32::MAX;
        for i in 0..nums.len() {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let diff = target - (nums[i] + nums[left] + nums[right]);
                if diff.abs() < min_diff.abs() {
                    min_diff = diff;
                }
                if diff > 0 {
                    left += 1;
                } else if diff < 0 {
                    right -= 1;
                } else {
                    break;
                }
            }
        }
        target - min_diff
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::three_sum_closest(vec![-1, 2, 1, -4], 1);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::three_sum_closest(vec![1, 2, 3, 4, 5], 8);
        assert_eq!(8, result)
    }
}
