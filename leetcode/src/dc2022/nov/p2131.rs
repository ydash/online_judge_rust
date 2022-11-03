// 2131. Longest Palindrome by Concatenating Two Letter Words

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut seen = HashMap::new();
        let mut pair = 0;
        let mut pal = 0;
        for word in words.iter() {
            let rev_word: String = word.chars().rev().collect();
            if let Some(&count) = seen.get(&rev_word) {
                pair += 1;
                if word == &rev_word {
                    pal -= 1;
                }
                if count == 1 {
                    seen.remove(&rev_word);
                } else {
                    *seen.entry(rev_word).or_insert(0) -= 1;
                }
            } else {
                *seen.entry(word.to_owned()).or_insert(0) += 1;
                if word == &rev_word {
                    pal += 1;
                }
            }
        }
        pair * 4 + if pal > 0 { 2 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result =
            Solution::longest_palindrome(vec!["lc".to_owned(), "cl".to_owned(), "gg".to_owned()]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::longest_palindrome(vec![
            "ab".to_owned(),
            "ty".to_owned(),
            "yt".to_owned(),
            "lc".to_owned(),
            "cl".to_owned(),
            "ab".to_owned(),
        ]);
        assert_eq!(8, result)
    }

    #[test]
    fn test_case_03() {
        let result =
            Solution::longest_palindrome(vec!["cc".to_owned(), "ll".to_owned(), "xx".to_owned()]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::longest_palindrome(
            vec![
                "qo", "fo", "fq", "qf", "fo", "ff", "qq", "qf", "of", "of", "oo", "of", "of", "qf",
                "qf", "of",
            ]
            .into_iter()
            .map(|s| s.to_owned())
            .collect(),
        );
        assert_eq!(14, result)
    }
}
