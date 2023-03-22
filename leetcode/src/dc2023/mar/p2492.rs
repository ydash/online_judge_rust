// 2492. Minimum Score of a Path Between Two Cities

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut ds = DisjointSet::new(n as usize);
        for e in roads {
            let a = e[0] - 1;
            let b = e[1] - 1;
            let dist = e[2];
            ds.union(a as usize, b as usize, dist);
        }
        let root = ds.find_parent(0);
        -ds.parent[root]
    }
}

struct DisjointSet {
    parent: Vec<i32>,
}
impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: vec![i32::MIN; size],
        }
    }

    fn find_parent(&mut self, i: usize) -> usize {
        if self.parent[i] < 0 {
            i
        } else {
            let parent = self.find_parent(self.parent[i] as usize);
            self.parent[i] = parent as i32;
            parent
        }
    }

    fn union(&mut self, x: usize, y: usize, cost: i32) {
        let px = self.find_parent(x);
        let py = self.find_parent(y);
        let min_cost = self.parent[px].max(self.parent[py]);
        if px != py {
            self.parent[py] = px as i32;
        }
        self.parent[px] = min_cost.max(-cost);
    }
}

#[cfg(test)]
mod tests {
    use crate::dc2023::mar::p2492::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_score(
            4,
            vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]],
        );
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_score(4, vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]]);
        assert_eq!(2, result)
    }
}
