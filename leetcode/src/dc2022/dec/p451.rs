// 451. Sort Characters By Frequency

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn frequency_sort(s: String) -> String {
        let mut freq = HashMap::new();
        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        let mut freq = freq.into_iter().collect::<Vec<_>>();
        freq.sort_by(|a, b| b.1.cmp(&a.1));
        dbg!(&freq);
        let mut result = String::new();
        for (c, n) in freq {
            for _ in 0..n {
                result.push(c);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::frequency_sort("cccaaa".to_owned());
        assert!(vec!["aaaccc".to_owned(), "cccaaa".to_owned()].contains(&result))
    }

    #[test]
    fn test_case_02() {
        let result = Solution::frequency_sort("tree".to_owned());
        assert!(vec!["eert".to_owned(), "eetr".to_owned()].contains(&result))
    }
}
