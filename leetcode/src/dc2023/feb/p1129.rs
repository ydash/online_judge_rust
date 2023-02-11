// 1129. Shortest Path with Alternating Colors

use std::collections::{HashMap, VecDeque};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut edges = HashMap::new();
        for edge in red_edges {
            edges.entry(edge[0] as usize).or_insert([vec![], vec![]])[0].push(edge[1] as usize);
        }
        for edge in blue_edges {
            edges.entry(edge[0] as usize).or_insert([vec![], vec![]])[1].push(edge[1] as usize);
        }
        let n = n as usize;
        let mut result = vec![-1; n];
        let mut seen = vec![0; n];
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        q.push_back((0, 1));
        let mut step = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let (node, color) = q.pop_front().unwrap();
                if seen[node] & (color + 1) == 0 {
                    edges.get(&node).map(|v| {
                        for &next in v[color ^ 1].iter() {
                            q.push_back((next, color ^ 1));
                        }
                    });
                    if result[node] < 0 {
                        result[node] = step;
                    }
                    seen[node] += color + 1;
                }
            }
            step += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![]);
        assert_eq!(vec![0, 1, -1], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]]);
        assert_eq!(vec![0, 1, -1], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![1, 2]]);
        assert_eq!(vec![0, 1, 2], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::shortest_alternating_paths(
            6,
            vec![vec![0, 1], vec![1, 2], vec![3, 4], vec![5, 2]],
            vec![vec![0, 3], vec![4, 5]],
        );
        assert_eq!(vec![0, 1, 4, 1, 2, 3], result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::shortest_alternating_paths(
            6,
            vec![vec![0, 1], vec![3, 4], vec![5, 2]],
            vec![vec![0, 3], vec![1, 2], vec![4, 5]],
        );
        assert_eq!(vec![0, 1, 2, 1, 2, 3], result)
    }
}
