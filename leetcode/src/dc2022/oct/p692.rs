// 692. Top K Frequent Words

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let k = k as usize;
        let mut freq = HashMap::new();
        for word in words.iter() {
            *freq.entry(word).or_insert(0) += 1;
        }

        let mut freq: Vec<_> = freq.iter().collect();
        freq.sort_by(|&(s1, c1), &(s2, c2)| match c2.cmp(c1) {
            less @ std::cmp::Ordering::Less => less,
            std::cmp::Ordering::Equal => s1.cmp(s2),
            greater @ std::cmp::Ordering::Greater => greater,
        });
        freq.iter().take(k).map(|(&s, _)| s.to_owned()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let words = vec!["i", "love", "leetcode", "i", "love", "coding"]
            .iter()
            .map(|&s| s.to_owned())
            .collect();
        let result = Solution::top_k_frequent(words, 2);
        assert_eq!(vec!["i", "love"], result)
    }

    #[test]
    fn test_case_02() {
        let words = vec![
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
        let result = Solution::top_k_frequent(words, 4);
        assert_eq!(vec!["the", "is", "sunny", "day"], result)
    }
}
