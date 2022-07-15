// 695. Max Area of Island
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[m - 1].len();
        let mut ds = DisjointSet::new(m * n);
        let mut max_area = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let x = n * i + j;
                    if i > 0 && grid[i - 1][j] == 1 && !ds.is_same(x, x - n) {
                        ds.union(x, x - n);
                    }
                    if j > 0 && grid[i][j - 1] == 1 && !ds.is_same(x, x - 1) {
                        ds.union(x, x - 1);
                    }
                    max_area = max_area.max(ds.sum(x));
                }
            }
        }
        max_area
    }
}

struct DisjointSet {
    parent: Vec<i32>,
}

impl DisjointSet {
    fn new(size: usize) -> DisjointSet {
        DisjointSet {
            parent: vec![-1; size],
        }
    }
    fn find_parent(&self, x: usize) -> usize {
        let mut current = x as i32;
        while self.parent[current as usize] >= 0 {
            current = self.parent[current as usize];
        }
        current as usize
    }
    fn union(&mut self, x: usize, y: usize) {
        let x = self.find_parent(x);
        let y = self.find_parent(y);

        if self.parent[x] < self.parent[y] {
            self.parent[x] += self.parent[y];
            self.parent[y] = x as i32;
        } else {
            self.parent[y] += self.parent[x];
            self.parent[x] = y as i32;
        }
    }
    fn sum(&self, x: usize) -> i32 {
        let root = self.find_parent(x);
        -self.parent[root]
    }
    fn is_same(&self, x: usize, y: usize) -> bool {
        self.find_parent(x) == self.find_parent(y)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::max_area_of_island(vec![vec![1]]);
        assert_eq!(1, result)
    }
}
