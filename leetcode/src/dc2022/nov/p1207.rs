// 1207. Unique Number of Occurrences

use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occurrence = HashMap::new();
        for &n in arr.iter() {
            *occurrence.entry(n).or_insert(0) += 1;
        }
        let mut seen = HashSet::new();
        for o in occurrence.values() {
            if seen.contains(o) {
                return false;
            }
            seen.insert(*o);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::unique_occurrences(vec![1, 2]);
        assert_eq!(false, result)
    }
}
