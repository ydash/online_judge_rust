// 30. Substring with Concatenation of All Words

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let n = s.len();
        let substr_len = words.len() * word_len;
        let mut result = vec![];
        let mut count: HashMap<&str, i32> = HashMap::new();
        for word in words.iter() {
            let entry = count.entry(word).or_insert(0);
            *entry += 1;
        }
        let mut seen: HashMap<&str, i32> = HashMap::new();
        for i in 0..word_len {
            let mut left = i;
            let mut right = left;
            while right + word_len <= n {
                let word = &s[right..right + word_len];
                if count.contains_key(word) {
                    if seen.get(word).unwrap_or(&0) < &count[&word] {
                        *seen.entry(word).or_insert(0) += 1;
                        right += word_len;
                        if right - left == substr_len {
                            result.push(left as i32);
                        }
                    } else {
                        let word = &s[left..left + word_len];
                        *seen.entry(word).or_insert(0) -= 1;
                        left += word_len;
                    }
                } else {
                    seen.clear();
                    left = right + word_len;
                    right = left;
                }
            }
            seen.clear();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_substring(
            String::from("barfoothefoobarman"),
            util::str_vec_2_string_vec(vec!["foo", "bar"]),
        );
        assert_eq!(vec![0, 9], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_substring(
            String::from("wordgoodgoodgoodbestword"),
            util::str_vec_2_string_vec(vec!["word", "good", "best", "word"]),
        );
        assert_eq!(Vec::<i32>::new(), result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_substring(
            String::from("barfoofoobarthefoobarman"),
            util::str_vec_2_string_vec(vec!["foo", "bar", "the"]),
        );
        assert_eq!(vec![6, 9, 12], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::find_substring(
            String::from("wordgoodgoodgoodbestword"),
            util::str_vec_2_string_vec(vec!["word", "good", "best", "good"]),
        );
        assert_eq!(vec![8], result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::find_substring(
            String::from("a"),
            util::str_vec_2_string_vec(vec!["a", "a"]),
        );
        assert_eq!(Vec::<i32>::new(), result)
    }

    #[test]
    fn test_case_06() {
        let result = Solution::find_substring(
            String::from("aaaaaa"),
            util::str_vec_2_string_vec(vec!["aa", "aa"]),
        );
        assert_eq!(vec![0, 2, 1], result)
    }
}
