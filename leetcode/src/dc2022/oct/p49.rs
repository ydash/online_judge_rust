// 49. Group Anagrams

use std::collections::HashMap;

struct Solution {}
#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs {
            let mut count = vec![0; 26];
            for &b in s.as_bytes() {
                count[(b - b'a') as usize] += 1;
            }
            map.entry(count).or_insert(vec![]).push(s);
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let mut result = Solution::group_anagrams(vec![
            "eat".to_owned(),
            "tea".to_owned(),
            "tan".to_owned(),
            "ate".to_owned(),
            "nat".to_owned(),
            "bat".to_owned(),
        ]);
        result.sort();
        assert_eq!(
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]],
            result
        )
    }
}
