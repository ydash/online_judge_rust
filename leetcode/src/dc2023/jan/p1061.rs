// 1061. Lexicographically Smallest Equivalent String

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut cm = CharMinimizer::new();
        for (&b1, &b2) in s1.as_bytes().iter().zip(s2.as_bytes()) {
            cm.register_equivalent(b1, b2);
        }
        String::from_utf8(
            base_str
                .as_bytes()
                .iter()
                .map(|&b| cm.minimize(b))
                .collect(),
        )
        .unwrap()
    }
}

struct CharMinimizer {
    ds: DisjointSet,
}

impl CharMinimizer {
    fn new() -> Self {
        CharMinimizer {
            ds: DisjointSet::new(),
        }
    }

    fn register_equivalent(&mut self, b1: u8, b2: u8) {
        self.ds.union((b1 - b'a').into(), (b2 - b'a').into());
    }

    fn minimize(&mut self, b: u8) -> u8 {
        let p = self.ds.find_parent((b - b'a').into());
        (-self.ds.parent[p]) as u8
    }
}

struct DisjointSet {
    parent: Vec<i32>,
}

impl DisjointSet {
    fn new() -> Self {
        DisjointSet {
            parent: (b'a'..=b'z').map(|b| -(b as i32)).collect(),
        }
    }

    fn find_parent(&mut self, i: usize) -> usize {
        if self.parent[i] < 0 {
            i
        } else {
            let p = self.find_parent(self.parent[i] as usize);
            self.parent[i] = p as i32;
            p
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find_parent(x);
        let py = self.find_parent(y);
        if px != py {
            let min_x = self.parent[px];
            let min_y = self.parent[py];
            self.parent[py as usize] = px as i32;
            self.parent[px as usize] = min_x.max(min_y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::smallest_equivalent_string(
            "parker".to_owned(),
            "morris".to_owned(),
            "parser".to_owned(),
        );
        assert_eq!("makkek", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::smallest_equivalent_string(
            "hello".to_owned(),
            "world".to_owned(),
            "hold".to_owned(),
        );
        assert_eq!("hdld", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::smallest_equivalent_string(
            "leetcode".to_owned(),
            "programs".to_owned(),
            "sourcecode".to_owned(),
        );
        assert_eq!("aauaaaaada", result)
    }
}
