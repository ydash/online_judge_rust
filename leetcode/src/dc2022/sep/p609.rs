// 609. Find Duplicate File in System

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for path in paths.iter() {
            let splitted: Vec<&str> = path.split(' ').collect();
            let dir = splitted[0];
            for s in splitted.into_iter().skip(1) {
                let x: Vec<&str> = s.split('(').collect();
                let file = x[0];
                let content = x[1];
                map.entry(content)
                    .or_insert(vec![])
                    .push(format!("{}/{}", dir, file));
            }
        }
        map.into_values().filter(|v| v.len() > 1).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".to_owned(),
            "root/c 3.txt(abcd)".to_owned(),
            "root/c/d 4.txt(efgh)".to_owned(),
            "root 4.txt(efgh)".to_owned(),
        ]);
        assert_eq!(
            vec![
                vec![
                    "root/a/2.txt".to_owned(),
                    "root/c/d/4.txt".to_owned(),
                    "root/4.txt".to_owned()
                ],
                vec!["root/a/1.txt".to_owned(), "root/c/3.txt".to_owned()]
            ]
            .iter()
            .collect::<HashSet<_>>(),
            result.iter().collect()
        )
    }
}
