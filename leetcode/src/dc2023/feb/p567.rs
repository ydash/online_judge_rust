// 567. Permutation in String

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let mut s1_count = [0; 26];
        for &b in s1 {
            s1_count[(b - b'a') as usize] += 1;
        }
        let mut s2_substr_count = [0; 26];
        for &b in s2.iter().take(n) {
            s2_substr_count[(b - b'a') as usize] += 1;
        }
        if s1_count == s2_substr_count {
            return true;
        }
        for i in n..s2.len() {
            s2_substr_count[(s2[i - n] - b'a') as usize] -= 1;
            s2_substr_count[(s2[i] - b'a') as usize] += 1;
            if s1_count == s2_substr_count {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::check_inclusion("ab".to_owned(), "eidbaooo".to_owned());
        assert!(result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::check_inclusion("ab".to_owned(), "eidboaoo".to_owned());
        assert!(!result)
    }
}
