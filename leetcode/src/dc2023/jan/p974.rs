// 974. Subarray Sums Divisible by K

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut prefix_sum = HashMap::new();
        prefix_sum.insert(0, 1);
        let mut result = 0;
        for n in nums {
            sum = (sum + n) % k;
            if sum < 0 {
                sum += k;
            }
            prefix_sum.get(&sum).map(|count| result += *count);
            *prefix_sum.entry(sum).or_insert(0) += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::subarrays_div_by_k(vec![5], 9);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::subarrays_div_by_k(vec![-1, 2, 9], 2);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::subarrays_div_by_k(vec![-1, -9, -4, 0], 9);
        assert_eq!(2, result)
    }
}
