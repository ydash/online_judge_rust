// 1328. Break a Palindrome

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn break_palindrome(mut palindrome: String) -> String {
        if palindrome.len() == 1 {
            return "".to_owned();
        }
        unsafe {
            let bytes = palindrome.as_bytes_mut();
            match bytes.iter().position(|&b| b != b'a') {
                Some(i) if i < bytes.len() / 2 => bytes[i] = b'a',
                _ => bytes[bytes.len() - 1] = b'b',
            }
        }
        palindrome
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::break_palindrome("abccba".to_owned());
        assert_eq!("aaccba", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::break_palindrome("a".to_owned());
        assert_eq!("", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::break_palindrome("aaa".to_owned());
        assert_eq!("aab", result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::break_palindrome("aba".to_owned());
        assert_eq!("abb", result)
    }
}
