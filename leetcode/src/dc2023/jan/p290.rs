// 290. Word Pattern

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words = s.split(' ').collect::<Vec<_>>();
        if words.len() != pattern.len() {
            return false;
        }
        let mut p2w = vec![""; 26];
        let mut w2p = std::collections::HashMap::new();
        for (i, &b) in pattern.as_bytes().iter().enumerate() {
            let word = words[i];
            let j = usize::from(b - b'a');
            if p2w[j].is_empty() && w2p.get(word).is_none() {
                p2w[j] = word;
                w2p.insert(word, b);
            } else if p2w[j] != word || w2p.get(word).map_or(false, |&p| p != b) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::word_pattern("abba".to_owned(), "dog cat cat fish".to_owned());
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::word_pattern("aaaa".to_owned(), "dog cat cat dog".to_owned());
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::word_pattern("abba".to_owned(), "dog dog dog dog".to_owned());
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::word_pattern("abc".to_owned(), "dog cat dog".to_owned());
        assert_eq!(false, result)
    }
}
