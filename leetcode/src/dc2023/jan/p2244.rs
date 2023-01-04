// 2244. Minimum Rounds to Complete All Tasks

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        for level in tasks.into_iter() {
            *count.entry(level).or_insert(0) += 1;
        }
        let mut result = 0;
        for &c in count.values() {
            if c < 2 {
                return -1;
            } else {
                result += c / 3;
                if c % 3 != 0 {
                    result += 1;
                }
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
        let result = Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::minimum_rounds(vec![2, 3, 3]);
        assert_eq!(-1, result)
    }
}
