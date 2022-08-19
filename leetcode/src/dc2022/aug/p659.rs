// 659. Split Array into Consecutive Subsequences

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut count_of_seq_last: HashMap<i32, i32> = HashMap::new();
        for &n in nums.iter() {
            *freq.entry(n).or_insert(0) += 1;
        }
        for &n in nums.iter() {
            if *freq.get(&n).unwrap_or(&0) == 0 {
                continue;
            }
            *freq.get_mut(&n).unwrap() -= 1;
            if *count_of_seq_last.get(&(n - 1)).unwrap_or(&0) > 0 {
                *count_of_seq_last.entry(n - 1).or_insert(0) -= 1;
                *count_of_seq_last.entry(n).or_insert(0) += 1;
            } else if *freq.get(&(n + 1)).unwrap_or(&0) > 0 && *freq.get(&(n + 2)).unwrap_or(&0) > 0
            {
                *freq.entry(n + 1).or_insert(0) -= 1;
                *freq.entry(n + 2).or_insert(0) -= 1;
                *count_of_seq_last.entry(n + 2).or_insert(0) += 1;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_possible(vec![1, 2, 3, 3, 4, 5]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_possible(vec![1, 2, 3, 4, 4, 5]);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::is_possible(vec![1, 2, 3, 5, 6, 7]);
        assert_eq!(true, result)
    }
}
