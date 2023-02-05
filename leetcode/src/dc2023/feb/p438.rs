// 438. Find All Anagrams in a String

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let s_len = s.len();
        let p_len = p.len();
        let mut p_count = [0; 26];
        let mut s_count = [0; 26];
        for &b in p {
            p_count[(b - b'a') as usize] += 1;
        }
        for &b in s.iter().take(p_len) {
            s_count[(b - b'a') as usize] += 1;
        }
        let mut result = vec![];
        let mut left = 0;
        let mut right = p_len;
        while right < s_len {
            if s_count == p_count {
                result.push(left as i32);
            }
            s_count[(s[left] - b'a') as usize] -= 1;
            left += 1;
            s_count[(s[right] - b'a') as usize] += 1;
            right += 1;
        }
        if s_count == p_count {
            result.push(left as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_anagrams("cbaebabacd".to_owned(), "abc".to_owned());
        assert_eq!(vec![0, 6], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_anagrams("abab".to_owned(), "ab".to_owned());
        assert_eq!(vec![0, 1, 2], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_anagrams("aaa".to_owned(), "a".to_owned());
        assert_eq!(vec![0, 1, 2], result)
    }
}
