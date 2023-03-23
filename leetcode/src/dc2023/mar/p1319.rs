// 1319. Number of Operations to Make Network Connected

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut extra_cables = 0;
        let mut ds = DisjointSet::new(n);
        for edge in connections {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            if ds.same(a, b) {
                extra_cables += 1;
            } else {
                ds.union(a, b);
            }
        }
        let mut needed_cables = 0;
        for i in 1..n {
            if !ds.same(0, i) {
                ds.union(0, i);
                needed_cables += 1;
            }
        }
        if extra_cables < needed_cables {
            -1
        } else {
            needed_cables
        }
    }
}

struct DisjointSet {
    parent: Vec<usize>,
}
impl DisjointSet {
    fn new(size: usize) -> Self {
        DisjointSet {
            parent: (0..size).collect(),
        }
    }

    fn find_parent(&mut self, i: usize) -> usize {
        if i == self.parent[i] {
            i
        } else {
            self.parent[i] = self.find_parent(self.parent[i]);
            self.parent[i]
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let pi = self.find_parent(i);
        let pj = self.find_parent(j);
        if pi != pj {
            self.parent[pj] = pi;
        }
    }

    fn same(&mut self, i: usize, j: usize) -> bool {
        self.find_parent(i) == self.find_parent(j)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::make_connected(
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]],
        );
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result =
            Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]);
        assert_eq!(-1, result)
    }
}
