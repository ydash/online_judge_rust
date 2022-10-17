// 1832. Check if the Sentence Is Pangram

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut not_seen: HashSet<_> = (b'a'..=b'z').collect();
        for b in sentence.as_bytes() {
            not_seen.remove(b);
        }
        not_seen.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::check_if_pangram("leetcode".to_owned());
        assert_eq!(false, result)
    }
}
