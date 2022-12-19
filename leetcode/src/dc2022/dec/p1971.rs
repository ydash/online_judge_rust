// 1971. Find if Path Exists in Graph

use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn valid_path(_n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut adj = HashMap::new();
        for e in edges {
            adj.entry(e[0]).or_insert(vec![]).push(e[1]);
            adj.entry(e[1]).or_insert(vec![]).push(e[0]);
        }
        let mut stack = vec![];
        let mut seen = HashSet::new();
        stack.push(source);
        seen.insert(source);
        while let Some(current) = stack.pop() {
            if destination == current {
                return true;
            }
            for &next in adj.get(&current).unwrap() {
                if !seen.contains(&next) {
                    seen.insert(next);
                    stack.push(next);
                }
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
        let result = Solution::valid_path(6, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5,
        );
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::valid_path(1, vec![], 0, 0);
        assert_eq!(true, result)
    }
}
