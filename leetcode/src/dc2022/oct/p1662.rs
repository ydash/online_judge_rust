// 1662. Check If Two String Arrays are Equivalent

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::array_strings_are_equal(
            vec!["ab".to_owned(), "c".to_owned()],
            vec!["a".to_owned(), "bc".to_owned()],
        );
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::array_strings_are_equal(
            vec!["a".to_owned(), "cb".to_owned()],
            vec!["ab".to_owned(), "c".to_owned()],
        );
        assert_eq!(false, result)
    }
}
