// 151. Reverse Words in a String

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::reverse_words("the sky is blue".to_owned());
        assert_eq!("blue is sky the", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::reverse_words("  hello world  ".to_owned());
        assert_eq!("world hello", result)
    }
}
