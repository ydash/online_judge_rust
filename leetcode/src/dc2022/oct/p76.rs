// 76. Minimum Window Substring

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_window(s: String, t: String) -> String {
        let mut count = HashMap::new();
        for c in t.as_bytes() {
            *count.entry(c).or_insert(0) += 1;
        }
        let mut left = 0;
        let mut right = 0;
        let m = s.len();
        let s_bytes = s.as_bytes();
        let mut result_range = (0, usize::MAX);
        while right < m {
            if count.get(&s_bytes[right]).is_none() {
                right += 1;
                continue;
            }
            *count.get_mut(&s_bytes[right]).unwrap() -= 1;
            while left < right && *count.get(&s_bytes[left]).unwrap_or(&-1) < 0 {
                if let Some(x) = count.get_mut(&s_bytes[left]) {
                    *x += 1;
                }
                left += 1;
            }
            if count.values().all(|&x| x <= 0) && right - left < result_range.1 - result_range.0 {
                result_range = (left, right)
            }
            right += 1;
        }
        if result_range.1 - result_range.0 < s.len() {
            s[result_range.0..=result_range.1].to_owned()
        } else {
            "".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned());
        assert_eq!("BANC", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_window("ADOBECODEBANC".to_owned(), "DE".to_owned());
        assert_eq!("DE", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_window("a".to_owned(), "a".to_owned());
        assert_eq!("a", result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::min_window("a".to_owned(), "aa".to_owned());
        assert_eq!("", result)
    }
}
