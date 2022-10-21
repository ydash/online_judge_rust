// 219. Contains Duplicate II

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut seen: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            if let Some(j) = seen.get(&n) {
                if i - j <= k {
                    return true;
                }
            }
            seen.insert(n, i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2);
        assert_eq!(false, result)
    }
}
