// 433. Minimum Genetic Mutation

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_mutation(start: String, end: String, mut bank: Vec<String>) -> i32 {
        fn is_adj(s1: &str, s2: &str) -> bool {
            let mut count = 0;
            for (c1, c2) in s1.chars().zip(s2.chars()) {
                if c1 != c2 {
                    count += 1;
                }
            }
            count == 1
        }
        bank.push(start.clone());
        let n = bank.len();
        let mut adj = HashMap::new();
        for i in 0..n {
            for j in i + 1..n {
                if is_adj(&bank[i], &bank[j]) {
                    adj.entry(&bank[i]).or_insert(vec![]).push(&bank[j]);
                    adj.entry(&bank[j]).or_insert(vec![]).push(&bank[i]);
                }
            }
        }

        let mut q = VecDeque::new();
        q.push_back(&start);
        let mut seen = HashSet::new();
        let mut count = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let s = q.pop_front().unwrap();
                if *s == end {
                    return count;
                }
                seen.insert(s);
                adj.get(s).map(|v| {
                    v.iter()
                        .filter(|&s2| !seen.contains(s2))
                        .for_each(|&s2| q.push_back(s2))
                });
            }
            count += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_mutation(
            "AACCGGTT".to_owned(),
            "AACCGGTA".to_owned(),
            vec!["AACCGGTA".to_owned()],
        );
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_mutation(
            "AACCGGTT".to_owned(),
            "AAACGGTA".to_owned(),
            vec![
                "AACCGGTA".to_owned(),
                "AACCGCTA".to_owned(),
                "AAACGGTA".to_owned(),
            ],
        );
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_mutation("AACCGGTT".to_owned(), "AACCGGTA".to_owned(), vec![]);
        assert_eq!(-1, result)
    }
}
