// 523. Continuous Subarray Sum

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut prefix_sum = HashMap::new();
        let mut sum = nums[0] % k;
        prefix_sum.insert(sum, 0);
        for i in 1..n {
            let num = nums[i];
            sum += num;
            sum %= k;
            if sum == 0 {
                return true;
            }
            if let Some(j) = prefix_sum.get(&sum) {
                if i - j > 1 {
                    return true;
                }
            } else {
                prefix_sum.insert(sum, i);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::check_subarray_sum(vec![23, 2, 4, 6, 8], 6);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::check_subarray_sum(vec![23, 2, 4, 6, 8], 13);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::check_subarray_sum(vec![1, 2, 3], 5);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::check_subarray_sum(vec![23, 0, 0], 6);
        assert_eq!(true, result)
    }
}
