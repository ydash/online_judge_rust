// 2421. Number of Good Paths

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();
        let mut result = n as i32;
        let mut max_connected_val: Vec<(i32, i32)> = vals.iter().map(|&v| (v, 1)).collect();
        let mut val_and_edges: Vec<(i32, usize, usize)> = edges
            .iter()
            .map(|e| {
                let a = e[0] as usize;
                let b = e[1] as usize;
                (vals[a].max(vals[b]), a, b)
            })
            .collect();
        val_and_edges.sort_by(|v1, v2| v1.0.cmp(&v2.0));
        let mut ds = DisjointSet::new(n);
        for (v, a, b) in val_and_edges {
            let pa = ds.find_parent(a);
            let pb = ds.find_parent(b);
            let count_a = if max_connected_val[pa].0 == v {
                max_connected_val[pa].1
            } else {
                0
            };
            let count_b = if max_connected_val[pb].0 == v {
                max_connected_val[pb].1
            } else {
                0
            };
            ds.union(pa, pb);
            result += count_a * count_b;
            max_connected_val[pa] = (v, count_a + count_b);
        }
        result
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
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find_parent(self.parent[i] as usize);
            self.parent[i]
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find_parent(x);
        let py = self.find_parent(y);
        if px != py {
            self.parent[py] = px;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::number_of_good_paths(
            vec![1, 3, 2, 1, 3],
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
        );
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::number_of_good_paths(
            vec![1, 1, 2, 2, 3],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]],
        );
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::number_of_good_paths(vec![1], vec![]);
        assert_eq!(1, result)
    }
}
