// 645. Set Mismatch

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = HashSet::new();
        let mut dup = -1;
        for n in nums.iter() {
            if seen.contains(n) {
                dup = *n;
            }
            seen.insert(n);
        }
        for n in 1..=nums.len() as i32 {
            if !seen.contains(&n) {
                return vec![dup, n];
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_error_nums(vec![1, 2, 2, 4]);
        assert_eq!(vec![2, 3], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_error_nums(vec![2, 2]);
        assert_eq!(vec![2, 1], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_error_nums(vec![2, 5, 4, 3, 3]);
        assert_eq!(vec![3, 1], result)
    }
}
