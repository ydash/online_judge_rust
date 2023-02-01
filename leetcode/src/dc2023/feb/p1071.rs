// 1071. Greatest Common Divisor of Strings

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        fn is_divided_by(s: &str, t: &str) -> bool {
            if s.len() % t.len() != 0 {
                false
            } else {
                for i in (0..s.len()).step_by(t.len()) {
                    if &s[i..i + t.len()] != t {
                        return false;
                    }
                }
                true
            }
        }
        let mut result = String::new();
        for i in 1..=str1.len() {
            let t = &str1[..i];
            if is_divided_by(&str1, t) && is_divided_by(&str2, t) {
                result = t.to_owned();
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
        let result = Solution::gcd_of_strings("ABCABC".to_owned(), "ABC".to_owned());
        assert_eq!("ABC", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::gcd_of_strings("ABABAB".to_owned(), "ABAB".to_owned());
        assert_eq!("AB", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::gcd_of_strings("LEET".to_owned(), "CODE".to_owned());
        assert_eq!("", result)
    }
}
