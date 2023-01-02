// 520. Detect Capital

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn detect_capital_use(word: String) -> bool {
        let count = word
            .as_bytes()
            .iter()
            .filter(|&b| (b'A'..=b'Z').contains(b))
            .count();
        count == 0
            || count == word.len()
            || (count == 1 && (b'A'..=b'Z').contains(&word.as_bytes()[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::detect_capital_use("USA".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::detect_capital_use("Google".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::detect_capital_use("leetcode".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::detect_capital_use("FlaG".to_owned());
        assert_eq!(false, result)
    }
}
