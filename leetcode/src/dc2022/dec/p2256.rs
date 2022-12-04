// 2256. Minimum Average Difference

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let nums = nums.into_iter().map(|n| i64::from(n)).collect::<Vec<_>>();
        let mut sum_from_left: i64 = 0;
        let mut sum_from_right: i64 = nums.iter().sum();
        let mut result = 0;
        let mut min_diff = i64::MAX;
        for i in 0..n {
            sum_from_left += nums[i];
            sum_from_right -= nums[i];
            let ls_avg = sum_from_left / (i as i64 + 1);
            let rs_avg = if i == n - 1 {
                0
            } else {
                sum_from_right / ((n - i) as i64 - 1)
            };
            let diff = (ls_avg - rs_avg).abs();
            if diff < min_diff {
                min_diff = diff;
                result = i;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::minimum_average_difference(vec![0]);
        assert_eq!(0, result)
    }
}
