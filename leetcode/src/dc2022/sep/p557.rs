// 557. Reverse Words in a String III

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_words(mut s: String) -> String {
        unsafe {
            s.as_bytes_mut()
                .split_mut(u8::is_ascii_whitespace)
                .for_each(|word| word.reverse())
        };
        s
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case_01() {
        let result = Solution::reverse_words("Let's take LeetCode contest".to_owned());
        assert_eq!("s'teL ekat edoCteeL tsetnoc".to_owned(), result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::reverse_words("a b c".to_owned());
        assert_eq!("a b c".to_owned(), result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::reverse_words("abc".to_owned());
        assert_eq!("cba".to_owned(), result)
    }
}
