// 2316. Count Unreachable Pairs of Nodes in an Undirected Graph

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut ds = DisjointSet::new(n as usize);
        for e in edges {
            ds.union(e[0], e[1]);
        }
        let mut result = 0;
        let mut seen = HashSet::new();
        let mut rest = n as i64;
        for i in 0..n {
            let root = ds.find_parent(i) as usize;
            if !seen.contains(&root) {
                let size = -ds.parent[root] as i64;
                rest -= size;
                result += size * rest;
                seen.insert(root);
            }
        }
        result
    }
}

struct DisjointSet {
    parent: Vec<i32>,
}
impl DisjointSet {
    fn new(size: usize) -> Self {
        DisjointSet {
            parent: vec![-1; size],
        }
    }

    fn find_parent(&mut self, i: i32) -> i32 {
        let i_usize = i as usize;
        if self.parent[i_usize] < 0 {
            i
        } else {
            let root = self.find_parent(self.parent[i_usize]);
            self.parent[i_usize] = root;
            self.parent[i_usize]
        }
    }

    fn union(&mut self, i: i32, j: i32) {
        let pi = self.find_parent(i) as usize;
        let pj = self.find_parent(j) as usize;
        if pi != pj {
            self.parent[pi] += self.parent[pj];
            self.parent[pj] = pi as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::count_pairs(
            7,
            vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]],
        );
        assert_eq!(14, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::count_pairs(100000, vec![]);
        assert_eq!(4999950000, result)
    }
}
