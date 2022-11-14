// 947. Most Stones Removed with Same Row or Column

use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut ds = DisjointSet::new();
        let mut row = HashMap::new();
        let mut col = HashMap::new();
        let mut seen = HashSet::new();
        for stone in stones.iter() {
            let x = stone[0];
            let y = stone[1];
            row.entry(x).or_insert(vec![]).push(y);
            col.entry(y).or_insert(vec![]).push(x);
        }
        for x in row.keys() {
            let ys = &row[x];
            for i in 1..ys.len() {
                ds.link((*x, ys[i - 1]), (*x, ys[i]));
            }
        }
        for y in col.keys() {
            let xs = &col[y];
            for i in 1..xs.len() {
                ds.link((xs[i - 1], *y), (xs[i], *y));
            }
        }

        for stone in stones.iter() {
            let x = stone[0];
            let y = stone[1];
            let root = ds.find_parent((x, y));
            if !seen.contains(&root) {
                result += ds.size(root) - 1;
                seen.insert(root);
            }
        }
        result
    }
}

#[allow(dead_code)]
struct DisjointSet {
    parent: HashMap<(i32, i32), (i32, i32)>,
}

impl DisjointSet {
    fn new() -> Self {
        DisjointSet {
            parent: HashMap::new(),
        }
    }

    fn find_parent(&mut self, x: (i32, i32)) -> (i32, i32) {
        self.parent.entry(x).or_insert((-1, -1));
        if self.parent[&x].0 < 0 {
            x
        } else {
            let root = self.find_parent(self.parent[&x]);
            self.parent.insert(x, root);
            root
        }
    }

    fn link(&mut self, x: (i32, i32), y: (i32, i32)) {
        let x_root = self.find_parent(x);
        let y_root = self.find_parent(y);
        if x_root == y_root {
            return;
        }
        let (x0, x1) = self.parent[&x_root];
        let (y0, y1) = self.parent[&y_root];
        self.parent.insert(x_root, (x0 + y0, x1 + y1));
        self.parent.insert(y_root, x_root);
    }

    fn size(&mut self, x: (i32, i32)) -> i32 {
        let root = self.find_parent(x);
        -self.parent[&root].0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ]);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 2],
            vec![1, 1],
            vec![2, 0],
            vec![2, 2],
        ]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::remove_stones(vec![vec![0, 0]]);
        assert_eq!(0, result)
    }
}
