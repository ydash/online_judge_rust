use std::collections::VecDeque;

// 792. Number of Matching Subsequences
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut map = vec![VecDeque::new(); usize::from(b'z' - b'a' + 1)];
        for word in words.iter() {
            let mut iter = word.bytes();
            if let Some(c) = iter.next() {
                map[usize::from(c - b'a')].push_back(iter);
            }
        }
        let mut count = 0;
        for c in s.bytes() {
            let i = usize::from(c - b'a');
            for _ in 0..map[i].len() {
                if let Some(mut iter) = map[i].pop_back() {
                    if let Some(initial) = iter.next() {
                        map[usize::from(initial - b'a')].push_front(iter);
                    } else {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::num_matching_subseq(
            "abcde".to_string(),
            vec!["a", "bb", "bb", "acd", "ace"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::num_matching_subseq(
            "dsahjpjauf".to_string(),
            vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(2, result)
    }
}
