// 1345. Jump Game IV

use std::collections::{HashMap, VecDeque};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut indices = HashMap::new();
        for (i, &n) in arr.iter().enumerate() {
            indices.entry(n).or_insert(vec![]).push(i);
        }
        let goal = arr.len() - 1;
        let mut seen = vec![false; arr.len()];
        let mut q = VecDeque::new();
        q.push_back(0);
        seen[0] = true;
        let mut step = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let i = q.pop_front().unwrap();
                if i == goal {
                    return step;
                }
                if i > 0 && !seen[i - 1] {
                    seen[i - 1] = true;
                    q.push_back(i - 1);
                }
                if i < goal && !seen[i + 1] {
                    seen[i + 1] = true;
                    q.push_back(i + 1);
                }
                if let Some(adj) = indices.get_mut(&arr[i]) {
                    while let Some(j) = adj.pop() {
                        q.push_back(j);
                        seen[j] = true;
                    }
                }
            }
            step += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_jumps(vec![7]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]);
        assert_eq!(1, result)
    }
}
