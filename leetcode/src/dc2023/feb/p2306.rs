// 2306. Naming a Company

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut suffix = vec![HashSet::new(); 26];
        for idea in ideas.iter() {
            suffix[usize::from(idea.as_bytes()[0] - b'a')].insert(&idea[1..]);
        }
        let mut result = 0;
        for i in 0..26 {
            for j in i..26 {
                let duplications = suffix[i].intersection(&suffix[j]).count();
                result += (suffix[i].len() - duplications) * (suffix[j].len() - duplications)
            }
        }
        result as i64 * 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::distinct_names(vec![
            "coffee".to_owned(),
            "donuts".to_owned(),
            "time".to_owned(),
            "toffee".to_owned(),
        ]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::distinct_names(vec!["ai".to_owned(), "be".to_owned()]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::distinct_names(vec![
            "time".to_owned(),
            "donuts".to_owned(),
            "coffee".to_owned(),
            "toffee".to_owned(),
        ]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::distinct_names(vec!["lack".to_owned(), "back".to_owned()]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::distinct_names(vec![
            "aaa".to_owned(),
            "baa".to_owned(),
            "caa".to_owned(),
            "bbb".to_owned(),
            "cbb".to_owned(),
            "dbb".to_owned(),
        ]);
        assert_eq!(2, result)
    }
}
