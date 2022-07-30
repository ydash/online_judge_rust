// 916. Word Subsets

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut count = vec![0; 26];
        for word in words2.iter() {
            let tmp = Self::count_char_of(word);
            for i in 0..26 {
                count[i] = count[i].max(tmp[i]);
            }
        }
        words1
            .into_iter()
            .filter(|word| {
                let tmp = Self::count_char_of(word);
                (0..26).all(|i| count[i] <= tmp[i])
            })
            .collect()
    }

    fn count_char_of(str: &String) -> Vec<usize> {
        let mut count = vec![0; 26];
        for i in str.bytes().map(|v| usize::from(v - b'a')) {
            count[i] += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::str_vec_2_string_vec;

    #[test]
    fn test_case_01() {
        let result = Solution::word_subsets(
            str_vec_2_string_vec(vec!["amazon", "apple", "facebook", "google", "leetcode"]),
            str_vec_2_string_vec(vec!["e", "o"]),
        );
        let expected = str_vec_2_string_vec(vec!["facebook", "google", "leetcode"]);
        assert_eq!(expected, result)
    }
}
