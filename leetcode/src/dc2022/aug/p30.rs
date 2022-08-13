// 30. Substring with Concatenation of All Words

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        fn check(s: &str, word_len: usize, word_num: usize, count: &HashMap<&str, i32>) -> bool {
            let mut seen = HashMap::new();
            for j in 0..word_num {
                let sub_str = &s[word_len * j..word_len * (j + 1)];
                let c2 = seen.entry(sub_str).or_insert(0);
                if *c2 < *count.get(sub_str).unwrap_or(&0) {
                    *c2 += 1;
                } else {
                    return false;
                }
            }
            true
        }
        let word_len = words[0].len();
        let m = words.len();
        let n = s.len();
        if n < m * word_len {
            return vec![];
        }
        let mut result = vec![];
        let mut count: HashMap<&str, i32> = HashMap::new();
        for word in words.iter() {
            let entry = count.entry(word).or_insert(0);
            *entry += 1;
        }
        for i in 0..=(n - word_len * m) {
            if check(&s[i..], word_len, m, &count) {
                result.push(i as i32);
            }
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
}
