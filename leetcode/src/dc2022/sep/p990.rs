// 990. Satisfiability of Equality Equations

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let (eq, neq): (Vec<_>, Vec<_>) =
            equations.into_iter().partition(|s| s.as_bytes()[1] == b'=');
        let mut ds = DisjointSet::new();
        for s in eq.into_iter() {
            let bytes = s.as_bytes();
            let x = (bytes[0] - b'a') as usize;
            let y = (bytes[3] - b'a') as usize;
            ds.union(x, y);
        }
        for s in neq.into_iter() {
            let bytes = s.as_bytes();
            let x = (bytes[0] - b'a') as usize;
            let y = (bytes[3] - b'a') as usize;
            if ds.is_same(x, y) {
                return false;
            }
        }
        true
    }
}

struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new() -> Self {
        DisjointSet {
            parent: (0..26).collect(),
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find_parent(x) == self.find_parent(y)
    }

    fn find_parent(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find_parent(self.parent[x]);
            self.parent[x]
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_parent = self.find_parent(x);
        let y_parent = self.find_parent(y);
        self.parent[y_parent] = x_parent;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::equations_possible(vec!["a==b".to_owned(), "b!=a".to_owned()]);
        assert!(!result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::equations_possible(vec!["z==a".to_owned(), "a==z".to_owned()]);
        assert!(result)
    }
}
