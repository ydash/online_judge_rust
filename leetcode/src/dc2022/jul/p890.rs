// 890. Find and Replace Pattern

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        words
            .into_iter()
            .filter(|word| Self::is_same_pattern(word, &pattern))
            .collect::<_>()
    }

    fn is_same_pattern(s1: &String, s2: &String) -> bool {
        let mut used = vec![false; 26];
        let mut map = vec![None; 26];
        for (b1, b2) in s1.bytes().zip(s2.bytes()) {
            match map[usize::from(b1 - b'a')] {
                Some(mapped) => {
                    if mapped != b2 {
                        return false;
                    }
                }
                None => {
                    if !used[usize::from(b2 - b'a')] {
                        used[usize::from(b2 - b'a')] = true;
                        map[usize::from(b1 - b'a')] = Some(b2)
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn str_vec_2_string_vec(slice: Vec<&str>) -> Vec<String> {
        slice.into_iter().map(|s| s.into()).collect()
    }

    #[test]
    fn test_case_01() {
        let result = Solution::find_and_replace_pattern(
            str_vec_2_string_vec(vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"]),
            "abb".to_string(),
        );
        assert_eq!(str_vec_2_string_vec(vec!["mee", "aqq"]), result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_and_replace_pattern(
            str_vec_2_string_vec(vec!["a", "b", "c"]),
            "a".to_string(),
        );
        assert_eq!(str_vec_2_string_vec(vec!["a", "b", "c"]), result)
    }
}
