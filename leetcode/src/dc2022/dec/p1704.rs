// 1704. Determine if String Halves Are Alike

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn halves_are_alike(s: String) -> bool {
        let vowels: HashSet<char> =
            HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let chars = s.chars().collect::<Vec<_>>();
        let half = s.len() / 2;
        (0..half).filter(|&i| vowels.contains(&chars[i])).count()
            == (half..s.len())
                .filter(|&i| vowels.contains(&chars[i]))
                .count()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::halves_are_alike("book".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::halves_are_alike("textbook".to_owned());
        assert_eq!(false, result)
    }
}
