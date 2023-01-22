// 131. Palindrome Partitioning

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(s: &[u8]) -> bool {
            let mut left = 0;
            let mut right = s.len() - 1;
            while left < right {
                if s[left] != s[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true
        }
        fn back_tracking(s: &[u8], acc: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
            if s.is_empty() {
                result.push(acc.clone());
                return;
            }
            for i in 1..=s.len() {
                if is_palindrome(&s[..i]) {
                    acc.push(String::from_utf8(s[..i].to_vec()).unwrap());
                    back_tracking(&s[i..], acc, result);
                    acc.pop();
                }
            }
        }
        let mut result = vec![];
        back_tracking(s.as_bytes(), &mut vec![], &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::partition("aab".to_owned());
        assert_eq!(vec![vec!["a", "a", "b"], vec!["aa", "b"]], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::partition("a".to_owned());
        assert_eq!(vec![vec!["a"]], result)
    }
}
