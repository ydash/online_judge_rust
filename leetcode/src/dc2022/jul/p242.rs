// 242. Valid Anagram

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut count = vec![0; 26];
        for b in s.bytes() {
            count[usize::from(b - b'a')] += 1;
        }

        for b in t.bytes() {
            count[usize::from(b - b'a')] -= 1;
        }

        count.iter().all(|&c| c == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_anagram("anagram".to_string(), "nagaram".to_string());
        assert!(result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_anagram("rat".to_string(), "car".to_string());
        assert!(!result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_anagram("a".to_string(), "ab".to_string());
        assert!(!result)
    }
}
